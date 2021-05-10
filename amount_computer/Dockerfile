FROM rust:buster as builder

#RUN apt-get update -y

#RUN apt-get install cmake openssl-dev build-base -y

#FROM registry.cn-hongkong.aliyuncs.com/cloud-api/users:dev-builder as builder

WORKDIR /workspace

ADD ./ /workspace

RUN mkdir .cargo && cp cargo_config .cargo/config

RUN CARGO_HTTP_MULTIPLEXING=false cargo build --release

FROM ubuntu:focal

WORKDIR /workspace

RUN apt-get update -y

RUN apt-get install cmake openssl -y

COPY --from=builder /workspace/target/release/amount_computer .

EXPOSE 5000

CMD [ "./amount_computer" ]
