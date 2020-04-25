
    pub fn hash_data(data:&str) {
        let u_array:&[u8] = data.as_bytes();
        for (i, x) in u_array.iter().enumerate() {
            println!("Item {} = {}", i, x);
        }
    }