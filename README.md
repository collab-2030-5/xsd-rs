WIP XSD -> Rust generator.

The XSD parser and object model was forked from [xsd-parser-rs](https://github.com/lumeohq/xsd-parser-rs). License and attribution have been preserved.

# Not referencing any xmldsig or oar_xml
cargo run --bin xsd-rs -- generate -i openadr/xsd/oadr_20b.xsd -i openadr/xsd/oadr_ei_20b.xsd -i openadr/xsd/oadr_emix_20b.xsd -i openadr/xsd/oadr_gml_20b.xsd -i openadr/xsd/oadr_ISO_ISO3AlphaCurrencyCode_20100407.xsd -i openadr/xsd/oadr_power_20b.xsd -i openadr/xsd/oadr_pyld_20b.xsd -i openadr/xsd/oadr_siscale_20b.xsd -i openadr/xsd/oadr_strm_20b.xsd -i openadr/xsd/oadr_xcal_20b.xsd -c openadr/config.json -o openadr/src/generated/ -