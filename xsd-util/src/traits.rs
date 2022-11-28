/// Defines what enums based on string values (i.e. the kind built into XSD)
/// must provide for (de)serialization support
pub trait StringEnumeration: Copy + Send + Sync + 'static {
    fn to_str(self) -> &'static str;
    fn find(value: &str) -> Option<Self>;
}
