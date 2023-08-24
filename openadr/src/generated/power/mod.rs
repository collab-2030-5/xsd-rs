mod aggregated_pnode_type;
mod end_device_asset_type;
mod energy_apparent_type;
mod energy_item_type;
mod energy_reactive_type;
mod energy_real_type;
mod meter_asset_type;
mod pnode_type;
mod power_apparent_type;
mod power_attributes_type;
mod power_item_type;
mod power_reactive_type;
mod power_real_type;
mod service_delivery_point_type;
mod service_location_type;
mod transport_interface_type;
mod voltage_type;

// re-export all the types in this namespace

pub use aggregated_pnode_type::*;
pub use end_device_asset_type::*;
pub use energy_apparent_type::*;
pub use energy_item_type::*;
pub use energy_reactive_type::*;
pub use energy_real_type::*;
pub use meter_asset_type::*;
pub use pnode_type::*;
pub use power_apparent_type::*;
pub use power_attributes_type::*;
pub use power_item_type::*;
pub use power_reactive_type::*;
pub use power_real_type::*;
pub use service_delivery_point_type::*;
pub use service_location_type::*;
pub use transport_interface_type::*;
pub use voltage_type::*;

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
