mod aggregated_pnode_type;
mod end_device_asset_type;
mod energy_apparent_type;
mod energy_item;
mod energy_item_type;
mod energy_reactive_type;
mod energy_real_type;
mod meter_asset_type;
mod pnode_type;
mod power_apparent_type;
mod power_attributes_type;
mod power_item;
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
pub use energy_item::*;
pub use energy_item_type::*;
pub use energy_reactive_type::*;
pub use energy_real_type::*;
pub use meter_asset_type::*;
pub use pnode_type::*;
pub use power_apparent_type::*;
pub use power_attributes_type::*;
pub use power_item::*;
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
        .attr("xmlns:xsi", "http://www.w3.org/2001/XMLSchema-instance")
        .attr("xmlns:xsd", "http://www.w3.org/2001/XMLSchema")
        .attr("xmlns", "power")
}
