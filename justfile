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
    bacon watch-client-{{NAME}}

watch-server NAME PORT:
    bacon watch-server-{{NAME}} -- -- -p{{PORT}} service

precommit:
    cargo fmt
    cargo clippy -- -D clippy::expect_used -D clippy::panic  -D clippy::unwrap_used
    cargo test

test-mutation:
    cargo test
    cargo mutants -p mono-state
