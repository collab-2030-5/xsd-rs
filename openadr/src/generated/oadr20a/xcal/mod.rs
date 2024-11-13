mod components;
mod dtstart;
mod duration_prop_type;
mod properties;
mod tolerance;
mod tolerate;
mod uid;

// re-export all the types in this namespace

pub use components::*;
pub use dtstart::*;
pub use duration_prop_type::*;
pub use properties::*;
pub use tolerance::*;
pub use tolerate::*;
pub use uid::*;

// helpers specific to this namespace
pub(crate) fn add_schema_attr(
    start: xml::writer::events::StartElementBuilder,
) -> xml::writer::events::StartElementBuilder {
    start
        .attr(
            "xmlns:ei",
            "http://docs.oasis-open.org/ns/energyinterop/201110",
        )
        .attr("xmlns:emix", "http://docs.oasis-open.org/ns/emix/2011/06")
        .attr("xmlns:oadr", "http://openadr.org/oadr-2.0a/2012/07")
        .attr(
            "xmlns:pyld",
            "http://docs.oasis-open.org/ns/energyinterop/201110/payloads",
        )
        .attr("xmlns:strm", "urn:ietf:params:xml:ns:icalendar-2.0:stream")
        .attr("xmlns:xcal", "urn:ietf:params:xml:ns:icalendar-2.0")
        .attr("xmlns:xs", "http://www.w3.org/2001/XMLSchema")
}
