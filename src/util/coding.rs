
pub fn decode_fix_u32(data:&[u8]) -> u32{
    return data[0] as u32 
            | (data[1] << 8) as u32
            | (data[2] << 16) as u32
            | (data[3] << 24) as u32; 
}