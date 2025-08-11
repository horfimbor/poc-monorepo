# syntax=docker/dockerfile:1

# Comments are provided throughout this file to help you get started.
# If you need more help, visit the Dockerfile reference guide at
# https://docs.docker.com/engine/reference/builder/

################################################################################
# Create a stage for building the application.

ARG RUST_VERSION=1.89.0
FROM rust:${RUST_VERSION}-slim-trixie AS build
WORKDIR /app

RUN apt update && apt install -y pkg-config ca-certificates libsasl2-dev libssl-dev


RUN cargo install wasm-pack

ARG WASM_TAG=v0.1.0
ENV WASM_TAG=${WASM_TAG}

ARG APP_NAME="NONE"
ENV APP_NAME=${APP_NAME}

# Build the application.
# Leverage a cache mount to /usr/local/cargo/registry/
# for downloaded dependencies and a cache mount to /app/target/ for
# compiled dependencies which will speed up subsequent builds.
# Leverage a bind mount to the src directory to avoid having to copy the
# source code into the container. Once built, copy the executable to an
# output directory before the cache mounted /app/target is unmounted.
RUN --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=bind,source=client,target=client \
    --mount=type=bind,source=server,target=server \
    --mount=type=bind,source=shared,target=shared \
    --mount=type=bind,source=state,target=state \
    --mount=type=bind,source=public,target=public \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
<<EOF
#!/bin/bash

set -e
cargo build --locked --release --bin mono-$APP_NAME-server
cp ./target/release/mono-$APP_NAME-server /bin/mono-$APP_NAME-server

WASM_TAG=${WASM_TAG//./-}

wasm-pack build ./client/$APP_NAME/ --target web --out-dir /bin/webcomponent/ --out-name index-$WASM_TAG

EOF


################################################################################
# Create a new stage for running the application that contains the minimal
# runtime dependencies for the application. This often uses a different base
# image from the build stage where the necessary files are copied from the build
# stage.
#
# The example below uses the debian trixie image as the foundation for running the app.
# By specifying the "trixie-slim" tag, it will also use whatever happens to be the
# most recent version of that tag when you build your Dockerfile. If
# reproducability is important, consider using a digest
# (e.g., debian@sha256:ac707220fbd7b67fc19b112cee8170b41a9e97f703f588b2cdbbcdcecdd8af57).
FROM debian:trixie-slim AS final-server

# add certificate
RUN apt update \
    && apt install -y ca-certificates libsasl2-dev libssl-dev adduser
RUN update-ca-certificates

RUN mkdir -p /app/server/web/

# Create a non-privileged user that the app will run under.
# See https://docs.docker.com/develop/develop-images/dockerfile_best-practices/#user
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

ARG APP_NAME="NONE"

# Copy the executable from the "build" stage.
COPY --from=build /bin/mono-$APP_NAME-server /bin/
COPY --from=build /bin/webcomponent/ /app/server/$APP_NAME/web/client/
COPY server/$APP_NAME/templates /app/server/$APP_NAME/templates/

ARG APP_PORT=8000

# Expose the port that the application listens on.
EXPOSE $APP_PORT

WORKDIR /app

# doenst work
#ARG COMMAND="/bin/mono-$APP_NAME-server --real-env service"
#
## What the container should run when it is started.
#CMD $COMMAND
