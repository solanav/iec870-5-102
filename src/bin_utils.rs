pub fn u64_to_bin(num: u64) -> [u8; 8] {
    let b0 : u8 = ((num >> 56) & 0xff) as u8;
    let b1 : u8 = ((num >> 48) & 0xff) as u8;
    let b2 : u8 = ((num >> 40) & 0xff) as u8;
    let b3 : u8 = ((num >> 32) & 0xff) as u8;
    let b4 : u8 = ((num >> 24) & 0xff) as u8;
    let b5 : u8 = ((num >> 16) & 0xff) as u8;
    let b6 : u8 = ((num >> 8) & 0xff) as u8;
    let b7 : u8 = (num & 0xff) as u8;

    [b0, b1, b2, b3, b4, b5, b6, b7]
}

pub fn u32_to_bin(num: u32) -> [u8; 4] {
    let b0 : u8 = ((num >> 24) & 0xff) as u8;
    let b1 : u8 = ((num >> 16) & 0xff) as u8;
    let b2 : u8 = ((num >> 8) & 0xff) as u8;
    let b3 : u8 = (num & 0xff) as u8;

    [b0, b1, b2, b3]
}

pub fn u16_to_bin(num: u16) -> [u8; 2] {
    let b0 : u8 = ((num >> 8) & 0xff) as u8;
    let b1 : u8 = (num & 0xff) as u8;

    [b0, b1]
}
