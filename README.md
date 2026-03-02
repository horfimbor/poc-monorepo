# horfimbor/poc-monorepo

this repository is a monorepo as a poc for the horfimbor game

after the poc phase, this repo will most likely be broken into pieces and archieved.

you can find what this repo must be doing in [game-rules](doc/game-rules.md)

## development : 

install [Rust](https://rustup.rs/)

if you don't have the db installed :
install [Docker](https://www.docker.com/)

you should also create a networks: 
```shell
docker network create common_network
```

install necessary toolchain : 
```shell
rustup toolchain install beta
rustup target add wasm32-unknown-unknown 
```

install tools : 
```shell 
cargo install just
cargo install wasm-pack
cargo install bacon
cargo install cargo-mutants
```

create the client :
```shell
just watch-client
```

start the server :
```shell 
just watch-server
```

```shell
cargo build -p mono-server
```

before any commit please run the following : 

```shell
just precommit
```
