mod oadr_transport_type;
mod oadr_transport_type;
mod oadr_transports;

// re-export all the types in this namespace

pub use oadr_transport_type::*;
pub use oadr_transport_type::*;
pub use oadr_transports::*;

// helpers specific to this namespace
pub(crate) fn add_schema_attr(start: xml::writer::events::StartElementBuilder) -> xml::writer::events::StartElementBuilder {
    start
        .attr("xmlns:xsi", "http://www.w3.org/2001/XMLSchema-instance")
        .attr("xmlns:xsd", "http://www.w3.org/2001/XMLSchema")
        .attr("xmlns", "oadr")
}