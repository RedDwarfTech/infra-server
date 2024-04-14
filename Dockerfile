ARG BASE_IMAGE=dolphinjiang/rust-musl-builder:latest
FROM ${BASE_IMAGE} AS builder
ADD --chown=rust:rust . ./

FROM alpine:3.18.2
LABEL maintainer="jiangtingqiang@gmail.com"
WORKDIR /app
ENV ROCKET_ADDRESS=0.0.0.0
COPY --from=builder /home/rust/src/settings.toml /app
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/infra-server /app/
RUN apk update && apk add curl websocat zlib zlib-dev openssl-dev openssl tzdata musl-locales
ENV TZ=Asia/Shanghai
RUN cp /usr/share/zoneinfo/Asia/Shanghai /etc/localtime
RUN echo 'export LC_ALL=en_GB.UTF-8' >> /etc/profile.d/locale.sh && \
  sed -i 's|LANG=C.UTF-8|LANG=en_GB.UTF-8|' /etc/profile.d/locale.sh
CMD ["./infra-server"]