From rust:1.67 as build
WORKDIR /app
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc-debian11
COPY --from=build /app/target/release/axum_change /app/

USER nonroot:nonroot

ENV APP_HOME=/app
EXPOSE 3000

ENTRYPOINT ["/app/axum_change"]