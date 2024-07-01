FROM rust:1.78.0 as build
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/api-service
COPY . .

RUN cargo install --path .

FROM gcr.io/distroless/cc-debian10

COPY --from=build /usr/local/cargo/bin/api-service /usr/local/bin/api-service

CMD ["api-service"]