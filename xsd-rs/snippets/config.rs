#[derive(Copy, Clone, Debug)]
pub struct WriteConfig {
    pub write_document_declaration: bool,
}

impl Default for WriteConfig {
    fn default() -> Self {
        Self {
            write_document_declaration: false
        }
    }
}

impl WriteConfig {
    pub(crate) fn to_xml_rs(self) -> xml::EmitterConfig {
        xml::EmitterConfig::new().write_document_declaration(self.write_document_declaration)
    }
}
