mod intervals;
mod stream_base_type;
mod stream_payload_base;
mod stream_payload_base_type;

// re-export all the types in this namespace

pub use intervals::*;
pub use stream_base_type::*;
pub use stream_payload_base::*;
pub use stream_payload_base_type::*;

// helpers specific to this namespace
pub(crate) fn add_schema_attr(
    start: xml::writer::events::StartElementBuilder,
) -> xml::writer::events::StartElementBuilder {
    start
        .attr("xmlns:xsi", "http://www.w3.org/2001/XMLSchema-instance")
        .attr("xmlns:xsd", "http://www.w3.org/2001/XMLSchema")
        .attr("xmlns", "strm")
}
