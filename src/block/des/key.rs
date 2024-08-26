use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
pub struct Key {
    data: u64,
}

impl FromStr for Key {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 8 {
            return Err(());
        }

        if !s.is_ascii() {
            return Err(());
        }

        let bytes: [u8; 8] = s.as_bytes().try_into().map_err(|_| ())?;
        let data = u64::from_be_bytes(bytes);
        Ok(Key { data })
    }
}

impl Key {
    pub fn get_data(&self) -> u64 {
        self.data
    }

    fn permutated_choice2(c: u32, d: u32) -> u64 {
        const PC2_TABLE: [usize; 48] = [
            14, 17, 11, 24,  1,  5,  3, 28,
            15,  6, 21, 10, 23, 19, 12,  4,
            26,  8, 16,  7, 27, 20, 13,  2,
            41, 52, 31, 37, 47, 55, 30, 40,
            51, 45, 33, 48, 44, 49, 39, 56,
            34, 53, 46, 42, 50, 36, 29, 32,
        ];

        let mut permuted_key: u64 = 0;

        for (i, &position) in PC2_TABLE.iter().enumerate() {
            let bit_position = position - 1; // Convertir a base 0
            let bit_value = if bit_position < 28 {
                (c >> (27 - bit_position)) & 1
            } else {
                (d >> (55 - bit_position)) & 1
            };
            permuted_key |= (bit_value as u64) << (47 - i);
        }

        permuted_key
    }

    fn permutated_choice1(&self) -> (u32, u32) {
        // Tabla de permutación PC1
        const PC1_TABLE: [usize; 56] = [
            57, 49, 41, 33, 25, 17, 9,
            1, 58, 50, 42, 34, 26, 18,
            10, 2, 59, 51, 43, 35, 27,
            19, 11, 3, 60, 52, 44, 36,
            63, 55, 47, 39, 31, 23, 15,
            7, 62, 54, 46, 38, 30, 22,
            14, 6, 61, 53, 45, 37, 29,
            21, 13, 5, 28, 20, 12, 4,
        ];
    
        let mut c: u32 = 0;
        let mut d: u32 = 0;
    
        for (i, &position) in PC1_TABLE.iter().enumerate() {
            let bit_value = (self.data >> (64 - position)) & 1;
    
            if i < 28 {
                c |= (bit_value as u32) << (27 - i);
            } else {
                d |= (bit_value as u32) << (27 - (i - 28));
            }
        }
    
        (c, d)
    }
    

    fn circular_left_shift(bits: u32, shift_by: u32) -> u32 {
        let mask = 0x0FFFFFFF; // Máscara de 28 bits
        let bits = bits & mask; // Aseguramos que solo los 28 bits menos significativos se utilicen
        ((bits << shift_by) & mask) | (bits >> (28 - shift_by))
    }
    

    pub fn generate_subkeys(&self) -> [Key; 16] {
        let mut subkeys = [Key { data: 0 }; 16];

        let (mut c, mut d) = self.permutated_choice1();

        for round in 0..16 {
            let shifts = match round + 1 {
                1 | 2 | 9 | 16 => 1,
                _ => 2,
            };
            c = Self::circular_left_shift(c, shifts);
            d = Self::circular_left_shift(d, shifts);
            
            subkeys[round] = Key { data: Self::permutated_choice2(c, d) };
        }

        subkeys
    }

    pub fn string_bits(&self) -> String {
        let mut output = String::new();
        for i in 0..64 {
            let curr_bit = if (self.data >> (63 - i)) & 1 == 1 { '1' } else { '0' };
            output.push(curr_bit);
        }
        output
    }

    pub fn mock_key() -> Self {
        Key {
            data: 0x0000FFFFFFFFFFFF,
        }
    }

    pub fn from_64bits_number(number:u64) -> Self{
        Key{
            data: number
        }
    }

    pub fn to_hex_string(&self) -> String {
        let mut result = String::with_capacity(16);
        let input = self.data;

        for i in 0..8 {
            let byte = (input >> (56 - i * 8)) as u8;
            result.push_str(&format!("{:02X}", byte));
        }
        result.insert_str(0, "0x");
        result
    }
}
