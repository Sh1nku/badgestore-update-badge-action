FROM rust:1.69 as build
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

# create a new empty shell project
RUN USER=root cargo new --bin badgestore-update-badge-action
WORKDIR /badgestore-update-badge-action

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN ls -la ./target/release/deps/
RUN rm ./target/release/deps/badgestore_update_badge_action*
RUN cargo build --release


FROM gcr.io/distroless/cc AS runtime
COPY --from=build /badgestore-update-badge-action/target/release/badgestore-update-badge-action .
ENTRYPOINT ["/badgestore-update-badge-action"]