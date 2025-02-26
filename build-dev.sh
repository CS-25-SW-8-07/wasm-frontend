cd "$(dirname "$0")"
cd ./rustywasm
wasm-pack build --dev
cd ../webywasm/
yarn install ../rustywasm/pkg/ --check-files
yarn dev


