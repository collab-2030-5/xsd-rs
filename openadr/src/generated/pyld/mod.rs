mod ei_created_event;
mod ei_request_event;

// re-export all the types in this namespace

pub use ei_created_event::*;
pub use ei_request_event::*;

// helpers specific to this namespace
pub(crate) fn add_schema_attr(
    start: xml::writer::events::StartElementBuilder,
) -> xml::writer::events::StartElementBuilder {
    start
        .attr("xmlns:xsi", "http://www.w3.org/2001/XMLSchema-instance")
        .attr("xmlns:xsd", "http://www.w3.org/2001/XMLSchema")
        .attr("xmlns", "pyld")
}
