FROM rust:1.43.1 as build-env
WORKDIR /app
ADD . /app
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=build-env /app/target/release/tide-api /
EXPOSE 8090
CMD ["./tide-api"]