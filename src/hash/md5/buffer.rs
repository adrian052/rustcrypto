use crate::hash::md5::Block512;



pub struct Buffer{
    data: [u32; 4]
}


impl Buffer{
    pub fn new() -> Self {
        Buffer {
            data: [
                0x67452301, 
                0xefcdab89, 
                0x98badcfe, 
                0x10325476,
            ]
        }
    }

    pub fn process_block(&mut self, block: &Block512) {
    
        let mut a = self.data[0].clone();
        let mut b = self.data[1].clone();
        let mut c = self.data[2].clone();
        let mut d = self.data[3].clone();
        
        let mut new_buffer = Buffer {
            data: [0,0,0,0]
        };

        let s = [
            7, 12, 17, 22, 5, 9, 14, 20,
            4, 11, 16, 23, 6, 10, 15, 21,
        ];
    
        let k = [
            0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee,
            0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
            0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be,
            0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
    
            0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa,
            0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
            0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
            0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
    
            0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c,
            0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
            0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05,
            0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
    
            0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039,
            0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
            0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1,
            0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391
        ];
    
        for i in 0..64 {
            let (f, g) = match i {
                0..=15 => ((b & c) | (!b & d), i),                  
                16..=31 => ((d & b) | (!d & c), (5 * i + 1) % 16),  
                32..=47 => (b ^ c ^ d, (3 * i + 5) % 16),           
                _ => (c ^ (b | !d), (7 * i) % 16),                  
            };

            let temp = d;
            d = c;
            c = b;
            b = b.wrapping_add(
                a.wrapping_add(f)
                    .wrapping_add(k[i])
                    .wrapping_add(block.get_word(g))
                    .rotate_left(s[(i / 16) * 4 + i % 4]),
            );
            a = temp;

            new_buffer = Buffer {
                data: [a, b, c, d]
            };
        }
        self.sum_mod_2_32_inplace(&new_buffer);
        self.reorganize_bytes_inplace();
        self.data.reverse();
    }


    pub fn get_digest(&self) -> String {
        self.data
            .iter()
            .map(|&word| format!("{:08x}", word))
            .collect::<Vec<String>>()
            .concat()
    }

    pub fn sum_mod_2_32_inplace(&mut self, other: &Buffer) {
        for i in 0..4 {
            self.data[i] = self.data[i].wrapping_add(other.data[i]);
        }
    }

    pub fn reorganize_bytes_inplace(&mut self) {

        let mut temp = [0u8; 16];

        let bytes: Vec<u8> = self.data.iter()
            .flat_map(|&n| n.to_le_bytes())
            .collect();

        for (i, &byte) in bytes.iter().enumerate() {
            let idx = 15 - i;
            temp[idx] = byte;
        }

        for i in 0..4 {
            self.data[i] = u32::from_le_bytes([
                temp[i * 4],
                temp[i * 4 + 1],
                temp[i * 4 + 2],
                temp[i * 4 + 3],
            ]);
        }
    }
}