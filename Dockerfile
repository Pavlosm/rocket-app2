FROM messense/rust-musl-cross:x86_64-musl as builder
WORKDIR /rocket-app2
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl
COPY Rocket.toml /rocket-app2/target/x86_64-unknown-linux-musl/release
#COPY /rocket-app2/target/x86_64-unknown-linux-musl/release/rocket-app2 /rocket-app2
ENTRYPOINT [ "/rocket-app2/target/x86_64-unknown-linux-musl/release/rocket-app2" ]
EXPOSE 8020

FROM scratch
COPY --from=builder /rocket-app2/target/x86_64-unknown-linux-musl/release/rocket-app2 /rocket-app2
COPY --from=builder /rocket-app2/Rocket.toml /Rocket.toml
ENTRYPOINT [ "/rocket-app2" ]
EXPOSE 8020