mod arrayof_responses;
mod current_value_type;
mod current_value_type_choice;
mod ei_active_period_type;
mod ei_event_baseline_type;
mod ei_event_signal_type;
mod ei_event_signals_type;
mod ei_event_type;
mod ei_market_context;
mod ei_opt_type;
mod ei_response_type;
mod ei_target_type;
mod event_descriptor_type;
mod event_response;
mod event_responses;
mod event_status_enumerated_type;
mod interval_type;
mod opt_reason_enumerated_type;
mod opt_type_type;
mod payload_base_type;
mod payload_float_type;
mod qualified_event_id_type;
mod ref_id;
mod registration_id;
mod report_enumerated_type;
mod report_payload_type;
mod report_specifier_type;
mod signal_payload_type;
mod signal_type_enumerated_type;
mod specifier_payload_type;

// re-export all the types in this namespace

pub use arrayof_responses::*;
pub use current_value_type::*;
pub use current_value_type_choice::*;
pub use ei_active_period_type::*;
pub use ei_event_baseline_type::*;
pub use ei_event_signal_type::*;
pub use ei_event_signals_type::*;
pub use ei_event_type::*;
pub use ei_market_context::*;
pub use ei_opt_type::*;
pub use ei_response_type::*;
pub use ei_target_type::*;
pub use event_descriptor_type::*;
pub use event_response::*;
pub use event_responses::*;
pub use event_status_enumerated_type::*;
pub use interval_type::*;
pub use opt_reason_enumerated_type::*;
pub use opt_type_type::*;
pub use payload_base_type::*;
pub use payload_float_type::*;
pub use qualified_event_id_type::*;
pub use ref_id::*;
pub use registration_id::*;
pub use report_enumerated_type::*;
pub use report_payload_type::*;
pub use report_specifier_type::*;
pub use signal_payload_type::*;
pub use signal_type_enumerated_type::*;
pub use specifier_payload_type::*;

// helpers specific to this namespace
pub(crate) fn add_schema_attr(
    start: xml::writer::events::StartElementBuilder,
) -> xml::writer::events::StartElementBuilder {
    start
        .attr("xmlns:xsi", "http://www.w3.org/2001/XMLSchema-instance")
        .attr("xmlns:xsd", "http://www.w3.org/2001/XMLSchema")
        .attr("xmlns", "ei")
}
