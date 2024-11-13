WIP XSD -> Rust generator.

The XSD parser and object model was forked from [xsd-parser-rs](https://github.com/lumeohq/xsd-parser-rs). License and attribution have been preserved.

## Schema specified in the config
cargo run --bin xsd-rs -- generate -c openadr/config.json -o openadr/src/generated/ -r

## Build and format
cargo run --bin xsd-rs -- generate -c openadr/config.json -o openadr/src/generated/oadr2b -r && cd openadr && cargo fmt && cd ../

# Testing the sample
cargo run --bin xsd-rs -- generate -i openadr/testxsd/oadr_20b.xsd -c openadr/config.json -o openadr/testxsd/src/ -r
cargo run --example samples