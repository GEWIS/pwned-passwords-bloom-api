FROM rust:1.65-alpine3.16 AS builder
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

# Now actually build our own binary, Pwned Passwords Bloom API.
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./src ./src
RUN cargo build --release --target=x86_64-unknown-linux-musl

FROM alpine:3.16

ENV ROCKET_PROFILE="release" \
    ROCKET_ADDRESS=0.0.0.0 \
    ROCKET_PORT=8000

# Ensure that the correct timezone is present.
RUN apk add --no-cache --virtual .build-deps \
        tzdata \
    && cp /usr/share/zoneinfo/Europe/Amsterdam /etc/localtime \
    && echo 'Europe/Amsterdam' > /etc/timezone \
    && apk del .build-deps

WORKDIR /
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/pwned-passwords-bloom-api .

CMD ["./pwned-passwords-bloom-api"]
