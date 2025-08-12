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
    cargo watch -w client/{{NAME}}/ -w shared/{{NAME}}/ -w public -w client/horfimbor-client -- \
        wasm-pack build ./client/{{NAME}} \
          --target web \
          --out-dir ../../server/{{NAME}}/web/client \
          --out-name index-v0-1-0

watch-server NAME PORT:
    cargo watch -w server/{{NAME}}/ -w shared/{{NAME}}/ -w state/{{NAME}}/ -w public -i server/{{NAME}}/web/ -i server/{{NAME}}/templates \
        -x "run -p mono-{{NAME}}-server -- -p{{PORT}} service"

precommit:
    cargo fmt
    cargo clippy -- -D clippy::expect_used -D clippy::panic  -D clippy::unwrap_used
    cargo test

test-mutation:
    cargo test
    cargo mutants -p mono-state