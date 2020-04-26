use crate::util::coding;
pub fn hash_data(u_array: &[u8], n: usize, seed: u32) -> u32 {
    let m: u32 = 0xc6a4a793;
    let r: u32 = 24;
    let mut h: u32 = seed ^ (n * m as usize) as u32;
    let mut index: usize = 0;
    
    while index + 4 <= n {
        let w: u32 = coding::decode_fix_u32(&u_array[index..index+4]);
        index += 4;
        h += w;
        h = h.wrapping_mul(m);
        h ^= h >> 16;
    }

    let remain: usize = n - index;

    if remain == 3 {
        h += (u_array[2] as u32).wrapping_shl(16);
        h += (u_array[1] as u32).wrapping_shl(8);
        h += u_array[0] as u32;
        h = h.wrapping_mul(m);
        h ^= h >> r;
    } else if remain == 2 {
        h += (u_array[1] as u32).wrapping_shl(8) ;
        h += u_array[0] as u32;
        h = h.wrapping_mul(m);
        h ^= h >> r;
    } else if remain == 1 {
        h += u_array[0] as u32;
        h = h.wrapping_mul(m);
        h ^= h >> r;
    }
    return h;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hash() {
        let test_array1:[u8;1] = [0x62];
        let hased_value1 = hash_data(&test_array1, test_array1.len(), 0xbc9f1d34);
        assert_eq!(hased_value1, 0xef1345c4);
        let test_array2:[u8;2] = [0xc3, 0x97];
        let hased_value2 = hash_data(&test_array2, test_array2.len(), 0xbc9f1d34);
        assert_eq!(hased_value2, 0x5b663814);
        let test_array3:[u8;3] = [0xe2, 0x99, 0xa5];
        let hased_value3 = hash_data(&test_array3, test_array3.len(), 0xbc9f1d34);
        assert_eq!(hased_value3, 0x323c078f);
        let test_array4:[u8;4] = [0xe1, 0x80, 0xb9, 0x32];
        let hased_value4 = hash_data(&test_array4, test_array4.len(), 0xbc9f1d34);
        assert_eq!(hased_value4, 0xed21633a);
        let test_array5:[u8;48] = [0x01, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x14, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00,
        0x00, 0x00, 0x00, 0x14, 0x00, 0x00, 0x00, 0x18, 0x28, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let hased_value5 = hash_data(&test_array5, test_array5.len(), 0x12345678);
        assert_eq!(hased_value5, 0xf333dabb);
    }
}