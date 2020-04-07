/// Frame binary structure
mod frame;

/// Time labels used in frames
mod time;

#[cfg(test)]
mod tests {
    use crate::frame::{MAX_FRAME, DynamicFrame};

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
        for i in 0..data.len() { bin[i] = data[i]; }

        // Try to deserialize the bin
        let df = DynamicFrame::from(bin);

        println!("{:?}", df);
    }
}
