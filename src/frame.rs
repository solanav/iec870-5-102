pub const MAX_FRAME: usize = 255;
const START_FRAME_BYTE: u8 = 0x68;
const END_BYTE: u8 = 0x16;

#[derive(Debug)]
pub struct DynamicFrame {
    control: u8,
    em_address: u16,
    asdu_type_id: u8,
    number_obj: usize,
    cause: u8,
    measurement_point: u8,
    record_address: u16,
    data: Vec<u8>,
}

pub fn get_checksum(data: Vec<u8>) -> i32 {
    let mut checksum: i32 = 0;
    for i in 0..data.len() {
        checksum += data[i] as i32;
    }

    checksum % 256
}

pub struct StaticFrame {
    control: u8,
    address: u16,
}

impl DynamicFrame {
    pub fn data(&self) -> Vec<u8> {
        self.data.clone()
    }

    pub fn n(&self) -> usize {
        self.number_obj
    }
}

impl From<[u8; MAX_FRAME]> for DynamicFrame {
    fn from(bin: [u8; 255]) -> Self {
        assert_eq!(bin[0], START_FRAME_BYTE);

        assert_eq!(bin[1], bin[2]);
        let length: usize = bin[1] as usize;

        assert_eq!(bin[3], START_FRAME_BYTE);
        let control = bin[4];
        let em_address = u16::from_le_bytes([bin[5], bin[6]]);
        let asdu_type_id = bin[7];

        assert!(bin[8] < 128);
        let number_obj = bin[8] as usize;
        let cause = bin[9];
        let measurement_point = bin[10];
        let record_address =
            ((bin[12] as u16) << 8) +
            bin[11] as u16;

        // Save variable data
        let mut data: Vec<u8> = Vec::new();
        for i in 13..length+4 {
            data.push(bin[i]);
        }

        // Calculate the checksum
        let checksum = bin[length + 4];
        let checksum_bin = (&bin[4..length+4]).to_vec();
        assert_eq!(checksum as i32, get_checksum(checksum_bin));

        assert_eq!(bin[length + 5], END_BYTE);

        DynamicFrame {
            control,
            em_address,
            asdu_type_id,
            number_obj,
            cause,
            measurement_point,
            record_address,
            data,
        }
    }
}

impl StaticFrame {
    pub fn new(control: u8, address: u16) -> Self {
        StaticFrame {
            control,
            address,
        }
    }
}