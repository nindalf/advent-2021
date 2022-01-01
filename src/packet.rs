use crate::hexreader::BitReader;

use anyhow::Result;

pub trait ReadFrom<T: BitReader> {
    fn read_from(reader: &mut T) -> Result<Self>
    where
        Self: Sized;
}

#[derive(Debug, PartialEq, Eq)]
pub struct Packet {
    header: PacketHeader,
    content: PacketContent,
}

impl<T: BitReader> ReadFrom<T> for Packet {
    fn read_from(reader: &mut T) -> Result<Self> {
        let header = PacketHeader::read_from(reader)?;
        let content = PacketContent::read_from(&header.packet_type, reader)?;

        Ok(Packet { header, content })
    }
}

impl Packet {
    fn len(&self) -> usize {
        PacketHeader::len() + self.content.len()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct PacketHeader {
    pub version: Version,
    pub packet_type: PacketType,
}

impl<T: BitReader> ReadFrom<T> for PacketHeader {
    fn read_from(reader: &mut T) -> Result<Self> {
        let version = Version::read_from(reader)?;
        let packet_type = PacketType::read_from(reader)?;
        Ok(PacketHeader {
            version,
            packet_type,
        })
    }
}

impl PacketHeader {
    fn len() -> usize {
        Version::len() + PacketType::len()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Version(u16);

impl<T: BitReader> ReadFrom<T> for Version {
    fn read_from(reader: &mut T) -> Result<Self> {
        Ok(Version(reader.read_bits(Version::len())?))
    }
}

impl Version {
    fn len() -> usize {
        3
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum PacketType {
    Literal,
    Operator(u16),
}

impl<T: BitReader> ReadFrom<T> for PacketType {
    fn read_from(reader: &mut T) -> Result<Self> {
        let packet_type = match reader.read_bits(PacketType::len())? {
            4 => PacketType::Literal,
            val => PacketType::Operator(val),
        };
        Ok(packet_type)
    }
}

impl PacketType {
    fn len() -> usize {
        3
    }
}

#[derive(Debug, Eq)]
enum PacketContent {
    Literal(LiteralValue),
    SubPacketsInBits(Vec<Packet>),
    NSubPackets(Vec<Packet>),
}

impl PacketContent {
    fn len(&self) -> usize {
        match self {
            PacketContent::Literal(literal) => literal.len(),
            PacketContent::SubPacketsInBits(packets) => {
                LengthType::len()
                    + SubPacketLengthInBits::len()
                    + packets.iter().map(Packet::len).sum::<usize>()
            }
            PacketContent::NSubPackets(packets) => {
                LengthType::len()
                    + NumSubPackets::len()
                    + packets.iter().map(Packet::len).sum::<usize>()
            }
        }
    }

    // It's a hack. Ideally I would like to implement the trait for uniformity.
    // Or I move this code to Packet
    fn read_from<T: BitReader>(packet_type: &PacketType, reader: &mut T) -> Result<Self> {
        if packet_type == &PacketType::Literal {
            return Ok(PacketContent::Literal(LiteralValue::read_from(reader)?));
        }

        let length_type = LengthType::read_from(reader)?;
        let mut sub_packets = Vec::new();
        match length_type {
            LengthType::LengthInBits => {
                let expected_length = SubPacketLengthInBits::read_from(reader)?;
                let mut length_so_far = 0;
                while length_so_far < expected_length.0 {
                    let sub_packet = Packet::read_from(reader)?;
                    length_so_far += sub_packet.len() as u16;
                    sub_packets.push(sub_packet);
                }
                Ok(PacketContent::SubPacketsInBits(sub_packets))
            }
            LengthType::LengthInPackets => {
                let n = NumSubPackets::read_from(reader)?;
                for _ in 0..n.0 {
                    sub_packets.push(Packet::read_from(reader)?);
                }
                Ok(PacketContent::NSubPackets(sub_packets))
            }
        }
    }
}

impl PartialEq for PacketContent {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Literal(l0), Self::Literal(r0)) => l0 == r0,
            (Self::SubPacketsInBits(l0), Self::SubPacketsInBits(r0)) => {
                l0.len() == r0.len() && l0.iter().zip(r0.iter()).all(|(x, y)| *x == *y)
            }
            (Self::NSubPackets(l0), Self::NSubPackets(r0)) => {
                l0.len() == r0.len() && l0.iter().zip(r0.iter()).all(|(x, y)| *x == *y)
            }
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct LiteralValue {
    value: u16,
    len: usize,
}

impl<T: BitReader> ReadFrom<T> for LiteralValue {
    fn read_from(reader: &mut T) -> Result<Self> {
        let mut value = 0u16;
        let mut len = 0;

        let mut keep_reading = reader.read_bool()?;
        while keep_reading {
            value = (value << 4) + reader.read_bits(4)?;
            len += 5;
            keep_reading = reader.read_bool()?;
        }
        value = (value << 4) + reader.read_bits(4)?;
        len += 5;

        Ok(LiteralValue { value, len })
    }
}

impl LiteralValue {
    fn len(&self) -> usize {
        self.len
    }
}

#[derive(Debug)]
pub struct SubPacketLengthInBits(u16);

impl<T: BitReader> ReadFrom<T> for SubPacketLengthInBits {
    fn read_from(reader: &mut T) -> Result<Self> {
        Ok(SubPacketLengthInBits(
            reader.read_bits(SubPacketLengthInBits::len())?,
        ))
    }
}

impl SubPacketLengthInBits {
    fn len() -> usize {
        15
    }
}

#[derive(Debug)]
pub struct NumSubPackets(u16);

impl<T: BitReader> ReadFrom<T> for NumSubPackets {
    fn read_from(reader: &mut T) -> Result<Self> {
        Ok(NumSubPackets(reader.read_bits(NumSubPackets::len())?))
    }
}

impl NumSubPackets {
    fn len() -> usize {
        11
    }
}

#[derive(Debug)]
pub enum LengthType {
    LengthInBits,
    LengthInPackets,
}

impl<T: BitReader> ReadFrom<T> for LengthType {
    fn read_from(reader: &mut T) -> Result<Self> {
        match reader.read_bits(LengthType::len())? {
            0 => Ok(LengthType::LengthInBits),
            1 => Ok(LengthType::LengthInPackets),
            _ => unreachable!("Invalid length type"),
        }
    }
}

impl LengthType {
    fn len() -> usize {
        1
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::hexreader::HexReader;
    use anyhow::Result;

    #[test]
    fn parse_literal() -> Result<()> {
        let mut reader = HexReader::new("D2FE28");

        let packet = Packet::read_from(&mut reader)?;
        assert_eq!(packet.header.version, Version(6));
        assert_eq!(packet.header.packet_type, PacketType::Literal);
        assert!(matches!(
            packet.content,
            PacketContent::Literal(LiteralValue {
                value: 2021,
                len: 15
            })
        ));
        assert_eq!(packet.len(), 21);
        Ok(())
    }

    #[test]
    fn parse_bit_wise_subpackets() -> Result<()> {
        let mut reader = HexReader::new("38006F45291200");

        let packet = Packet::read_from(&mut reader)?;

        assert_eq!(
            packet,
            Packet {
                header: PacketHeader {
                    version: Version(0b001),
                    packet_type: PacketType::Operator(0b110),
                },
                content: PacketContent::SubPacketsInBits(vec![
                    Packet {
                        header: PacketHeader {
                            version: Version(0b110),
                            packet_type: PacketType::Literal,
                        },
                        content: PacketContent::Literal(LiteralValue { value: 10, len: 5 }),
                    },
                    Packet {
                        header: PacketHeader {
                            version: Version(0b010),
                            packet_type: PacketType::Literal,
                        },
                        content: PacketContent::Literal(LiteralValue { value: 20, len: 10 }),
                    }
                ])
            }
        );
        assert_eq!(packet.len(), 49);
        Ok(())
    }

    #[test]
    fn parse_number_wise_subpackets() -> Result<()> {
        let mut reader = HexReader::new("EE00D40C823060");

        let packet = Packet::read_from(&mut reader)?;

        assert_eq!(
            packet,
            Packet {
                header: PacketHeader {
                    version: Version(0b111),
                    packet_type: PacketType::Operator(0b011),
                },
                content: PacketContent::NSubPackets(vec![
                    Packet {
                        header: PacketHeader {
                            version: Version(0b010),
                            packet_type: PacketType::Literal,
                        },
                        content: PacketContent::Literal(LiteralValue { value: 1, len: 5 }),
                    },
                    Packet {
                        header: PacketHeader {
                            version: Version(0b100),
                            packet_type: PacketType::Literal,
                        },
                        content: PacketContent::Literal(LiteralValue { value: 2, len: 5 }),
                    },
                    Packet {
                        header: PacketHeader {
                            version: Version(0b001),
                            packet_type: PacketType::Literal,
                        },
                        content: PacketContent::Literal(LiteralValue { value: 3, len: 5 }),
                    }
                ])
            }
        );
        assert_eq!(packet.len(), 51);
        Ok(())
    }
}
