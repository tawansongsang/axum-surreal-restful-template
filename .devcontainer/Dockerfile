FROM rust:slim-bookworm
RUN apt update && apt upgrade -y \
    && apt install -y git
RUN rustup component add rustfmt
