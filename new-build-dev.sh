cd $(dirname $0)
cd ./rustywasm
echo "BUILD"
wasm-pack build --dev
cd ../webywasm/
echo "Clear yarn"
rm -rf ./node_modules/
yarn
echo "Run"
yarn dev


