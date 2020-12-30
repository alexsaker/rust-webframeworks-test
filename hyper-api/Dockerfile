FROM rust:1.43.1 as build-env
WORKDIR /app
ADD . /app
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=build-env /app/target/release/hyper-api /
EXPOSE 3000
CMD ["./hyper-api"]