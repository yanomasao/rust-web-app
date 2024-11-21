# FROM ubuntu:20.04
FROM rust:1.82.0-bookworm

RUN apt-get update && apt-get upgrade -y
RUN apt-get install -y tzdata
RUN apt-get -y install apt-utils
RUN apt-get -y install wget

# Dockerのインストール
RUN wget -qO- https://get.docker.com | sh

RUN apt-get install -y tree

RUN rustup component add rustfmt # なぜこれがrustのイメージに入っていないのか？

RUN cargo install cargo-make
RUN cargo install cargo-nextest # cargo make testすればインストールされるが、ここでインストールしておく
RUN cargo install cargo-watch # cargo make watchすればインストールされるが、ここでインストールしておく
