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
        .attr("xmlns:atom", "http://www.w3.org/2005/Atom")
        .attr(
            "xmlns:ccts",
            "urn:un:unece:uncefact:documentation:standard:CoreComponentsTechnicalSpecification:2",
        )
        .attr(
            "xmlns:clm5ISO42173A",
            "urn:un:unece:uncefact:codelist:standard:5:ISO42173A:2010-04-07",
        )
        .attr(
            "xmlns:ei",
            "http://docs.oasis-open.org/ns/energyinterop/201110",
        )
        .attr("xmlns:emix", "http://docs.oasis-open.org/ns/emix/2011/06")
        .attr("xmlns:gb", "http://naesb.org/espi")
        .attr("xmlns:gml", "http://www.opengis.net/gml/3.2")
        .attr("xmlns:gmlsf", "http://www.opengis.net/gmlsf/2.0")
        .attr("xmlns:oadr", "http://openadr.org/oadr-2.0b/2012/07")
        .attr(
            "xmlns:power",
            "http://docs.oasis-open.org/ns/emix/2011/06/power",
        )
        .attr(
            "xmlns:pyld",
            "http://docs.oasis-open.org/ns/energyinterop/201110/payloads",
        )
        .attr(
            "xmlns:scale",
            "http://docs.oasis-open.org/ns/emix/2011/06/siscale",
        )
        .attr("xmlns:strm", "urn:ietf:params:xml:ns:icalendar-2.0:stream")
        .attr("xmlns:xcal", "urn:ietf:params:xml:ns:icalendar-2.0")
        .attr("xmlns:xs", "http://www.w3.org/2001/XMLSchema")
        .attr("xmlns:xsd", "http://www.w3.org/2001/XMLSchema")
}
