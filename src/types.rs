use crate::time::{TimeLabel, TimeLabelA};

pub enum ImplementedTypes {
    MITTG2(M_IT_TG_2),
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct M_IT_TG_2 {
    addr_obj: Vec<u8>, // Address of the objects
    total: Vec<u64>, // Integrated total of the objects
    time: TimeLabel,
}

impl M_IT_TG_2 {
    pub fn from_bin(bin: Vec<u8>, n: usize) -> Self {
        let mut addr_obj = Vec::new();
        let mut total = Vec::new();
        for i in 0..n {
            let offset = i * 6;
            let a = bin[offset];
            let t = u64::from_le_bytes([
                bin[offset+1],
                bin[offset+2],
                bin[offset+3],
                bin[offset+4],
                bin[offset+5],
                0, 0, 0
            ]);

            addr_obj.push(a);
            total.push(t);
        }

        let time_offset = n * 6;

        M_IT_TG_2 {
            addr_obj,
            total,
            time: TimeLabel::A(TimeLabelA::from_be_bytes([
                bin[time_offset],
                bin[time_offset+1],
                bin[time_offset+2],
                bin[time_offset+3],
                bin[time_offset+4]
            ]))
        }
    }

    pub fn into_bin(self) -> Vec<u8> {
        let mut bin = Vec::new();

        for i in 0..self.total.len() {
            // Serialize address (u8)
            bin.push(self.addr_obj[i]);

            // Serialize totals (u64)
            let total_array: [u8; 8] = self.total[i].to_be_bytes();
            let mut total_array = total_array.split_at(3).1.to_vec();
            total_array.reverse();

            for byte in total_array {
                bin.push(byte);
            }
        }

        bin.push(0x00);
        bin.push(0x81);
        bin.push(0xB2);
        bin.push(0x09);
        bin.push(0x09);

        bin
    }
}