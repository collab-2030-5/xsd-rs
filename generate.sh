cargo run --bin xsd-rs -- generate -c openadr/config.json -o openadr/src/generated/oadr20b -r --nsroot oadr20b:: && cd openadr && cargo fmt && cd ../
cargo run --bin xsd-rs -- generate -c openadr/config_20a.json -o openadr/src/generated/oadr20a -r --nsroot oadr20a:: && cd openadr && cargo fmt && cd ../

cargo run --example samples
