FROM rust:latest as build
MAINTAINER Naftuli Kay <me@naftuli.wtf>

# install system musl tools and development requirements
RUN DEBIAN_FRONTEND=noninteractive apt-get update >/dev/null && \
  DEBIAN_FRONTEND=noninteractive apt-get install -y musl musl-dev musl-tools >/dev/null && \
  rm -fr /var/lib/apt/lists/* && \
  DEBIAN_FRONTEND=noninteractive apt-get clean >/dev/null

# install the rust musl target
RUN rustup target add x86_64-unknown-linux-musl

# install cargo audit to audit our dependencies
RUN cargo install --force cargo-audit

# deploy source files
COPY ./ /usr/src/slumberd

WORKDIR /usr/src/slumberd

RUN cargo audit
RUN cargo test
RUN cargo build --release --target x86_64-unknown-linux-musl
RUN strip target/x86_64-unknown-linux-musl/release/slumberd

# runtime image
FROM scratch
COPY --from=build /usr/src/slumberd/target/x86_64-unknown-linux-musl/release/slumberd /

ENTRYPOINT ["/slumberd"]
