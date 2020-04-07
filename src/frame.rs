use crate::time::TimeLabel;

const MAX_FRAME: usize = 255;

pub struct DynamicFrame {
    control: u8,
    address: u16,
    asdu: u8,
}

pub struct StaticFrame {
    control: u8,
    address: u16,
}

impl DynamicFrame {
    /// Initializes the structure with all values equal to zero
    pub fn new() -> Self{
        DynamicFrame {
            control: 0,
            address: 0,
            asdu: 0,
        }
    }

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
    pub fn set_control(prm: u8, fcb: u8, fcv: u8, dfc: u8) {

    }

    /// Address
    /// Address from 0x0000 to 0xFFFF unique to the energy meter
    pub fn set_address(addr: u16) {

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
    pub fn set_dui(type_id: u8, vsq: u8, cause: u8, mp_addr: u16, r_addr: u8) {
        if cause >= 128 {
            panic!("Error creating dynamic frame. Cause field's value has to be [0, 127]");
        }
    }

    /// Information Objects
    /// {8 bit} addr: object address
    /// {32 bit} totals: 32 bit number representing energy
    /// {8 bit} qb: qualifier byte for the totals
    /// {x bit} time: time label
    pub fn set_infobj(addr: u8, totals: u32, qb: u8, time: TimeLabel) {

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