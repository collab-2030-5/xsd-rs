mod canonicalization_method_type;
mod digest_method_type;
mod dsa_key_value_type;
mod key_info_type;
mod key_info_type_choice;
mod key_value_type;
mod key_value_type_choice;
mod manifest_type;
mod object_type;
mod pgp_data_type;
mod reference_type;
mod retrieval_method_type;
mod rsa_key_value_type;
mod signature_method_type;
mod signature_properties_type;
mod signature_property_type;
mod signature_type;
mod signature_value_type;
mod signed_info_type;
mod spki_data_type;
mod transform_type;
mod transform_type_choice;
mod transforms_type;
mod x509_data_type;
mod x509_data_type_choice;
mod x509_issuer_serial_type;

// re-export all the types in this namespace

pub use canonicalization_method_type::*;
pub use digest_method_type::*;
pub use dsa_key_value_type::*;
pub use key_info_type::*;
pub use key_info_type_choice::*;
pub use key_value_type::*;
pub use key_value_type_choice::*;
pub use manifest_type::*;
pub use object_type::*;
pub use pgp_data_type::*;
pub use reference_type::*;
pub use retrieval_method_type::*;
pub use rsa_key_value_type::*;
pub use signature_method_type::*;
pub use signature_properties_type::*;
pub use signature_property_type::*;
pub use signature_type::*;
pub use signature_value_type::*;
pub use signed_info_type::*;
pub use spki_data_type::*;
pub use transform_type::*;
pub use transform_type_choice::*;
pub use transforms_type::*;
pub use x509_data_type::*;
pub use x509_data_type_choice::*;
pub use x509_issuer_serial_type::*;

// helpers specific to this namespace
pub(crate) fn add_schema_attr(
    start: xml::writer::events::StartElementBuilder,
) -> xml::writer::events::StartElementBuilder {
    start
        .attr("xmlns:xsi", "http://www.w3.org/2001/XMLSchema-instance")
        .attr("xmlns:xsd", "http://www.w3.org/2001/XMLSchema")
        .attr("xmlns", "ds")
}
