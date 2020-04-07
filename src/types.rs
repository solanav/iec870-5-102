use crate::time::TimeLabel;

pub const IMPLEMENTED_TYPES: [u8; 2] = [
    8,
    122,
];

pub trait Message {
    type DataStructure;
    type DataBinary;

    // Get the type of the message
    fn type_id() -> u8;

    // Get the number of objects
    fn n() -> u8;

    // Get the cause of the message
    fn cause() -> u8;

    // Serialize a structure into binary data
    fn serialize() -> DataBinary;

    // Deserialize the binary data of this type
    fn deserialize() -> DataStructure;
}

#[allow(non_camel_case_types)]
pub struct M_IT_TG_2 {
    addr_mp: u8, // Address of the measurement point
    addr_reg: u8, // Address of the register (11 or 21)
    addr_obj: [u8; std::u8::MAX], // Address of the objects
    total: [u8; std::u8::MAX], // Integrated total of the objects
    time: TimeLabel,
}

