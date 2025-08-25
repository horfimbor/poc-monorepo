set shell := ["bash", "-uc"]
set dotenv-load


alias dc-up := dc-start
dc-start *SRV:
    docker compose up -d --build --force-recreate --remove-orphans {{SRV}}
    docker compose logs --follow {{SRV}}

alias dc-down := dc-stop
dc-stop:
    docker compose down --remove-orphans

dc-reset:
    docker compose down -v
    just dc-start


alias ff := open
open:
    firefox $APP_HOST

watch-client NAME:
    reset
    cargo watch -w {{NAME}}/client/ -w {{NAME}}/shared/ -w public -- \
        wasm-pack build ./{{NAME}}/client \
          --target web \
          --out-dir ../../{{NAME}}/server/web/client \
          --out-name index-v0-1-0

watch-server NAME PORT:
    reset
    cargo watch -w {{NAME}}/server/ -w {{NAME}}/shared/ -w {{NAME}}/state/ -w public -i {{NAME}}/server/web/ -i {{NAME}}/server/templates \
        -x "run -p mono-{{NAME}}-server -- -p{{PORT}} service"

precommit:
    cargo fmt
    cargo clippy -- -D clippy::expect_used -D clippy::panic  -D clippy::unwrap_used
    cargo test

test-mutation:
    cargo test
    cargo mutants -p mono-state