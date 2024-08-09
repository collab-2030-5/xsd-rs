WIP XSD -> Rust generator.

The XSD parser and object model was forked from [xsd-parser-rs](https://github.com/lumeohq/xsd-parser-rs). License and attribution have been preserved.

To generate sepxml:
cargo run -- -c ../ieee-2030.5-lib/sep-xml/config.json -i ../ieee-2030.5-lib/sep-xml/sep.xsd -o ../ieee-2030.5-lib/sep-xml/src/generated -r
