mkdir -p ./chess_engine
mkdir -p ../chess_engine
rm -Rf ./chess_engine
rm -Rf ../chess_engine
wasm-pack build --target web --out-dir ./chess_engine
mv ./chess_engine ../chess_engine