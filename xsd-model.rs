pub struct DeviceCapability {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from FunctionSetAssignmentsBase --- 

  customer_account_list_link: CustomerAccountListLink,
  demand_response_program_list_link: DemandResponseProgramListLink,
  der_program_list_link: DERProgramListLink,
  file_list_link: FileListLink,
  messaging_program_list_link: MessagingProgramListLink,
  prepayment_list_link: PrepaymentListLink,
  response_set_list_link: ResponseSetListLink,
  tariff_profile_list_link: TariffProfileListLink,
  time_link: TimeLink,
  usage_point_list_link: UsagePointListLink,


  // --- these fields come from DeviceCapability --- 

  end_device_list_link: EndDeviceListLink,
  mirror_usage_point_list_link: MirrorUsagePointListLink,
  self_device_link: SelfDeviceLink,
  // The default polling rate for this function set (this resource and all resources below), in seconds. If not specified, a default of 900 seconds (15 minutes) is used. It is RECOMMENDED a client poll the resources of this function set every pollRate seconds.
  poll_rate: u32,

}
pub struct AbstractDevice {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from SubscribableResource --- 

  // Indicates whether or not subscriptions are supported for this resource, and whether or not conditional (thresholds) are supported. If not specified, is "not subscribable" (0).
  subscribable: u8,


  // --- these fields come from AbstractDevice --- 

  configuration_link: ConfigurationLink,
  der_list_link: DERListLink,
  // This field is for use in devices that can adjust energy usage (e.g., demand response, distributed energy resources).  For devices that do not respond to EndDeviceControls or DERControls (for instance, an ESI), this field should not have any bits set.
  device_category: Vec<u8>,
  device_information_link: DeviceInformationLink,
  device_status_link: DeviceStatusLink,
  file_status_link: FileStatusLink,
  ip_interface_list_link: IPInterfaceListLink,
  // Long form of device identifier. See the Security section for additional details.
  l_fdi: Vec<u8>,
  load_shed_availability_list_link: LoadShedAvailabilityListLink,
  log_event_list_link: LogEventListLink,
  power_status_link: PowerStatusLink,
  // Short form of device identifier, WITH the checksum digit. See the Security section for additional details.
  s_fdi: u64,

}
pub struct DeviceStatus {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from DeviceStatus --- 

  // The time at which the reported values were recorded.
  changed_time: i64,
  // The number of times that the device has been turned on: Count of "device on" times, since the last time the counter was reset
  on_count: u16,
  // Device operational state: 
  // 0 - Not applicable / Unknown
  // 1 - Not operating
  // 2 - Operating
  // 3 - Starting up
  // 4 - Shutting down
  // 5 - At disconnect level
  // 6 - kW ramping
  // 7 - kVar ramping
  op_state: u8,
  // Total time device has operated: re-settable: Accumulated time in seconds since the last time the counter was reset.
  op_time: u32,
  temperature: Temperature,
  time_link: TimeLink,
  // The default polling rate for this function set (this resource and all resources below), in seconds. If not specified, a default of 900 seconds (15 minutes) is used. It is RECOMMENDED a client poll the resources of this function set every pollRate seconds.
  poll_rate: u32,

}
pub struct EndDevice {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from SubscribableResource --- 

  // Indicates whether or not subscriptions are supported for this resource, and whether or not conditional (thresholds) are supported. If not specified, is "not subscribable" (0).
  subscribable: u8,


  // --- these fields come from AbstractDevice --- 

  configuration_link: ConfigurationLink,
  der_list_link: DERListLink,
  // This field is for use in devices that can adjust energy usage (e.g., demand response, distributed energy resources).  For devices that do not respond to EndDeviceControls or DERControls (for instance, an ESI), this field should not have any bits set.
  device_category: Vec<u8>,
  device_information_link: DeviceInformationLink,
  device_status_link: DeviceStatusLink,
  file_status_link: FileStatusLink,
  ip_interface_list_link: IPInterfaceListLink,
  // Long form of device identifier. See the Security section for additional details.
  l_fdi: Vec<u8>,
  load_shed_availability_list_link: LoadShedAvailabilityListLink,
  log_event_list_link: LogEventListLink,
  power_status_link: PowerStatusLink,
  // Short form of device identifier, WITH the checksum digit. See the Security section for additional details.
  s_fdi: u64,


  // --- these fields come from EndDevice --- 

  // The time at which this resource was last modified or created.
  changed_time: i64,
  // This attribute indicates whether or not an EndDevice is enabled, or registered, on the server. If a server sets this attribute to false, the device is no longer registered. It should be noted that servers can delete EndDevice instances, but using this attribute for some time is more convenient for clients.
  enabled: bool,
  flow_reservation_request_list_link: FlowReservationRequestListLink,
  flow_reservation_response_list_link: FlowReservationResponseListLink,
  function_set_assignments_list_link: FunctionSetAssignmentsListLink,
  // POST rate, or how often EndDevice and subordinate resources should be POSTed, in seconds. A client MAY indicate a preferred postRate when POSTing EndDevice. A server MAY add or modify postRate to indicate its preferred posting rate.
  post_rate: u32,
  registration_link: RegistrationLink,
  subscription_list_link: SubscriptionListLink,

}
pub struct EndDeviceList {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from SubscribableResource --- 

  // Indicates whether or not subscriptions are supported for this resource, and whether or not conditional (thresholds) are supported. If not specified, is "not subscribable" (0).
  subscribable: u8,


  // --- these fields come from SubscribableList --- 

  // The number specifying "all" of the items in the list. Required on GET, ignored otherwise.
  all: u32,
  // Indicates the number of items in this page of results.
  results: u32,


  // --- these fields come from EndDeviceList --- 

  end_device: EndDevice,
  // The default polling rate for this function set (this resource and all resources below), in seconds. If not specified, a default of 900 seconds (15 minutes) is used. It is RECOMMENDED a client poll the resources of this function set every pollRate seconds.
  poll_rate: u32,

}
pub struct Registration {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from Registration --- 

  // Contains the time at which this registration was created, by which clients MAY prioritize information providers with the most recent registrations, when no additional direction from the consumer is available.
  date_time_registered: i64,
  // Contains the registration PIN number associated with the device, including the checksum digit.
  p_in: u32,
  // The default polling rate for this function set (this resource and all resources below), in seconds. If not specified, a default of 900 seconds (15 minutes) is used. It is RECOMMENDED a client poll the resources of this function set every pollRate seconds.
  poll_rate: u32,

}
pub struct SelfDevice {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from SubscribableResource --- 

  // Indicates whether or not subscriptions are supported for this resource, and whether or not conditional (thresholds) are supported. If not specified, is "not subscribable" (0).
  subscribable: u8,


  // --- these fields come from AbstractDevice --- 

  configuration_link: ConfigurationLink,
  der_list_link: DERListLink,
  // This field is for use in devices that can adjust energy usage (e.g., demand response, distributed energy resources).  For devices that do not respond to EndDeviceControls or DERControls (for instance, an ESI), this field should not have any bits set.
  device_category: Vec<u8>,
  device_information_link: DeviceInformationLink,
  device_status_link: DeviceStatusLink,
  file_status_link: FileStatusLink,
  ip_interface_list_link: IPInterfaceListLink,
  // Long form of device identifier. See the Security section for additional details.
  l_fdi: Vec<u8>,
  load_shed_availability_list_link: LoadShedAvailabilityListLink,
  log_event_list_link: LogEventListLink,
  power_status_link: PowerStatusLink,
  // Short form of device identifier, WITH the checksum digit. See the Security section for additional details.
  s_fdi: u64,


  // --- these fields come from SelfDevice --- 

  // The default polling rate for this function set (this resource and all resources below), in seconds. If not specified, a default of 900 seconds (15 minutes) is used. It is RECOMMENDED a client poll the resources of this function set every pollRate seconds.
  poll_rate: u32,

}
pub struct Temperature {

  // --- these fields come from Temperature --- 

  // Multiplier for 'unit'.
  multiplier: i8,
  // The subject of the temperature measurement
  // 0 - Enclosure
  // 1 - Transformer
  // 2 - HeatSink
  subject: u8,
  // Value in Degrees Celsius (uom 23).
  value: i16,

}
pub struct FunctionSetAssignmentsBase {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from FunctionSetAssignmentsBase --- 

  customer_account_list_link: CustomerAccountListLink,
  demand_response_program_list_link: DemandResponseProgramListLink,
  der_program_list_link: DERProgramListLink,
  file_list_link: FileListLink,
  messaging_program_list_link: MessagingProgramListLink,
  prepayment_list_link: PrepaymentListLink,
  response_set_list_link: ResponseSetListLink,
  tariff_profile_list_link: TariffProfileListLink,
  time_link: TimeLink,
  usage_point_list_link: UsagePointListLink,

}
pub struct FunctionSetAssignments {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from FunctionSetAssignmentsBase --- 

  customer_account_list_link: CustomerAccountListLink,
  demand_response_program_list_link: DemandResponseProgramListLink,
  der_program_list_link: DERProgramListLink,
  file_list_link: FileListLink,
  messaging_program_list_link: MessagingProgramListLink,
  prepayment_list_link: PrepaymentListLink,
  response_set_list_link: ResponseSetListLink,
  tariff_profile_list_link: TariffProfileListLink,
  time_link: TimeLink,
  usage_point_list_link: UsagePointListLink,


  // --- these fields come from FunctionSetAssignments --- 

  // The global identifier of the object.
  m_rid: Vec<u8>,
  // The description is a human readable text describing or naming the object.
  description: String,
  // Contains the version number of the object. See the type definition for details.
  version: u16,
  // Indicates whether or not subscriptions are supported for this resource, and whether or not conditional (thresholds) are supported. If not specified, is "not subscribable" (0).
  subscribable: u8,

}
pub struct FunctionSetAssignmentsList {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from SubscribableResource --- 

  // Indicates whether or not subscriptions are supported for this resource, and whether or not conditional (thresholds) are supported. If not specified, is "not subscribable" (0).
  subscribable: u8,


  // --- these fields come from SubscribableList --- 

  // The number specifying "all" of the items in the list. Required on GET, ignored otherwise.
  all: u32,
  // Indicates the number of items in this page of results.
  results: u32,


  // --- these fields come from FunctionSetAssignmentsList --- 

  function_set_assignments: FunctionSetAssignments,
  // The default polling rate for this function set (this resource and all resources below), in seconds. If not specified, a default of 900 seconds (15 minutes) is used. It is RECOMMENDED a client poll the resources of this function set every pollRate seconds.
  poll_rate: u32,

}
pub struct Condition {

  // --- these fields come from Condition --- 

  // 0 = Reading value
  // 1-255 = Reserved
  attribute_identifier: u8,
  // The value of the lower threshold
  lower_threshold: i64,
  // The value of the upper threshold
  upper_threshold: i64,

}
pub struct SubscriptionBase {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from SubscriptionBase --- 

  // The resource for which the subscription applies. Query string parameters SHALL NOT be specified when subscribing to list resources.  Should a query string parameter be specified, servers SHALL ignore them.
  subscribed_resource: String,

}
pub struct Subscription {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from SubscriptionBase --- 

  // The resource for which the subscription applies. Query string parameters SHALL NOT be specified when subscribing to list resources.  Should a query string parameter be specified, servers SHALL ignore them.
  subscribed_resource: String,


  // --- these fields come from Subscription --- 

  condition: Condition,
  // 0 - application/sep+xml
  // 1 - application/sep-exi
  // 2-255 - reserved
  encoding: u8,
  // Contains the preferred schema and extensibility level indication such as "+S1"
  level: String,
  // This element is used to indicate the maximum number of list items that should be included in a notification when the subscribed resource changes. This limit is meant to be functionally equivalent to the ‘limit’ query string parameter, but applies to both list resources as well as other resources.  For list resources, if a limit of ‘0’ is specified, then notifications SHALL contain a list resource with results=’0’ (equivalent to a simple change notification).  For list resources, if a limit greater than ‘0’ is specified, then notifications SHALL contain a list resource with results equal to the limit specified (or less, should the list contain fewer items than the limit specified or should the server be unable to provide the requested number of items for any reason) and follow the same rules for list resources (e.g., ordering).  For non-list resources, if a limit of ‘0’ is specified, then notifications SHALL NOT contain a resource representation (equivalent to a simple change notification).  For non-list resources, if a limit greater than ‘0’ is specified, then notifications SHALL contain the representation of the changed resource.
  limit: u32,
  // The resource to which to post the notifications about the requested subscribed resource. Because this URI will exist on a server other than the one being POSTed to, this attribute SHALL be a fully-qualified absolute URI, not a relative reference.
  notification_uri: String,

}
pub struct SubscriptionList {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from List --- 

  // The number specifying "all" of the items in the list. Required on a response to a GET, ignored otherwise.
  all: u32,
  // Indicates the number of items in this page of results.
  results: u32,


  // --- these fields come from SubscriptionList --- 

  subscription: Subscription,
  // The default polling rate for this function set (this resource and all resources below), in seconds. If not specified, a default of 900 seconds (15 minutes) is used. It is RECOMMENDED a client poll the resources of this function set every pollRate seconds.
  poll_rate: u32,

}
pub struct Notification {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from SubscriptionBase --- 

  // The resource for which the subscription applies. Query string parameters SHALL NOT be specified when subscribing to list resources.  Should a query string parameter be specified, servers SHALL ignore them.
  subscribed_resource: String,


  // --- these fields come from Notification --- 

  // The new location of the resource, if moved. This attribute SHALL be a fully-qualified absolute URI, not a relative reference.
  new_resource_uri: String,
  resource: Resource,
  // 0 = Default Status
  // 1 = Subscription canceled, no additional information
  // 2 = Subscription canceled, resource moved
  // 3 = Subscription canceled, resource definition changed (e.g., a new version of IEEE 2030.5)
  // 4 = Subscription canceled, resource deleted
  // All other values reserved.
  status: u8,
  // The subscription from which this notification was triggered. This attribute SHALL be a fully-qualified absolute URI, not a relative reference.
  subscription_uri: String,

}
pub struct NotificationList {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from List --- 

  // The number specifying "all" of the items in the list. Required on a response to a GET, ignored otherwise.
  all: u32,
  // Indicates the number of items in this page of results.
  results: u32,


  // --- these fields come from NotificationList --- 

  notification: Notification,

}
pub struct DerControlResponse {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from Response --- 

  // The createdDateTime field contains the date and time when the acknowledgement/status occurred in the client. The client will provide the timestamp to ensure the proper time is captured in case the response is delayed in reaching the server (server receipt time would not be the same as the actual confirmation time). The time reported from the client should be relative to the time server indicated by the FunctionSetAssignment that also indicated the event resource; if no FunctionSetAssignment exists, the time of the server where the event resource was hosted.
  created_date_time: i64,
  // Contains the LFDI of the device providing the response.
  end_device_lfdi: Vec<u8>,
  // The status field contains the acknowledgement or status. Each event type (DRLC, DER, Price, or Text) can return different status information (e.g. an Acknowledge will be returned for a Price event where a DRLC event can return Event Received, Event Started, and Event Completed). The Status field value definitions are defined in Table 27: Response Types by Function Set.
  status: u8,
  // The subject field provides a method to match the response with the originating event. It is populated with the mRID of the original object.
  subject: Vec<u8>,


  // --- these fields come from DERControlResponse --- 


}
pub struct FlowReservationResponseResponse {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from Response --- 

  // The createdDateTime field contains the date and time when the acknowledgement/status occurred in the client. The client will provide the timestamp to ensure the proper time is captured in case the response is delayed in reaching the server (server receipt time would not be the same as the actual confirmation time). The time reported from the client should be relative to the time server indicated by the FunctionSetAssignment that also indicated the event resource; if no FunctionSetAssignment exists, the time of the server where the event resource was hosted.
  created_date_time: i64,
  // Contains the LFDI of the device providing the response.
  end_device_lfdi: Vec<u8>,
  // The status field contains the acknowledgement or status. Each event type (DRLC, DER, Price, or Text) can return different status information (e.g. an Acknowledge will be returned for a Price event where a DRLC event can return Event Received, Event Started, and Event Completed). The Status field value definitions are defined in Table 27: Response Types by Function Set.
  status: u8,
  // The subject field provides a method to match the response with the originating event. It is populated with the mRID of the original object.
  subject: Vec<u8>,


  // --- these fields come from FlowReservationResponseResponse --- 


}
pub struct AppliedTargetReduction {

  // --- these fields come from AppliedTargetReduction --- 

  // Enumerated field representing the type of reduction requested.
  type: u8,
  // Indicates the requested amount of the relevant commodity to be reduced.
  value: u16,

}
pub struct DrResponse {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from Response --- 

  // The createdDateTime field contains the date and time when the acknowledgement/status occurred in the client. The client will provide the timestamp to ensure the proper time is captured in case the response is delayed in reaching the server (server receipt time would not be the same as the actual confirmation time). The time reported from the client should be relative to the time server indicated by the FunctionSetAssignment that also indicated the event resource; if no FunctionSetAssignment exists, the time of the server where the event resource was hosted.
  created_date_time: i64,
  // Contains the LFDI of the device providing the response.
  end_device_lfdi: Vec<u8>,
  // The status field contains the acknowledgement or status. Each event type (DRLC, DER, Price, or Text) can return different status information (e.g. an Acknowledge will be returned for a Price event where a DRLC event can return Event Received, Event Started, and Event Completed). The Status field value definitions are defined in Table 27: Response Types by Function Set.
  status: u8,
  // The subject field provides a method to match the response with the originating event. It is populated with the mRID of the original object.
  subject: Vec<u8>,


  // --- these fields come from DrResponse --- 

  appliance_load_reduction: ApplianceLoadReduction,
  applied_target_reduction: AppliedTargetReduction,
  duty_cycle: DutyCycle,
  offset: Offset,
  // Indicates the amount of time, in seconds, that the client partially opts-out during the demand response event. When overriding within the allowed override duration, the client SHALL send a partial opt-out (Response status code 8) for partial opt-out upon completion, with the total time the event was overridden (this attribute) populated. The client SHALL send a no participation status response (status type 10) if the user partially opts-out for longer than EndDeviceControl.overrideDuration.
  override_duration: u16,
  set_point: SetPoint,

}
pub struct PriceResponse {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from Response --- 

  // The createdDateTime field contains the date and time when the acknowledgement/status occurred in the client. The client will provide the timestamp to ensure the proper time is captured in case the response is delayed in reaching the server (server receipt time would not be the same as the actual confirmation time). The time reported from the client should be relative to the time server indicated by the FunctionSetAssignment that also indicated the event resource; if no FunctionSetAssignment exists, the time of the server where the event resource was hosted.
  created_date_time: i64,
  // Contains the LFDI of the device providing the response.
  end_device_lfdi: Vec<u8>,
  // The status field contains the acknowledgement or status. Each event type (DRLC, DER, Price, or Text) can return different status information (e.g. an Acknowledge will be returned for a Price event where a DRLC event can return Event Received, Event Started, and Event Completed). The Status field value definitions are defined in Table 27: Response Types by Function Set.
  status: u8,
  // The subject field provides a method to match the response with the originating event. It is populated with the mRID of the original object.
  subject: Vec<u8>,


  // --- these fields come from PriceResponse --- 


}
pub struct Response {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from Response --- 

  // The createdDateTime field contains the date and time when the acknowledgement/status occurred in the client. The client will provide the timestamp to ensure the proper time is captured in case the response is delayed in reaching the server (server receipt time would not be the same as the actual confirmation time). The time reported from the client should be relative to the time server indicated by the FunctionSetAssignment that also indicated the event resource; if no FunctionSetAssignment exists, the time of the server where the event resource was hosted.
  created_date_time: i64,
  // Contains the LFDI of the device providing the response.
  end_device_lfdi: Vec<u8>,
  // The status field contains the acknowledgement or status. Each event type (DRLC, DER, Price, or Text) can return different status information (e.g. an Acknowledge will be returned for a Price event where a DRLC event can return Event Received, Event Started, and Event Completed). The Status field value definitions are defined in Table 27: Response Types by Function Set.
  status: u8,
  // The subject field provides a method to match the response with the originating event. It is populated with the mRID of the original object.
  subject: Vec<u8>,

}
pub struct ResponseList {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from List --- 

  // The number specifying "all" of the items in the list. Required on a response to a GET, ignored otherwise.
  all: u32,
  // Indicates the number of items in this page of results.
  results: u32,


  // --- these fields come from ResponseList --- 

  response: Response,

}
pub struct ResponseSet {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from IdentifiedObject --- 

  // The global identifier of the object.
  m_rid: Vec<u8>,
  // The description is a human readable text describing or naming the object.
  description: String,
  // Contains the version number of the object. See the type definition for details.
  version: u16,


  // --- these fields come from ResponseSet --- 

  response_list_link: ResponseListLink,

}
pub struct ResponseSetList {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from List --- 

  // The number specifying "all" of the items in the list. Required on a response to a GET, ignored otherwise.
  all: u32,
  // Indicates the number of items in this page of results.
  results: u32,


  // --- these fields come from ResponseSetList --- 

  response_set: ResponseSet,
  // The default polling rate for this function set (this resource and all resources below), in seconds. If not specified, a default of 900 seconds (15 minutes) is used. It is RECOMMENDED a client poll the resources of this function set every pollRate seconds.
  poll_rate: u32,

}
pub struct TextResponse {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from Response --- 

  // The createdDateTime field contains the date and time when the acknowledgement/status occurred in the client. The client will provide the timestamp to ensure the proper time is captured in case the response is delayed in reaching the server (server receipt time would not be the same as the actual confirmation time). The time reported from the client should be relative to the time server indicated by the FunctionSetAssignment that also indicated the event resource; if no FunctionSetAssignment exists, the time of the server where the event resource was hosted.
  created_date_time: i64,
  // Contains the LFDI of the device providing the response.
  end_device_lfdi: Vec<u8>,
  // The status field contains the acknowledgement or status. Each event type (DRLC, DER, Price, or Text) can return different status information (e.g. an Acknowledge will be returned for a Price event where a DRLC event can return Event Received, Event Started, and Event Completed). The Status field value definitions are defined in Table 27: Response Types by Function Set.
  status: u8,
  // The subject field provides a method to match the response with the originating event. It is populated with the mRID of the original object.
  subject: Vec<u8>,


  // --- these fields come from TextResponse --- 


}
pub struct Time {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from Time --- 

  // The current time, in the format defined by TimeType.
  current_time: i64,
  // Time at which daylight savings ends (dstOffset no longer applied).  Result of dstEndRule calculation.
  dst_end_time: i64,
  // Daylight savings time offset from local standard time. A typical practice is advancing clocks one hour when daylight savings time is in effect, which would result in a positive dstOffset.
  dst_offset: i32,
  // Time at which daylight savings begins (apply dstOffset).  Result of dstStartRule calculation.
  dst_start_time: i64,
  // Local time: localTime = currentTime + tzOffset (+ dstOffset when in effect).
  local_time: i64,
  // Metric indicating the quality of the time source from which the service acquired time. Lower (smaller) quality enumeration values are assumed to be more accurate.
  // 3 - time obtained from external authoritative source such as NTP
  // 4 - time obtained from level 3 source
  // 5 - time manually set or obtained from level 4 source
  // 6 - time obtained from level 5 source
  // 7 - time intentionally uncoordinated
  // All other values are reserved for future use.
  quality: u8,
  // Local time zone offset from currentTime. Does not include any daylight savings time offsets. For American time zones, a negative tzOffset SHALL be used (eg, EST = GMT-5 which is -18000).
  tz_offset: i32,
  // The default polling rate for this function set (this resource and all resources below), in seconds. If not specified, a default of 900 seconds (15 minutes) is used. It is RECOMMENDED a client poll the resources of this function set every pollRate seconds.
  poll_rate: u32,

}
pub struct DeviceInformation {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from DeviceInformation --- 

  drlc_capabilities: DRLCCapabilities,
  // Bitmap indicating the function sets used by the device as a client.
  // 0 - Device Capability
  // 1 - Self Device Resource
  // 2 - End Device Resource
  // 3 - Function Set Assignments
  // 4 - Subscription/Notification Mechanism
  // 5 - Response
  // 6 - Time
  // 7 - Device Information
  // 8 - Power Status
  // 9 - Network Status
  // 10 - Log Event
  // 11 - Configuration Resource
  // 12 - Software Download
  // 13 - DRLC
  // 14 - Metering
  // 15 - Pricing
  // 16 - Messaging
  // 17 - Billing
  // 18 - Prepayment
  // 19 - Flow Reservation
  // 20 - DER Control
  functions_implemented: Vec<u8>,
  // GPS location of this device.
  gps_location: GPSLocationType,
  // Long form device identifier. See the Security section for full details.
  l_fdi: Vec<u8>,
  // Date/time of manufacture
  mf_date: i64,
  // Manufacturer hardware version
  mf_hw_ver: String,
  // The manufacturer's IANA Enterprise Number.
  mf_id: u32,
  // Manufacturer dependent information related to the manufacture of this device
  mf_info: String,
  // Manufacturer's xsd-model number
  mf_model: String,
  // Manufacturer assigned serial number
  mf_ser_num: String,
  // Primary source of power.
  primary_power: u8,
  // Secondary source of power
  secondary_power: u8,
  supported_locale_list_link: SupportedLocaleListLink,
  // Activation date/time of currently running software
  sw_act_time: i64,
  // Currently running software version
  sw_ver: String,
  // The default polling rate for this function set (this resource and all resources below), in seconds. If not specified, a default of 900 seconds (15 minutes) is used. It is RECOMMENDED a client poll the resources of this function set every pollRate seconds.
  poll_rate: u32,

}
pub struct DrlcCapabilities {

  // --- these fields come from DRLCCapabilities --- 

  // The average hourly energy usage when in normal operating mode.
  average_energy: RealEnergy,
  // The maximum demand rating of this end device.
  max_demand: ActivePower,
  // Bitmap indicating the DRLC options implemented by the device.
  // 0 - Target reduction (kWh)
  // 1 - Target reduction (kW)
  // 2 - Target reduction (Watts)
  // 3 - Target reduction (Cubic Meters)
  // 4 - Target reduction (Cubic Feet)
  // 5 - Target reduction (US Gallons)
  // 6 - Target reduction (Imperial Gallons)
  // 7 - Target reduction (BTUs)
  // 8 - Target reduction (Liters)
  // 9 - Target reduction (kPA (gauge))
  // 10 - Target reduction (kPA (absolute))
  // 11 - Target reduction (Mega Joule)
  // 12 - Target reduction (Unitless)
  // 13-15 - Reserved
  // 16 - Temperature set point
  // 17 - Temperature offset
  // 18 - Duty cycle
  // 19 - Load adjustment percentage
  // 20 - Appliance load reduction
  // 21-31 - Reserved
  options_implemented: Vec<u8>,

}
pub struct SupportedLocale {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from SupportedLocale --- 

  // The code for a locale that is supported
  locale: String,

}
pub struct SupportedLocaleList {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from List --- 

  // The number specifying "all" of the items in the list. Required on a response to a GET, ignored otherwise.
  all: u32,
  // Indicates the number of items in this page of results.
  results: u32,


  // --- these fields come from SupportedLocaleList --- 

  supported_locale: SupportedLocale,

}
pub struct PowerStatus {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from PowerStatus --- 

  // Battery system status
  // 
  // 0 = unknown
  // 1 = normal (more than LowChargeThreshold remaining)
  // 2 = low (less than LowChargeThreshold remaining) 
  // 3 = depleted (0% charge remaining)
  // 4 = not applicable (mains powered only)
  battery_status: u8,
  // The time at which the reported values were recorded.
  changed_time: i64,
  // This value will be fixed for devices powered by a single source.  This value may change for devices able to transition between multiple power sources (mains to battery backup, etc.).
  current_power_source: u8,
  // Estimate of remaining battery charge as a percent of full charge.
  estimated_charge_remaining: u16,
  // Estimated time (in seconds) to total battery charge depletion (under current load)
  estimated_time_remaining: u32,
  pev_info: PEVInfo,
  // If the device has a battery, this is the time since the device last switched to battery power, or the time since the device was restarted, whichever is less, in seconds.
  session_time_on_battery: u32,
  // If the device has a battery, this is the total time the device has been on battery power, in seconds. It may be reset when the battery is replaced.
  total_time_on_battery: u32,
  // The default polling rate for this function set (this resource and all resources below), in seconds. If not specified, a default of 900 seconds (15 minutes) is used. It is RECOMMENDED a client poll the resources of this function set every pollRate seconds.
  poll_rate: u32,

}
pub struct PevInfo {

  // --- these fields come from PEVInfo --- 

  // This is the actual power flow in or out of the charger or inverter. This is calculated by the vehicle based on actual measurements. This number is positive for charging.
  charging_power_now: ActivePower,
  // This is the amount of energy that must be transferred from the grid to EVSE and PEV to achieve the target state of charge allowing for charger efficiency and any vehicle and EVSE parasitic loads. This is calculated by the vehicle and changes throughout the connection as forward or reverse power flow change the battery state of charge.  This number is positive for charging.
  energy_request_now: RealEnergy,
  // This is maximum power transfer capability that could be used for charging the PEV to perform the requested energy transfer.  It is the lower of the vehicle or EVSE physical power limitations. It is not based on economic considerations. The vehicle may draw less power than this value based on its charging cycle. The vehicle defines this parameter. This number is positive for charging power flow.
  max_forward_power: ActivePower,
  // This is computed by the PEV based on the charging profile to complete the energy transfer if the maximum power is authorized.  The value will never be smaller than the ratio of the energy request to the power request because the charging profile may not allow the maximum power to be used throughout the transfer.   This is a critical parameter for determining whether any slack time exists in the charging cycle between the current time and the TCIN.
  minimum_charging_duration: u32,
  // This is the target state of charge that is to be achieved during charging before the time of departure (TCIN).  The default value is 100%. The value cannot be set to a value less than the actual state of charge.
  target_state_of_charge: u16,
  // Time Charge is Needed (TCIN) is the time that the PEV is expected to depart. The value is manually entered using controls and displays in the vehicle or on the EVSE or using a mobile device.  It is authenticated and saved by the PEV.  This value may be updated during a charging session.
  time_charge_is_needed: i64,
  // This is the time that the parameters are updated, except for changes to TCIN.
  time_charging_status_pev: i64,

}
pub struct Ieee802154 {

  // --- these fields come from IEEE_802_15_4 --- 

  // As defined by IEEE 802.15.4
  capability_info: u8,
  neighbor_list_link: NeighborListLink,
  // As defined by IEEE 802.15.4
  short_address: u16,

}
pub struct IpAddr {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from IPAddr --- 

  // An IP address value.
  address: Vec<u8>,
  rpl_instance_list_link: RPLInstanceListLink,

}
pub struct IpAddrList {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from List --- 

  // The number specifying "all" of the items in the list. Required on a response to a GET, ignored otherwise.
  all: u32,
  // Indicates the number of items in this page of results.
  results: u32,


  // --- these fields come from IPAddrList --- 

  ip_addr: IPAddr,

}
pub struct IpInterface {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from IPInterface --- 

  // Use rules from [RFC 2863].
  if_descr: String,
  // Use rules from [RFC 2863].
  if_high_speed: u32,
  // Use rules from [RFC 2863].
  if_in_broadcast_pkts: u32,
  // Use rules from [RFC 2863].
  if_index: u32,
  // Use rules from [RFC 2863]. Can be thought of as Input Datagrams Discarded.
  if_in_discards: u32,
  // Use rules from [RFC 2863].
  if_in_errors: u32,
  // Use rules from [RFC 2863]. Can be thought of as Multicast Datagrams Received.
  if_in_multicast_pkts: u32,
  // Use rules from [RFC 2863]. Can be thought of as Bytes Received.
  if_in_octets: u32,
  // Use rules from [RFC 2863]. Can be thought of as Datagrams Received.
  if_in_ucast_pkts: u32,
  // Use rules from [RFC 2863]. Can be thought of as Datagrams with Unknown Protocol Received.
  if_in_unknown_protos: u32,
  // Use rules from [RFC 2863].
  if_mtu: u32,
  // Use rules from [RFC 2863].
  if_name: String,
  // Use rules and assignments from [RFC 2863].
  if_oper_status: u8,
  // Use rules from [RFC 2863]. Can be thought of as Broadcast Datagrams Sent.
  if_out_broadcast_pkts: u32,
  // Use rules from [RFC 2863]. Can be thought of as Output Datagrams Discarded.
  if_out_discards: u32,
  // Use rules from [RFC 2863].
  if_out_errors: u32,
  // Use rules from [RFC 2863]. Can be thought of as Multicast Datagrams Sent.
  if_out_multicast_pkts: u32,
  // Use rules from [RFC 2863]. Can be thought of as Bytes Sent.
  if_out_octets: u32,
  // Use rules from [RFC 2863]. Can be thought of as Datagrams Sent.
  if_out_ucast_pkts: u32,
  // Use rules from [RFC 2863].
  if_promiscuous_mode: bool,
  // Use rules from [RFC 2863].
  if_speed: u32,
  // Use rules and assignments from [RFC 2863].
  if_type: u16,
  ip_addr_list_link: IPAddrListLink,
  // Similar to ifLastChange in [RFC 2863].
  last_reset_time: i64,
  // The date/time of the reported status.
  last_updated_time: i64,
  ll_interface_list_link: LLInterfaceListLink,

}
pub struct IpInterfaceList {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from List --- 

  // The number specifying "all" of the items in the list. Required on a response to a GET, ignored otherwise.
  all: u32,
  // Indicates the number of items in this page of results.
  results: u32,


  // --- these fields come from IPInterfaceList --- 

  ip_interface: IPInterface,
  // The default polling rate for this function set (this resource and all resources below), in seconds. If not specified, a default of 900 seconds (15 minutes) is used. It is RECOMMENDED a client poll the resources of this function set every pollRate seconds.
  poll_rate: u32,

}
pub struct LlInterface {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from LLInterface --- 

  // Contains the number of CRC errors since reset.
  cr_cerrors: u32,
  // Contains the EUI-64 of the link layer interface. 48 bit MAC addresses SHALL be changed into an EUI-64 using the method defined in [RFC 4291], Appendix A. (The method is to insert "0xFFFE" as described in the reference.)
  eui64: Vec<u8>,
  ieee_802_15_4: IEEE_802_15_4,
  // Specifies the type of link layer interface associated with the IPInterface. Values are below.
  // 0 = Unspecified 
  // 1 = IEEE 802.3 (Ethernet)
  // 2 = IEEE 802.11 (WLAN)
  // 3 = IEEE 802.15 (PAN)
  // 4 = IEEE 1901 (PLC)
  // All other values reserved.
  link_layer_type: u8,
  // Number of times an ACK was not received for a frame transmitted (when ACK was requested).
  ll_ack_not_rx: u32,
  // Number of times CSMA failed.
  llcsma_fail: u32,
  // Number of dropped receive frames.
  ll_frames_drop_rx: u32,
  // Number of dropped transmit frames.
  ll_frames_drop_tx: u32,
  // Number of link layer frames received.
  ll_frames_rx: u32,
  // Number of link layer frames transmitted.
  ll_frames_tx: u32,
  // Number of times access to media failed.
  ll_media_access_fail: u32,
  // Number of Bytes received.
  ll_octets_rx: u32,
  // Number of Bytes transmitted.
  ll_octets_tx: u32,
  // Number of MAC transmit retries.
  ll_retry_count: u32,
  // Number of receive security errors.
  ll_security_error_rx: u32,
  lo_wpan: loWPAN,

}
pub struct LlInterfaceList {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from List --- 

  // The number specifying "all" of the items in the list. Required on a response to a GET, ignored otherwise.
  all: u32,
  // Indicates the number of items in this page of results.
  results: u32,


  // --- these fields come from LLInterfaceList --- 

  ll_interface: LLInterface,

}
pub struct LoWpan {

  // --- these fields come from loWPAN --- 

  // Number of Bytes received
  octets_rx: u32,
  // Number of Bytes transmitted
  octets_tx: u32,
  // Number of packets received
  packets_rx: u32,
  // Number of packets transmitted
  packets_tx: u32,
  // Number of errors receiving fragments
  rx_frag_error: u32,

}
pub struct Neighbor {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from Neighbor --- 

  // True if the neighbor is a child.
  is_child: bool,
  // The quality of the link, as defined by 802.15.4
  link_quality: u8,
  // As defined by IEEE 802.15.4
  short_address: u16,

}
pub struct NeighborList {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from List --- 

  // The number specifying "all" of the items in the list. Required on a response to a GET, ignored otherwise.
  all: u32,
  // Indicates the number of items in this page of results.
  results: u32,


  // --- these fields come from NeighborList --- 

  neighbor: Neighbor,

}
pub struct RplInstance {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from RPLInstance --- 

  // See [RFC 6550].
  doda_gid: u8,
  // See [RFC 6550].
  doda_groot: bool,
  // See [RFC 6550].
  flags: u8,
  // See [RFC 6550].
  grounded_flag: bool,
  // See [RFC 6550].
  mop: u8,
  // See [RFC 6550].
  prf: u8,
  // See [RFC 6550].
  rank: u16,
  // See [RFC 6550].
  rpl_instance_id: u8,
  rpl_source_routes_list_link: RPLSourceRoutesListLink,
  // See [RFC 6550].
  version_number: u8,

}
pub struct RplInstanceList {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from List --- 

  // The number specifying "all" of the items in the list. Required on a response to a GET, ignored otherwise.
  all: u32,
  // Indicates the number of items in this page of results.
  results: u32,


  // --- these fields come from RPLInstanceList --- 

  rpl_instance: RPLInstance,

}
pub struct RplSourceRoutes {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from RPLSourceRoutes --- 

  // See [RFC 6554].
  dest_address: Vec<u8>,
  // See [RFC 6554].
  source_route: Vec<u8>,

}
pub struct RplSourceRoutesList {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from List --- 

  // The number specifying "all" of the items in the list. Required on a response to a GET, ignored otherwise.
  all: u32,
  // Indicates the number of items in this page of results.
  results: u32,


  // --- these fields come from RPLSourceRoutesList --- 

  rpl_source_routes: RPLSourceRoutes,

}
pub struct LogEvent {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from LogEvent --- 

  // The date and time that the event occurred.
  created_date_time: i64,
  // Human readable text that MAY be used to transmit additional details about the event. A host MAY remove this field when received.
  details: String,
  // May be used to transmit additional details about the event.
  extended_data: u32,
  // If the profileID indicates this is IEEE 2030.5, the functionSet is defined by IEEE 2030.5 and SHALL be one of the values from the table below (IEEE 2030.5 function set identifiers). If the profileID is anything else, the functionSet is defined by the identified profile. 
  // 0	General (not specific to a function set)
  // 1	Publish and Subscribe
  // 2	End Device
  // 3	Function Set Assignment
  // 4	Response
  // 5	Demand Response and Load Control
  // 6	Metering
  // 7	Pricing
  // 8	Messaging
  // 9	Billing
  // 10	Prepayment
  // 11	Distributed Energy Resources
  // 12	Time
  // 13	Software  Download
  // 14	Device Information
  // 15	Power Status
  // 16	Network Status
  // 17	Log Event List
  // 18	Configuration
  // 19	Security
  // All other values are reserved.
  function_set: u8,
  // An 8 bit unsigned integer. logEventCodes are scoped to a profile and a function set. If the profile is IEEE 2030.5, the logEventCode is defined by IEEE 2030.5 within one of the function sets of IEEE 2030.5. If the profile is anything else, the logEventCode is defined by the specified profile.
  log_event_code: u8,
  // This 16-bit value, combined with createdDateTime, profileID, and logEventPEN, should provide a reasonable level of uniqueness.
  log_event_id: u16,
  // The Private Enterprise Number(PEN) of the entity that defined the profileID, functionSet, and logEventCode of the logEvent. IEEE 2030.5-assigned logEventCodes SHALL use the IEEE 2030.5 PEN.  Combinations of profileID, functionSet, and logEventCode SHALL have unique meaning within a logEventPEN and are defined by the owner of the PEN.
  log_event_pen: u32,
  // The profileID identifies which profile (HA, BA, SE, etc) defines the following event information.
  // 0	Not profile specific.
  // 1	Vendor Defined
  // 2	IEEE 2030.5
  // 3	Home Automation
  // 4	Building Automation
  // All other values are reserved.
  profile_id: u8,

}
pub struct LogEventList {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from SubscribableResource --- 

  // Indicates whether or not subscriptions are supported for this resource, and whether or not conditional (thresholds) are supported. If not specified, is "not subscribable" (0).
  subscribable: u8,


  // --- these fields come from SubscribableList --- 

  // The number specifying "all" of the items in the list. Required on GET, ignored otherwise.
  all: u32,
  // Indicates the number of items in this page of results.
  results: u32,


  // --- these fields come from LogEventList --- 

  log_event: LogEvent,
  // The default polling rate for this function set (this resource and all resources below), in seconds. If not specified, a default of 900 seconds (15 minutes) is used. It is RECOMMENDED a client poll the resources of this function set every pollRate seconds.
  poll_rate: u32,

}
pub struct Configuration {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from SubscribableResource --- 

  // Indicates whether or not subscriptions are supported for this resource, and whether or not conditional (thresholds) are supported. If not specified, is "not subscribable" (0).
  subscribable: u8,


  // --- these fields come from Configuration --- 

  // [RFC 4646] identifier of the language-region currently in use.
  current_locale: String,
  power_configuration: PowerConfiguration,
  price_response_cfg_list_link: PriceResponseCfgListLink,
  time_configuration: TimeConfiguration,
  // User assigned, convenience name used for network browsing displays, etc.  Example "My Thermostat"
  user_device_name: String,
  // The default polling rate for this function set (this resource and all resources below), in seconds. If not specified, a default of 900 seconds (15 minutes) is used. It is RECOMMENDED a client poll the resources of this function set every pollRate seconds.
  poll_rate: u32,

}
pub struct PowerConfiguration {

  // --- these fields come from PowerConfiguration --- 

  // Time/Date at which battery was installed,
  battery_install_time: i64,
  // In context of the PowerStatus resource, this is the value of EstimatedTimeRemaining below which BatteryStatus "low" is indicated and the PS_LOW_BATTERY is raised.
  low_charge_threshold: u32,

}
pub struct PriceResponseCfg {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from PriceResponseCfg --- 

  // Price responsive clients acting upon the associated RateComponent SHOULD consume the associated commodity while the price is less than this threshold.
  consume_threshold: i32,
  // Price responsive clients acting upon the associated RateComponent SHOULD reduce consumption to the maximum extent possible while the price is greater than this threshold.
  max_reduction_threshold: i32,
  rate_component_link: RateComponentLink,

}
pub struct PriceResponseCfgList {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from List --- 

  // The number specifying "all" of the items in the list. Required on a response to a GET, ignored otherwise.
  all: u32,
  // Indicates the number of items in this page of results.
  results: u32,


  // --- these fields come from PriceResponseCfgList --- 

  price_response_cfg: PriceResponseCfg,

}
pub struct TimeConfiguration {

  // --- these fields come from TimeConfiguration --- 

  // Rule to calculate end of daylight savings time in the current year.  Result of dstEndRule must be greater than result of dstStartRule.
  dst_end_rule: Vec<u8>,
  // Daylight savings time offset from local standard time.
  dst_offset: i32,
  // Rule to calculate start of daylight savings time in the current year. Result of dstEndRule must be greater than result of dstStartRule.
  dst_start_rule: Vec<u8>,
  // Local time zone offset from UTCTime. Does not include any daylight savings time offsets.
  tz_offset: i32,

}
pub struct File {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from File --- 

  // This element MUST be set to the date/time at which this file is activated. If the activation time is less than or equal to current time, the LD MUST immediately place the file into the activated state (in the case of a firmware file, the file is now the running image).  If the activation time is greater than the current time, the LD MUST wait until the specified activation time is reached, then MUST place the file into the activated state. Omission of this element means that the LD MUST NOT take any action to activate the file until a subsequent GET to this File resource provides an activateTime.
  activate_time: i64,
  // This element MUST be set to the URI location of the file binary artifact.  This is the BLOB (binary large object) that is actually loaded by the LD
  file_uri: String,
  // This element MUST be set to the LFDI of the device for which this file in targeted.
  l_fdi: Vec<u8>,
  // This element MUST be set to the hardware version for which this file is targeted.
  mf_hw_ver: String,
  // This element MUST be set to the manufacturer's Private Enterprise Number (assigned by IANA).
  mf_id: u32,
  // This element MUST be set to the manufacturer xsd-model number for which this file is targeted. The syntax and semantics are left to the manufacturer.
  mf_model: String,
  // This element MUST be set to the manufacturer serial number for which this file is targeted. The syntax and semantics are left to the manufacturer.
  mf_ser_num: String,
  // This element MUST be set to the software version information for this file. The syntax and semantics are left to the manufacturer.
  mf_ver: String,
  // This element MUST be set to the total size (in bytes) of the file referenced by fileURI.
  size: u32,
  // A value indicating the type of the file.  MUST be one of the following values:
  // 00 = Software Image
  // 01 = Security Credential
  // 02 = Configuration
  // 03 = Log
  // 04–7FFF = reserved
  // 8000-FFFF = Manufacturer defined
  type: Vec<u8>,

}
pub struct FileList {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from List --- 

  // The number specifying "all" of the items in the list. Required on a response to a GET, ignored otherwise.
  all: u32,
  // Indicates the number of items in this page of results.
  results: u32,


  // --- these fields come from FileList --- 

  file: File,
  // The default polling rate for this function set (this resource and all resources below), in seconds. If not specified, a default of 900 seconds (15 minutes) is used. It is RECOMMENDED a client poll the resources of this function set every pollRate seconds.
  poll_rate: u32,

}
pub struct FileStatus {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from FileStatus --- 

  // Date/time at which this File, referred to by FileLink, will be activated. Omission of or presence and value of this element MUST exactly match omission or presence and value of the activateTime element from the File resource.
  activate_time: i64,
  file_link: FileLink,
  // This element MUST be set to the percentage of the file, indicated by FileLink, that was loaded during the latest load attempt. This value MUST be reset to 0 each time a load attempt is started for the File indicated by FileLink. This value MUST be increased when an LD receives HTTP response containing file content. This value MUST be set to 100 when the full content of the file has been received by the LD
  load_percent: u8,
  // This element MUST be set to the time at which the LD will issue its next GET request for file content from the File indicated by FileLink
  next_request_attempt: i64,
  // This value MUST be reset to 0 when FileLink is first pointed at a new File. This value MUST be incremented each time an 
  // 
  // LD receives a 503 error from the FS.
  request503_count: u16,
  // This value MUST be reset to 0 when FileLink is first pointed at a new File. This value MUST be incremented each time a GET request for file content failed. 503 errors MUST be excluded from this counter.
  request_fail_count: u16,
  // Current loading status of the file indicated by FileLink. This element MUST be set to one of the following values:
  // 0 - No load operation in progress
  // 1 - File load in progress (first request for file content has been issued by LD)
  // 2 - File load failed
  // 3 - File loaded successfully (full content of file has been received by the LD), signature verification in progress
  // 4 - File signature verification failed
  // 5 - File signature verified, waiting to activate file.
  // 6 - File activation failed
  // 7 - File activation in progress  
  // 8 - File activated successfully (this state may not be reached/persisted through an image activation)
  // 9-255 - Reserved for future use.
  status: u8,
  // This element MUST be set to the time at which file status transitioned to the value indicated in the status element.
  status_time: i64,
  // The default polling rate for this function set (this resource and all resources below), in seconds. If not specified, a default of 900 seconds (15 minutes) is used. It is RECOMMENDED a client poll the resources of this function set every pollRate seconds.
  poll_rate: u32,

}
pub struct LoadShedAvailabilityList {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from List --- 

  // The number specifying "all" of the items in the list. Required on a response to a GET, ignored otherwise.
  all: u32,
  // Indicates the number of items in this page of results.
  results: u32,


  // --- these fields come from LoadShedAvailabilityList --- 

  load_shed_availability: LoadShedAvailability,
  // The default polling rate for this function set (this resource and all resources below), in seconds. If not specified, a default of 900 seconds (15 minutes) is used. It is RECOMMENDED a client poll the resources of this function set every pollRate seconds.
  poll_rate: u32,

}
pub struct ApplianceLoadReduction {

  // --- these fields come from ApplianceLoadReduction --- 

  // Indicates the type of appliance load reduction requested.
  type: u8,

}
pub struct DemandResponseProgram {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from IdentifiedObject --- 

  // The global identifier of the object.
  m_rid: Vec<u8>,
  // The description is a human readable text describing or naming the object.
  description: String,
  // Contains the version number of the object. See the type definition for details.
  version: u16,


  // --- these fields come from DemandResponseProgram --- 

  active_end_device_control_list_link: ActiveEndDeviceControlListLink,
  // This attribute allows program providers to specify the requested granularity of updates to LoadShedAvailability sheddablePercent. If not present, or set to 0, then updates to LoadShedAvailability SHALL NOT be provided. If present and greater than zero, then clients SHALL provide their LoadShedAvailability if it has not previously been provided, and thereafter if the difference between the previously provided value and the current value of LoadShedAvailability sheddablePercent is greater than availabilityUpdatePercentChangeThreshold.
  availability_update_percent_change_threshold: u16,
  // This attribute allows program providers to specify the requested granularity of updates to LoadShedAvailability sheddablePower. If not present, or set to 0, then updates to LoadShedAvailability SHALL NOT be provided. If present and greater than zero, then clients SHALL provide their LoadShedAvailability if it has not previously been provided, and thereafter if the difference between the previously provided value and the current value of LoadShedAvailability sheddablePower is greater than availabilityUpdatePowerChangeThreshold.
  availability_update_power_change_threshold: ActivePower,
  end_device_control_list_link: EndDeviceControlListLink,
  // Indicates the relative primacy of the provider of this program.
  primacy: u8,

}
pub struct DemandResponseProgramList {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from SubscribableResource --- 

  // Indicates whether or not subscriptions are supported for this resource, and whether or not conditional (thresholds) are supported. If not specified, is "not subscribable" (0).
  subscribable: u8,


  // --- these fields come from SubscribableList --- 

  // The number specifying "all" of the items in the list. Required on GET, ignored otherwise.
  all: u32,
  // Indicates the number of items in this page of results.
  results: u32,


  // --- these fields come from DemandResponseProgramList --- 

  demand_response_program: DemandResponseProgram,
  // The default polling rate for this function set (this resource and all resources below), in seconds. If not specified, a default of 900 seconds (15 minutes) is used. It is RECOMMENDED a client poll the resources of this function set every pollRate seconds.
  poll_rate: u32,

}
pub struct DutyCycle {

  // --- these fields come from DutyCycle --- 

  // Contains the maximum On state duty cycle applied by the end device, as a percentage of time.  The field not present indicates that this field has not been used by the end device.
  normal_value: u8,

}
pub struct EndDeviceControl {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from RespondableResource --- 

  // A reference to the response resource address (URI). Required on a response to a GET if responseRequired is "true".
  reply_to: String,
  // Indicates whether or not a response is required upon receipt, creation or update of this resource. Responses shall be posted to the collection specified in "replyTo". 
  // 
  // If the resource has a deviceCategory field, devices that match one or more of the device types indicated in deviceCategory SHALL respond according to the rules listed below.  If the category does not match, the device SHALL NOT respond. If the resource does not have a deviceCategory field, a device receiving the resource SHALL respond according to the rules listed below.
  // 
  // Value encoded as hex according to the following bit assignments, any combination is possible. 
  // See Table 27 for the list of appropriate Response status codes to be sent for these purposes.
  // 0 - End device shall indicate that message was received 
  // 1 - End device shall indicate specific response. 
  // 2 - End user / customer response is required. 
  // All other values reserved.
  response_required: u8,


  // --- these fields come from RespondableSubscribableIdentifiedObject --- 

  // The global identifier of the object.
  m_rid: Vec<u8>,
  // The description is a human readable text describing or naming the object.
  description: String,
  // Contains the version number of the object. See the type definition for details.
  version: u16,
  // Indicates whether or not subscriptions are supported for this resource, and whether or not conditional (thresholds) are supported. If not specified, is "not subscribable" (0).
  subscribable: u8,


  // --- these fields come from Event --- 

  // The time at which the Event was created.
  creation_time: i64,
  event_status: EventStatus,
  // The period during which the Event applies.
  interval: DateTimeInterval,


  // --- these fields come from RandomizableEvent --- 

  // Number of seconds boundary inside which a random value must be selected to be applied to the associated interval duration, to avoid sudden synchronized demand changes. If related to price level changes, sign may be ignored. Valid range is -3600 to 3600. If not specified, 0 is the default.
  randomize_duration: i16,
  // Number of seconds boundary inside which a random value must be selected to be applied to the associated interval start time, to avoid sudden synchronized demand changes. If related to price level changes, sign may be ignored. Valid range is -3600 to 3600. If not specified, 0 is the default.
  randomize_start: i16,


  // --- these fields come from EndDeviceControl --- 

  appliance_load_reduction: ApplianceLoadReduction,
  // Specifies the bitmap indicating  the categories of devices that SHOULD respond. Devices SHOULD ignore events that do not indicate their device category.
  device_category: Vec<u8>,
  // A flag to indicate if the EndDeviceControl is considered a mandatory event as defined by the service provider issuing the EndDeviceControl. The drProgramMandatory flag alerts the client/user that they will be subject to penalty or ineligibility based on the service provider’s program rules for that deviceCategory.
  dr_program_mandatory: bool,
  duty_cycle: DutyCycle,
  // Indicates that the event intends to increase consumption. A value of true indicates the intention to increase usage value, and a value of false indicates the intention to decrease usage.
  load_shift_forward: bool,
  offset: Offset,
  // The overrideDuration attribute provides a duration, in seconds, for which a client device is allowed to override this EndDeviceControl and still meet the contractual agreement with a service provider without opting out. If overrideDuration is not specified, then it SHALL default to 0.
  override_duration: u16,
  set_point: SetPoint,
  target_reduction: TargetReduction,

}
pub struct EndDeviceControlList {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from SubscribableResource --- 

  // Indicates whether or not subscriptions are supported for this resource, and whether or not conditional (thresholds) are supported. If not specified, is "not subscribable" (0).
  subscribable: u8,


  // --- these fields come from SubscribableList --- 

  // The number specifying "all" of the items in the list. Required on GET, ignored otherwise.
  all: u32,
  // Indicates the number of items in this page of results.
  results: u32,


  // --- these fields come from EndDeviceControlList --- 

  end_device_control: EndDeviceControl,

}
pub struct LoadShedAvailability {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from LoadShedAvailability --- 

  // Indicates for how many seconds the consuming device will be able to reduce consumption at the maximum response level.
  availability_duration: u32,
  demand_response_program_link: DemandResponseProgramLink,
  // Maximum percent of current operating load that is estimated to be sheddable.
  sheddable_percent: u16,
  // Maximum amount of current operating load that is estimated to be sheddable, in Watts.
  sheddable_power: ActivePower,

}
pub struct Offset {

  // --- these fields come from Offset --- 

  // The value change requested for the cooling offset, in degree C / 10. The value should be added to the normal set point for cooling, or if loadShiftForward is true, then the value should be subtracted from the normal set point.
  cooling_offset: u8,
  // The value change requested for the heating offset, in degree C / 10. The value should be subtracted for heating, or if loadShiftForward is true, then the value should be added to the normal set point.
  heating_offset: u8,
  // The value change requested for the load adjustment percentage. The value should be subtracted from the normal setting, or if loadShiftForward is true, then the value should be added to the normal setting.
  load_adjustment_percentage_offset: u16,

}
pub struct SetPoint {

  // --- these fields come from SetPoint --- 

  // This attribute represents the cooling temperature set point in degrees Celsius / 100. (Hundredths of a degree C)
  cooling_setpoint: i16,
  // This attribute represents the heating temperature set point in degrees Celsius / 100. (Hundredths of a degree C)
  heating_setpoint: i16,

}
pub struct TargetReduction {

  // --- these fields come from TargetReduction --- 

  // Indicates the type of reduction requested.
  type: u8,
  // Indicates the requested amount of the relevant commodity to be reduced.
  value: u16,

}
pub struct MeterReading {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from IdentifiedObject --- 

  // The global identifier of the object.
  m_rid: Vec<u8>,
  // The description is a human readable text describing or naming the object.
  description: String,
  // Contains the version number of the object. See the type definition for details.
  version: u16,


  // --- these fields come from MeterReadingBase --- 



  // --- these fields come from MeterReading --- 

  rate_component_list_link: RateComponentListLink,
  reading_link: ReadingLink,
  reading_set_list_link: ReadingSetListLink,
  reading_type_link: ReadingTypeLink,

}
pub struct MeterReadingList {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from SubscribableResource --- 

  // Indicates whether or not subscriptions are supported for this resource, and whether or not conditional (thresholds) are supported. If not specified, is "not subscribable" (0).
  subscribable: u8,


  // --- these fields come from SubscribableList --- 

  // The number specifying "all" of the items in the list. Required on GET, ignored otherwise.
  all: u32,
  // Indicates the number of items in this page of results.
  results: u32,


  // --- these fields come from MeterReadingList --- 

  meter_reading: MeterReading,

}
pub struct Reading {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from ReadingBase --- 

  // Indicates the consumption block related to the reading. REQUIRED if ReadingType numberOfConsumptionBlocks is non-zero. If not specified, is assumed to be "0 - N/A".
  consumption_block: u8,
  // List of codes indicating the quality of the reading, using specification:
  // 
  // Bit 0 - valid: data that has gone through all required validation checks and either passed them all or has been verified 
  // Bit 1 - manually edited: Replaced or approved by a human
  // Bit 2 - estimated using reference day: data value was replaced by a machine computed value based on analysis of historical data using the same type of measurement.
  // Bit 3 - estimated using linear interpolation: data value was computed using linear interpolation based on the readings before and after it
  // Bit 4 - questionable: data that has failed one or more checks
  // Bit 5 - derived: data that has been calculated (using logic or mathematical operations), not necessarily measured directly 
  // Bit 6 - projected (forecast): data that has been calculated as a projection or forecast of future readings
  quality_flags: Vec<u8>,
  // The time interval associated with the reading. If not specified, then defaults to the intervalLength specified in the associated ReadingType.
  time_period: DateTimeInterval,
  // Indicates the time of use tier related to the reading. REQUIRED if ReadingType numberOfTouTiers is non-zero. If not specified, is assumed to be "0 - N/A".
  tou_tier: u8,
  // Value in units specified by ReadingType
  value: i64,


  // --- these fields come from Reading --- 

  // The local identifier for this reading within the reading set. localIDs are assigned in order of creation time. For interval data, this value SHALL increase with each interval time, and for block/tier readings, localID SHALL not be specified.
  local_id: Vec<u8>,
  // Indicates whether or not subscriptions are supported for this resource, and whether or not conditional (thresholds) are supported. If not specified, is "not subscribable" (0).
  subscribable: u8,

}
pub struct ReadingList {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from SubscribableResource --- 

  // Indicates whether or not subscriptions are supported for this resource, and whether or not conditional (thresholds) are supported. If not specified, is "not subscribable" (0).
  subscribable: u8,


  // --- these fields come from SubscribableList --- 

  // The number specifying "all" of the items in the list. Required on GET, ignored otherwise.
  all: u32,
  // Indicates the number of items in this page of results.
  results: u32,


  // --- these fields come from ReadingList --- 

  reading: Reading,

}
pub struct ReadingSet {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from IdentifiedObject --- 

  // The global identifier of the object.
  m_rid: Vec<u8>,
  // The description is a human readable text describing or naming the object.
  description: String,
  // Contains the version number of the object. See the type definition for details.
  version: u16,


  // --- these fields come from ReadingSetBase --- 

  // Specifies the time range during which the contained readings were taken.
  time_period: DateTimeInterval,


  // --- these fields come from ReadingSet --- 

  reading_list_link: ReadingListLink,

}
pub struct ReadingSetList {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from SubscribableResource --- 

  // Indicates whether or not subscriptions are supported for this resource, and whether or not conditional (thresholds) are supported. If not specified, is "not subscribable" (0).
  subscribable: u8,


  // --- these fields come from SubscribableList --- 

  // The number specifying "all" of the items in the list. Required on GET, ignored otherwise.
  all: u32,
  // Indicates the number of items in this page of results.
  results: u32,


  // --- these fields come from ReadingSetList --- 

  reading_set: ReadingSet,

}
pub struct ReadingType {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from ReadingType --- 

  // The “accumulation behaviour” indicates how the value is represented to accumulate over time.
  accumulation_behaviour: u8,
  // The amount of heat generated when a given mass of fuel is completely burned. The CalorificValue is used to convert the measured volume or mass of gas into kWh. The CalorificValue attribute represents the current active value.
  calorific_value: UnitValueType,
  // Indicates the commodity applicable to this ReadingType.
  commodity: u8,
  // Accounts for changes in the volume of gas based on temperature and pressure. The ConversionFactor attribute represents the current active value. The ConversionFactor is dimensionless. The default value for the ConversionFactor is 1, which means no conversion is applied. A price server can advertise a new/different value at any time.
  conversion_factor: UnitValueType,
  // The data type can be used to describe a salient attribute of the data. Possible values are average, absolute, and etc.
  data_qualifier: u8,
  // Anything involving current might have a flow direction. Possible values include forward and reverse.
  flow_direction: u8,
  // Default interval length specified in seconds.
  interval_length: u32,
  // Compound class that contains kindCategory and kindIndex
  kind: u8,
  // To be populated for mirrors of interval data to set the expected number of intervals per ReadingSet. Servers may discard intervals received that exceed this number.
  max_number_of_intervals: u8,
  // Number of consumption blocks. 0 means not applicable, and is the default if not specified. The value needs to be at least 1 if any actual prices are provided.
  number_of_consumption_blocks: u8,
  // The number of TOU tiers that can be used by any resource configured by this ReadingType. Servers SHALL populate this value with the largest touTier value that will <i>ever</i> be used while this ReadingType is in effect. Servers SHALL set numberOfTouTiers equal to the number of standard TOU tiers plus the number of CPP tiers that may be used while this ReadingType is in effect. Servers SHALL specify a value between 0 and 255 (inclusive) for numberOfTouTiers (servers providing flat rate pricing SHOULD set numberOfTouTiers to 0, as in practice there is no difference between having no tiers and having one tier).
  number_of_tou_tiers: u8,
  // Contains phase information associated with the type.
  phase: u8,
  // Indicates the power of ten multiplier applicable to the unit of measure of this ReadingType.
  power_of_ten_multiplier: i8,
  // Default sub-interval length specified in seconds for Readings of ReadingType. Some demand calculations are done over a number of smaller intervals. For example, in a rolling demand calculation, the demand value is defined as the rolling sum of smaller intervals over the intervalLength. The subintervalLength is the length of the smaller interval in this calculation. It SHALL be an integral division of the intervalLength. The number of sub-intervals can be calculated by dividing the intervalLength by the subintervalLength.
  sub_interval_length: u32,
  // Reflects the supply limit set in the meter. This value can be compared to the Reading value to understand if limits are being approached or exceeded. Units follow the same definition as in this ReadingType.
  supply_limit: u64,
  // Specifies whether or not the consumption blocks are differentiated by TOUTier or not. Default is false, if not specified.
  // true = consumption accumulated over individual tiers
  // false = consumption accumulated over all tiers
  tiered_consumption_blocks: bool,
  // Indicates the measurement type for the units of measure for the readings of this type.
  uom: u8,

}
pub struct UsagePoint {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from IdentifiedObject --- 

  // The global identifier of the object.
  m_rid: Vec<u8>,
  // The description is a human readable text describing or naming the object.
  description: String,
  // Contains the version number of the object. See the type definition for details.
  version: u16,


  // --- these fields come from UsagePointBase --- 

  // Specifies the roles that apply to the usage point.
  role_flags: Vec<u8>,
  // The kind of service provided by this usage point.
  service_category_kind: u8,
  // Specifies the current status of the service at this usage point.
  // 0 = off
  // 1 = on
  status: u8,


  // --- these fields come from UsagePoint --- 

  // The LFDI of the source device. This attribute SHALL be present when mirroring.
  device_lfdi: Vec<u8>,
  meter_reading_list_link: MeterReadingListLink,

}
pub struct UsagePointList {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from SubscribableResource --- 

  // Indicates whether or not subscriptions are supported for this resource, and whether or not conditional (thresholds) are supported. If not specified, is "not subscribable" (0).
  subscribable: u8,


  // --- these fields come from SubscribableList --- 

  // The number specifying "all" of the items in the list. Required on GET, ignored otherwise.
  all: u32,
  // Indicates the number of items in this page of results.
  results: u32,


  // --- these fields come from UsagePointList --- 

  usage_point: UsagePoint,
  // The default polling rate for this function set (this resource and all resources below), in seconds. If not specified, a default of 900 seconds (15 minutes) is used. It is RECOMMENDED a client poll the resources of this function set every pollRate seconds.
  poll_rate: u32,

}
pub struct ConsumptionTariffInterval {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from ConsumptionTariffInterval --- 

  // Indicates the consumption block related to the reading. If not specified, is assumed to be "0 - N/A".
  consumption_block: u8,
  environmental_cost: EnvironmentalCost,
  // The charge for this rate component, per unit of measure defined by the associated ReadingType, in currency specified in TariffProfile. 
  // 
  // The Pricing service provider determines the appropriate price attribute value based on its applicable regulatory rules. For example, price could be net or inclusive of applicable taxes, fees, or levies.
  // 
  // The Billing function set provides the ability to represent billing information in a more detailed manner.
  price: i32,
  // The lowest level of consumption that defines the starting point of this consumption step or block. Thresholds start at zero for each billing period.    
  // 
  // If specified, the first ConsumptionTariffInterval.startValue for a TimeTariffInteral instance SHALL begin at "0." Subsequent ConsumptionTariffInterval.startValue elements SHALL be greater than the previous one.
  start_value: u64,

}
pub struct ConsumptionTariffIntervalList {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from List --- 

  // The number specifying "all" of the items in the list. Required on a response to a GET, ignored otherwise.
  all: u32,
  // Indicates the number of items in this page of results.
  results: u32,


  // --- these fields come from ConsumptionTariffIntervalList --- 

  consumption_tariff_interval: ConsumptionTariffInterval,

}
pub struct EnvironmentalCost {

  // --- these fields come from EnvironmentalCost --- 

  // The estimated or actual environmental or other cost, per commodity unit defined by the ReadingType, for this RateComponent (e.g., grams of carbon dioxide emissions each per kWh).
  amount: u32,
  // The kind of cost referred to in the amount.
  cost_kind: u8,
  // The relative level of the amount attribute.  In conjunction with numCostLevels, this attribute informs a device of the relative scarcity of the amount attribute (e.g., a high or low availability of renewable generation).
  // 
  // numCostLevels and costLevel values SHALL ascend in order of scarcity, where "0" signals the lowest relative cost and higher values signal increasing cost.  For example, if numCostLevels is equal to “3,” then if the lowest relative costLevel were equal to “0,” devices would assume this is the lowest relative period to operate.  Likewise, if the costLevel in the next TimeTariffInterval instance is equal to “1,” then the device would assume it is relatively more expensive, in environmental terms, to operate during this TimeTariffInterval instance than the previous one.
  // 
  // There is no limit to the number of relative price levels other than that indicated in the attribute type, but for practicality, service providers should strive for simplicity and recognize the diminishing returns derived from increasing the numCostLevel value greater than four.
  cost_level: u8,
  // The number of all relative cost levels. 
  // 
  // In conjunction with costLevel, numCostLevels signals the relative scarcity of the commodity for the duration of the TimeTariffInterval instance (e.g., a relative indication of cost). This is useful in providing context for nominal cost signals to consumers or devices that might see a range of amount values from different service providres or from the same service provider.
  num_cost_levels: u8,

}
pub struct RateComponent {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from IdentifiedObject --- 

  // The global identifier of the object.
  m_rid: Vec<u8>,
  // The description is a human readable text describing or naming the object.
  description: String,
  // Contains the version number of the object. See the type definition for details.
  version: u16,


  // --- these fields come from RateComponent --- 

  active_time_tariff_interval_list_link: ActiveTimeTariffIntervalListLink,
  // Specifies the maximum flow rate (e.g. kW for electricity) for which this RateComponent applies, for the usage point and given rate / tariff. 
  // 
  // In combination with flowRateStartLimit, allows a service provider to define the demand or output characteristics for the particular tariff design.  If a server includes the flowRateEndLimit attribute, then it SHALL also include flowRateStartLimit attribute.
  // 
  // For example, a service provider’s tariff limits customers to 20 kWs of demand for the given rate structure.  Above this threshold (from 20-50 kWs), there are different demand charges per unit of consumption.  The service provider can use flowRateStartLimit and flowRateEndLimit to describe the demand characteristics of the different rates.  Similarly, these attributes can be used to describe limits on premises DERs that might be producing a commodity and sending it back into the distribution network. 
  // 
  // Note: At the time of writing, service provider tariffs with demand-based components were not originally identified as being in scope, and service provider tariffs vary widely in their use of demand components and the method for computing charges.  It is expected that industry groups (e.g., OpenSG) will document requirements in the future that the IEEE 2030.5 community can then use as source material for the next version of IEEE 2030.5.
  flow_rate_end_limit: UnitValueType,
  // Specifies the minimum flow rate (e.g., kW for electricity) for which this RateComponent applies, for the usage point and given rate / tariff.
  // 
  // In combination with flowRateEndLimit, allows a service provider to define the demand or output characteristics for the particular tariff design.  If a server includes the flowRateStartLimit attribute, then it SHALL also include flowRateEndLimit attribute.
  flow_rate_start_limit: UnitValueType,
  // Provides indication of the ReadingType with which this price is associated.
  reading_type_link: ReadingTypeLink,
  // Specifies the roles that this usage point has been assigned.
  role_flags: Vec<u8>,
  time_tariff_interval_list_link: TimeTariffIntervalListLink,

}
pub struct RateComponentList {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from List --- 

  // The number specifying "all" of the items in the list. Required on a response to a GET, ignored otherwise.
  all: u32,
  // Indicates the number of items in this page of results.
  results: u32,


  // --- these fields come from RateComponentList --- 

  rate_component: RateComponent,

}
pub struct TariffProfile {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from IdentifiedObject --- 

  // The global identifier of the object.
  m_rid: Vec<u8>,
  // The description is a human readable text describing or naming the object.
  description: String,
  // Contains the version number of the object. See the type definition for details.
  version: u16,


  // --- these fields come from TariffProfile --- 

  // The currency code indicating the currency for this TariffProfile.
  currency: u16,
  // Indicates the power of ten multiplier for the price attribute.
  price_power_of_ten_multiplier: i8,
  // Indicates the relative primacy of the provider of this program.
  primacy: u8,
  // The rate code for this tariff profile.  Provided by the Pricing service provider per its internal business needs and practices and provides a method to identify the specific rate code for the TariffProfile instance.  This would typically not be communicated to the user except to facilitate troubleshooting due to its service provider-specific technical nature.
  rate_code: String,
  rate_component_list_link: RateComponentListLink,
  // The kind of service provided by this usage point.
  service_category_kind: u8,

}
pub struct TariffProfileList {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from SubscribableResource --- 

  // Indicates whether or not subscriptions are supported for this resource, and whether or not conditional (thresholds) are supported. If not specified, is "not subscribable" (0).
  subscribable: u8,


  // --- these fields come from SubscribableList --- 

  // The number specifying "all" of the items in the list. Required on GET, ignored otherwise.
  all: u32,
  // Indicates the number of items in this page of results.
  results: u32,


  // --- these fields come from TariffProfileList --- 

  tariff_profile: TariffProfile,
  // The default polling rate for this function set (this resource and all resources below), in seconds. If not specified, a default of 900 seconds (15 minutes) is used. It is RECOMMENDED a client poll the resources of this function set every pollRate seconds.
  poll_rate: u32,

}
pub struct TimeTariffInterval {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from RespondableResource --- 

  // A reference to the response resource address (URI). Required on a response to a GET if responseRequired is "true".
  reply_to: String,
  // Indicates whether or not a response is required upon receipt, creation or update of this resource. Responses shall be posted to the collection specified in "replyTo". 
  // 
  // If the resource has a deviceCategory field, devices that match one or more of the device types indicated in deviceCategory SHALL respond according to the rules listed below.  If the category does not match, the device SHALL NOT respond. If the resource does not have a deviceCategory field, a device receiving the resource SHALL respond according to the rules listed below.
  // 
  // Value encoded as hex according to the following bit assignments, any combination is possible. 
  // See Table 27 for the list of appropriate Response status codes to be sent for these purposes.
  // 0 - End device shall indicate that message was received 
  // 1 - End device shall indicate specific response. 
  // 2 - End user / customer response is required. 
  // All other values reserved.
  response_required: u8,


  // --- these fields come from RespondableSubscribableIdentifiedObject --- 

  // The global identifier of the object.
  m_rid: Vec<u8>,
  // The description is a human readable text describing or naming the object.
  description: String,
  // Contains the version number of the object. See the type definition for details.
  version: u16,
  // Indicates whether or not subscriptions are supported for this resource, and whether or not conditional (thresholds) are supported. If not specified, is "not subscribable" (0).
  subscribable: u8,


  // --- these fields come from Event --- 

  // The time at which the Event was created.
  creation_time: i64,
  event_status: EventStatus,
  // The period during which the Event applies.
  interval: DateTimeInterval,


  // --- these fields come from RandomizableEvent --- 

  // Number of seconds boundary inside which a random value must be selected to be applied to the associated interval duration, to avoid sudden synchronized demand changes. If related to price level changes, sign may be ignored. Valid range is -3600 to 3600. If not specified, 0 is the default.
  randomize_duration: i16,
  // Number of seconds boundary inside which a random value must be selected to be applied to the associated interval start time, to avoid sudden synchronized demand changes. If related to price level changes, sign may be ignored. Valid range is -3600 to 3600. If not specified, 0 is the default.
  randomize_start: i16,


  // --- these fields come from TimeTariffInterval --- 

  consumption_tariff_interval_list_link: ConsumptionTariffIntervalListLink,
  // Indicates the time of use tier related to the reading. If not specified, is assumed to be "0 - N/A".
  tou_tier: u8,

}
pub struct TimeTariffIntervalList {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from SubscribableResource --- 

  // Indicates whether or not subscriptions are supported for this resource, and whether or not conditional (thresholds) are supported. If not specified, is "not subscribable" (0).
  subscribable: u8,


  // --- these fields come from SubscribableList --- 

  // The number specifying "all" of the items in the list. Required on GET, ignored otherwise.
  all: u32,
  // Indicates the number of items in this page of results.
  results: u32,


  // --- these fields come from TimeTariffIntervalList --- 

  time_tariff_interval: TimeTariffInterval,

}
pub struct MessagingProgram {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from SubscribableResource --- 

  // Indicates whether or not subscriptions are supported for this resource, and whether or not conditional (thresholds) are supported. If not specified, is "not subscribable" (0).
  subscribable: u8,


  // --- these fields come from SubscribableIdentifiedObject --- 

  // The global identifier of the object.
  m_rid: Vec<u8>,
  // The description is a human readable text describing or naming the object.
  description: String,
  // Contains the version number of the object. See the type definition for details.
  version: u16,


  // --- these fields come from MessagingProgram --- 

  active_text_message_list_link: ActiveTextMessageListLink,
  // Indicates the language and region of the messages in this collection.
  locale: String,
  // Indicates the relative primacy of the provider of this program.
  primacy: u8,
  text_message_list_link: TextMessageListLink,

}
pub struct MessagingProgramList {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from SubscribableResource --- 

  // Indicates whether or not subscriptions are supported for this resource, and whether or not conditional (thresholds) are supported. If not specified, is "not subscribable" (0).
  subscribable: u8,


  // --- these fields come from SubscribableList --- 

  // The number specifying "all" of the items in the list. Required on GET, ignored otherwise.
  all: u32,
  // Indicates the number of items in this page of results.
  results: u32,


  // --- these fields come from MessagingProgramList --- 

  messaging_program: MessagingProgram,
  // The default polling rate for this function set (this resource and all resources below), in seconds. If not specified, a default of 900 seconds (15 minutes) is used. It is RECOMMENDED a client poll the resources of this function set every pollRate seconds.
  poll_rate: u32,

}
pub struct TextMessage {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from RespondableResource --- 

  // A reference to the response resource address (URI). Required on a response to a GET if responseRequired is "true".
  reply_to: String,
  // Indicates whether or not a response is required upon receipt, creation or update of this resource. Responses shall be posted to the collection specified in "replyTo". 
  // 
  // If the resource has a deviceCategory field, devices that match one or more of the device types indicated in deviceCategory SHALL respond according to the rules listed below.  If the category does not match, the device SHALL NOT respond. If the resource does not have a deviceCategory field, a device receiving the resource SHALL respond according to the rules listed below.
  // 
  // Value encoded as hex according to the following bit assignments, any combination is possible. 
  // See Table 27 for the list of appropriate Response status codes to be sent for these purposes.
  // 0 - End device shall indicate that message was received 
  // 1 - End device shall indicate specific response. 
  // 2 - End user / customer response is required. 
  // All other values reserved.
  response_required: u8,


  // --- these fields come from RespondableSubscribableIdentifiedObject --- 

  // The global identifier of the object.
  m_rid: Vec<u8>,
  // The description is a human readable text describing or naming the object.
  description: String,
  // Contains the version number of the object. See the type definition for details.
  version: u16,
  // Indicates whether or not subscriptions are supported for this resource, and whether or not conditional (thresholds) are supported. If not specified, is "not subscribable" (0).
  subscribable: u8,


  // --- these fields come from Event --- 

  // The time at which the Event was created.
  creation_time: i64,
  event_status: EventStatus,
  // The period during which the Event applies.
  interval: DateTimeInterval,


  // --- these fields come from TextMessage --- 

  // Indicates the human-readable name of the publisher of the message
  originator: String,
  // The priority is used to inform the client of the priority of the particular message.  Devices with constrained or limited resources for displaying Messages should use this attribute to determine how to handle displaying currently active Messages (e.g. if a device uses a scrolling method with a single Message viewable at a time it MAY want to push a low priority Message to the background and bring a newly received higher priority Message to the foreground).
  priority: u8,
  // The textMessage attribute contains the actual UTF-8 encoded text to be displayed in conjunction with the messageLength attribute which contains the overall length of the textMessage attribute.  Clients and servers SHALL support a reception of a Message of 100 bytes in length.  Messages that exceed the clients display size will be left to the client to choose what method to handle the message (truncation, scrolling, etc.).
  text_message: String,

}
pub struct TextMessageList {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from SubscribableResource --- 

  // Indicates whether or not subscriptions are supported for this resource, and whether or not conditional (thresholds) are supported. If not specified, is "not subscribable" (0).
  subscribable: u8,


  // --- these fields come from SubscribableList --- 

  // The number specifying "all" of the items in the list. Required on GET, ignored otherwise.
  all: u32,
  // Indicates the number of items in this page of results.
  results: u32,


  // --- these fields come from TextMessageList --- 

  text_message: TextMessage,

}
pub struct BillingPeriod {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from BillingPeriod --- 

  // The amount of the bill for the previous billing period.
  bill_last_period: i64,
  // The bill amount related to the billing period as of the statusTimeStamp.
  bill_to_date: i64,
  // The time interval for this billing period.
  interval: DateTimeInterval,
  // The date / time of the last update of this resource.
  status_time_stamp: i64,

}
pub struct BillingPeriodList {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from SubscribableResource --- 

  // Indicates whether or not subscriptions are supported for this resource, and whether or not conditional (thresholds) are supported. If not specified, is "not subscribable" (0).
  subscribable: u8,


  // --- these fields come from SubscribableList --- 

  // The number specifying "all" of the items in the list. Required on GET, ignored otherwise.
  all: u32,
  // Indicates the number of items in this page of results.
  results: u32,


  // --- these fields come from BillingPeriodList --- 

  billing_period: BillingPeriod,

}
pub struct BillingMeterReadingBase {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from IdentifiedObject --- 

  // The global identifier of the object.
  m_rid: Vec<u8>,
  // The description is a human readable text describing or naming the object.
  description: String,
  // Contains the version number of the object. See the type definition for details.
  version: u16,


  // --- these fields come from MeterReadingBase --- 



  // --- these fields come from BillingMeterReadingBase --- 

  billing_reading_set_list_link: BillingReadingSetListLink,
  reading_type_link: ReadingTypeLink,

}
pub struct BillingReading {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from ReadingBase --- 

  // Indicates the consumption block related to the reading. REQUIRED if ReadingType numberOfConsumptionBlocks is non-zero. If not specified, is assumed to be "0 - N/A".
  consumption_block: u8,
  // List of codes indicating the quality of the reading, using specification:
  // 
  // Bit 0 - valid: data that has gone through all required validation checks and either passed them all or has been verified 
  // Bit 1 - manually edited: Replaced or approved by a human
  // Bit 2 - estimated using reference day: data value was replaced by a machine computed value based on analysis of historical data using the same type of measurement.
  // Bit 3 - estimated using linear interpolation: data value was computed using linear interpolation based on the readings before and after it
  // Bit 4 - questionable: data that has failed one or more checks
  // Bit 5 - derived: data that has been calculated (using logic or mathematical operations), not necessarily measured directly 
  // Bit 6 - projected (forecast): data that has been calculated as a projection or forecast of future readings
  quality_flags: Vec<u8>,
  // The time interval associated with the reading. If not specified, then defaults to the intervalLength specified in the associated ReadingType.
  time_period: DateTimeInterval,
  // Indicates the time of use tier related to the reading. REQUIRED if ReadingType numberOfTouTiers is non-zero. If not specified, is assumed to be "0 - N/A".
  tou_tier: u8,
  // Value in units specified by ReadingType
  value: i64,


  // --- these fields come from BillingReading --- 

  charge: Charge,

}
pub struct BillingReadingList {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from List --- 

  // The number specifying "all" of the items in the list. Required on a response to a GET, ignored otherwise.
  all: u32,
  // Indicates the number of items in this page of results.
  results: u32,


  // --- these fields come from BillingReadingList --- 

  billing_reading: BillingReading,

}
pub struct BillingReadingSet {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from IdentifiedObject --- 

  // The global identifier of the object.
  m_rid: Vec<u8>,
  // The description is a human readable text describing or naming the object.
  description: String,
  // Contains the version number of the object. See the type definition for details.
  version: u16,


  // --- these fields come from ReadingSetBase --- 

  // Specifies the time range during which the contained readings were taken.
  time_period: DateTimeInterval,


  // --- these fields come from BillingReadingSet --- 

  billing_reading_list_link: BillingReadingListLink,

}
pub struct BillingReadingSetList {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from SubscribableResource --- 

  // Indicates whether or not subscriptions are supported for this resource, and whether or not conditional (thresholds) are supported. If not specified, is "not subscribable" (0).
  subscribable: u8,


  // --- these fields come from SubscribableList --- 

  // The number specifying "all" of the items in the list. Required on GET, ignored otherwise.
  all: u32,
  // Indicates the number of items in this page of results.
  results: u32,


  // --- these fields come from BillingReadingSetList --- 

  billing_reading_set: BillingReadingSet,

}
pub struct Charge {

  // --- these fields come from Charge --- 

  // A description of the charge.
  description: String,
  // The type (kind) of charge.
  kind: u8,
  // A monetary charge.
  value: i32,

}
pub struct CustomerAccount {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from IdentifiedObject --- 

  // The global identifier of the object.
  m_rid: Vec<u8>,
  // The description is a human readable text describing or naming the object.
  description: String,
  // Contains the version number of the object. See the type definition for details.
  version: u16,


  // --- these fields come from CustomerAccount --- 

  // The ISO 4217 code indicating the currency applicable to the bill amounts in the summary. See list at http://www.unece.org/cefact/recommendations/rec09/rec09_ecetrd203.pdf
  currency: u16,
  // The account number for the customer (if applicable).
  customer_account: String,
  customer_agreement_list_link: CustomerAgreementListLink,
  // The name of the customer.
  customer_name: String,
  // Indicates the power of ten multiplier for the prices in this function set.
  price_power_of_ten_multiplier: i8,
  service_supplier_link: ServiceSupplierLink,

}
pub struct CustomerAccountList {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from SubscribableResource --- 

  // Indicates whether or not subscriptions are supported for this resource, and whether or not conditional (thresholds) are supported. If not specified, is "not subscribable" (0).
  subscribable: u8,


  // --- these fields come from SubscribableList --- 

  // The number specifying "all" of the items in the list. Required on GET, ignored otherwise.
  all: u32,
  // Indicates the number of items in this page of results.
  results: u32,


  // --- these fields come from CustomerAccountList --- 

  customer_account: CustomerAccount,
  // The default polling rate for this function set (this resource and all resources below), in seconds. If not specified, a default of 900 seconds (15 minutes) is used. It is RECOMMENDED a client poll the resources of this function set every pollRate seconds.
  poll_rate: u32,

}
pub struct CustomerAgreement {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from IdentifiedObject --- 

  // The global identifier of the object.
  m_rid: Vec<u8>,
  // The description is a human readable text describing or naming the object.
  description: String,
  // Contains the version number of the object. See the type definition for details.
  version: u16,


  // --- these fields come from CustomerAgreement --- 

  active_billing_period_list_link: ActiveBillingPeriodListLink,
  active_projection_reading_list_link: ActiveProjectionReadingListLink,
  active_target_reading_list_link: ActiveTargetReadingListLink,
  billing_period_list_link: BillingPeriodListLink,
  historical_reading_list_link: HistoricalReadingListLink,
  prepayment_link: PrepaymentLink,
  projection_reading_list_link: ProjectionReadingListLink,
  // The account number of the service account (if applicable).
  service_account: String,
  // The address or textual description of the service location.
  service_location: String,
  target_reading_list_link: TargetReadingListLink,
  tariff_profile_link: TariffProfileLink,
  usage_point_link: UsagePointLink,

}
pub struct CustomerAgreementList {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from SubscribableResource --- 

  // Indicates whether or not subscriptions are supported for this resource, and whether or not conditional (thresholds) are supported. If not specified, is "not subscribable" (0).
  subscribable: u8,


  // --- these fields come from SubscribableList --- 

  // The number specifying "all" of the items in the list. Required on GET, ignored otherwise.
  all: u32,
  // Indicates the number of items in this page of results.
  results: u32,


  // --- these fields come from CustomerAgreementList --- 

  customer_agreement: CustomerAgreement,

}
pub struct HistoricalReading {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from IdentifiedObject --- 

  // The global identifier of the object.
  m_rid: Vec<u8>,
  // The description is a human readable text describing or naming the object.
  description: String,
  // Contains the version number of the object. See the type definition for details.
  version: u16,


  // --- these fields come from MeterReadingBase --- 



  // --- these fields come from BillingMeterReadingBase --- 

  billing_reading_set_list_link: BillingReadingSetListLink,
  reading_type_link: ReadingTypeLink,


  // --- these fields come from HistoricalReading --- 


}
pub struct HistoricalReadingList {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from List --- 

  // The number specifying "all" of the items in the list. Required on a response to a GET, ignored otherwise.
  all: u32,
  // Indicates the number of items in this page of results.
  results: u32,


  // --- these fields come from HistoricalReadingList --- 

  historical_reading: HistoricalReading,

}
pub struct ProjectionReading {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from IdentifiedObject --- 

  // The global identifier of the object.
  m_rid: Vec<u8>,
  // The description is a human readable text describing or naming the object.
  description: String,
  // Contains the version number of the object. See the type definition for details.
  version: u16,


  // --- these fields come from MeterReadingBase --- 



  // --- these fields come from BillingMeterReadingBase --- 

  billing_reading_set_list_link: BillingReadingSetListLink,
  reading_type_link: ReadingTypeLink,


  // --- these fields come from ProjectionReading --- 


}
pub struct ProjectionReadingList {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from List --- 

  // The number specifying "all" of the items in the list. Required on a response to a GET, ignored otherwise.
  all: u32,
  // Indicates the number of items in this page of results.
  results: u32,


  // --- these fields come from ProjectionReadingList --- 

  projection_reading: ProjectionReading,

}
pub struct TargetReading {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from IdentifiedObject --- 

  // The global identifier of the object.
  m_rid: Vec<u8>,
  // The description is a human readable text describing or naming the object.
  description: String,
  // Contains the version number of the object. See the type definition for details.
  version: u16,


  // --- these fields come from MeterReadingBase --- 



  // --- these fields come from BillingMeterReadingBase --- 

  billing_reading_set_list_link: BillingReadingSetListLink,
  reading_type_link: ReadingTypeLink,


  // --- these fields come from TargetReading --- 


}
pub struct TargetReadingList {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from List --- 

  // The number specifying "all" of the items in the list. Required on a response to a GET, ignored otherwise.
  all: u32,
  // Indicates the number of items in this page of results.
  results: u32,


  // --- these fields come from TargetReadingList --- 

  target_reading: TargetReading,

}
pub struct ServiceSupplier {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from IdentifiedObject --- 

  // The global identifier of the object.
  m_rid: Vec<u8>,
  // The description is a human readable text describing or naming the object.
  description: String,
  // Contains the version number of the object. See the type definition for details.
  version: u16,


  // --- these fields come from ServiceSupplier --- 

  // E-mail address for this service supplier.
  email: String,
  // Human-readable phone number for this service supplier.
  phone: String,
  // Contains the IANA PEN for the commodity provider.
  provider_id: u32,
  // Website URI address for this service supplier.
  web: String,

}
pub struct ServiceSupplierList {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from List --- 

  // The number specifying "all" of the items in the list. Required on a response to a GET, ignored otherwise.
  all: u32,
  // Indicates the number of items in this page of results.
  results: u32,


  // --- these fields come from ServiceSupplierList --- 

  service_supplier: ServiceSupplier,

}
pub struct AccountBalance {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from AccountBalance --- 

  // AvailableCredit shows the balance of the sum of credits minus the sum of charges. In a Central Wallet mode this value may be passed down to the Prepayment server via an out-of-band mechanism. In Local or ESI modes, this value may be calculated based upon summation of CreditRegister transactions minus consumption charges calculated using Metering (and possibly Pricing) function set data. This value may be negative; for instance, if disconnection is prevented due to a Supply Interruption Override.
  available_credit: AccountingUnit,
  // CreditStatus identifies whether the present value of availableCredit is considered OK, low, exhausted, or negative.
  credit_status: u8,
  // EmergencyCredit is the amount of credit still available for the given service or commodity prepayment instance. If both availableCredit and emergyCredit are exhausted, then service will typically be disconnected.
  emergency_credit: AccountingUnit,
  // EmergencyCreditStatus identifies whether the present value of emergencyCredit is considered OK, low, exhausted, or negative.
  emergency_credit_status: u8,

}
pub struct AccountingUnit {

  // --- these fields come from AccountingUnit --- 

  // Unit of service.
  energy_unit: RealEnergy,
  // Unit of currency.
  monetary_unit: u16,
  // Multiplier for the 'energyUnit' or 'monetaryUnit'.
  multiplier: i8,
  // Value of the monetary aspect
  value: i32,

}
pub struct CreditRegister {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from IdentifiedObject --- 

  // The global identifier of the object.
  m_rid: Vec<u8>,
  // The description is a human readable text describing or naming the object.
  description: String,
  // Contains the version number of the object. See the type definition for details.
  version: u16,


  // --- these fields come from CreditRegister --- 

  // CreditAmount is the amount of credit being added by a particular CreditRegister transaction. Negative values indicate that credit is being subtracted.
  credit_amount: AccountingUnit,
  // CreditType indicates whether the credit transaction applies to regular or emergency credit.
  credit_type: u8,
  // EffectiveTime identifies the time at which the credit transaction goes into effect. For credit addition transactions, this is typically the moment at which the transaction takes place. For credit subtraction transactions, (e.g., non-fuel debt recovery transactions initiated from a back-haul or ESI) this may be a future time at which credit is deducted.
  effective_time: i64,
  // Token is security data that authenticates the legitimacy of the transaction. The details of this token are not defined by IEEE 2030.5. How a Prepayment server handles this field is left as vendor specific implementation or will be defined by one or more other standards.
  token: String,

}
pub struct CreditRegisterList {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from List --- 

  // The number specifying "all" of the items in the list. Required on a response to a GET, ignored otherwise.
  all: u32,
  // Indicates the number of items in this page of results.
  results: u32,


  // --- these fields come from CreditRegisterList --- 

  credit_register: CreditRegister,

}
pub struct Prepayment {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from IdentifiedObject --- 

  // The global identifier of the object.
  m_rid: Vec<u8>,
  // The description is a human readable text describing or naming the object.
  description: String,
  // Contains the version number of the object. See the type definition for details.
  version: u16,


  // --- these fields come from Prepayment --- 

  account_balance_link: AccountBalanceLink,
  active_credit_register_list_link: ActiveCreditRegisterListLink,
  active_supply_interruption_override_list_link: ActiveSupplyInterruptionOverrideListLink,
  // CreditExpiryLevel is the set point for availableCredit at which the service level may be changed. The typical value for this attribute is 0, regardless of whether the account balance is measured in a monetary or commodity basis. The units for this attribute SHALL match the units used for availableCredit.
  credit_expiry_level: AccountingUnit,
  credit_register_list_link: CreditRegisterListLink,
  // LowCreditWarningLevel is the set point for availableCredit at which the creditStatus attribute in the AccountBalance resource SHALL indicate that available credit is low. The units for this attribute SHALL match the units used for availableCredit. Typically, this value is set by the service provider.
  low_credit_warning_level: AccountingUnit,
  // LowEmergencyCreditWarningLevel is the set point for emergencyCredit at which the creditStatus attribute in the AccountBalance resource SHALL indicate that emergencycredit is low. The units for this attribute SHALL match the units used for availableCredit. Typically, this value is set by the service provider.
  low_emergency_credit_warning_level: AccountingUnit,
  // PrepayMode specifies whether the given Prepayment instance is operating in Credit, Central Wallet, ESI, or Local prepayment mode. The Credit mode indicates that prepayment is not presently in effect. The other modes are described in the Overview Section above.
  prepay_mode: u8,
  prepay_operation_status_link: PrepayOperationStatusLink,
  supply_interruption_override_list_link: SupplyInterruptionOverrideListLink,
  usage_point: UsagePoint,
  usage_point_link: UsagePointLink,

}
pub struct PrepaymentList {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from SubscribableResource --- 

  // Indicates whether or not subscriptions are supported for this resource, and whether or not conditional (thresholds) are supported. If not specified, is "not subscribable" (0).
  subscribable: u8,


  // --- these fields come from SubscribableList --- 

  // The number specifying "all" of the items in the list. Required on GET, ignored otherwise.
  all: u32,
  // Indicates the number of items in this page of results.
  results: u32,


  // --- these fields come from PrepaymentList --- 

  prepayment: Prepayment,
  // The default polling rate for this function set (this resource and all resources below), in seconds. If not specified, a default of 900 seconds (15 minutes) is used. It is RECOMMENDED a client poll the resources of this function set every pollRate seconds.
  poll_rate: u32,

}
pub struct PrepayOperationStatus {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from PrepayOperationStatus --- 

  // CreditTypeChange is used to define a pending change of creditTypeInUse, which will activate at a specified time.
  credit_type_change: CreditTypeChange,
  // CreditTypeInUse identifies whether the present mode of operation is consuming regular credit or emergency credit.
  credit_type_in_use: u8,
  // ServiceChange is used to define a pending change of serviceStatus, which will activate at a specified time.
  service_change: ServiceChange,
  // ServiceStatus identifies whether the service is connected or disconnected, or armed for connection or disconnection.
  service_status: u8,

}
pub struct ServiceChange {

  // --- these fields come from ServiceChange --- 

  // The new service status, to take effect at the time specified by startTime
  new_status: u8,
  // The date/time when the change is to take effect.
  start_time: i64,

}
pub struct SupplyInterruptionOverride {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from SupplyInterruptionOverride --- 

  // The description is a human readable text describing or naming the object.
  description: String,
  // Interval defines the period of time during which supply should not be interrupted.
  interval: DateTimeInterval,

}
pub struct SupplyInterruptionOverrideList {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from List --- 

  // The number specifying "all" of the items in the list. Required on a response to a GET, ignored otherwise.
  all: u32,
  // Indicates the number of items in this page of results.
  results: u32,


  // --- these fields come from SupplyInterruptionOverrideList --- 

  supply_interruption_override: SupplyInterruptionOverride,

}
pub struct CreditTypeChange {

  // --- these fields come from CreditTypeChange --- 

  // The new credit type, to take effect at the time specified by startTime
  new_type: u8,
  // The date/time when the change is to take effect.
  start_time: i64,

}
pub struct RequestStatus {

  // --- these fields come from RequestStatus --- 

  // The dateTime attribute will provide a timestamp of when the request status was set. dateTime MUST be set to the time at which the status change occurred, not a time in the future or past.
  date_time: i64,
  // Field representing the request status type. 
  // 0 = Requested
  // 1 = Cancelled
  // All other values reserved.
  request_status: u8,

}
pub struct FlowReservationRequest {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from IdentifiedObject --- 

  // The global identifier of the object.
  m_rid: Vec<u8>,
  // The description is a human readable text describing or naming the object.
  description: String,
  // Contains the version number of the object. See the type definition for details.
  version: u16,


  // --- these fields come from FlowReservationRequest --- 

  // The time at which the request was created.
  creation_time: i64,
  // A value that is calculated by the storage device that defines the minimum duration, in seconds, that it will take to complete the actual flow transaction, including any ramp times and conditioning times, if applicable.
  duration_requested: u16,
  // Indicates the total amount of energy, in Watt-Hours, requested to be transferred between the storage device and the electric power system. Positive values indicate charging and negative values indicate discharging. This sign convention is different than for the DER function where discharging is positive.  Note that the energyRequestNow attribute in the PowerStatus Object must always represent a charging solution and it is not allowed to have a negative value.
  energy_requested: SignedRealEnergy,
  // The time window during which the flow reservation is needed. For example, if an electric vehicle is set with a 7:00 AM time charge is needed, and price drops to the lowest tier at 11:00 PM, then this window would likely be from 11:00 PM until 7:00 AM.
  interval_requested: DateTimeInterval,
  // Indicates the sustained level of power, in Watts, that is requested. For charging this is calculated by the storage device and it represents the charging system capability (which for an electric vehicle must also account for any power limitations due to the EVSE control pilot). For discharging, a lower value than the inverter capability can be used as a target.
  power_requested: ActivePower,
  request_status: RequestStatus,

}
pub struct FlowReservationRequestList {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from List --- 

  // The number specifying "all" of the items in the list. Required on a response to a GET, ignored otherwise.
  all: u32,
  // Indicates the number of items in this page of results.
  results: u32,


  // --- these fields come from FlowReservationRequestList --- 

  flow_reservation_request: FlowReservationRequest,
  // The default polling rate for this function set (this resource and all resources below), in seconds. If not specified, a default of 900 seconds (15 minutes) is used. It is RECOMMENDED a client poll the resources of this function set every pollRate seconds.
  poll_rate: u32,

}
pub struct FlowReservationResponse {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from RespondableResource --- 

  // A reference to the response resource address (URI). Required on a response to a GET if responseRequired is "true".
  reply_to: String,
  // Indicates whether or not a response is required upon receipt, creation or update of this resource. Responses shall be posted to the collection specified in "replyTo". 
  // 
  // If the resource has a deviceCategory field, devices that match one or more of the device types indicated in deviceCategory SHALL respond according to the rules listed below.  If the category does not match, the device SHALL NOT respond. If the resource does not have a deviceCategory field, a device receiving the resource SHALL respond according to the rules listed below.
  // 
  // Value encoded as hex according to the following bit assignments, any combination is possible. 
  // See Table 27 for the list of appropriate Response status codes to be sent for these purposes.
  // 0 - End device shall indicate that message was received 
  // 1 - End device shall indicate specific response. 
  // 2 - End user / customer response is required. 
  // All other values reserved.
  response_required: u8,


  // --- these fields come from RespondableSubscribableIdentifiedObject --- 

  // The global identifier of the object.
  m_rid: Vec<u8>,
  // The description is a human readable text describing or naming the object.
  description: String,
  // Contains the version number of the object. See the type definition for details.
  version: u16,
  // Indicates whether or not subscriptions are supported for this resource, and whether or not conditional (thresholds) are supported. If not specified, is "not subscribable" (0).
  subscribable: u8,


  // --- these fields come from Event --- 

  // The time at which the Event was created.
  creation_time: i64,
  event_status: EventStatus,
  // The period during which the Event applies.
  interval: DateTimeInterval,


  // --- these fields come from FlowReservationResponse --- 

  // Indicates the amount of energy available.
  energy_available: SignedRealEnergy,
  // Indicates the amount of power available.
  power_available: ActivePower,
  // The subject field provides a method to match the response with the originating event. It is populated with the mRID of the corresponding FlowReservationRequest object.
  subject: Vec<u8>,

}
pub struct FlowReservationResponseList {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from SubscribableResource --- 

  // Indicates whether or not subscriptions are supported for this resource, and whether or not conditional (thresholds) are supported. If not specified, is "not subscribable" (0).
  subscribable: u8,


  // --- these fields come from SubscribableList --- 

  // The number specifying "all" of the items in the list. Required on GET, ignored otherwise.
  all: u32,
  // Indicates the number of items in this page of results.
  results: u32,


  // --- these fields come from FlowReservationResponseList --- 

  flow_reservation_response: FlowReservationResponse,
  // The default polling rate for this function set (this resource and all resources below), in seconds. If not specified, a default of 900 seconds (15 minutes) is used. It is RECOMMENDED a client poll the resources of this function set every pollRate seconds.
  poll_rate: u32,

}
pub struct DefaultDerControl {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from SubscribableResource --- 

  // Indicates whether or not subscriptions are supported for this resource, and whether or not conditional (thresholds) are supported. If not specified, is "not subscribable" (0).
  subscribable: u8,


  // --- these fields come from SubscribableIdentifiedObject --- 

  // The global identifier of the object.
  m_rid: Vec<u8>,
  // The description is a human readable text describing or naming the object.
  description: String,
  // Contains the version number of the object. See the type definition for details.
  version: u16,


  // --- these fields come from DefaultDERControl --- 

  der_control_base: DERControlBase,
  // Enter service delay, in hundredths of a second. When present, this value SHALL update the value of the corresponding setting (DERSettings::setESDelay).
  set_es_delay: u32,
  // Enter service frequency high. Specified in hundredths of Hz. When present, this value SHALL update the value of the corresponding setting (DERSettings::setESHighFreq).
  set_es_high_freq: u16,
  // Enter service voltage high. Specified as an effective percent voltage, defined as (100% * (locally measured voltage - setVRefOfs) / setVRef), in hundredths of a percent. When present, this value SHALL update the value of the corresponding setting (DERSettings::setESHighVolt).
  set_es_high_volt: i16,
  // Enter service frequency low. Specified in hundredths of Hz. When present, this value SHALL update the value of the corresponding setting (DERSettings::setESLowFreq).
  set_es_low_freq: u16,
  // Enter service voltage low. Specified as an effective percent voltage, defined as (100% * (locally measured voltage - setVRefOfs) / setVRef), in hundredths of a percent. When present, this value SHALL update the value of the corresponding setting (DERSettings::setESLowVolt).
  set_es_low_volt: i16,
  // Enter service ramp time, in hundredths of a second. When present, this value SHALL update the value of the corresponding setting (DERSettings::setESRampTms).
  set_es_ramp_tms: u32,
  // Enter service randomized delay, in hundredths of a second. When present, this value SHALL update the value of the corresponding setting (DERSettings::setESRandomDelay).
  set_es_random_delay: u32,
  // Set default rate of change (ramp rate) of active power output due to command or internal action, defined in %setWMax / second.  Resolution is in hundredths of a percent/second. A value of 0 means there is no limit. Interpreted as a percentage change in output capability limit per second when used as a default ramp rate. When present, this value SHALL update the value of the corresponding setting (DERSettings::setGradW).
  set_grad_w: u16,
  // Set soft-start rate of change (soft-start ramp rate) of active power output due to command or internal action, defined in %setWMax / second.  Resolution is in hundredths of a percent/second. A value of 0 means there is no limit. Interpreted as a percentage change in output capability limit per second when used as a ramp rate. When present, this value SHALL update the value of the corresponding setting (DERSettings::setSoftGradW).
  set_soft_grad_w: u16,

}
pub struct FreqDroopType {

  // --- these fields come from FreqDroopType --- 

  // Frequency droop dead band for over-frequency conditions. In thousandths of Hz.
  d_bof: u32,
  // Frequency droop dead band for under-frequency conditions. In thousandths of Hz.
  d_buf: u32,
  // Frequency droop per-unit frequency change for over-frequency conditions corresponding to 1 per-unit power output change. In thousandths, unitless.
  k_of: u16,
  // Frequency droop per-unit frequency change for under-frequency conditions corresponding to 1 per-unit power output change. In thousandths, unitless.
  k_uf: u16,
  // Open loop response time, the duration from a step change in control signal input until the output changes by 90% of its final change before any overshoot, in hundredths of a second. Resolution is 1/100 sec. A value of 0 is used to mean no limit.
  open_loop_tms: u16,

}
pub struct Der {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from SubscribableResource --- 

  // Indicates whether or not subscriptions are supported for this resource, and whether or not conditional (thresholds) are supported. If not specified, is "not subscribable" (0).
  subscribable: u8,


  // --- these fields come from DER --- 

  associated_der_program_list_link: AssociatedDERProgramListLink,
  associated_usage_point_link: AssociatedUsagePointLink,
  current_der_program_link: CurrentDERProgramLink,
  der_availability_link: DERAvailabilityLink,
  der_capability_link: DERCapabilityLink,
  der_settings_link: DERSettingsLink,
  der_status_link: DERStatusLink,

}
pub struct DerList {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from List --- 

  // The number specifying "all" of the items in the list. Required on a response to a GET, ignored otherwise.
  all: u32,
  // Indicates the number of items in this page of results.
  results: u32,


  // --- these fields come from DERList --- 

  der: DER,
  // The default polling rate for this function set (this resource and all resources below), in seconds. If not specified, a default of 900 seconds (15 minutes) is used. It is RECOMMENDED a client poll the resources of this function set every pollRate seconds.
  poll_rate: u32,

}
pub struct DerSettings {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from SubscribableResource --- 

  // Indicates whether or not subscriptions are supported for this resource, and whether or not conditional (thresholds) are supported. If not specified, is "not subscribable" (0).
  subscribable: u8,


  // --- these fields come from DERSettings --- 

  // Bitmap indicating the DER Controls enabled on the device. See DERControlType for values. If a control is supported (see DERCapability::modesSupported), but not enabled, the control will not be executed if encountered.
  modes_enabled: Vec<u8>,
  // Enter service delay, in hundredths of a second.
  set_es_delay: u32,
  // Enter service frequency high. Specified in hundredths of Hz.
  set_es_high_freq: u16,
  // Enter service voltage high. Specified as an effective percent voltage, defined as (100% * (locally measured voltage - setVRefOfs) / setVRef), in hundredths of a percent.
  set_es_high_volt: i16,
  // Enter service frequency low. Specified in hundredths of Hz.
  set_es_low_freq: u16,
  // Enter service voltage low. Specified as an effective percent voltage, defined as (100% * (locally measured voltage - setVRefOfs) / setVRef), in hundredths of a percent.
  set_es_low_volt: i16,
  // Enter service ramp time, in hundredths of a second.
  set_es_ramp_tms: u32,
  // Enter service randomized delay, in hundredths of a second.
  set_es_random_delay: u32,
  // Set default rate of change (ramp rate) of active power output due to command or internal action, defined in %setWMax / second.  Resolution is in hundredths of a percent/second. A value of 0 means there is no limit. Interpreted as a percentage change in output capability limit per second when used as a default ramp rate.
  set_grad_w: u16,
  // AC current maximum. Maximum AC current in RMS Amperes.
  set_max_a: CurrentRMS,
  // Maximum usable energy storage capacity of the DER, in AmpHours. Note: this may be different from physical capability.
  set_max_ah: AmpereHour,
  // Apparent power charge maximum. Maximum apparent power the DER can absorb from the grid in Volt-Amperes. May differ from the apparent power maximum (setMaxVA).
  set_max_charge_rate_va: ApparentPower,
  // Maximum rate of energy transfer received by the storage device, in Watts. Defaults to rtgMaxChargeRateW.
  set_max_charge_rate_w: ActivePower,
  // Apparent power discharge maximum. Maximum apparent power the DER can deliver to the grid in Volt-Amperes. May differ from the apparent power maximum (setMaxVA).
  set_max_discharge_rate_va: ApparentPower,
  // Maximum rate of energy transfer delivered by the storage device, in Watts. Defaults to rtgMaxDischargeRateW.
  set_max_discharge_rate_w: ActivePower,
  // AC voltage maximum setting.
  set_max_v: VoltageRMS,
  // Set limit for maximum apparent power capability of the DER (in VA). Defaults to rtgMaxVA.
  set_max_va: ApparentPower,
  // Set limit for maximum reactive power delivered by the DER (in var). SHALL be a positive value &lt;= rtgMaxVar (default).
  set_max_var: ReactivePower,
  // Set limit for maximum reactive power received by the DER (in var). If present, SHALL be a negative value &gt;= rtgMaxVarNeg (default). If absent, defaults to negative setMaxVar.
  set_max_var_neg: ReactivePower,
  // Set limit for maximum active power capability of the DER (in W). Defaults to rtgMaxW.
  set_max_w: ActivePower,
  // Maximum energy storage capacity of the DER, in WattHours. Note: this may be different from physical capability.
  set_max_wh: WattHour,
  // Set minimum Power Factor displacement limit of the DER when injecting reactive power (over-excited); SHALL be a positive value between 0.0 (typically &gt; 0.7) and 1.0.  SHALL be &gt;= rtgMinPFOverExcited (default).
  set_min_pf_over_excited: PowerFactor,
  // Set minimum Power Factor displacement limit of the DER when absorbing reactive power (under-excited); SHALL be a positive value between 0.0 (typically &gt; 0.7) and 0.9999.  If present, SHALL be &gt;= rtgMinPFUnderExcited (default).  If absent, defaults to setMinPFOverExcited.
  set_min_pf_under_excited: PowerFactor,
  // AC voltage minimum setting.
  set_min_v: VoltageRMS,
  // Set soft-start rate of change (soft-start ramp rate) of active power output due to command or internal action, defined in %setWMax / second.  Resolution is in hundredths of a percent/second. A value of 0 means there is no limit. Interpreted as a percentage change in output capability limit per second when used as a ramp rate.
  set_soft_grad_w: u16,
  // AC voltage nominal setting.
  set_v_nom: VoltageRMS,
  // The nominal AC voltage (RMS) at the utility's point of common coupling.
  set_v_ref: VoltageRMS,
  // The nominal AC voltage (RMS) offset between the DER's electrical connection point and the utility's point of common coupling.
  set_v_ref_ofs: VoltageRMS,
  // Specifies the time at which the DER information was last updated.
  updated_time: i64,

}
pub struct DerAvailability {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from SubscribableResource --- 

  // Indicates whether or not subscriptions are supported for this resource, and whether or not conditional (thresholds) are supported. If not specified, is "not subscribable" (0).
  subscribable: u8,


  // --- these fields come from DERAvailability --- 

  // Indicates number of seconds the DER will be able to deliver active power at the reservePercent level.
  availability_duration: u32,
  // Indicates number of seconds the DER will be able to receive active power at the reserveChargePercent level.
  max_charge_duration: u32,
  // The timestamp when the DER availability was last updated.
  reading_time: i64,
  // Percent of continuous received active power (%setMaxChargeRateW) that is estimated to be available in reserve.
  reserve_charge_percent: u16,
  // Percent of continuous delivered active power (%setMaxW) that is estimated to be available in reserve.
  reserve_percent: u16,
  // Estimated reserve reactive power, in var.  Represents the lesser of received or delivered reactive power.
  stat_var_avail: ReactivePower,
  // Estimated reserve active power, in watts.
  stat_w_avail: ActivePower,

}
pub struct DerCapability {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from DERCapability --- 

  // Bitmap indicating the DER Controls implemented by the device. See DERControlType for values.
  modes_supported: Vec<u8>,
  // Abnormal operating performance category as defined by IEEE 1547-2018. One of:
  // 0 - not specified
  // 1 - Category I
  // 2 - Category II
  // 3 - Category III
  // All other values reserved.
  rtg_abnormal_category: u8,
  // Maximum continuous AC current capability of the DER, in Amperes (RMS).
  rtg_max_a: CurrentRMS,
  // Usable energy storage capacity of the DER, in AmpHours.
  rtg_max_ah: AmpereHour,
  // Maximum apparent power charge rating in Volt-Amperes. May differ from the maximum apparent power rating.
  rtg_max_charge_rate_va: ApparentPower,
  // Maximum rate of energy transfer received by the storage DER, in Watts.
  rtg_max_charge_rate_w: ActivePower,
  // Maximum apparent power discharge rating in Volt-Amperes. May differ from the maximum apparent power rating.
  rtg_max_discharge_rate_va: ApparentPower,
  // Maximum rate of energy transfer delivered by the storage DER, in Watts. Required for combined generation/storage DERs (e.g. DERType == 83).
  rtg_max_discharge_rate_w: ActivePower,
  // AC voltage maximum rating.
  rtg_max_v: VoltageRMS,
  // Maximum continuous apparent power output capability of the DER, in VA.
  rtg_max_va: ApparentPower,
  // Maximum continuous reactive power delivered by the DER, in var.
  rtg_max_var: ReactivePower,
  // Maximum continuous reactive power received by the DER, in var.  If absent, defaults to negative rtgMaxVar.
  rtg_max_var_neg: ReactivePower,
  // Maximum continuous active power output capability of the DER, in watts. Represents combined generation plus storage output if DERType == 83.
  rtg_max_w: ActivePower,
  // Maximum energy storage capacity of the DER, in WattHours.
  rtg_max_wh: WattHour,
  // Minimum Power Factor displacement capability of the DER when injecting reactive power (over-excited); SHALL be a positive value between 0.0 (typically &gt; 0.7) and 1.0. If absent, defaults to unity.
  rtg_min_pf_over_excited: PowerFactor,
  // Minimum Power Factor displacement capability of the DER when absorbing reactive power (under-excited); SHALL be a positive value between 0.0 (typically &gt; 0.7) and 0.9999.  If absent, defaults to rtgMinPFOverExcited.
  rtg_min_pf_under_excited: PowerFactor,
  // AC voltage minimum rating.
  rtg_min_v: VoltageRMS,
  // Normal operating performance category as defined by IEEE 1547-2018. One of:
  // 0 - not specified
  // 1 - Category A
  // 2 - Category B
  // All other values reserved.
  rtg_normal_category: u8,
  // Specified over-excited power factor.
  rtg_over_excited_pf: PowerFactor,
  // Active power rating in Watts at specified over-excited power factor (rtgOverExcitedPF). If present, rtgOverExcitedPF SHALL be present.
  rtg_over_excited_w: ActivePower,
  // Reactive susceptance that remains connected to the Area EPS in the cease to energize and trip state.
  rtg_reactive_susceptance: ReactiveSusceptance,
  // Specified under-excited power factor.
  rtg_under_excited_pf: PowerFactor,
  // Active power rating in Watts at specified under-excited power factor (rtgUnderExcitedPF). If present, rtgUnderExcitedPF SHALL be present.
  rtg_under_excited_w: ActivePower,
  // AC voltage nominal rating.
  rtg_v_nom: VoltageRMS,
  // Type of DER; see DERType object
  type: u8,

}
pub struct DerControlBase {

  // --- these fields come from DERControlBase --- 

  // Set DER as connected (true) or disconnected (false). Used in conjunction with ramp rate when re-connecting. Implies galvanic isolation.
  op_mod_connect: bool,
  // Set DER as energized (true) or de-energized (false). Used in conjunction with ramp rate when re-energizing.
  op_mod_energize: bool,
  // The opModFixedPFAbsorbW function specifies a requested fixed Power Factor (PF) setting for when active power is being absorbed. The actual displacement SHALL be within the limits established by setMinPFOverExcited and setMinPFUnderExcited. If issued simultaneously with other reactive power controls (e.g. opModFixedVar) the control resulting in least var magnitude SHOULD take precedence.
  op_mod_fixed_pf_absorb_w: PowerFactorWithExcitation,
  // The opModFixedPFInjectW function specifies a requested fixed Power Factor (PF) setting for when active power is being injected. The actual displacement SHALL be within the limits established by setMinPFOverExcited and setMinPFUnderExcited. If issued simultaneously with other reactive power controls (e.g. opModFixedVar) the control resulting in least var magnitude SHOULD take precedence.
  op_mod_fixed_pf_inject_w: PowerFactorWithExcitation,
  // The opModFixedVar function specifies the delivered or received reactive power setpoint.  The context for the setpoint value is determined by refType and SHALL be one of %setMaxW, %setMaxVar, or %statVarAvail.  If issued simultaneously with other reactive power controls (e.g. opModFixedPFInjectW) the control resulting in least var magnitude SHOULD take precedence.
  op_mod_fixed_var: FixedVar,
  // The opModFixedW function specifies a requested charge or discharge mode setpoint, in %setMaxChargeRateW if negative value or %setMaxW or %setMaxDischargeRateW if positive value (in hundredths).
  op_mod_fixed_w: i16,
  // Specifies a frequency-watt operation. This operation limits active power generation or consumption when the line frequency deviates from nominal by a specified amount.
  op_mod_freq_droop: FreqDroopType,
  // Specify DERCurveLink for curveType == 0.  The Frequency-Watt function limits active power generation or consumption when the line frequency deviates from nominal by a specified amount. The Frequency-Watt curve is specified as an array of Frequency-Watt pairs that are interpolated into a piecewise linear function with hysteresis.  The x value of each pair specifies a frequency in Hz. The y value specifies a corresponding active power output in %setMaxW.
  op_mod_freq_watt: DERCurveLink,
  // Specify DERCurveLink for curveType == 1. The High Frequency Ride-Through (HFRT) function is specified by one or two duration-frequency curves that define the operating region under high frequency conditions. Each HFRT curve is specified by an array of duration-frequency pairs that will be interpolated into a piecewise linear function that defines an operating region. The x value of each pair specifies a duration (time at a given frequency in seconds). The y value of each pair specifies a frequency, in Hz. This control specifies the "may trip" region.
  op_mod_hfrt_may_trip: DERCurveLink,
  // Specify DERCurveLink for curveType == 2.  The High Frequency Ride-Through (HFRT) function is specified by a duration-frequency curve that defines the operating region under high frequency conditions.  Each HFRT curve is specified by an array of duration-frequency pairs that will be interpolated into a piecewise linear function that defines an operating region.  The x value of each pair specifies a duration (time at a given frequency in seconds). The y value of each pair specifies a frequency, in Hz. This control specifies the "must trip" region.
  op_mod_hfrt_must_trip: DERCurveLink,
  // Specify DERCurveLink for curveType == 3. The High Voltage Ride-Through (HVRT) function is specified by one, two, or three duration-volt curves that define the operating region under high voltage conditions. Each HVRT curve is specified by an array of duration-volt pairs that will be interpolated into a piecewise linear function that defines an operating region. The x value of each pair specifies a duration (time at a given voltage in seconds). The y value of each pair specifies an effective percentage voltage, defined as ((locally measured voltage - setVRefOfs / setVRef). This control specifies the "may trip" region.
  op_mod_hvrt_may_trip: DERCurveLink,
  // Specify DERCurveLink for curveType == 4.  The High Voltage Ride-Through (HVRT) function is specified by duration-volt curves that define the operating region under high voltage conditions.  Each HVRT curve is specified by an array of duration-volt pairs that will be interpolated into a piecewise linear function that defines an operating region.  The x value of each pair specifies a duration (time at a given voltage in seconds). The y value of each pair specifies an effective percent voltage, defined as ((locally measured voltage - setVRefOfs) / setVRef). This control specifies the "momentary cessation" region.
  op_mod_hvrt_momentary_cessation: DERCurveLink,
  // Specify DERCurveLink for curveType == 5.  The High Voltage Ride-Through (HVRT) function is specified by duration-volt curves that define the operating region under high voltage conditions.  Each HVRT curve is specified by an array of duration-volt pairs that will be interpolated into a piecewise linear function that defines an operating region.  The x value of each pair specifies a duration (time at a given voltage in seconds). The y value of each pair specifies an effective percent voltage, defined as ((locally measured voltage - setVRefOfs) / setVRef). This control specifies the "must trip" region.
  op_mod_hvrt_must_trip: DERCurveLink,
  // Specify DERCurveLink for curveType == 6. The Low Frequency Ride-Through (LFRT) function is specified by one or two duration-frequency curves that define the operating region under low frequency conditions. Each LFRT curve is specified by an array of duration-frequency pairs that will be interpolated into a piecewise linear function that defines an operating region. The x value of each pair specifies a duration (time at a given frequency in seconds). The y value of each pair specifies a frequency, in Hz. This control specifies the "may trip" region.
  op_mod_lfrt_may_trip: DERCurveLink,
  // Specify DERCurveLink for curveType == 7.  The Low Frequency Ride-Through (LFRT) function is specified by a duration-frequency curve that defines the operating region under low frequency conditions.  Each LFRT curve is specified by an array of duration-frequency pairs that will be interpolated into a piecewise linear function that defines an operating region.  The x value of each pair specifies a duration (time at a given frequency in seconds). The y value of each pair specifies a frequency, in Hz. This control specifies the "must trip" region.
  op_mod_lfrt_must_trip: DERCurveLink,
  // Specify DERCurveLink for curveType == 8. The Low Voltage Ride-Through (LVRT) function is specified by one, two, or three duration-volt curves that define the operating region under low voltage conditions. Each LVRT curve is specified by an array of duration-volt pairs that will be interpolated into a piecewise linear function that defines an operating region. The x value of each pair specifies a duration (time at a given voltage in seconds). The y value of each pair specifies an effective percent voltage, defined as ((locally measured voltage - setVRefOfs) / setVRef). This control specifies the "may trip" region.
  op_mod_lvrt_may_trip: DERCurveLink,
  // Specify DERCurveLink for curveType == 9.  The Low Voltage Ride-Through (LVRT) function is specified by duration-volt curves that define the operating region under low voltage conditions.  Each LVRT curve is specified by an array of duration-volt pairs that will be interpolated into a piecewise linear function that defines an operating region.  The x value of each pair specifies a duration (time at a given voltage in seconds). The y value of each pair specifies an effective percent voltage, defined as ((locally measured voltage - setVRefOfs) / setVRef). This control specifies the "momentary cessation" region.
  op_mod_lvrt_momentary_cessation: DERCurveLink,
  // Specify DERCurveLink for curveType == 10.  The Low Voltage Ride-Through (LVRT) function is specified by duration-volt curves that define the operating region under low voltage conditions.  Each LVRT curve is specified by an array of duration-volt pairs that will be interpolated into a piecewise linear function that defines an operating region.  The x value of each pair specifies a duration (time at a given voltage in seconds). The y value of each pair specifies an effective percent voltage, defined as ((locally measured voltage - setVRefOfs) / setVRef). This control specifies the "must trip" region.
  op_mod_lvrt_must_trip: DERCurveLink,
  // The opModMaxLimW function sets the maximum active power generation level at the electrical coupling point as a percentage of set capacity (%setMaxW, in hundredths). This limitation may be met e.g. by reducing PV output or by using excess PV output to charge associated storage.
  op_mod_max_lim_w: u16,
  // Target reactive power, in var. This control is likely to be more useful for aggregators, as individual DERs may not be able to maintain a target setting.
  op_mod_target_var: ReactivePower,
  // Target output power, in Watts. This control is likely to be more useful for aggregators, as individual DERs may not be able to maintain a target setting.
  op_mod_target_w: ActivePower,
  // Specify DERCurveLink for curveType == 11.  The static volt-var function provides over- or under-excited var compensation as a function of measured voltage. The volt-var curve is specified as an array of volt-var pairs that are interpolated into a piecewise linear function with hysteresis. The x value of each pair specifies an effective percent voltage, defined as ((locally measured voltage - setVRefOfs) / setVRef) and SHOULD support a domain of at least 0 - 135. If VRef is present in DERCurve, then the x value of each pair is additionally multiplied by (VRef / 10000). The y value specifies a target var output interpreted as a signed percentage (-100 to 100). The meaning of the y value is determined by yRefType and must be one of %setMaxW, %setMaxVar, or %statVarAvail.
  op_mod_volt_var: DERCurveLink,
  // Specify DERCurveLink for curveType == 12.  The Volt-Watt reduces active power output as a function of measured voltage. The Volt-Watt curve is specified as an array of Volt-Watt pairs that are interpolated into a piecewise linear function with hysteresis. The x value of each pair specifies an effective percent voltage, defined as ((locally measured voltage - setVRefOfs) / setVRef) and SHOULD support a domain of at least 0 - 135. The y value specifies an active power output interpreted as a percentage (0 - 100). The meaning of the y value is determined by yRefType and must be one of %setMaxW or %statWAvail.
  op_mod_volt_watt: DERCurveLink,
  // Specify DERCurveLink for curveType == 13.  The Watt-PF function varies Power Factor (PF) as a function of delivered active power. The Watt-PF curve is specified as an array of Watt-PF coordinates that are interpolated into a piecewise linear function with hysteresis.  The x value of each pair specifies a watt setting in %setMaxW, (0 - 100). The PF output setting is a signed displacement in y value (PF sign SHALL be interpreted according to the EEI convention, where unity PF is considered unsigned). These settings are not expected to be updated very often during the life of the installation, therefore only a single curve is required.  If issued simultaneously with other reactive power controls (e.g. opModFixedPFInjectW) the control resulting in least var magnitude SHOULD take precedence.
  op_mod_watt_pf: DERCurveLink,
  // Specify DERCurveLink for curveType == 14. The Watt-Var function varies vars as a function of delivered active power. The Watt-Var curve is specified as an array of Watt-Var pairs that are interpolated into a piecewise linear function with hysteresis. The x value of each pair specifies a watt setting in %setMaxW, (0-100). The y value specifies a target var output interpreted as a signed percentage (-100 to 100). The meaning of the y value is determined by yRefType and must be one of %setMaxW, %setMaxVar, or %statVarAvail.
  op_mod_watt_var: DERCurveLink,
  // Requested ramp time, in hundredths of a second, for the device to transition from the current DERControl  mode setting(s) to the new mode setting(s). If absent, use default ramp rate (setGradW).  Resolution is 1/100 sec.
  ramp_tms: u16,

}
pub struct DerControl {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from RespondableResource --- 

  // A reference to the response resource address (URI). Required on a response to a GET if responseRequired is "true".
  reply_to: String,
  // Indicates whether or not a response is required upon receipt, creation or update of this resource. Responses shall be posted to the collection specified in "replyTo". 
  // 
  // If the resource has a deviceCategory field, devices that match one or more of the device types indicated in deviceCategory SHALL respond according to the rules listed below.  If the category does not match, the device SHALL NOT respond. If the resource does not have a deviceCategory field, a device receiving the resource SHALL respond according to the rules listed below.
  // 
  // Value encoded as hex according to the following bit assignments, any combination is possible. 
  // See Table 27 for the list of appropriate Response status codes to be sent for these purposes.
  // 0 - End device shall indicate that message was received 
  // 1 - End device shall indicate specific response. 
  // 2 - End user / customer response is required. 
  // All other values reserved.
  response_required: u8,


  // --- these fields come from RespondableSubscribableIdentifiedObject --- 

  // The global identifier of the object.
  m_rid: Vec<u8>,
  // The description is a human readable text describing or naming the object.
  description: String,
  // Contains the version number of the object. See the type definition for details.
  version: u16,
  // Indicates whether or not subscriptions are supported for this resource, and whether or not conditional (thresholds) are supported. If not specified, is "not subscribable" (0).
  subscribable: u8,


  // --- these fields come from Event --- 

  // The time at which the Event was created.
  creation_time: i64,
  event_status: EventStatus,
  // The period during which the Event applies.
  interval: DateTimeInterval,


  // --- these fields come from RandomizableEvent --- 

  // Number of seconds boundary inside which a random value must be selected to be applied to the associated interval duration, to avoid sudden synchronized demand changes. If related to price level changes, sign may be ignored. Valid range is -3600 to 3600. If not specified, 0 is the default.
  randomize_duration: i16,
  // Number of seconds boundary inside which a random value must be selected to be applied to the associated interval start time, to avoid sudden synchronized demand changes. If related to price level changes, sign may be ignored. Valid range is -3600 to 3600. If not specified, 0 is the default.
  randomize_start: i16,


  // --- these fields come from DERControl --- 

  der_control_base: DERControlBase,
  // Specifies the bitmap indicating  the categories of devices that SHOULD respond. Devices SHOULD ignore events that do not indicate their device category. If not present, all devices SHOULD respond.
  device_category: Vec<u8>,

}
pub struct DerControlList {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from SubscribableResource --- 

  // Indicates whether or not subscriptions are supported for this resource, and whether or not conditional (thresholds) are supported. If not specified, is "not subscribable" (0).
  subscribable: u8,


  // --- these fields come from SubscribableList --- 

  // The number specifying "all" of the items in the list. Required on GET, ignored otherwise.
  all: u32,
  // Indicates the number of items in this page of results.
  results: u32,


  // --- these fields come from DERControlList --- 

  der_control: DERControl,

}
pub struct DerCurve {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from IdentifiedObject --- 

  // The global identifier of the object.
  m_rid: Vec<u8>,
  // The description is a human readable text describing or naming the object.
  description: String,
  // Contains the version number of the object. See the type definition for details.
  version: u16,


  // --- these fields come from DERCurve --- 

  // If the curveType is opModVoltVar, then this field MAY be present. If the curveType is not opModVoltVar, then this field SHALL NOT be present. Enable/disable autonomous vRef adjustment. When enabled, the Volt-Var curve characteristic SHALL be adjusted autonomously as vRef changes and autonomousVRefTimeConstant SHALL be present. If a DER is able to support Volt-Var mode but is unable to support autonomous vRef adjustment, then the DER SHALL execute the curve without autonomous vRef adjustment. If not specified, then the value is false.
  autonomous_v_ref_enable: bool,
  // If the curveType is opModVoltVar, then this field MAY be present. If the curveType is not opModVoltVar, then this field SHALL NOT be present. Adjustment range for vRef time constant, in hundredths of a second.
  autonomous_v_ref_time_constant: u32,
  // The time at which the object was created.
  creation_time: i64,
  curve_data: CurveData,
  // Specifies the associated curve-based control mode.
  curve_type: u8,
  // Open loop response time, the time to ramp up to 90% of the new target in response to the change in voltage, in hundredths of a second. Resolution is 1/100 sec. A value of 0 is used to mean no limit. When not present, the device SHOULD follow its default behavior.
  open_loop_tms: u16,
  // Decreasing ramp rate, interpreted as a percentage change in output capability limit per second (e.g. %setMaxW / sec).  Resolution is in hundredths of a percent/second. A value of 0 means there is no limit. If absent, ramp rate defaults to setGradW.
  ramp_dec_tms: u16,
  // Increasing ramp rate, interpreted as a percentage change in output capability limit per second (e.g. %setMaxW / sec).  Resolution is in hundredths of a percent/second. A value of 0 means there is no limit. If absent, ramp rate defaults to rampDecTms.
  ramp_inc_tms: u16,
  // The configuration parameter for a low-pass filter, PT1 is a time, in hundredths of a second, in which the filter will settle to 95% of a step change in the input value. Resolution is 1/100 sec.
  ramp_pt1_tms: u16,
  // If the curveType is opModVoltVar, then this field MAY be present. If the curveType is not opModVoltVar, then this field SHALL NOT be present. The nominal AC voltage (RMS) adjustment to the voltage curve points for Volt-Var curves.
  v_ref: u16,
  // Exponent for X-axis value.
  x_multiplier: i8,
  // Exponent for Y-axis value.
  y_multiplier: i8,
  // The Y-axis units context.
  y_ref_type: u8,

}
pub struct CurrentDerProgramLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from CurrentDERProgramLink --- 


}
pub struct DerCurveList {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from List --- 

  // The number specifying "all" of the items in the list. Required on a response to a GET, ignored otherwise.
  all: u32,
  // Indicates the number of items in this page of results.
  results: u32,


  // --- these fields come from DERCurveList --- 

  der_curve: DERCurve,

}
pub struct CurveData {

  // --- these fields come from CurveData --- 

  // If yvalue is Power Factor, then this field SHALL be present. If yvalue is not Power Factor, then this field SHALL NOT be present.
  // True when DER is absorbing reactive power (under-excited), false
  // when DER is injecting reactive power (over-excited).
  excitation: bool,
  // The data value of the X-axis (independent) variable, depending on the curve type. See definitions in DERControlBase for further information.
  xvalue: i32,
  // The data value of the Y-axis (dependent) variable, depending on the curve type. See definitions in DERControlBase for further information. If yvalue is Power Factor, the excitation field SHALL be present and yvalue SHALL be a positive value. If yvalue is not Power Factor, the excitation field SHALL NOT be present.
  yvalue: i32,

}
pub struct DerProgram {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from SubscribableResource --- 

  // Indicates whether or not subscriptions are supported for this resource, and whether or not conditional (thresholds) are supported. If not specified, is "not subscribable" (0).
  subscribable: u8,


  // --- these fields come from SubscribableIdentifiedObject --- 

  // The global identifier of the object.
  m_rid: Vec<u8>,
  // The description is a human readable text describing or naming the object.
  description: String,
  // Contains the version number of the object. See the type definition for details.
  version: u16,


  // --- these fields come from DERProgram --- 

  active_der_control_list_link: ActiveDERControlListLink,
  default_der_control_link: DefaultDERControlLink,
  der_control_list_link: DERControlListLink,
  der_curve_list_link: DERCurveListLink,
  // Indicates the relative primacy of the provider of this Program.
  primacy: u8,

}
pub struct DerProgramList {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from SubscribableResource --- 

  // Indicates whether or not subscriptions are supported for this resource, and whether or not conditional (thresholds) are supported. If not specified, is "not subscribable" (0).
  subscribable: u8,


  // --- these fields come from SubscribableList --- 

  // The number specifying "all" of the items in the list. Required on GET, ignored otherwise.
  all: u32,
  // Indicates the number of items in this page of results.
  results: u32,


  // --- these fields come from DERProgramList --- 

  der_program: DERProgram,
  // The default polling rate for this function set (this resource and all resources below), in seconds. If not specified, a default of 900 seconds (15 minutes) is used. It is RECOMMENDED a client poll the resources of this function set every pollRate seconds.
  poll_rate: u32,

}
pub struct DerStatus {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from SubscribableResource --- 

  // Indicates whether or not subscriptions are supported for this resource, and whether or not conditional (thresholds) are supported. If not specified, is "not subscribable" (0).
  subscribable: u8,


  // --- these fields come from DERStatus --- 

  // Bitmap indicating the status of DER alarms (see DER LogEvents for more details).
  // 0 - DER_FAULT_OVER_CURRENT
  // 1 - DER_FAULT_OVER_VOLTAGE
  // 2 - DER_FAULT_UNDER_VOLTAGE
  // 3 - DER_FAULT_OVER_FREQUENCY
  // 4 - DER_FAULT_UNDER_FREQUENCY
  // 5 - DER_FAULT_VOLTAGE_IMBALANCE
  // 6 - DER_FAULT_CURRENT_IMBALANCE
  // 7 - DER_FAULT_EMERGENCY_LOCAL
  // 8 - DER_FAULT_EMERGENCY_REMOTE
  // 9 - DER_FAULT_LOW_POWER_INPUT
  // 10 - DER_FAULT_PHASE_ROTATION
  // 11-31 - Reserved
  alarm_status: Vec<u8>,
  // Connect/status value for generator DER. 
  // See ConnectStatusType for values.
  gen_connect_status: ConnectStatusType,
  // DER InverterStatus/value.
  // See InverterStatusType for values.
  inverter_status: InverterStatusType,
  // The local control mode status.
  // See LocalControlModeStatusType for values.
  local_control_mode_status: LocalControlModeStatusType,
  // Manufacturer status code.
  manufacturer_status: ManufacturerStatusType,
  // Operational mode currently in use.
  // See OperationalModeStatusType for values.
  operational_mode_status: OperationalModeStatusType,
  // The timestamp when the current status was last updated.
  reading_time: i64,
  // State of charge status.
  // See StateOfChargeStatusType for values.
  state_of_charge_status: StateOfChargeStatusType,
  // Storage mode status.
  // See StorageModeStatusType for values.
  storage_mode_status: StorageModeStatusType,
  // Connect/status value for storage DER.
  // See ConnectStatusType for values.
  stor_connect_status: ConnectStatusType,

}
pub struct CurrentRms {

  // --- these fields come from CurrentRMS --- 

  // Specifies exponent of value.
  multiplier: i8,
  // Value in amperes RMS (uom 5)
  value: u16,

}
pub struct FixedPointType {

  // --- these fields come from FixedPointType --- 

  // Specifies exponent of uom.
  multiplier: i8,
  // Dimensionless value
  value: i16,

}
pub struct UnsignedFixedPointType {

  // --- these fields come from UnsignedFixedPointType --- 

  // Specifies exponent of uom.
  multiplier: i8,
  // Dimensionless value
  value: u16,

}
pub struct ActivePower {

  // --- these fields come from ActivePower --- 

  // Specifies exponent for uom.
  multiplier: i8,
  // Value in watts (uom 38)
  value: i16,

}
pub struct AmpereHour {

  // --- these fields come from AmpereHour --- 

  // Specifies exponent of uom.
  multiplier: i8,
  // Value in ampere-hours (uom 106)
  value: u16,

}
pub struct ApparentPower {

  // --- these fields come from ApparentPower --- 

  // Specifies exponent of uom.
  multiplier: i8,
  // Value in volt-amperes (uom 61)
  value: u16,

}
pub struct ReactivePower {

  // --- these fields come from ReactivePower --- 

  // Specifies exponent of uom.
  multiplier: i8,
  // Value in volt-amperes reactive (var) (uom 63)
  value: i16,

}
pub struct ReactiveSusceptance {

  // --- these fields come from ReactiveSusceptance --- 

  // Specifies exponent of uom.
  multiplier: i8,
  // Value in siemens (uom 53)
  value: u16,

}
pub struct PowerFactor {

  // --- these fields come from PowerFactor --- 

  // Significand of an unsigned value of cos(theta) between 0 and 1.0. E.g. a value of 0.95 may be specified as a displacement of 950 and a multiplier of -3.
  displacement: u16,
  // Specifies exponent of 'displacement'.
  multiplier: i8,

}
pub struct PowerFactorWithExcitation {

  // --- these fields come from PowerFactorWithExcitation --- 

  // Significand of an unsigned value of cos(theta) between 0 and 1.0. E.g. a value of 0.95 may be specified as a displacement of 950 and a multiplier of -3.
  displacement: u16,
  // True when DER is absorbing reactive power (under-excited), false
  // when DER is injecting reactive power (over-excited).
  excitation: bool,
  // Specifies exponent of 'displacement'.
  multiplier: i8,

}
pub struct FixedVar {

  // --- these fields come from FixedVar --- 

  // Indicates whether to interpret 'value' as %setMaxVar or %statVarAvail.
  ref_type: u8,
  // Specify a signed setpoint for reactive power in % (see 'refType' for context).
  value: i16,

}
pub struct WattHour {

  // --- these fields come from WattHour --- 

  // Specifies exponent of uom.
  multiplier: i8,
  // Value in watt-hours (uom 72)
  value: u16,

}
pub struct VoltageRms {

  // --- these fields come from VoltageRMS --- 

  // Specifies exponent of uom.
  multiplier: i8,
  // Value in volts RMS (uom 29)
  value: u16,

}
pub struct ConnectStatusType {

  // --- these fields come from ConnectStatusType --- 

  // The date and time at which the state applied.
  date_time: i64,
  // The value indicating the state.
  value: u8,

}
pub struct InverterStatusType {

  // --- these fields come from InverterStatusType --- 

  // The date and time at which the state applied.
  date_time: i64,
  // The value indicating the state.
  value: u8,

}
pub struct LocalControlModeStatusType {

  // --- these fields come from LocalControlModeStatusType --- 

  // The date and time at which the state applied.
  date_time: i64,
  // The value indicating the state.
  value: u8,

}
pub struct ManufacturerStatusType {

  // --- these fields come from ManufacturerStatusType --- 

  // The date and time at which the state applied.
  date_time: i64,
  // The value indicating the state.
  value: String,

}
pub struct OperationalModeStatusType {

  // --- these fields come from OperationalModeStatusType --- 

  // The date and time at which the state applied.
  date_time: i64,
  // The value indicating the state.
  value: u8,

}
pub struct StateOfChargeStatusType {

  // --- these fields come from StateOfChargeStatusType --- 

  // The date and time at which the state applied.
  date_time: i64,
  // The value indicating the state.
  value: u16,

}
pub struct StorageModeStatusType {

  // --- these fields come from StorageModeStatusType --- 

  // The date and time at which the state applied.
  date_time: i64,
  // The value indicating the state.
  value: u8,

}
pub struct AccountBalanceLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from AccountBalanceLink --- 


}
pub struct ActiveBillingPeriodListLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ListLink --- 

  // Indicates the total number of items in the referenced list. This attribute SHALL be present if the href is a local or relative URI. This attribute SHOULD NOT be present if the href is a remote or absolute URI, as the server may be unaware of changes to the value.
  all: u32,


  // --- these fields come from ActiveBillingPeriodListLink --- 


}
pub struct ActiveCreditRegisterListLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ListLink --- 

  // Indicates the total number of items in the referenced list. This attribute SHALL be present if the href is a local or relative URI. This attribute SHOULD NOT be present if the href is a remote or absolute URI, as the server may be unaware of changes to the value.
  all: u32,


  // --- these fields come from ActiveCreditRegisterListLink --- 


}
pub struct ActiveDerControlListLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ListLink --- 

  // Indicates the total number of items in the referenced list. This attribute SHALL be present if the href is a local or relative URI. This attribute SHOULD NOT be present if the href is a remote or absolute URI, as the server may be unaware of changes to the value.
  all: u32,


  // --- these fields come from ActiveDERControlListLink --- 


}
pub struct ActiveEndDeviceControlListLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ListLink --- 

  // Indicates the total number of items in the referenced list. This attribute SHALL be present if the href is a local or relative URI. This attribute SHOULD NOT be present if the href is a remote or absolute URI, as the server may be unaware of changes to the value.
  all: u32,


  // --- these fields come from ActiveEndDeviceControlListLink --- 


}
pub struct ActiveFlowReservationListLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ListLink --- 

  // Indicates the total number of items in the referenced list. This attribute SHALL be present if the href is a local or relative URI. This attribute SHOULD NOT be present if the href is a remote or absolute URI, as the server may be unaware of changes to the value.
  all: u32,


  // --- these fields come from ActiveFlowReservationListLink --- 


}
pub struct ActiveProjectionReadingListLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ListLink --- 

  // Indicates the total number of items in the referenced list. This attribute SHALL be present if the href is a local or relative URI. This attribute SHOULD NOT be present if the href is a remote or absolute URI, as the server may be unaware of changes to the value.
  all: u32,


  // --- these fields come from ActiveProjectionReadingListLink --- 


}
pub struct ActiveSupplyInterruptionOverrideListLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ListLink --- 

  // Indicates the total number of items in the referenced list. This attribute SHALL be present if the href is a local or relative URI. This attribute SHOULD NOT be present if the href is a remote or absolute URI, as the server may be unaware of changes to the value.
  all: u32,


  // --- these fields come from ActiveSupplyInterruptionOverrideListLink --- 


}
pub struct ActiveTargetReadingListLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ListLink --- 

  // Indicates the total number of items in the referenced list. This attribute SHALL be present if the href is a local or relative URI. This attribute SHOULD NOT be present if the href is a remote or absolute URI, as the server may be unaware of changes to the value.
  all: u32,


  // --- these fields come from ActiveTargetReadingListLink --- 


}
pub struct ActiveTextMessageListLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ListLink --- 

  // Indicates the total number of items in the referenced list. This attribute SHALL be present if the href is a local or relative URI. This attribute SHOULD NOT be present if the href is a remote or absolute URI, as the server may be unaware of changes to the value.
  all: u32,


  // --- these fields come from ActiveTextMessageListLink --- 


}
pub struct ActiveTimeTariffIntervalListLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ListLink --- 

  // Indicates the total number of items in the referenced list. This attribute SHALL be present if the href is a local or relative URI. This attribute SHOULD NOT be present if the href is a remote or absolute URI, as the server may be unaware of changes to the value.
  all: u32,


  // --- these fields come from ActiveTimeTariffIntervalListLink --- 


}
pub struct AssociatedDerProgramListLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ListLink --- 

  // Indicates the total number of items in the referenced list. This attribute SHALL be present if the href is a local or relative URI. This attribute SHOULD NOT be present if the href is a remote or absolute URI, as the server may be unaware of changes to the value.
  all: u32,


  // --- these fields come from AssociatedDERProgramListLink --- 


}
pub struct AssociatedUsagePointLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from AssociatedUsagePointLink --- 


}
pub struct BillingPeriodListLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ListLink --- 

  // Indicates the total number of items in the referenced list. This attribute SHALL be present if the href is a local or relative URI. This attribute SHOULD NOT be present if the href is a remote or absolute URI, as the server may be unaware of changes to the value.
  all: u32,


  // --- these fields come from BillingPeriodListLink --- 


}
pub struct BillingReadingListLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ListLink --- 

  // Indicates the total number of items in the referenced list. This attribute SHALL be present if the href is a local or relative URI. This attribute SHOULD NOT be present if the href is a remote or absolute URI, as the server may be unaware of changes to the value.
  all: u32,


  // --- these fields come from BillingReadingListLink --- 


}
pub struct BillingReadingSetListLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ListLink --- 

  // Indicates the total number of items in the referenced list. This attribute SHALL be present if the href is a local or relative URI. This attribute SHOULD NOT be present if the href is a remote or absolute URI, as the server may be unaware of changes to the value.
  all: u32,


  // --- these fields come from BillingReadingSetListLink --- 


}
pub struct ConfigurationLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ConfigurationLink --- 


}
pub struct ConsumptionTariffIntervalListLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ListLink --- 

  // Indicates the total number of items in the referenced list. This attribute SHALL be present if the href is a local or relative URI. This attribute SHOULD NOT be present if the href is a remote or absolute URI, as the server may be unaware of changes to the value.
  all: u32,


  // --- these fields come from ConsumptionTariffIntervalListLink --- 


}
pub struct CreditRegisterListLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ListLink --- 

  // Indicates the total number of items in the referenced list. This attribute SHALL be present if the href is a local or relative URI. This attribute SHOULD NOT be present if the href is a remote or absolute URI, as the server may be unaware of changes to the value.
  all: u32,


  // --- these fields come from CreditRegisterListLink --- 


}
pub struct CustomerAccountLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from CustomerAccountLink --- 


}
pub struct CustomerAccountListLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ListLink --- 

  // Indicates the total number of items in the referenced list. This attribute SHALL be present if the href is a local or relative URI. This attribute SHOULD NOT be present if the href is a remote or absolute URI, as the server may be unaware of changes to the value.
  all: u32,


  // --- these fields come from CustomerAccountListLink --- 


}
pub struct CustomerAgreementListLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ListLink --- 

  // Indicates the total number of items in the referenced list. This attribute SHALL be present if the href is a local or relative URI. This attribute SHOULD NOT be present if the href is a remote or absolute URI, as the server may be unaware of changes to the value.
  all: u32,


  // --- these fields come from CustomerAgreementListLink --- 


}
pub struct DemandResponseProgramLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from DemandResponseProgramLink --- 


}
pub struct DemandResponseProgramListLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ListLink --- 

  // Indicates the total number of items in the referenced list. This attribute SHALL be present if the href is a local or relative URI. This attribute SHOULD NOT be present if the href is a remote or absolute URI, as the server may be unaware of changes to the value.
  all: u32,


  // --- these fields come from DemandResponseProgramListLink --- 


}
pub struct DerAvailabilityLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from DERAvailabilityLink --- 


}
pub struct DefaultDerControlLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from DefaultDERControlLink --- 


}
pub struct DerCapabilityLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from DERCapabilityLink --- 


}
pub struct DerControlListLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ListLink --- 

  // Indicates the total number of items in the referenced list. This attribute SHALL be present if the href is a local or relative URI. This attribute SHOULD NOT be present if the href is a remote or absolute URI, as the server may be unaware of changes to the value.
  all: u32,


  // --- these fields come from DERControlListLink --- 


}
pub struct DerCurveLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from DERCurveLink --- 


}
pub struct DerCurveListLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ListLink --- 

  // Indicates the total number of items in the referenced list. This attribute SHALL be present if the href is a local or relative URI. This attribute SHOULD NOT be present if the href is a remote or absolute URI, as the server may be unaware of changes to the value.
  all: u32,


  // --- these fields come from DERCurveListLink --- 


}
pub struct DerLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from DERLink --- 


}
pub struct DerListLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ListLink --- 

  // Indicates the total number of items in the referenced list. This attribute SHALL be present if the href is a local or relative URI. This attribute SHOULD NOT be present if the href is a remote or absolute URI, as the server may be unaware of changes to the value.
  all: u32,


  // --- these fields come from DERListLink --- 


}
pub struct DerProgramLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from DERProgramLink --- 


}
pub struct DerProgramListLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ListLink --- 

  // Indicates the total number of items in the referenced list. This attribute SHALL be present if the href is a local or relative URI. This attribute SHOULD NOT be present if the href is a remote or absolute URI, as the server may be unaware of changes to the value.
  all: u32,


  // --- these fields come from DERProgramListLink --- 


}
pub struct DerSettingsLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from DERSettingsLink --- 


}
pub struct DerStatusLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from DERStatusLink --- 


}
pub struct DeviceCapabilityLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from DeviceCapabilityLink --- 


}
pub struct DeviceInformationLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from DeviceInformationLink --- 


}
pub struct DeviceStatusLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from DeviceStatusLink --- 


}
pub struct EndDeviceControlListLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ListLink --- 

  // Indicates the total number of items in the referenced list. This attribute SHALL be present if the href is a local or relative URI. This attribute SHOULD NOT be present if the href is a remote or absolute URI, as the server may be unaware of changes to the value.
  all: u32,


  // --- these fields come from EndDeviceControlListLink --- 


}
pub struct EndDeviceLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from EndDeviceLink --- 


}
pub struct EndDeviceListLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ListLink --- 

  // Indicates the total number of items in the referenced list. This attribute SHALL be present if the href is a local or relative URI. This attribute SHOULD NOT be present if the href is a remote or absolute URI, as the server may be unaware of changes to the value.
  all: u32,


  // --- these fields come from EndDeviceListLink --- 


}
pub struct FileLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from FileLink --- 


}
pub struct FileListLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ListLink --- 

  // Indicates the total number of items in the referenced list. This attribute SHALL be present if the href is a local or relative URI. This attribute SHOULD NOT be present if the href is a remote or absolute URI, as the server may be unaware of changes to the value.
  all: u32,


  // --- these fields come from FileListLink --- 


}
pub struct FileStatusLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from FileStatusLink --- 


}
pub struct FlowReservationRequestListLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ListLink --- 

  // Indicates the total number of items in the referenced list. This attribute SHALL be present if the href is a local or relative URI. This attribute SHOULD NOT be present if the href is a remote or absolute URI, as the server may be unaware of changes to the value.
  all: u32,


  // --- these fields come from FlowReservationRequestListLink --- 


}
pub struct FlowReservationResponseListLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ListLink --- 

  // Indicates the total number of items in the referenced list. This attribute SHALL be present if the href is a local or relative URI. This attribute SHOULD NOT be present if the href is a remote or absolute URI, as the server may be unaware of changes to the value.
  all: u32,


  // --- these fields come from FlowReservationResponseListLink --- 


}
pub struct FunctionSetAssignmentsListLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ListLink --- 

  // Indicates the total number of items in the referenced list. This attribute SHALL be present if the href is a local or relative URI. This attribute SHOULD NOT be present if the href is a remote or absolute URI, as the server may be unaware of changes to the value.
  all: u32,


  // --- these fields come from FunctionSetAssignmentsListLink --- 


}
pub struct HistoricalReadingListLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ListLink --- 

  // Indicates the total number of items in the referenced list. This attribute SHALL be present if the href is a local or relative URI. This attribute SHOULD NOT be present if the href is a remote or absolute URI, as the server may be unaware of changes to the value.
  all: u32,


  // --- these fields come from HistoricalReadingListLink --- 


}
pub struct IpAddrListLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ListLink --- 

  // Indicates the total number of items in the referenced list. This attribute SHALL be present if the href is a local or relative URI. This attribute SHOULD NOT be present if the href is a remote or absolute URI, as the server may be unaware of changes to the value.
  all: u32,


  // --- these fields come from IPAddrListLink --- 


}
pub struct IpInterfaceListLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ListLink --- 

  // Indicates the total number of items in the referenced list. This attribute SHALL be present if the href is a local or relative URI. This attribute SHOULD NOT be present if the href is a remote or absolute URI, as the server may be unaware of changes to the value.
  all: u32,


  // --- these fields come from IPInterfaceListLink --- 


}
pub struct LlInterfaceListLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ListLink --- 

  // Indicates the total number of items in the referenced list. This attribute SHALL be present if the href is a local or relative URI. This attribute SHOULD NOT be present if the href is a remote or absolute URI, as the server may be unaware of changes to the value.
  all: u32,


  // --- these fields come from LLInterfaceListLink --- 


}
pub struct LoadShedAvailabilityListLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ListLink --- 

  // Indicates the total number of items in the referenced list. This attribute SHALL be present if the href is a local or relative URI. This attribute SHOULD NOT be present if the href is a remote or absolute URI, as the server may be unaware of changes to the value.
  all: u32,


  // --- these fields come from LoadShedAvailabilityListLink --- 


}
pub struct LogEventListLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ListLink --- 

  // Indicates the total number of items in the referenced list. This attribute SHALL be present if the href is a local or relative URI. This attribute SHOULD NOT be present if the href is a remote or absolute URI, as the server may be unaware of changes to the value.
  all: u32,


  // --- these fields come from LogEventListLink --- 


}
pub struct MessagingProgramListLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ListLink --- 

  // Indicates the total number of items in the referenced list. This attribute SHALL be present if the href is a local or relative URI. This attribute SHOULD NOT be present if the href is a remote or absolute URI, as the server may be unaware of changes to the value.
  all: u32,


  // --- these fields come from MessagingProgramListLink --- 


}
pub struct MeterReadingLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from MeterReadingLink --- 


}
pub struct MeterReadingListLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ListLink --- 

  // Indicates the total number of items in the referenced list. This attribute SHALL be present if the href is a local or relative URI. This attribute SHOULD NOT be present if the href is a remote or absolute URI, as the server may be unaware of changes to the value.
  all: u32,


  // --- these fields come from MeterReadingListLink --- 


}
pub struct MirrorUsagePointListLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ListLink --- 

  // Indicates the total number of items in the referenced list. This attribute SHALL be present if the href is a local or relative URI. This attribute SHOULD NOT be present if the href is a remote or absolute URI, as the server may be unaware of changes to the value.
  all: u32,


  // --- these fields come from MirrorUsagePointListLink --- 


}
pub struct NeighborListLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ListLink --- 

  // Indicates the total number of items in the referenced list. This attribute SHALL be present if the href is a local or relative URI. This attribute SHOULD NOT be present if the href is a remote or absolute URI, as the server may be unaware of changes to the value.
  all: u32,


  // --- these fields come from NeighborListLink --- 


}
pub struct NotificationListLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ListLink --- 

  // Indicates the total number of items in the referenced list. This attribute SHALL be present if the href is a local or relative URI. This attribute SHOULD NOT be present if the href is a remote or absolute URI, as the server may be unaware of changes to the value.
  all: u32,


  // --- these fields come from NotificationListLink --- 


}
pub struct PowerStatusLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from PowerStatusLink --- 


}
pub struct PrepaymentLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from PrepaymentLink --- 


}
pub struct PrepaymentListLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ListLink --- 

  // Indicates the total number of items in the referenced list. This attribute SHALL be present if the href is a local or relative URI. This attribute SHOULD NOT be present if the href is a remote or absolute URI, as the server may be unaware of changes to the value.
  all: u32,


  // --- these fields come from PrepaymentListLink --- 


}
pub struct PrepayOperationStatusLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from PrepayOperationStatusLink --- 


}
pub struct PriceResponseCfgListLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ListLink --- 

  // Indicates the total number of items in the referenced list. This attribute SHALL be present if the href is a local or relative URI. This attribute SHOULD NOT be present if the href is a remote or absolute URI, as the server may be unaware of changes to the value.
  all: u32,


  // --- these fields come from PriceResponseCfgListLink --- 


}
pub struct ProjectionReadingListLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ListLink --- 

  // Indicates the total number of items in the referenced list. This attribute SHALL be present if the href is a local or relative URI. This attribute SHOULD NOT be present if the href is a remote or absolute URI, as the server may be unaware of changes to the value.
  all: u32,


  // --- these fields come from ProjectionReadingListLink --- 


}
pub struct RateComponentLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from RateComponentLink --- 


}
pub struct RateComponentListLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ListLink --- 

  // Indicates the total number of items in the referenced list. This attribute SHALL be present if the href is a local or relative URI. This attribute SHOULD NOT be present if the href is a remote or absolute URI, as the server may be unaware of changes to the value.
  all: u32,


  // --- these fields come from RateComponentListLink --- 


}
pub struct ReadingLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ReadingLink --- 


}
pub struct ReadingListLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ListLink --- 

  // Indicates the total number of items in the referenced list. This attribute SHALL be present if the href is a local or relative URI. This attribute SHOULD NOT be present if the href is a remote or absolute URI, as the server may be unaware of changes to the value.
  all: u32,


  // --- these fields come from ReadingListLink --- 


}
pub struct ReadingSetListLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ListLink --- 

  // Indicates the total number of items in the referenced list. This attribute SHALL be present if the href is a local or relative URI. This attribute SHOULD NOT be present if the href is a remote or absolute URI, as the server may be unaware of changes to the value.
  all: u32,


  // --- these fields come from ReadingSetListLink --- 


}
pub struct ReadingTypeLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ReadingTypeLink --- 


}
pub struct RegistrationLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from RegistrationLink --- 


}
pub struct ResponseListLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ListLink --- 

  // Indicates the total number of items in the referenced list. This attribute SHALL be present if the href is a local or relative URI. This attribute SHOULD NOT be present if the href is a remote or absolute URI, as the server may be unaware of changes to the value.
  all: u32,


  // --- these fields come from ResponseListLink --- 


}
pub struct ResponseSetListLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ListLink --- 

  // Indicates the total number of items in the referenced list. This attribute SHALL be present if the href is a local or relative URI. This attribute SHOULD NOT be present if the href is a remote or absolute URI, as the server may be unaware of changes to the value.
  all: u32,


  // --- these fields come from ResponseSetListLink --- 


}
pub struct RplInstanceListLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ListLink --- 

  // Indicates the total number of items in the referenced list. This attribute SHALL be present if the href is a local or relative URI. This attribute SHOULD NOT be present if the href is a remote or absolute URI, as the server may be unaware of changes to the value.
  all: u32,


  // --- these fields come from RPLInstanceListLink --- 


}
pub struct RplSourceRoutesListLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ListLink --- 

  // Indicates the total number of items in the referenced list. This attribute SHALL be present if the href is a local or relative URI. This attribute SHOULD NOT be present if the href is a remote or absolute URI, as the server may be unaware of changes to the value.
  all: u32,


  // --- these fields come from RPLSourceRoutesListLink --- 


}
pub struct SelfDeviceLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from SelfDeviceLink --- 


}
pub struct ServiceSupplierLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ServiceSupplierLink --- 


}
pub struct SubscriptionListLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ListLink --- 

  // Indicates the total number of items in the referenced list. This attribute SHALL be present if the href is a local or relative URI. This attribute SHOULD NOT be present if the href is a remote or absolute URI, as the server may be unaware of changes to the value.
  all: u32,


  // --- these fields come from SubscriptionListLink --- 


}
pub struct SupplyInterruptionOverrideListLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ListLink --- 

  // Indicates the total number of items in the referenced list. This attribute SHALL be present if the href is a local or relative URI. This attribute SHOULD NOT be present if the href is a remote or absolute URI, as the server may be unaware of changes to the value.
  all: u32,


  // --- these fields come from SupplyInterruptionOverrideListLink --- 


}
pub struct SupportedLocaleListLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ListLink --- 

  // Indicates the total number of items in the referenced list. This attribute SHALL be present if the href is a local or relative URI. This attribute SHOULD NOT be present if the href is a remote or absolute URI, as the server may be unaware of changes to the value.
  all: u32,


  // --- these fields come from SupportedLocaleListLink --- 


}
pub struct TargetReadingListLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ListLink --- 

  // Indicates the total number of items in the referenced list. This attribute SHALL be present if the href is a local or relative URI. This attribute SHOULD NOT be present if the href is a remote or absolute URI, as the server may be unaware of changes to the value.
  all: u32,


  // --- these fields come from TargetReadingListLink --- 


}
pub struct TariffProfileLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from TariffProfileLink --- 


}
pub struct TariffProfileListLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ListLink --- 

  // Indicates the total number of items in the referenced list. This attribute SHALL be present if the href is a local or relative URI. This attribute SHOULD NOT be present if the href is a remote or absolute URI, as the server may be unaware of changes to the value.
  all: u32,


  // --- these fields come from TariffProfileListLink --- 


}
pub struct TextMessageListLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ListLink --- 

  // Indicates the total number of items in the referenced list. This attribute SHALL be present if the href is a local or relative URI. This attribute SHOULD NOT be present if the href is a remote or absolute URI, as the server may be unaware of changes to the value.
  all: u32,


  // --- these fields come from TextMessageListLink --- 


}
pub struct TimeLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from TimeLink --- 


}
pub struct TimeTariffIntervalListLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ListLink --- 

  // Indicates the total number of items in the referenced list. This attribute SHALL be present if the href is a local or relative URI. This attribute SHOULD NOT be present if the href is a remote or absolute URI, as the server may be unaware of changes to the value.
  all: u32,


  // --- these fields come from TimeTariffIntervalListLink --- 


}
pub struct UsagePointLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from UsagePointLink --- 


}
pub struct UsagePointListLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ListLink --- 

  // Indicates the total number of items in the referenced list. This attribute SHALL be present if the href is a local or relative URI. This attribute SHOULD NOT be present if the href is a remote or absolute URI, as the server may be unaware of changes to the value.
  all: u32,


  // --- these fields come from UsagePointListLink --- 


}
pub struct IdentifiedObject {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from IdentifiedObject --- 

  // The global identifier of the object.
  m_rid: Vec<u8>,
  // The description is a human readable text describing or naming the object.
  description: String,
  // Contains the version number of the object. See the type definition for details.
  version: u16,

}
pub struct Link {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,

}
pub struct List {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from List --- 

  // The number specifying "all" of the items in the list. Required on a response to a GET, ignored otherwise.
  all: u32,
  // Indicates the number of items in this page of results.
  results: u32,

}
pub struct ListLink {

  // --- these fields come from Link --- 

  // A URI reference.
  href: String,


  // --- these fields come from ListLink --- 

  // Indicates the total number of items in the referenced list. This attribute SHALL be present if the href is a local or relative URI. This attribute SHOULD NOT be present if the href is a remote or absolute URI, as the server may be unaware of changes to the value.
  all: u32,

}
pub struct Resource {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,

}
pub struct RespondableIdentifiedObject {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from RespondableResource --- 

  // A reference to the response resource address (URI). Required on a response to a GET if responseRequired is "true".
  reply_to: String,
  // Indicates whether or not a response is required upon receipt, creation or update of this resource. Responses shall be posted to the collection specified in "replyTo". 
  // 
  // If the resource has a deviceCategory field, devices that match one or more of the device types indicated in deviceCategory SHALL respond according to the rules listed below.  If the category does not match, the device SHALL NOT respond. If the resource does not have a deviceCategory field, a device receiving the resource SHALL respond according to the rules listed below.
  // 
  // Value encoded as hex according to the following bit assignments, any combination is possible. 
  // See Table 27 for the list of appropriate Response status codes to be sent for these purposes.
  // 0 - End device shall indicate that message was received 
  // 1 - End device shall indicate specific response. 
  // 2 - End user / customer response is required. 
  // All other values reserved.
  response_required: u8,


  // --- these fields come from RespondableIdentifiedObject --- 

  // The global identifier of the object.
  m_rid: Vec<u8>,
  // The description is a human readable text describing or naming the object.
  description: String,
  // Contains the version number of the object. See the type definition for details.
  version: u16,

}
pub struct RespondableResource {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from RespondableResource --- 

  // A reference to the response resource address (URI). Required on a response to a GET if responseRequired is "true".
  reply_to: String,
  // Indicates whether or not a response is required upon receipt, creation or update of this resource. Responses shall be posted to the collection specified in "replyTo". 
  // 
  // If the resource has a deviceCategory field, devices that match one or more of the device types indicated in deviceCategory SHALL respond according to the rules listed below.  If the category does not match, the device SHALL NOT respond. If the resource does not have a deviceCategory field, a device receiving the resource SHALL respond according to the rules listed below.
  // 
  // Value encoded as hex according to the following bit assignments, any combination is possible. 
  // See Table 27 for the list of appropriate Response status codes to be sent for these purposes.
  // 0 - End device shall indicate that message was received 
  // 1 - End device shall indicate specific response. 
  // 2 - End user / customer response is required. 
  // All other values reserved.
  response_required: u8,

}
pub struct RespondableSubscribableIdentifiedObject {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from RespondableResource --- 

  // A reference to the response resource address (URI). Required on a response to a GET if responseRequired is "true".
  reply_to: String,
  // Indicates whether or not a response is required upon receipt, creation or update of this resource. Responses shall be posted to the collection specified in "replyTo". 
  // 
  // If the resource has a deviceCategory field, devices that match one or more of the device types indicated in deviceCategory SHALL respond according to the rules listed below.  If the category does not match, the device SHALL NOT respond. If the resource does not have a deviceCategory field, a device receiving the resource SHALL respond according to the rules listed below.
  // 
  // Value encoded as hex according to the following bit assignments, any combination is possible. 
  // See Table 27 for the list of appropriate Response status codes to be sent for these purposes.
  // 0 - End device shall indicate that message was received 
  // 1 - End device shall indicate specific response. 
  // 2 - End user / customer response is required. 
  // All other values reserved.
  response_required: u8,


  // --- these fields come from RespondableSubscribableIdentifiedObject --- 

  // The global identifier of the object.
  m_rid: Vec<u8>,
  // The description is a human readable text describing or naming the object.
  description: String,
  // Contains the version number of the object. See the type definition for details.
  version: u16,
  // Indicates whether or not subscriptions are supported for this resource, and whether or not conditional (thresholds) are supported. If not specified, is "not subscribable" (0).
  subscribable: u8,

}
pub struct SubscribableIdentifiedObject {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from SubscribableResource --- 

  // Indicates whether or not subscriptions are supported for this resource, and whether or not conditional (thresholds) are supported. If not specified, is "not subscribable" (0).
  subscribable: u8,


  // --- these fields come from SubscribableIdentifiedObject --- 

  // The global identifier of the object.
  m_rid: Vec<u8>,
  // The description is a human readable text describing or naming the object.
  description: String,
  // Contains the version number of the object. See the type definition for details.
  version: u16,

}
pub struct SubscribableList {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from SubscribableResource --- 

  // Indicates whether or not subscriptions are supported for this resource, and whether or not conditional (thresholds) are supported. If not specified, is "not subscribable" (0).
  subscribable: u8,


  // --- these fields come from SubscribableList --- 

  // The number specifying "all" of the items in the list. Required on GET, ignored otherwise.
  all: u32,
  // Indicates the number of items in this page of results.
  results: u32,

}
pub struct SubscribableResource {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from SubscribableResource --- 

  // Indicates whether or not subscriptions are supported for this resource, and whether or not conditional (thresholds) are supported. If not specified, is "not subscribable" (0).
  subscribable: u8,

}
pub struct Error {

  // --- these fields come from Error --- 

  // Contains the number of seconds the client SHOULD wait before retrying the request.
  max_retry_duration: u16,
  // Code indicating the reason for failure.
  // 
  // 0 - Invalid request format
  // 1 - Invalid request values (e.g. invalid threshold values)
  // 2 - Resource limit reached
  // 3 - Conditional subscription field not supported
  // 4 - Maximum request frequency exceeded
  // All other values reserved
  reason_code: u16,

}
pub struct Event {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from RespondableResource --- 

  // A reference to the response resource address (URI). Required on a response to a GET if responseRequired is "true".
  reply_to: String,
  // Indicates whether or not a response is required upon receipt, creation or update of this resource. Responses shall be posted to the collection specified in "replyTo". 
  // 
  // If the resource has a deviceCategory field, devices that match one or more of the device types indicated in deviceCategory SHALL respond according to the rules listed below.  If the category does not match, the device SHALL NOT respond. If the resource does not have a deviceCategory field, a device receiving the resource SHALL respond according to the rules listed below.
  // 
  // Value encoded as hex according to the following bit assignments, any combination is possible. 
  // See Table 27 for the list of appropriate Response status codes to be sent for these purposes.
  // 0 - End device shall indicate that message was received 
  // 1 - End device shall indicate specific response. 
  // 2 - End user / customer response is required. 
  // All other values reserved.
  response_required: u8,


  // --- these fields come from RespondableSubscribableIdentifiedObject --- 

  // The global identifier of the object.
  m_rid: Vec<u8>,
  // The description is a human readable text describing or naming the object.
  description: String,
  // Contains the version number of the object. See the type definition for details.
  version: u16,
  // Indicates whether or not subscriptions are supported for this resource, and whether or not conditional (thresholds) are supported. If not specified, is "not subscribable" (0).
  subscribable: u8,


  // --- these fields come from Event --- 

  // The time at which the Event was created.
  creation_time: i64,
  event_status: EventStatus,
  // The period during which the Event applies.
  interval: DateTimeInterval,

}
pub struct EventStatus {

  // --- these fields come from EventStatus --- 

  // Field representing the current status type. 
  // 
  // 0 = Scheduled
  // This status indicates that the event has been scheduled and the event has not yet started.  The server SHALL set the event to this status when the event is first scheduled and persist until the event has become active or has been cancelled.  For events with a start time less than or equal to the current time, this status SHALL never be indicated, the event SHALL start with a status of “Active”. 
  // 
  // 1 = Active
  // This status indicates that the event is currently active. The server SHALL set the event to this status when the event reaches its earliest Effective Start Time.
  // 
  // 2 = Cancelled 
  // When events are cancelled, the Status.dateTime attribute SHALL be set to the time the cancellation occurred, which cannot be in the future.  The server is responsible for maintaining the cancelled event in its collection for the duration of the original event, or until the server has run out of space and needs to store a new event. Client devices SHALL be aware of Cancelled events, determine if the Cancelled event applies to them, and cancel the event immediately if applicable.
  // 
  // 3 = Cancelled with Randomization 
  // The server is responsible for maintaining the cancelled event in its collection for the duration of the Effective Scheduled Period. Client devices SHALL be aware of Cancelled with Randomization events, determine if the Cancelled event applies to them, and cancel the event immediately, using the larger of (absolute value of randomizeStart) and (absolute value of randomizeDuration) as the end randomization, in seconds. This Status.type SHALL NOT be used with "regular" Events, only with specializations of RandomizableEvent.
  // 
  // 4 = Superseded
  // Events marked as Superseded by servers are events that may have been replaced by new events from the same program that target the exact same set of deviceCategory's (if applicable) AND DERControl controls (e.g., opModTargetW) (if applicable) and overlap for a given period of time. Servers SHALL mark an event as Superseded at the earliest Effective Start Time of the overlapping event. Servers are responsible for maintaining the Superseded event in their collection for the duration of the Effective Scheduled Period. 
  // Client devices encountering a Superseded event SHALL terminate execution of the event immediately and commence execution of the new event immediately, unless the current time is within the start randomization window of the superseded event, in which case the client SHALL obey the start randomization of the new event. This Status.type SHALL NOT be used with TextMessage, since multiple text messages can be active. 
  // 
  // All other values reserved.
  current_status: u8,
  // The dateTime attribute will provide a timestamp of when the current status was defined. dateTime MUST be set to the time at which the status change occurred, not a time in the future or past.
  date_time: i64,
  // Set to true by a server of this event if there are events that overlap this event in time and also overlap in some, but not all, deviceCategory's (if applicable) AND DERControl controls (e.g., opModTargetW) (if applicable) in the same function set instance.
  potentially_superseded: bool,
  // Indicates the time that the potentiallySuperseded flag was set.
  potentially_superseded_time: i64,
  // The Reason attribute allows a Service provider to provide a textual explanation of the status.
  reason: String,

}
pub struct RandomizableEvent {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from RespondableResource --- 

  // A reference to the response resource address (URI). Required on a response to a GET if responseRequired is "true".
  reply_to: String,
  // Indicates whether or not a response is required upon receipt, creation or update of this resource. Responses shall be posted to the collection specified in "replyTo". 
  // 
  // If the resource has a deviceCategory field, devices that match one or more of the device types indicated in deviceCategory SHALL respond according to the rules listed below.  If the category does not match, the device SHALL NOT respond. If the resource does not have a deviceCategory field, a device receiving the resource SHALL respond according to the rules listed below.
  // 
  // Value encoded as hex according to the following bit assignments, any combination is possible. 
  // See Table 27 for the list of appropriate Response status codes to be sent for these purposes.
  // 0 - End device shall indicate that message was received 
  // 1 - End device shall indicate specific response. 
  // 2 - End user / customer response is required. 
  // All other values reserved.
  response_required: u8,


  // --- these fields come from RespondableSubscribableIdentifiedObject --- 

  // The global identifier of the object.
  m_rid: Vec<u8>,
  // The description is a human readable text describing or naming the object.
  description: String,
  // Contains the version number of the object. See the type definition for details.
  version: u16,
  // Indicates whether or not subscriptions are supported for this resource, and whether or not conditional (thresholds) are supported. If not specified, is "not subscribable" (0).
  subscribable: u8,


  // --- these fields come from Event --- 

  // The time at which the Event was created.
  creation_time: i64,
  event_status: EventStatus,
  // The period during which the Event applies.
  interval: DateTimeInterval,


  // --- these fields come from RandomizableEvent --- 

  // Number of seconds boundary inside which a random value must be selected to be applied to the associated interval duration, to avoid sudden synchronized demand changes. If related to price level changes, sign may be ignored. Valid range is -3600 to 3600. If not specified, 0 is the default.
  randomize_duration: i16,
  // Number of seconds boundary inside which a random value must be selected to be applied to the associated interval start time, to avoid sudden synchronized demand changes. If related to price level changes, sign may be ignored. Valid range is -3600 to 3600. If not specified, 0 is the default.
  randomize_start: i16,

}
pub struct DateTimeInterval {

  // --- these fields come from DateTimeInterval --- 

  // Duration of the interval, in seconds.
  duration: u32,
  // Date and time of the start of the interval.
  start: i64,

}
pub struct GpsLocationType {

  // --- these fields come from GPSLocationType --- 

  // Specifies the latitude from equator. -90 (south) to +90 (north) in decimal degrees.
  lat: String,
  // Specifies the longitude from Greenwich Meridian. -180 (west) to +180 (east) in decimal degrees.
  lon: String,

}
pub struct RealEnergy {

  // --- these fields come from RealEnergy --- 

  // Multiplier for 'unit'.
  multiplier: i8,
  // Value of the energy in Watt-hours. (uom 72)
  value: u64,

}
pub struct SignedRealEnergy {

  // --- these fields come from SignedRealEnergy --- 

  // Multiplier for 'unit'.
  multiplier: i8,
  // Value of the energy in Watt-hours. (uom 72)
  value: i64,

}
pub struct UnitValueType {

  // --- these fields come from UnitValueType --- 

  // Multiplier for 'unit'.
  multiplier: i8,
  // Unit in symbol
  unit: u8,
  // Value in units specified
  value: i32,

}
pub struct MirrorMeterReading {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from IdentifiedObject --- 

  // The global identifier of the object.
  m_rid: Vec<u8>,
  // The description is a human readable text describing or naming the object.
  description: String,
  // Contains the version number of the object. See the type definition for details.
  version: u16,


  // --- these fields come from MeterReadingBase --- 



  // --- these fields come from MirrorMeterReading --- 

  // The date and time of the last update.
  last_update_time: i64,
  mirror_reading_set: MirrorReadingSet,
  // The date and time of the next planned update.
  next_update_time: i64,
  reading: Reading,
  reading_type: ReadingType,

}
pub struct MirrorMeterReadingList {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from List --- 

  // The number specifying "all" of the items in the list. Required on a response to a GET, ignored otherwise.
  all: u32,
  // Indicates the number of items in this page of results.
  results: u32,


  // --- these fields come from MirrorMeterReadingList --- 

  mirror_meter_reading: MirrorMeterReading,

}
pub struct MeterReadingBase {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from IdentifiedObject --- 

  // The global identifier of the object.
  m_rid: Vec<u8>,
  // The description is a human readable text describing or naming the object.
  description: String,
  // Contains the version number of the object. See the type definition for details.
  version: u16,


  // --- these fields come from MeterReadingBase --- 


}
pub struct MirrorReadingSet {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from IdentifiedObject --- 

  // The global identifier of the object.
  m_rid: Vec<u8>,
  // The description is a human readable text describing or naming the object.
  description: String,
  // Contains the version number of the object. See the type definition for details.
  version: u16,


  // --- these fields come from ReadingSetBase --- 

  // Specifies the time range during which the contained readings were taken.
  time_period: DateTimeInterval,


  // --- these fields come from MirrorReadingSet --- 

  reading: Reading,

}
pub struct MirrorUsagePoint {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from IdentifiedObject --- 

  // The global identifier of the object.
  m_rid: Vec<u8>,
  // The description is a human readable text describing or naming the object.
  description: String,
  // Contains the version number of the object. See the type definition for details.
  version: u16,


  // --- these fields come from UsagePointBase --- 

  // Specifies the roles that apply to the usage point.
  role_flags: Vec<u8>,
  // The kind of service provided by this usage point.
  service_category_kind: u8,
  // Specifies the current status of the service at this usage point.
  // 0 = off
  // 1 = on
  status: u8,


  // --- these fields come from MirrorUsagePoint --- 

  // The LFDI of the device being mirrored.
  device_lfdi: Vec<u8>,
  mirror_meter_reading: MirrorMeterReading,
  // POST rate, or how often mirrored data should be POSTed, in seconds. A client MAY indicate a preferred postRate when POSTing MirrorUsagePoint. A server MAY add or modify postRate to indicate its preferred posting rate.
  post_rate: u32,

}
pub struct MirrorUsagePointList {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from List --- 

  // The number specifying "all" of the items in the list. Required on a response to a GET, ignored otherwise.
  all: u32,
  // Indicates the number of items in this page of results.
  results: u32,


  // --- these fields come from MirrorUsagePointList --- 

  mirror_usage_point: MirrorUsagePoint,
  // The default polling rate for this function set (this resource and all resources below), in seconds. If not specified, a default of 900 seconds (15 minutes) is used. It is RECOMMENDED a client poll the resources of this function set every pollRate seconds.
  poll_rate: u32,

}
pub struct ReadingBase {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from ReadingBase --- 

  // Indicates the consumption block related to the reading. REQUIRED if ReadingType numberOfConsumptionBlocks is non-zero. If not specified, is assumed to be "0 - N/A".
  consumption_block: u8,
  // List of codes indicating the quality of the reading, using specification:
  // 
  // Bit 0 - valid: data that has gone through all required validation checks and either passed them all or has been verified 
  // Bit 1 - manually edited: Replaced or approved by a human
  // Bit 2 - estimated using reference day: data value was replaced by a machine computed value based on analysis of historical data using the same type of measurement.
  // Bit 3 - estimated using linear interpolation: data value was computed using linear interpolation based on the readings before and after it
  // Bit 4 - questionable: data that has failed one or more checks
  // Bit 5 - derived: data that has been calculated (using logic or mathematical operations), not necessarily measured directly 
  // Bit 6 - projected (forecast): data that has been calculated as a projection or forecast of future readings
  quality_flags: Vec<u8>,
  // The time interval associated with the reading. If not specified, then defaults to the intervalLength specified in the associated ReadingType.
  time_period: DateTimeInterval,
  // Indicates the time of use tier related to the reading. REQUIRED if ReadingType numberOfTouTiers is non-zero. If not specified, is assumed to be "0 - N/A".
  tou_tier: u8,
  // Value in units specified by ReadingType
  value: i64,

}
pub struct ReadingSetBase {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from IdentifiedObject --- 

  // The global identifier of the object.
  m_rid: Vec<u8>,
  // The description is a human readable text describing or naming the object.
  description: String,
  // Contains the version number of the object. See the type definition for details.
  version: u16,


  // --- these fields come from ReadingSetBase --- 

  // Specifies the time range during which the contained readings were taken.
  time_period: DateTimeInterval,

}
pub struct UsagePointBase {

  // --- these fields come from Resource --- 

  // A reference to the resource address (URI). Required in a response to a GET, ignored otherwise.
  href: String,


  // --- these fields come from IdentifiedObject --- 

  // The global identifier of the object.
  m_rid: Vec<u8>,
  // The description is a human readable text describing or naming the object.
  description: String,
  // Contains the version number of the object. See the type definition for details.
  version: u16,


  // --- these fields come from UsagePointBase --- 

  // Specifies the roles that apply to the usage point.
  role_flags: Vec<u8>,
  // The kind of service provided by this usage point.
  service_category_kind: u8,
  // Specifies the current status of the service at this usage point.
  // 0 = off
  // 1 = on
  status: u8,

}
