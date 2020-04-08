use crate::time::TimeLabel;

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

pub struct StaticFrame {
    control: u8,
    address: u16,
}

impl DynamicFrame {
    /// Control
    /// {1 bit} ~ Reserve: Always 0
    /// {1 bit} PRM: 0 means its from slave, 1 means its from master
    /// {1 bit} FCB: bit that alternates on each message sent
    /// {1 bit} FCV: 0 means FCB is not active, 1 means it is
    /// {1 bit} ~ ACD: access bit, not used here
    /// {1 bit} DFC: Data flow (for controlling overflows), 0 means accepted, 1 means overflow
    /// {4 bit} Function codes:
    ///     0  - remote link reposition, FCV = 0
    ///     3  - user data, FCV = 1
    ///     9  - link state request, FCV = 0
    ///     11 - class 2 data request, FCV = 1
    pub fn set_control(&self, prm: u8, fcb: u8, fcv: u8, dfc: u8) {

    }

    /// Address
    /// Address from 0x0000 to 0xFFFF unique to the energy meter
    pub fn set_address(&self, addr: u16) {

    }

    /// DUI
    /// {8 bit} type_id: function number to indicaate the type of action or reading (1 - 255)
    /// {7 bit} vsq: number of objects we are sending. Its only 7 bits because the 8th is SQ (0 always)
    /// {8 bit} cause:
    ///     {0-5} - cause
    ///     {6}   - ack (positive is 0, negative is 1)
    ///     {7}   - test (0 is no test, 1 is test)
    /// {16 bit} mp_addr: 2 bytes of the measurment point address
    /// {8 bit} r_addr: 1 byte of the record address
    pub fn set_dui(&self, type_id: u8, vsq: u8, cause: u8, mp_addr: u16, r_addr: u8) {
        if cause >= 128 {
            panic!("Error creating dynamic frame. Cause field's value has to be [0, 127]");
        }
    }

    /// Information Objects
    /// {8 bit} addr: object address
    /// {32 bit} totals: 32 bit number representing energy
    /// {8 bit} qb: qualifier byte for the totals
    /// {x bit} time: time label
    pub fn set_infobj(&self, addr: u8, totals: u32, qb: u8, time: TimeLabel) {

    }

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
        let em_address =
            ((bin[6] as u16) << 8) +
            bin[5] as u16;
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

        // Check the checksum
        let checksum = bin[length + 4];
        let mut checksum_local: i32 = 0;
        for i in 4..length+4 {
            checksum_local += bin[i] as i32;
        }
        checksum_local = checksum_local % 256;
        assert_eq!(checksum as i32, checksum_local);

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