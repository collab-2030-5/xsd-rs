mod base_unit_type;
mod currency_item_description_type;
mod currency_type;
mod current_type;
mod frequency_type;
mod oadr_cancel_opt_type;
mod oadr_cancel_party_registration_type;
mod oadr_cancel_report_type;
mod oadr_canceled_opt_type;
mod oadr_canceled_party_registration_type;
mod oadr_canceled_report_type;
mod oadr_create_opt_type;
mod oadr_create_party_registration_type;
mod oadr_create_report_type;
mod oadr_created_event_type;
mod oadr_created_opt_type;
mod oadr_created_party_registration_type;
mod oadr_created_report_type;
mod oadr_distribute_event_type;
mod oadr_event;
mod oadr_extension;
mod oadr_extensions;
mod oadr_info;
mod oadr_load_control_state_type;
mod oadr_load_control_state_type_type;
mod oadr_payload;
mod oadr_payload_resource_status_type;
mod oadr_pending_reports_type;
mod oadr_poll_type;
mod oadr_profile;
mod oadr_profiles;
mod oadr_query_registration_type;
mod oadr_register_report_type;
mod oadr_registered_report_type;
mod oadr_report_description_type;
mod oadr_report_payload_type;
mod oadr_report_request_type;
mod oadr_report_type;
mod oadr_request_event_type;
mod oadr_request_reregistration_type;
mod oadr_response_type;
mod oadr_sampling_rate_type;
mod oadr_service;
mod oadr_service_name_type;
mod oadr_service_specific_info;
mod oadr_signed_object;
mod oadr_signed_object_choice;
mod oadr_transport;
mod oadr_transport_type;
mod oadr_transports;
mod oadr_update_report_type;
mod oadr_updated_report_type;
mod pulse_count_type;
mod response_required_type;
mod temperature_type;
mod temperature_unit_type;
mod therm_type;

// re-export all the types in this namespace

pub use base_unit_type::*;
pub use currency_item_description_type::*;
pub use currency_type::*;
pub use current_type::*;
pub use frequency_type::*;
pub use oadr_cancel_opt_type::*;
pub use oadr_cancel_party_registration_type::*;
pub use oadr_cancel_report_type::*;
pub use oadr_canceled_opt_type::*;
pub use oadr_canceled_party_registration_type::*;
pub use oadr_canceled_report_type::*;
pub use oadr_create_opt_type::*;
pub use oadr_create_party_registration_type::*;
pub use oadr_create_report_type::*;
pub use oadr_created_event_type::*;
pub use oadr_created_opt_type::*;
pub use oadr_created_party_registration_type::*;
pub use oadr_created_report_type::*;
pub use oadr_distribute_event_type::*;
pub use oadr_event::*;
pub use oadr_extension::*;
pub use oadr_extensions::*;
pub use oadr_info::*;
pub use oadr_load_control_state_type::*;
pub use oadr_load_control_state_type_type::*;
pub use oadr_payload::*;
pub use oadr_payload_resource_status_type::*;
pub use oadr_pending_reports_type::*;
pub use oadr_poll_type::*;
pub use oadr_profile::*;
pub use oadr_profiles::*;
pub use oadr_query_registration_type::*;
pub use oadr_register_report_type::*;
pub use oadr_registered_report_type::*;
pub use oadr_report_description_type::*;
pub use oadr_report_payload_type::*;
pub use oadr_report_request_type::*;
pub use oadr_report_type::*;
pub use oadr_request_event_type::*;
pub use oadr_request_reregistration_type::*;
pub use oadr_response_type::*;
pub use oadr_sampling_rate_type::*;
pub use oadr_service::*;
pub use oadr_service_name_type::*;
pub use oadr_service_specific_info::*;
pub use oadr_signed_object::*;
pub use oadr_signed_object_choice::*;
pub use oadr_transport::*;
pub use oadr_transport_type::*;
pub use oadr_transports::*;
pub use oadr_update_report_type::*;
pub use oadr_updated_report_type::*;
pub use pulse_count_type::*;
pub use response_required_type::*;
pub use temperature_type::*;
pub use temperature_unit_type::*;
pub use therm_type::*;

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
