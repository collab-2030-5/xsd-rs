mod array_of_vavailability_contained_components;
mod available_type;
mod components;
mod dtend;
mod dtstart;
mod duration_prop_type;
mod properties;
mod tolerance_type;
mod tolerate_type;
mod uid;
mod vavailability_type;
mod ws_calendar_interval_type;

// re-export all the types in this namespace

pub use array_of_vavailability_contained_components::*;
pub use available_type::*;
pub use components::*;
pub use dtend::*;
pub use dtstart::*;
pub use duration_prop_type::*;
pub use properties::*;
pub use tolerance_type::*;
pub use tolerate_type::*;
pub use uid::*;
pub use vavailability_type::*;
pub use ws_calendar_interval_type::*;

// helpers specific to this namespace
pub(crate) fn add_schema_attr(
    start: xml::writer::events::StartElementBuilder,
) -> xml::writer::events::StartElementBuilder {
    start
        .attr("xmlns:xsi", "http://www.w3.org/2001/XMLSchema-instance")
        .attr("xmlns:xsd", "http://www.w3.org/2001/XMLSchema")
        .attr("xmlns", "xcal")
}
