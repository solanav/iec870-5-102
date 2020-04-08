/// Frame binary structure
mod frame;

/// Time labels used in frames
mod time;

/// Types as defined on the protocol
mod types;

#[cfg(test)]
mod tests {
    use crate::frame::{MAX_FRAME, DynamicFrame};
    use crate::types::M_IT_TG_2;

    #[test]
    fn asdu_122() {
        // Create the bin and the data
        let mut bin= [0u8; MAX_FRAME];
        let data = [
            0x68, 0x15, 0x15, 0x68,
            0x73, 0x58, 0x1B, 0x7A,
            0x01, 0x06, 0x01, 0x00,
            0x0B, 0x01, 0x08, 0x00,
            0x0B, 0x07, 0x02, 0x0A,
            0x00, 0x11, 0x0A, 0x02,
            0x0A, 0xC1, 0x16,
        ];

        // Copy data to binary load
        for i in 0..data.len() {
            bin[i] = data[i];
        }

        // Try to deserialize the bin
        let df = DynamicFrame::from(bin);

        println!("122 > {:?}", df);
    }

    #[test]
    fn asdu_8() {
        // Create the bin and the data
        let mut bin= [0u8; MAX_FRAME];
        let data = [
            0x68, 0x3E, 0x3E, 0x68,
            0x08, 0x58, 0x1B, 0x08,
            0x08, 0x05, 0x01, 0x00,
            0x0B, 0x01, 0x18, 0x01,
            0x00, 0x00, 0x00, 0x02,
            0x6E, 0x1F, 0x03, 0x00,
            0x00, 0x03, 0x04, 0x00,
            0x00, 0x00, 0x00, 0x04,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x05, 0xCC, 0xBE,
            0x00, 0x00, 0x00, 0x06,
            0x98, 0x0D, 0x00, 0x00,
            0x00, 0x07, 0x00, 0x00,
            0x00, 0x00, 0x80, 0x08,
            0x00, 0x00, 0x00, 0x00,
            0x80, 0x00, 0x81, 0xB2,
            0x09, 0x09, 0xE1, 0x16,
        ];

        // Copy data to binary load
        for i in 0..data.len() {
            bin[i] = data[i];
        }

        // Try to deserialize the bin
        let df = DynamicFrame::from(bin);
        let parsed = M_IT_TG_2::from_bin(df.data(), df.n());
        let serialized = parsed.into_bin();

        assert_eq!(serialized, df.data());
    }
}
