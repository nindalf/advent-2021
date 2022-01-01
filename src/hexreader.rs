use anyhow::{anyhow, Result};
pub trait BitReader {
    fn read_bool(&mut self) -> Result<bool>;
    fn read_bits(&mut self, n: usize) -> Result<u16>;
}

pub struct HexReader {
    bytes: Vec<u8>,
    byte_offset: usize,
    bit_offset: usize,
}

impl BitReader for HexReader {
    #[allow(dead_code)]
    fn read_bool(&mut self) -> Result<bool> {
        Ok(self.read_bits(1)? == 1)
    }

    #[allow(dead_code)]
    fn read_bits(&mut self, n: usize) -> Result<u16> {
        // Only up to 16 bits supported
        if n > 16 || n == 0 {
            return Err(anyhow!("Unsupported length"));
        }
        // Nothing left to read
        if self.byte_offset >= self.bytes.len() {
            return Err(anyhow!("Reader empty"));
        }

        /*
        1. read n bits with offset in current byte
        2. read last n-x bits in current byte and first x bytes in the next byte. Join
        3. read n-x bits, 8 bits and x bits. Join.
        */
        if n <= 8 - self.bit_offset {
            let word = self.read_n_bits_with_offset(self.byte_offset, n, self.bit_offset)?;
            self.byte_offset += if n + self.bit_offset == 8 { 1 } else { 0 };
            self.bit_offset = (self.bit_offset + n) % 8;
            return Ok(word);
        }

        if self.byte_offset + 1 >= self.bytes.len() {
            return Err(anyhow!("Insufficient bits remaining"));
        }

        if n <= 16 - self.bit_offset {
            let first_byte_len = 8 - self.bit_offset;
            let second_byte_len = n - (8 - self.bit_offset);

            let first = self.read_last_n_bits(self.byte_offset, first_byte_len)? << second_byte_len;
            let second = self.read_first_n_bits(self.byte_offset + 1, second_byte_len)?;

            self.byte_offset += if n + self.bit_offset == 16 { 2 } else { 1 };
            self.bit_offset = (self.bit_offset + n) % 8;
            return Ok(first | second);
        }

        if self.byte_offset + 2 >= self.bytes.len() {
            return Err(anyhow!("Insufficient bits remaining"));
        }

        let first_byte_len = 8 - self.bit_offset;
        let second_byte_len = 8;
        let third_byte_len = n - (16 - self.bit_offset);
        let first = self.read_last_n_bits(self.byte_offset, first_byte_len)?
            << (second_byte_len + third_byte_len);
        let second =
            self.read_first_n_bits(self.byte_offset + 1, second_byte_len)? << third_byte_len;
        let third = self.read_first_n_bits(self.byte_offset + 2, third_byte_len)?;

        self.byte_offset += 2;
        self.bit_offset = (self.bit_offset + n) % 8;
        Ok(first | second | third)
    }
}

impl HexReader {
    #[allow(dead_code)]
    pub fn new(input: &str) -> HexReader {
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

    fn get_byte_at_offset(&self, offset: usize) -> Result<u8> {
        self.bytes
            .get(offset)
            .copied()
            .ok_or(anyhow!("Out of bounds access"))
    }

    fn read_last_n_bits(&self, byte_offset: usize, n: usize) -> Result<u16> {
        if n == 0 {
            return Ok(0);
        }
        let mut byte = self.get_byte_at_offset(byte_offset)?;
        byte <<= 8 - n;
        byte >>= 8 - n;
        Ok(byte as u16)
    }

    fn read_n_bits_with_offset(
        &self,
        byte_offset: usize,
        n: usize,
        bit_offset: usize,
    ) -> Result<u16> {
        if n == 0 {
            return Ok(0);
        }
        let mut byte = self.get_byte_at_offset(byte_offset)?;
        byte <<= bit_offset;
        byte >>= 8 - n;
        Ok(byte as u16)
    }

    fn read_first_n_bits(&self, byte_offset: usize, n: usize) -> Result<u16> {
        if n == 0 {
            return Ok(0);
        }
        let mut byte = self.get_byte_at_offset(byte_offset)?;
        byte >>= 8 - n;
        Ok(byte as u16)
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
    fn test_read_few_bits() {
        let mut reader = HexReader::new("D2FE28");
        assert_eq!(reader.bytes, vec![0b11010010, 0b11111110, 0b00101000]);
        assert_eq!(reader.read_bits(3).unwrap(), 0b110);
        assert_eq!(reader.read_bits(3).unwrap(), 0b100);
        assert_eq!(reader.read_bits(3).unwrap(), 0b101);
        assert_eq!(reader.read_bits(8).unwrap(), 0b11111100);
        assert_eq!(reader.read_bits(6).unwrap(), 0b010100);
        assert!(reader.read_bits(2).is_err());
        assert_eq!(reader.read_bits(1).unwrap(), 0b0);
        assert!(reader.read_bits(1).is_err());
    }

    #[test]
    fn test_read_many_bits() {
        let mut reader = HexReader::new("D2FE28D2FE28");
        assert_eq!(
            reader.bytes,
            vec![0b11010010, 0b11111110, 0b00101000, 0b11010010, 0b11111110, 0b00101000]
        );
        assert_eq!(reader.read_bits(10).unwrap(), 0b1101001011);
        assert_eq!(reader.read_bits(14).unwrap(), 0b11111000101000);
        assert_eq!(reader.read_bits(3).unwrap(), 0b110);
        assert_eq!(reader.read_bits(15).unwrap(), 0b100101111111000);
        assert_eq!(reader.read_bits(6).unwrap(), 0b101000);
    }

    #[test]
    fn test_read_last_n_bits() {
        let reader = HexReader::new("D2");
        assert_eq!(reader.bytes, vec![0b11010010]);
        assert_eq!(reader.read_last_n_bits(0, 0).unwrap(), 0b0);
        assert_eq!(reader.read_last_n_bits(0, 1).unwrap(), 0b0);
        assert_eq!(reader.read_last_n_bits(0, 4).unwrap(), 0b0010);
        assert_eq!(reader.read_last_n_bits(0, 5).unwrap(), 0b10010);
        assert_eq!(reader.read_last_n_bits(0, 7).unwrap(), 0b1010010);
        assert_eq!(reader.read_last_n_bits(0, 8).unwrap(), 0b11010010);
    }

    #[test]
    fn test_read_first_n_bits() {
        let reader = HexReader::new("D2");
        assert_eq!(reader.bytes, vec![0b11010010]);
        assert_eq!(reader.read_first_n_bits(0, 0).unwrap(), 0b0);
        assert_eq!(reader.read_first_n_bits(0, 1).unwrap(), 0b1);
        assert_eq!(reader.read_first_n_bits(0, 4).unwrap(), 0b1101);
        assert_eq!(reader.read_first_n_bits(0, 5).unwrap(), 0b11010);
        assert_eq!(reader.read_first_n_bits(0, 7).unwrap(), 0b1101001);
        assert_eq!(reader.read_first_n_bits(0, 8).unwrap(), 0b11010010);
    }

    #[test]
    fn test_read_n_bits_with_offset() {
        let reader = HexReader::new("D2");
        assert_eq!(reader.bytes, vec![0b11010010]);
        assert_eq!(reader.read_n_bits_with_offset(0, 0, 0).unwrap(), 0b0);
        assert_eq!(reader.read_n_bits_with_offset(0, 0, 4).unwrap(), 0b0);
        assert_eq!(reader.read_n_bits_with_offset(0, 1, 1).unwrap(), 0b1);
        assert_eq!(reader.read_n_bits_with_offset(0, 2, 1).unwrap(), 0b10);
        assert_eq!(reader.read_n_bits_with_offset(0, 4, 0).unwrap(), 0b1101);
        assert_eq!(reader.read_n_bits_with_offset(0, 4, 3).unwrap(), 0b1001);
        assert_eq!(reader.read_n_bits_with_offset(0, 5, 0).unwrap(), 0b11010);
        assert_eq!(reader.read_n_bits_with_offset(0, 5, 2).unwrap(), 0b01001);
        assert_eq!(reader.read_n_bits_with_offset(0, 7, 0).unwrap(), 0b1101001);
        assert_eq!(reader.read_n_bits_with_offset(0, 7, 1).unwrap(), 0b1010010);
        assert_eq!(reader.read_n_bits_with_offset(0, 8, 0).unwrap(), 0b11010010);
    }

    #[test]
    fn test_read_unsupported_lengths() {
        let mut reader = HexReader::new("D2FE28");
        assert_eq!(reader.bytes, vec![0b11010010, 0b11111110, 0b00101000]);
        assert!(reader.read_bits(0).is_err());
        assert!(reader.read_bits(17).is_err());

        // Unsupported read doesn't affect reader
        assert_eq!(reader.read_bits(8).unwrap(), 0b11010010);
        assert_eq!(reader.read_bits(16).unwrap(), 0b1111111000101000);
        assert!(reader.read_bits(8).is_err());
    }
}
