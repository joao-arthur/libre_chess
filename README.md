# libre_chess

A **FLOSS** implementation of _Chess_.

## Architecture

### lib

A **reusable**, **generic** implementation of _Chess_ in _Rust_.

### web_backend

A _Web Assembly_ application that works as a bridge between _lib_ and _web_frontend_, using **wasm-pack**.

### web_frontend

A minimal user application, responsible for render the canvas and init the _Web Assembly_, using **NextJS**.


## Roadmap

- [ ] All rules of chess
- [ ] Game working from begin to end 
- [ ] Better UI

## Run it

```sh
cd ./web_backend
sh ./build.sh
cd ../web_frontend
pnpm install
pnpm dev
```
