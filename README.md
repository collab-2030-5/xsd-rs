WIP XSD -> Rust generator.

The XSD parser and object model was forked from [xsd-parser-rs](https://github.com/lumeohq/xsd-parser-rs). License and attribution have been preserved.

## Schema specified in the config
cargo run --bin xsd-rs -- generate -c openadr/config.json -o openadr/src/generated/ -r

## Build and format
cargo run --bin xsd-rs -- generate -c openadr/config.json -o openadr/src/generated/oadr20b -r --nsroot oadr20b:: && cd openadr && cargo fmt && cd ../

# Testing the sample
cargo run --bin xsd-rs -- generate -i openadr/testxsd/oadr_20b.xsd -c openadr/config.json -o openadr/testxsd/src/ -r
cargo run --example samples

## Build and format 2.0a
cargo run --bin xsd-rs -- generate -c openadr/config_20a.json -o openadr/src/generated/oadr20a -r --nsroot oadr20a:: && cd openadr && cargo fmt && cd ../

## Changelog
* Add optional namespace (--nsroot ns::).  Name must end w double colons (::)
* Move 2.0b generated files under 20b directory.  Generate 2.0a files.