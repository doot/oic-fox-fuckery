FROM ghcr.io/cachix/devenv/devenv:latest AS builder

  WORKDIR /usr/src/

  COPY . .

  RUN devenv build

FROM debian:bookworm-slim

  WORKDIR /usr/app

  COPY --from=builder /usr/src/config config
  COPY --from=builder /usr/src/target/release/oic-fox-fuckery-cli oic-fox-fuckery-cli

  ENTRYPOINT ["/usr/app/oic-fox-fuckery-cli", "start", "-e", "production"]
