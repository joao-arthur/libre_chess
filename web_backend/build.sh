mkdir -p ./libre_chess_engine
mkdir -p ../libre_chess_engine
rm -Rf ./libre_chess_engine
rm -Rf ../libre_chess_engine
wasm-pack build --target web --out-dir ./libre_chess_engine
mv ./libre_chess_engine ../libre_chess_engine