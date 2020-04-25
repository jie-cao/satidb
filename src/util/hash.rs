use crate::util::coding;
    pub fn hash_data(u_array:&[u8], n:i64, seed:u32) -> u32{
        let m:u32 = 0xc6a4a793;
        let r:u32 = 24;
        let mut h:u32 = seed ^ (n * m as i64) as u32;
        //let u_array:&[u8] = data.as_bytes();
        let mut index:i64 = 0;
        while index + 4 < n {
            let w:u32 = coding::decode_fix_u32(&u_array);
            index += 4;
            h += w;
            h *= m;
            h ^= h>>16;
        }

        let remain:i64 = index - n;
        if remain == 3 {
            h += (u_array[2] << 16) as u32; 
            h += (u_array[1] << 8) as u32;

        } else if remain == 2 {
            h += (u_array[1] << 8) as u32;
        }

        h += u_array[0] as u32;
        h *= m;
        h ^= h >> r;
        return h;
    }