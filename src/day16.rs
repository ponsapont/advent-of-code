use std::convert::TryInto;

use anyhow::Result;

#[derive(Debug)]
pub struct Packet {
    // Three bits for version
    pub version: u32,
    // Three bits for type_id
    pub type_id: u32,
    // data
    pub packet_data: PacketData,
}

#[derive(Debug)]
pub enum PacketData {
    Literal(u64),
    Operator((Operation, Vec<Packet>)),
}

#[derive(Debug)]
pub enum Operation {
    Sum,
    Product,
    Minimum,
    Maximum,
    Greater,
    Less,
    Equal,
}

impl Packet {
    pub fn new(data: &mut [u8], byte_offset: &mut usize, bit_offset: &mut usize) -> Self {
        let version = Self::get_bits_val(data, byte_offset, bit_offset, 3);
        let type_id = Self::get_bits_val(data, byte_offset, bit_offset, 3);
        let packet_data = match type_id {
            4 => {
                let mut is_last = Self::get_bits_val(data, byte_offset, bit_offset, 1);
                let mut values = Vec::new();
                loop {
                    values.push(Self::get_bits_val(data, byte_offset, bit_offset, 4));
                    if is_last == 0 {
                        break;
                    }
                    is_last = Self::get_bits_val(data, byte_offset, bit_offset, 1);
                }
                let number = values
                    .iter()
                    .rev()
                    .enumerate()
                    .fold(0, |acc, (idx, x)| acc + ((*x as u64) << (idx * 4)));
                // Update data for other packets
                PacketData::Literal(number)
            }
            _ => {
                let operation = match type_id {
                    0 => Operation::Sum,
                    1 => Operation::Product,
                    2 => Operation::Minimum,
                    3 => Operation::Maximum,
                    5 => Operation::Greater,
                    6 => Operation::Less,
                    7 => Operation::Equal,
                    _ => panic!("Not supported"),
                };
                let length_type_id = Self::get_bits_val(data, byte_offset, bit_offset, 1);
                let mut subpackets = Vec::new();
                match length_type_id {
                    0 => {
                        let mut total_length = Self::get_bits_val(data, byte_offset, bit_offset, 15) as usize;
                        total_length += *byte_offset * 8 + *bit_offset;
                        while (*byte_offset * 8 + *bit_offset) < total_length as usize {
                            subpackets.push(Packet::new(data, byte_offset, bit_offset));
                        }
                    }
                    1 => {
                        let subpackets_num = Self::get_bits_val(data, byte_offset, bit_offset, 11);
                        for _ in 0..subpackets_num {
                            subpackets.push(Packet::new(data, byte_offset, bit_offset));
                        }
                    }
                    _ => panic!("Impossible"),
                }
                PacketData::Operator((operation, subpackets))
            }
        };
        Self {
            version,
            type_id,
            packet_data,
        }
    }

    fn get_bits_val(
        data: &[u8],
        byte_offset: &mut usize,
        bit_offset: &mut usize,
        num_bits: usize,
    ) -> u32 {
        let mut d = Vec::new();
        for n in *byte_offset..=*byte_offset + 3 {
            d.push(if n < data.len() { data[n] } else { 0 });
        }
        let val = u32::from_be_bytes(d.try_into().unwrap());
        let shift = 32 - num_bits - *bit_offset;
        *byte_offset += (*bit_offset + num_bits) / 8;
        *bit_offset = (*bit_offset + num_bits) % 8;
        let mask = match num_bits {
            1 => 0x01,
            3 => 0x07,
            4 => 0x0f,
            11 => 0x7ff,
            15 => 0x7fff,
            _ => panic!("Not implemented"),
        };
        (val >> shift) & mask
    }

    pub fn get_version_sum(&self) -> u32 {
        self.version
            + match &self.packet_data {
                PacketData::Operator((_, subpackets)) => {
                    subpackets.iter().map(|s| s.get_version_sum()).sum()
                }
                _ => 0,
            }
    }

    pub fn calculate(&self) -> u64 {
        match &self.packet_data {
            PacketData::Literal(value) => *value,
            PacketData::Operator((operator, packets)) => match operator {
                Operation::Sum => packets.iter().map(|p| p.calculate()).sum(),
                Operation::Product => packets.iter().map(|p| p.calculate()).product(),
                Operation::Minimum => packets.iter().map(|p| p.calculate()).min().unwrap(),
                Operation::Maximum => packets.iter().map(|p| p.calculate()).max().unwrap(),
                Operation::Greater => {
                    if packets[0].calculate() > packets[1].calculate() {
                        1
                    } else {
                        0
                    }
                }
                Operation::Less => {
                    if packets[0].calculate() < packets[1].calculate() {
                        1
                    } else {
                        0
                    }
                }
                Operation::Equal => {
                    if packets[0].calculate() == packets[1].calculate() {
                        1
                    } else {
                        0
                    }
                }
            },
        }
    }
}

pub fn parse_input(input: &str) -> Vec<u8> {
    let chars: Vec<char> = input.chars().collect();
    chars
        .chunks_exact(2)
        .map(|chunk| {
            let byte = format!("{}{}", chunk[0], chunk[1]);
            u8::from_str_radix(&byte, 16).unwrap()
        })
        .collect()
}

pub fn part1(input: &str) -> Result<()> {
    let mut bytes = parse_input(input);
    let (mut byte_offset, mut bit_offset) = (0, 0);
    let mut packets = Vec::new();
    while byte_offset < bytes.len() {
        let packet = Packet::new(&mut bytes, &mut byte_offset, &mut bit_offset);
        packets.push(packet);
    }
    println!(
        "Result: {}",
        packets.iter().map(|p| p.get_version_sum()).sum::<u32>()
    );
    Ok(())
}
pub fn part2(input: &str) -> Result<()> {
    let mut bytes = parse_input(input);
    let (mut byte_offset, mut bit_offset) = (0, 0);
    let mut packets = Vec::new();
    while byte_offset +3 < bytes.len() {
        let packet = Packet::new(&mut bytes, &mut byte_offset, &mut bit_offset);
        packets.push(packet);
    }
    println!("Result: {:?}", packets[0].calculate());
    Ok(())
}
