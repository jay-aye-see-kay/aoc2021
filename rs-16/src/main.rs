#![allow(unused)]
use std::collections::HashMap;
use std::fs;

use crate::binary::Binary;

mod binary;

// shorthand for creating a hashmap like vec![]
macro_rules! map {
    ($( $t: expr),*) => {{
         let mut map = HashMap::new();
         $( map.insert($t.0, $t.1); )*
         map
    }}
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    println!("part 1: {}", count_version_numbers(&input.trim()));
}

fn str_to_binary(input: &str) -> String {
    let hex_to_bin = map![
        ('0', "0000"),
        ('1', "0001"),
        ('2', "0010"),
        ('3', "0011"),
        ('4', "0100"),
        ('5', "0101"),
        ('6', "0110"),
        ('7', "0111"),
        ('8', "1000"),
        ('9', "1001"),
        ('A', "1010"),
        ('B', "1011"),
        ('C', "1100"),
        ('D', "1101"),
        ('E', "1110"),
        ('F', "1111")
    ];
    input.chars().map(|c| hex_to_bin[&c]).collect()
}

#[derive(Debug, PartialEq)]
struct LiteralPacket {
    packet_version: Binary,
    value: Binary,
}

#[derive(Debug, PartialEq)]
struct OperatorPacket {
    packet_version: Binary,
    packet_type: Binary,
    packets: Vec<Packet>,
}

#[derive(Debug, PartialEq)]
enum Packet {
    Literal(LiteralPacket),
    Operator(OperatorPacket),
}

#[derive(Debug, PartialEq)]
enum ExitCondition {
    BitLength(usize),
    PacketCount(usize),
}

fn split_bits(binary_str: &str) -> (Packet, usize) {
    let packet_version = &binary_str[..3];
    let packet_type = &binary_str[3..6];

    // {{{ handle literal type
    if packet_type == "100" {
        let increment: usize = 5;
        let mut index: usize = 6;
        let mut digits: Vec<Binary> = vec![];
        let mut digit_str = "".to_string();
        loop {
            let should_continue = &binary_str[index..index + 1] == "1";
            let literal_str = &binary_str[index + 1..index + increment];
            digit_str.extend(literal_str.chars());

            index += increment;
            if !should_continue {
                break;
            }
        }
        let packet = Packet::Literal(LiteralPacket {
            packet_version: packet_version.parse().unwrap(),
            value: digit_str.parse().unwrap(),
        });
        return (packet, index);
    }
    // }}} past here it must be operator type

    let exit_cond_str_start = 7;
    let exit_cond_str_end;
    let length_type_id = &binary_str[6..7];
    let exit_condition = match length_type_id == "0" {
        true => {
            exit_cond_str_end = 22;
            let str_range = exit_cond_str_start..exit_cond_str_end;
            let length = binary_str[str_range]
                .parse::<Binary>()
                .unwrap()
                .to_decimal() as usize;
            ExitCondition::BitLength(length)
        }
        false => {
            exit_cond_str_end = 18;
            let str_range = exit_cond_str_start..exit_cond_str_end;
            let length = binary_str[str_range]
                .parse::<Binary>()
                .unwrap()
                .to_decimal() as usize;
            ExitCondition::PacketCount(length)
        }
    };

    let mut packets_consumed = 0;
    let mut length_consumed = 0;

    let mut index = exit_cond_str_end;
    let mut packets = vec![];

    while !exit_condition_satisfied(&exit_condition, length_consumed, packets_consumed) {
        let (sub_packet, consumed) = split_bits(&binary_str[index..]);
        packets.push(sub_packet);
        index += consumed;
        length_consumed += consumed;
        packets_consumed += 1;
    }

    let packet = Packet::Operator(OperatorPacket {
        packet_version: packet_version.parse().unwrap(),
        packet_type: packet_type.parse().unwrap(),
        packets,
    });

    return (packet, index);
}

fn exit_condition_satisfied(
    condition: &ExitCondition,
    length_consumed: usize,
    packets_consumed: usize,
) -> bool {
    match condition {
        ExitCondition::BitLength(exit_count) => length_consumed >= *exit_count,
        ExitCondition::PacketCount(exit_count) => packets_consumed >= *exit_count,
    }
}

fn sum_packet_version_numbers(packet: Packet) -> i32 {
    match packet {
        Packet::Literal(packet) => packet.packet_version.to_decimal(),
        Packet::Operator(packet) => {
            let mut sum = packet.packet_version.to_decimal();
            for sub_packet in packet.packets {
                sum += sum_packet_version_numbers(sub_packet);
            }
            sum
        }
    }
}

fn count_version_numbers(input: &str) -> i32 {
    let binary_str = str_to_binary(input);
    let (packet, _) = split_bits(&binary_str);
    sum_packet_version_numbers(packet)
}

#[cfg(test)]
mod tests {
    use std::{fs, str::FromStr};

    use super::*;

    #[test]
    fn test_str_to_binary() {
        assert_eq!(str_to_binary("1"), "0001");
        assert_eq!(str_to_binary("F"), "1111");
        assert_eq!(str_to_binary("D2FE28"), "110100101111111000101000");
    }

    #[test]
    fn test_split_bits_literal() {
        assert_eq!(
            split_bits("110100101111111000101000"),
            (
                Packet::Literal(LiteralPacket {
                    packet_version: "110".parse().unwrap(),
                    value: "011111100101".parse().unwrap(),
                }),
                21
            )
        );
    }

    #[test]
    fn test_exit_condition_satified() {
        assert_eq!(
            exit_condition_satisfied(&ExitCondition::BitLength(10), 5, 0),
            false
        );
        assert_eq!(
            exit_condition_satisfied(&ExitCondition::BitLength(10), 10, 0),
            true
        );
        assert_eq!(
            exit_condition_satisfied(&ExitCondition::PacketCount(10), 0, 5),
            false
        );
        assert_eq!(
            exit_condition_satisfied(&ExitCondition::PacketCount(10), 0, 10),
            true
        );
    }

    #[test]
    fn test_split_bits_two_sub_packets() {
        assert_eq!(
            split_bits("00111000000000000110111101000101001010010001001000000000").0,
            Packet::Operator(OperatorPacket {
                packet_version: "001".parse().unwrap(),
                packet_type: "110".parse().unwrap(),
                packets: vec![
                    Packet::Literal(LiteralPacket {
                        // 110 100 01010
                        packet_version: "110".parse().unwrap(),
                        value: "1010".parse().unwrap(),
                    }),
                    Packet::Literal(LiteralPacket {
                        // 010 100 10001 00100
                        packet_version: "010".parse().unwrap(),
                        value: "00010100".parse().unwrap(),
                    }),
                ]
            })
        );
    }

    #[test]
    fn test_split_bits_three_sub_packets() {
        let (packet, consusumed) =
            split_bits("11101110000000001101010000001100100000100011000001100000");

        assert_eq!(consusumed, 51);

        match packet {
            Packet::Literal(_) => assert!(false),
            Packet::Operator(packet) => {
                assert_eq!(packet.packet_version, "111".parse().unwrap());
                assert_eq!(packet.packet_type, "011".parse().unwrap());
                assert_eq!(packet.packets.len(), 3);
                assert_eq!(
                    packet.packets[0],
                    Packet::Literal(LiteralPacket {
                        // 010 100 00001
                        packet_version: "010".parse().unwrap(),
                        value: "0001".parse().unwrap(),
                    }),
                );
                assert_eq!(
                    packet.packets[1],
                    Packet::Literal(LiteralPacket {
                        // 100 100 00010
                        packet_version: "100".parse().unwrap(),
                        value: "0010".parse().unwrap(),
                    }),
                );
                assert_eq!(
                    packet.packets[2],
                    Packet::Literal(LiteralPacket {
                        // 001 100 00011
                        packet_version: "001".parse().unwrap(),
                        value: "0011".parse().unwrap(),
                    }),
                );
            }
        }
    }

    #[test]
    fn test_part_1_sample() {
        assert_eq!(count_version_numbers("8A004A801A8002F478"), 16);
        assert_eq!(count_version_numbers("620080001611562C8802118E34"), 12);
        assert_eq!(count_version_numbers("C0015000016115A2E0802F182340"), 23);
        assert_eq!(count_version_numbers("A0016C880162017C3686B18A3D4780"), 31);
    }

    #[test]
    fn test_part_1_real() {
        let input = fs::read_to_string("input").unwrap();
        assert_eq!(count_version_numbers(&input.trim()), 854);
    }
}
