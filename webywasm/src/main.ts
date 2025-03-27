import { WebHandle, StateHandle } from "rustywasm"

// const sleep = async (time: number) => await new Promise(r => setTimeout(r, time));

function pullLocation(stateHandle: StateHandle) {
  navigator.geolocation.watchPosition(pos => stateHandle.add_point(pos.coords.longitude, pos.coords.latitude));
}

async function main() {
  const app = document.getElementById("app");
  const stateHandle = new StateHandle();

  const wh = new WebHandle(stateHandle.clone());

  if (app instanceof HTMLCanvasElement) {
    wh.start(app).catch(console.error);
    //backend.be.start().catch(console.error);
    pullLocation(stateHandle.clone());
  } else {
    console.error("App is not a canvas");
  }
}

main();

