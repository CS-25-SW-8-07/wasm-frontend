use eframe::{
    egui::{Color32, Painter, Pos2, Sense, Shape, Stroke, Ui, Vec2},
    epaint::{CircleShape, PathShape, PathStroke},
};
use geo_types::{Coord, LineString, coord};

use crate::state::StateHandle;

pub fn map(ui: &mut Ui, state: StateHandle) {
    let t = vec![
        coord! {x:57.011908, y:9.990876},
        coord! {x:57.012704, y:9.991820},
        coord! {x:57.011749, y:9.991776},
    ];
    let w = ui.available_width();
    let h = ui.available_height();

    let (responce, painter) = ui.allocate_painter(Vec2::new(w, h), Sense::drag());
    let mut transform = state.transform.lock().unwrap();

    if let Some(multi_touch) = responce.ctx.multi_touch() {
        transform.scale(multi_touch.zoom_delta);
        transform.rotate(multi_touch.rotation_delta);
    }

    //let user_location = state.user_location.lock().unwrap();
    let user_location = t;
    let Some(current_coord) = user_location.last() else {
        return;
    };

    let current = glam::Vec2::new(current_coord.x as f32, current_coord.y as f32);

    let transform = move |cord: &Coord<f64>| -> Pos2 {
        let coord = glam::Vec2::new(cord.x as f32, cord.y as f32);
        let pos = coord - current;
        let pos = pos + transform.translate;
        let pos = transform.scale_rotate() * pos;
        let pos = pos + glam::Vec2::new(w / 2.0, h / 2.0);

        Pos2 { x: pos.x, y: pos.y }
    };

    painter.add(Shape::Circle(CircleShape {
        center: transform(current_coord),
        radius: 10.0,
        fill: Color32::GOLD,
        stroke: Stroke::NONE,
    }));

    if user_location.len() > 1 {
        let points = user_location.iter().map(transform).collect();

        painter.add(Shape::Path(PathShape {
            points,
            closed: false,
            fill: Color32::TRANSPARENT,
            stroke: PathStroke::new(3.0, Color32::BLUE),
        }));
    }

    let roads = state.road_index.lock().unwrap();
    for road in roads {
        draw_linestring(
            &painter,
            &road,
            &transform,
            PathStroke::new(1.0, Color32::GRAY),
        );
    }
}

fn draw_linestring<F: Fn(&Coord) -> Pos2>(
    painter: &Painter,
    ls: &LineString<f64>,
    transform: &F,
    stroke: PathStroke,
) {
    let points = ls.coords().map(transform).collect();
    painter.add(Shape::Path(PathShape {
        points,
        closed: false,
        fill: Color32::TRANSPARENT,
        stroke,
    }));
}
