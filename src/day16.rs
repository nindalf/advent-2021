use std::collections::VecDeque;

use crate::{
    hexreader::HexReader,
    packet::{Packet, PacketContent, ReadFrom},
};

use anyhow::Result;

#[allow(dead_code)]
fn packet_version_sum_all(input: &[String]) -> Result<u16> {
    Ok(input
        .iter()
        .map(|x| packet_version_sum(x))
        .collect::<Result<Vec<u16>>>()?
        .iter()
        .sum())
}

fn packet_version_sum(input: &str) -> Result<u16> {
    let mut reader = HexReader::new(input);
    let packet = Packet::read_from(&mut reader)?;
    let mut version_sum = 0;
    let mut q = VecDeque::new();
    q.push_back(packet);
    while let Some(packet) = q.pop_front() {
        version_sum += packet.header.version.inner();
        match packet.content {
            PacketContent::Literal(_) => {}
            PacketContent::NSubPackets(sub_packets) => {
                q.extend(sub_packets.into_iter());
            }
            PacketContent::SubPacketsInBits(sub_packets) => q.extend(sub_packets.into_iter()),
        }
    }
    Ok(version_sum)
}

#[allow(dead_code)]
fn packet_evalate(input: &str) -> Result<u64> {
    let mut reader = HexReader::new(input);
    let packet = Packet::read_from(&mut reader)?;
    Ok(packet.evaluate())
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    #[test]
    fn part_1_test() -> Result<()> {
        let input = crate::files::read_lines("inputs/day16-test.txt")?;
        assert_eq!(super::packet_version_sum_all(&input)?, 82);
        Ok(())
    }

    #[test]
    fn part_1_real() -> Result<()> {
        let input = crate::files::read_lines("inputs/day16.txt")?;
        assert_eq!(super::packet_version_sum_all(&input)?, 974);
        Ok(())
    }

    #[test]
    fn part_2_real() -> Result<()> {
        let input = crate::files::read_string("inputs/day16.txt")?;
        assert_eq!(super::packet_evalate(&input)?, 180616437720);
        Ok(())
    }
}
