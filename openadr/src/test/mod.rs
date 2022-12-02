use crate::ei::{CurrentValueType, CurrentValueTypeChoice, PayloadFloatType};
use xsd_api::{WriteConfig, WriteXml};

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
