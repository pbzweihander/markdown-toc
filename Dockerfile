FROM rust:1.27

WORKDIR /usr/src/markdown-toc
COPY . /usr/src/markdown-toc

RUN cargo install --path .

ENTRYPOINT [ "markdown-toc" ]
