#/bin/sh

cd native && wasm-pack build --target web && cd ..
cd web && yarn add file:../native/pkg && yarn build
