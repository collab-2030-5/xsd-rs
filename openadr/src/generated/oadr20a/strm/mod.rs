mod intervals;
mod stream_base_type;
mod stream_payload_base_type;

// re-export all the types in this namespace

pub use intervals::*;
pub use stream_base_type::*;
pub use stream_payload_base_type::*;

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
