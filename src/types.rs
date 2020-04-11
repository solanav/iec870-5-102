use crate::time::{TimeLabel, TimeLabelA};

include!(concat!(env!("OUT_DIR"), "/auto_types.rs"));

pub trait Message {
    fn from_bin(bin: Vec<u8>, n: usize) -> Self;
    fn into_bin(self) -> Vec<u8>;
}
