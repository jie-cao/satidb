
pub fn decode_fix_u32(data:&[u8]) -> u32{
    return data[0] as u32 
            | (data[1] as u32).wrapping_shl(8)
            | (data[2] as u32).wrapping_shl(16)
            | (data[3] as u32).wrapping_shl(24); 
}