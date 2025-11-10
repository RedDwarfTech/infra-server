# build in China mainland
ARG BASE_IMAGE=dolphinjiang/rust-musl-builder:1.86.0
FROM ${BASE_IMAGE} AS builder
ADD --chown=rust:rust . ./
RUN sudo apt-get install \
    postgresql-dev \
    krb5-dev \
    openldap-dev \
    openssl-dev
RUN cargo build --release

FROM alpine:3.22.1
LABEL maintainer="jiangtingqiang@gmail.com"
WORKDIR /app
ENV ROCKET_ADDRESS=0.0.0.0
COPY --from=builder /home/rust/src/settings.toml /app
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/infra-server /app
COPY --from=builder /home/rust/src/log4rs.yaml /app
ENV TZ=Asia/Shanghai
# RUN cp /usr/share/zoneinfo/Asia/Shanghai /etc/localtime
RUN apk update && apk add curl websocat zlib zlib-dev openssl-dev openssl tzdata musl-locales
CMD ["./infra-server"]