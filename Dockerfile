FROM rust:1
WORKDIR /usr/src/markdown-toc
COPY . .
RUN cargo build --release

FROM bitnami/minideb:stretch
COPY --from=0 /usr/src/markdown-toc/target/release/md-toc /usr/bin/md-toc

ENTRYPOINT [ "md-toc" ]
