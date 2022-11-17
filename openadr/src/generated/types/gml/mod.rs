mod exterior_type;
mod feature_collection;
mod linear_ring_type;
mod location_type;
mod polygon_type;

// re-export all the types in this namespace

pub use exterior_type::*;
pub use feature_collection::*;
pub use linear_ring_type::*;
pub use location_type::*;
pub use polygon_type::*;

// helpers specific to this namespace
pub(crate) fn add_schema_attr(
    start: xml::writer::events::StartElementBuilder,
) -> xml::writer::events::StartElementBuilder {
    start
        .attr("xmlns:xsi", "http://www.w3.org/2001/XMLSchema-instance")
        .attr("xmlns:xsd", "http://www.w3.org/2001/XMLSchema")
        .attr("xmlns", "gml")
}
