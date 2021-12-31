pub trait BitReader {
    fn read_bits(&mut self, n: usize) -> Option<u8>;
}

struct HexReader {
    bytes: Vec<u8>,
    byte_offset: usize,
    bit_offset: usize,
}

impl BitReader for HexReader {
    // Got this right first try.
    #[allow(dead_code)]
    fn read_bits(&mut self, n: usize) -> Option<u8> {
        // Only up to 8 bits supported
        if n > 8 || n == 0 {
            return None;
        }
        // Nothing left to read
        if self.byte_offset >= self.bytes.len() {
            return None;
        }
        let mut byte = *self.bytes.get(self.byte_offset)?;
        if n <= 8 - self.bit_offset {
            // clear bit_offset bits
            byte <<= self.bit_offset;
            // select n bits
            byte >>= 8 - n;

            self.byte_offset += if n + self.bit_offset == 8 { 1 } else { 0 };
            self.bit_offset = (self.bit_offset + n) % 8;
        } else {
            if self.byte_offset + 1 >= self.bytes.len() {
                return None;
            }
            let first_len = 8 - self.bit_offset;
            let second_len = n - first_len;
            let second_byte = *self.bytes.get(self.byte_offset + 1)?;
            byte = ((byte << self.bit_offset) >> self.bit_offset) << second_len;
            byte |= second_byte >> (8 - second_len);

            self.byte_offset += 1;
            self.bit_offset = (self.bit_offset + n) % 8;
        }
        Some(byte)
    }
}

impl HexReader {
    #[allow(dead_code)]
    fn new(input: &str) -> HexReader {
        let nibbles = input
            .chars()
            .map(Self::map_char_to_bits)
            .collect::<Vec<u8>>();
        let bytes = HexReader::nibbles_to_bytes(nibbles);
        let byte_offset = 0;
        let bit_offset = 0;

        HexReader {
            bytes,
            byte_offset,
            bit_offset,
        }
    }

    fn map_char_to_bits(c: char) -> u8 {
        match c {
            '0' => 0b0000,
            '1' => 0b0001,
            '2' => 0b0010,
            '3' => 0b0011,
            '4' => 0b0100,
            '5' => 0b0101,
            '6' => 0b0110,
            '7' => 0b0111,
            '8' => 0b1000,
            '9' => 0b1001,
            'A' => 0b1010,
            'B' => 0b1011,
            'C' => 0b1100,
            'D' => 0b1101,
            'E' => 0b1110,
            'F' => 0b1111,
            _ => unreachable!("Illegal character"),
        }
    }

    fn nibbles_to_bytes(nibbles: Vec<u8>) -> Vec<u8> {
        let mut result = Vec::new();
        let n = if nibbles.len() % 2 == 0 {
            nibbles.len()
        } else {
            nibbles.len() + 1
        };
        for i in (0..n).step_by(2) {
            let higher = nibbles.get(i);
            let lower = nibbles.get(i + 1);
            let byte: u8 = match (higher, lower) {
                (Some(higher), Some(lower)) => (higher << 4) + lower,
                (Some(higher), None) => (higher << 4),
                (_, _) => unreachable!("Impossible"),
            };
            result.push(byte)
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::{BitReader, HexReader};

    #[test]
    fn test_construct() {
        let reader = HexReader::new("D2FE28");
        assert_eq!(reader.bytes, vec![0b11010010, 0b11111110, 0b00101000]);

        let reader = HexReader::new("E");
        assert_eq!(reader.bytes, vec![0b11100000]);

        let reader = HexReader::new("D2E");
        assert_eq!(reader.bytes, vec![0b11010010, 0b11100000]);

        let reader = HexReader::new("");
        assert_eq!(reader.bytes, vec![]);
    }

    #[test]
    #[should_panic(expected = "Illegal character")]
    fn test_illegal_character() {
        HexReader::new("G");
    }

    #[test]
    fn test_read_bits() {
        let mut reader = HexReader::new("D2FE28");
        assert_eq!(reader.bytes, vec![0b11010010, 0b11111110, 0b00101000]);
        assert_eq!(reader.read_bits(3).unwrap(), 0b110);
        assert_eq!(reader.read_bits(3).unwrap(), 0b100);
        assert_eq!(reader.read_bits(3).unwrap(), 0b101);
        assert_eq!(reader.read_bits(8).unwrap(), 0b11111100);
        assert_eq!(reader.read_bits(6).unwrap(), 0b010100);
        assert_eq!(reader.read_bits(2), None);
        assert_eq!(reader.read_bits(1).unwrap(), 0b0);
        assert_eq!(reader.read_bits(1), None);
    }

    #[test]
    fn test_read_unsupported_lengths() {
        let mut reader = HexReader::new("D2FE28");
        assert_eq!(reader.bytes, vec![0b11010010, 0b11111110, 0b00101000]);
        assert_eq!(reader.read_bits(0), None);
        assert_eq!(reader.read_bits(9), None);

        // Unsupported read doesn't affect reader
        assert_eq!(reader.read_bits(8).unwrap(), 0b11010010);
        assert_eq!(reader.read_bits(8).unwrap(), 0b11111110);
        assert_eq!(reader.read_bits(8).unwrap(), 0b00101000);
        assert_eq!(reader.read_bits(8), None);
    }
}
