mod exterior;
mod feature_collection;
mod linear_ring;
mod location;
mod polygon;

// re-export all the types in this namespace

pub use exterior::*;
pub use feature_collection::*;
pub use linear_ring::*;
pub use location::*;
pub use polygon::*;

// helpers specific to this namespace
pub(crate) fn add_schema_attr(
    start: xml::writer::events::StartElementBuilder,
) -> xml::writer::events::StartElementBuilder {
    start
        .attr("xmlns:xsi", "http://www.w3.org/2001/XMLSchema-instance")
        .attr("xmlns:xsd", "http://www.w3.org/2001/XMLSchema")
        .attr("xmlns", "gml")
}
