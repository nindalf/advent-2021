use crate::hexreader::BitReader;

pub trait ReadFrom<T: BitReader> {
    fn read_from(reader: &mut T) -> Self;
}

pub struct PacketHeader {
    pub version: Version,
    pub packet_type: PacketType,
}

impl<T: BitReader> ReadFrom<T> for PacketHeader {
    fn read_from(reader: &mut T) -> Self {
        let version = Version::read_from(reader);
        let packet_type = PacketType::read_from(reader);
        PacketHeader {
            version,
            packet_type,
        }
    }
}

pub struct Version(u8);

impl<T: BitReader> ReadFrom<T> for Version {
    fn read_from(reader: &mut T) -> Self {
        match reader.read_bits(3) {
            Some(version) => Version(version),
            None => panic!("Corrupt packet"),
        }
    }
}

pub enum PacketType {
    Literal,
    Operator,
}

impl<T: BitReader> ReadFrom<T> for PacketType {
    fn read_from(reader: &mut T) -> Self {
        match reader.read_bits(3) {
            Some(n) => match n {
                4 => PacketType::Literal,
                _ => PacketType::Operator,
            },
            None => panic!("Corrupt packet"),
        }
    }
}
