rm -Rf ./libre_chess_wasm
rm -Rf ../libre_chess_wasm
wasm-pack build --target web --out-dir libre_chess_wasm
mv ./libre_chess_wasm ../libre_chess_wasm