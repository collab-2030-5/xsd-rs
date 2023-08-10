use xml::common::Position;
use xml::writer::*;

#[derive(Debug, Clone, PartialEq)]
pub struct EiTargetType {
    pub power_aggregated_pnode: Vec<crate::power::AggregatedPnodeType>,
    pub power_end_device_asset: Vec<crate::power::EndDeviceAssetType>,
    pub power_meter_asset: Vec<crate::power::MeterAssetType>,
    pub power_pnode: Vec<crate::power::PnodeType>,
    pub emix_service_area: Vec<crate::emix::ServiceAreaType>,
    pub power_service_delivery_point: Vec<crate::power::ServiceDeliveryPointType>,
    pub power_service_location: Vec<crate::power::ServiceLocationType>,
    pub power_transport_interface: Vec<crate::power::TransportInterfaceType>,
    pub ei_group_id: Vec<String>,
    pub ei_group_name: Vec<String>,
    pub ei_resource_id: Vec<String>,
    pub ei_ven_id: Vec<String>,
    pub ei_party_id: Vec<String>,
}

impl EiTargetType {
    fn write_elem<W>(
        &self,
        writer: &mut EventWriter<W>,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        for item in &self.power_aggregated_pnode {
            item.write_with_name(writer, "power:aggregatedPnode", false, false)?;
        }
        for item in &self.power_end_device_asset {
            item.write_with_name(writer, "power:endDeviceAsset", false, false)?;
        }
        for item in &self.power_meter_asset {
            item.write_with_name(writer, "power:meterAsset", false, false)?;
        }
        for item in &self.power_pnode {
            item.write_with_name(writer, "power:pnode", false, false)?;
        }
        for item in &self.emix_service_area {
            item.write_with_name(writer, "emix:serviceArea", false, false)?;
        }
        for item in &self.power_service_delivery_point {
            item.write_with_name(writer, "power:serviceDeliveryPoint", false, false)?;
        }
        for item in &self.power_service_location {
            item.write_with_name(writer, "power:serviceLocation", false, false)?;
        }
        for item in &self.power_transport_interface {
            item.write_with_name(writer, "power:transportInterface", false, false)?;
        }
        for item in &self.ei_group_id {
            xsd_util::write_simple_element(writer, "ei:groupID", item.as_str())?;
        }
        for item in &self.ei_group_name {
            xsd_util::write_simple_element(writer, "ei:groupName", item.as_str())?;
        }
        for item in &self.ei_resource_id {
            xsd_util::write_simple_element(writer, "ei:resourceID", item.as_str())?;
        }
        for item in &self.ei_ven_id {
            xsd_util::write_simple_element(writer, "ei:venID", item.as_str())?;
        }
        for item in &self.ei_party_id {
            xsd_util::write_simple_element(writer, "ei:partyID", item.as_str())?;
        }
        Ok(())
    }

    pub(crate) fn write_with_name<W>(
        &self,
        writer: &mut EventWriter<W>,
        name: &str,
        top_level: bool,
        write_type: bool,
    ) -> core::result::Result<(), xml::writer::Error>
    where
        W: std::io::Write,
    {
        let start = if top_level {
            super::add_schema_attr(events::XmlEvent::start_element(name))
        } else {
            events::XmlEvent::start_element(name)
        };
        let start = if write_type {
            start.attr("xsi:type", "EiTargetType")
        } else {
            start
        };
        writer.write(start)?;
        self.write_elem(writer)?;
        writer.write(events::XmlEvent::end_element())?;
        Ok(())
    }
}

impl xsd_api::WriteXml for EiTargetType {
    fn write<W>(
        &self,
        config: xsd_api::WriteConfig,
        writer: &mut W,
    ) -> core::result::Result<(), xsd_api::WriteError>
    where
        W: std::io::Write,
    {
        let mut writer = config.build_xml_rs().create_writer(writer);
        self.write_with_name(&mut writer, "ei:EiTargetType", true, false)?;
        Ok(())
    }
}

impl EiTargetType {
    pub(crate) fn read<R>(
        reader: &mut xml::reader::EventReader<R>,
        attrs: &Vec<xml::attribute::OwnedAttribute>,
        parent_tag: &str,
    ) -> core::result::Result<Self, xsd_api::ReadError>
    where
        R: std::io::Read,
    {
        // one variable for each attribute and element
        let mut power_aggregated_pnode: Vec<crate::power::AggregatedPnodeType> = Default::default();
        let mut power_end_device_asset: Vec<crate::power::EndDeviceAssetType> = Default::default();
        let mut power_meter_asset: Vec<crate::power::MeterAssetType> = Default::default();
        let mut power_pnode: Vec<crate::power::PnodeType> = Default::default();
        let mut emix_service_area: Vec<crate::emix::ServiceAreaType> = Default::default();
        let mut power_service_delivery_point: Vec<crate::power::ServiceDeliveryPointType> =
            Default::default();
        let mut power_service_location: Vec<crate::power::ServiceLocationType> = Default::default();
        let mut power_transport_interface: Vec<crate::power::TransportInterfaceType> =
            Default::default();
        let mut ei_group_id: Vec<String> = Default::default();
        let mut ei_group_name: Vec<String> = Default::default();
        let mut ei_resource_id: Vec<String> = Default::default();
        let mut ei_ven_id: Vec<String> = Default::default();
        let mut ei_party_id: Vec<String> = Default::default();

        for attr in attrs.iter() {
            match attr.name.local_name.as_str() {
                _ => {} // ignore unknown attributes
            };
        }

        loop {
            match reader.next()? {
                xml::reader::XmlEvent::EndElement { name } => {
                    if name.local_name.as_str() == parent_tag {
                        // try to construct struct
                        break;
                    } else {
                        // TODO - make this more specific
                        return Err(xsd_api::ReadError::UnexpectedEvent);
                    }
                }
                xml::reader::XmlEvent::StartElement {
                    name, attributes, ..
                } => match name.local_name.as_str() {
                    "aggregatedPnode" => {
                        power_aggregated_pnode.push(crate::power::AggregatedPnodeType::read(
                            reader,
                            &attributes,
                            "aggregatedPnode",
                        )?)
                    }
                    "endDeviceAsset" => {
                        power_end_device_asset.push(crate::power::EndDeviceAssetType::read(
                            reader,
                            &attributes,
                            "endDeviceAsset",
                        )?)
                    }
                    "meterAsset" => power_meter_asset.push(crate::power::MeterAssetType::read(
                        reader,
                        &attributes,
                        "meterAsset",
                    )?),
                    "pnode" => power_pnode.push(crate::power::PnodeType::read(
                        reader,
                        &attributes,
                        "pnode",
                    )?),
                    "serviceArea" => emix_service_area.push(crate::emix::ServiceAreaType::read(
                        reader,
                        &attributes,
                        "serviceArea",
                    )?),
                    "serviceDeliveryPoint" => power_service_delivery_point.push(
                        crate::power::ServiceDeliveryPointType::read(
                            reader,
                            &attributes,
                            "serviceDeliveryPoint",
                        )?,
                    ),
                    "serviceLocation" => {
                        power_service_location.push(crate::power::ServiceLocationType::read(
                            reader,
                            &attributes,
                            "serviceLocation",
                        )?)
                    }
                    "transportInterface" => {
                        power_transport_interface.push(crate::power::TransportInterfaceType::read(
                            reader,
                            &attributes,
                            "transportInterface",
                        )?)
                    }
                    "groupID" => ei_group_id.push(xsd_util::read_string(reader, "groupID")?),
                    "groupName" => ei_group_name.push(xsd_util::read_string(reader, "groupName")?),
                    "resourceID" => {
                        ei_resource_id.push(xsd_util::read_string(reader, "resourceID")?)
                    }
                    "venID" => ei_ven_id.push(xsd_util::read_string(reader, "venID")?),
                    "partyID" => ei_party_id.push(xsd_util::read_string(reader, "partyID")?),
                    name => {
                        return Err(xsd_api::ReadError::UnexpectedToken(
                            xsd_api::ParentToken(parent_tag.to_owned()),
                            xsd_api::ChildToken(name.to_owned()),
                        ))
                    }
                },
                // treat these events as errors
                xml::reader::XmlEvent::StartDocument { .. } => {
                    return Err(xsd_api::ReadError::UnexpectedEvent)
                }
                xml::reader::XmlEvent::EndDocument => {
                    return Err(xsd_api::ReadError::UnexpectedEvent)
                }
                xml::reader::XmlEvent::Characters(_) => {
                    return Err(xsd_api::ReadError::UnexpectedEvent)
                }
                xml::reader::XmlEvent::ProcessingInstruction { .. } => {
                    return Err(xsd_api::ReadError::UnexpectedEvent)
                }
                // ignore these events
                xml::reader::XmlEvent::CData(_) => {}
                xml::reader::XmlEvent::Comment(_) => {}
                xml::reader::XmlEvent::Whitespace(_) => {}
            }
        }

        // construct the type from the cells
        Ok(EiTargetType {
            power_aggregated_pnode,
            power_end_device_asset,
            power_meter_asset,
            power_pnode,
            emix_service_area,
            power_service_delivery_point,
            power_service_location,
            power_transport_interface,
            ei_group_id,
            ei_group_name,
            ei_resource_id,
            ei_ven_id,
            ei_party_id,
        })
    }

    fn read_top_level<R>(
        reader: &mut xml::reader::EventReader<R>,
    ) -> core::result::Result<Self, xsd_api::ReadError>
    where
        R: std::io::Read,
    {
        let attr = xsd_util::read_start_tag(reader, "EiTargetType")?;
        EiTargetType::read(reader, &attr, "EiTargetType")
    }
}

impl xsd_api::ReadXml for EiTargetType {
    fn read<R>(r: &mut R) -> core::result::Result<Self, xsd_api::ErrorWithLocation>
    where
        R: std::io::Read,
    {
        let mut reader = xml::reader::EventReader::new(r);

        match EiTargetType::read_top_level(&mut reader) {
            Ok(x) => Ok(x),
            Err(err) => {
                let pos = reader.position();
                Err(xsd_api::ErrorWithLocation {
                    err,
                    line: pos.row,
                    col: pos.column,
                })
            }
        }
    }
}
