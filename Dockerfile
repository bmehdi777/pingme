FROM rust:1.75.0 as builder
WORKDIR app
COPY . . 
RUN ls configuration
RUN cargo build --release --bin ping

FROM rust:1.75.0 as runtime
WORKDIR app
COPY --from=builder /app/target/release/ping ./
COPY --from=builder /app/assets ./assets/
COPY --from=builder /app/src/templates ./src/templates/
COPY --from=builder /app/configuration/base.yml ./configuration/
COPY --from=builder /app/configuration/production.yml ./configuration/
EXPOSE 8000
ENV APP_ENVIRONMENT=production
ENTRYPOINT ["/app/ping"]
