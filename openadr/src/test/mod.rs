use crate::oadr20b::ei::{CurrentValueType, CurrentValueTypeChoice, PayloadFloatType};
use crate::xsd_util::{WriteConfig, WriteXml};

#[test]
fn test_write_choice() {
    let value = CurrentValueType {
        current_value_type_choice: CurrentValueTypeChoice::EiPayloadFloat(PayloadFloatType {
            value: 42.0,
        }),
    };

    let output = value.write_string(WriteConfig::default()).unwrap();
    println!("{}", output);
}
