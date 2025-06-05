# Use the specified Rust version and base OS as the base image
ARG BASE_OS
ARG RUST_VERSION
FROM rust:${RUST_VERSION}-${BASE_OS}

RUN apt update

# install some usefull tools/shortcuts

RUN apt install tree -y
RUN apt install curl -y

# create container user with same rights than host user

RUN groupadd -g 1000 appuser && \
    useradd -m -u 1000 -g 1000 -s /bin/bash appuser

# global configuration done ! let's focus on the project itself

WORKDIR /app
RUN chown -R appuser:appuser /app

USER appuser