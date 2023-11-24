FROM rust:1.74-alpine3.18 AS builder
# Needed to build some of the dependencies.
RUN apk add --no-cache musl-dev

# Temporary project to build all the dependencies in.
RUN USER=root cargo new --bin /app
WORKDIR /app

# We only need the files that actually are needed to get our dependencies.
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

# Specify that we want to run the project in Alpine.
RUN rustup target add x86_64-unknown-linux-musl

# Build the dependencies and remove everything except for the created binaries.
RUN cargo build --release --target=x86_64-unknown-linux-musl \
    && find . -not -path "./target*" -delete

# Copy all the files this time.
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./src ./src

# Needed to ensure that we rebuild the binary.
RUN touch src/main.rs

# Now actually build our own binary, Pwned Passwords Bloom API.
RUN cargo build --release --target=x86_64-unknown-linux-musl

FROM alpine:3.18 AS api

# Ensure that the correct timezone is present.
RUN apk add --no-cache --virtual .build-deps \
        tzdata \
    && cp /usr/share/zoneinfo/Europe/Amsterdam /etc/localtime \
    && echo 'Europe/Amsterdam' > /etc/timezone \
    && apk del .build-deps \
    && adduser -u 82 -D -S -G www-data www-data

WORKDIR /
COPY --chown=www-data:www-data --from=builder /app/target/x86_64-unknown-linux-musl/release/pwned-passwords-bloom-api .
COPY --chown=www-data:www-data ./Rocket.toml ./Rocket.toml

USER www-data
CMD ["./pwned-passwords-bloom-api"]
