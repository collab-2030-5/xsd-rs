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
            "xmlns:pyld",
            "http://docs.oasis-open.org/ns/energyinterop/201110/payloads",
        )
        .attr(
            "xmlns:ei",
            "http://docs.oasis-open.org/ns/energyinterop/201110",
        )
        .attr("xmlns:xs", "http://www.w3.org/2001/XMLSchema")
        .attr("xmlns:gmlsf", "http://www.opengis.net/gmlsf/2.0")
        .attr(
            "xmlns:scale",
            "http://docs.oasis-open.org/ns/emix/2011/06/siscale",
        )
        .attr("xmlns:strm", "urn:ietf:params:xml:ns:icalendar-2.0:stream")
        .attr(
            "xmlns:power",
            "http://docs.oasis-open.org/ns/emix/2011/06/power",
        )
        .attr("xmlns:xsd", "http://www.w3.org/2001/XMLSchema")
        .attr("xmlns:oadr", "http://openadr.org/oadr-2.0b/2012/07")
        .attr(
            "xmlns:ccts",
            "urn:un:unece:uncefact:documentation:standard:CoreComponentsTechnicalSpecification:2",
        )
        .attr("xmlns:emix", "http://docs.oasis-open.org/ns/emix/2011/06")
        .attr(
            "xmlns:clm5ISO42173A",
            "urn:un:unece:uncefact:codelist:standard:5:ISO42173A:2010-04-07",
        )
        .attr("xmlns:atom", "http://www.w3.org/2005/Atom")
        .attr("xmlns:xcal", "urn:ietf:params:xml:ns:icalendar-2.0")
        .attr("xmlns:gml", "http://www.opengis.net/gml/3.2")
        .attr("xmlns:gb", "http://naesb.org/espi")
}
