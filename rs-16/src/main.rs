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
    let (packet, _) = parse_to_packets(&str_to_binary(&input.trim()));
    println!("part 2: {}", compute_packet(&packet));
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
    packet_version: i64,
    value: i64,
}

#[derive(Debug, PartialEq)]
struct OperatorPacket {
    packet_version: i64,
    packet_type: i64,
    packets: Vec<Packet>,
}

#[derive(Debug, PartialEq)]
enum Packet {
    Literal(LiteralPacket),
    Operator(OperatorPacket),
}

#[derive(Debug, PartialEq)]
enum ExitCondition {
    BitLength(i64),
    PacketCount(i64),
}

fn binary_str_to_int(binary_str: &str) -> i64 {
    println!("binary_str: {:?}", binary_str);
    binary_str.parse::<Binary>().unwrap().to_decimal() as i64
}

fn parse_to_packets(binary_str: &str) -> (Packet, i64) {
    let packet_version = &binary_str[..3];
    let packet_type = &binary_str[3..6];

    // {{{ handle literal type
    if packet_type == "100" {
        let increment: usize = 5;
        let mut index: usize = 6;
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
            packet_version: binary_str_to_int(packet_version),
            value: binary_str_to_int(&digit_str),
        });
        return (packet, index as i64);
    }
    // }}} past here it must be operator type

    let exit_cond_str_start = 7;
    let exit_cond_str_end;
    let length_type_id = &binary_str[6..7];
    let exit_condition = match length_type_id == "0" {
        true => {
            exit_cond_str_end = 22;
            let str_range = exit_cond_str_start..exit_cond_str_end;
            let length = binary_str_to_int(&binary_str[str_range]);
            ExitCondition::BitLength(length)
        }
        false => {
            exit_cond_str_end = 18;
            let str_range = exit_cond_str_start..exit_cond_str_end;
            let length = binary_str_to_int(&binary_str[str_range]);
            ExitCondition::PacketCount(length)
        }
    };

    let mut packets_consumed = 0;
    let mut length_consumed = 0;

    let mut index = exit_cond_str_end;
    let mut packets = vec![];

    while !exit_condition_satisfied(&exit_condition, length_consumed, packets_consumed) {
        let (sub_packet, consumed) = parse_to_packets(&binary_str[index..]);
        packets.push(sub_packet);
        index += consumed as usize;
        length_consumed += consumed;
        packets_consumed += 1;
    }

    let packet = Packet::Operator(OperatorPacket {
        packet_version: binary_str_to_int(packet_version),
        packet_type: binary_str_to_int(packet_type),
        packets,
    });

    return (packet, index as i64);
}

fn exit_condition_satisfied(
    condition: &ExitCondition,
    length_consumed: i64,
    packets_consumed: i64,
) -> bool {
    match condition {
        ExitCondition::BitLength(exit_count) => length_consumed >= *exit_count,
        ExitCondition::PacketCount(exit_count) => packets_consumed >= *exit_count,
    }
}

fn sum_packet_version_numbers(packet: Packet) -> i64 {
    match packet {
        Packet::Literal(packet) => packet.packet_version,
        Packet::Operator(packet) => {
            let mut sum = packet.packet_version;
            for sub_packet in packet.packets {
                sum += sum_packet_version_numbers(sub_packet);
            }
            sum
        }
    }
}

fn count_version_numbers(input: &str) -> i64 {
    let binary_str = str_to_binary(input);
    let (packet, _) = parse_to_packets(&binary_str);
    sum_packet_version_numbers(packet)
}

fn compute_packet(packet: &Packet) -> i64 {
    match packet {
        Packet::Literal(packet) => packet.value,
        Packet::Operator(packet) => match packet.packet_type {
            0 => packet.packets.iter().map(|p| compute_packet(p)).sum(),
            1 => packet.packets.iter().map(|p| compute_packet(p)).product(),
            2 => packet
                .packets
                .iter()
                .map(|p| compute_packet(p))
                .min()
                .unwrap(),
            3 => packet
                .packets
                .iter()
                .map(|p| compute_packet(p))
                .max()
                .unwrap(),
            5 => {
                let computed: Vec<_> = packet.packets.iter().map(|p| compute_packet(p)).collect();
                if computed.len() != 2 {
                    panic!("more than packet requires exactly two sub packets")
                }
                (computed[0] > computed[1]) as i64
            }
            6 => {
                let computed: Vec<_> = packet.packets.iter().map(|p| compute_packet(p)).collect();
                if computed.len() != 2 {
                    panic!("less than packet requires exactly two sub packets")
                }
                (computed[0] < computed[1]) as i64
            }
            7 => {
                let computed: Vec<_> = packet.packets.iter().map(|p| compute_packet(p)).collect();
                if computed.len() != 2 {
                    panic!("less than packet requires exactly two sub packets")
                }
                (computed[0] == computed[1]) as i64
            }
            _ => panic!("unknown packet type: {}", packet.packet_type),
        },
    }
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
    fn test_parse_to_packets_literal() {
        assert_eq!(
            parse_to_packets("110100101111111000101000"),
            (
                Packet::Literal(LiteralPacket {
                    packet_version: 6,
                    value: 2021,
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
    fn test_parse_to_packets_two_sub_packets() {
        assert_eq!(
            parse_to_packets("00111000000000000110111101000101001010010001001000000000").0,
            Packet::Operator(OperatorPacket {
                packet_version: 1,
                packet_type: 6,
                packets: vec![
                    Packet::Literal(LiteralPacket {
                        // 110 100 01010
                        packet_version: 6,
                        value: 10,
                    }),
                    Packet::Literal(LiteralPacket {
                        // 010 100 10001 00100
                        packet_version: 2,
                        value: 20,
                    }),
                ]
            })
        );
    }

    #[test]
    fn test_parse_to_packets_three_sub_packets() {
        let (packet, consusumed) =
            parse_to_packets("11101110000000001101010000001100100000100011000001100000");

        assert_eq!(consusumed, 51);

        match packet {
            Packet::Literal(_) => assert!(false),
            Packet::Operator(packet) => {
                assert_eq!(packet.packet_version, 7);
                assert_eq!(packet.packet_type, 3);
                assert_eq!(packet.packets.len(), 3);
                assert_eq!(
                    packet.packets[0],
                    Packet::Literal(LiteralPacket {
                        // 010 100 00001
                        packet_version: 2,
                        value: 1,
                    }),
                );
                assert_eq!(
                    packet.packets[1],
                    Packet::Literal(LiteralPacket {
                        // 100 100 00010
                        packet_version: 4,
                        value: 2,
                    }),
                );
                assert_eq!(
                    packet.packets[2],
                    Packet::Literal(LiteralPacket {
                        // 001 100 00011
                        packet_version: 1,
                        value: 3,
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

    #[test]
    fn test_compute_sum() {
        let (packet, _) = parse_to_packets(&str_to_binary("C200B40A82"));
        assert_eq!(compute_packet(&packet), 3);
    }

    #[test]
    fn test_compute_product() {
        let (packet, _) = parse_to_packets(&str_to_binary("04005AC33890"));
        assert_eq!(compute_packet(&packet), 54);
    }

    #[test]
    fn test_compute_min() {
        let (packet, _) = parse_to_packets(&str_to_binary("880086C3E88112"));
        assert_eq!(compute_packet(&packet), 7);
    }

    #[test]
    fn test_compute_max() {
        let (packet, _) = parse_to_packets(&str_to_binary("CE00C43D881120"));
        assert_eq!(compute_packet(&packet), 9);
    }

    #[test]
    fn test_compute_less_than() {
        let (packet, _) = parse_to_packets(&str_to_binary("D8005AC2A8F0"));
        assert_eq!(compute_packet(&packet), 1);
    }

    #[test]
    fn test_compute_more_than() {
        let (packet, _) = parse_to_packets(&str_to_binary("F600BC2D8F"));
        assert_eq!(compute_packet(&packet), 0);
    }

    #[test]
    fn test_compute_equal() {
        let (packet, _) = parse_to_packets(&str_to_binary("9C005AC2F8F0"));
        assert_eq!(compute_packet(&packet), 0);
        let (packet, _) = parse_to_packets(&str_to_binary("9C0141080250320F1802104A08"));
        assert_eq!(compute_packet(&packet), 1);
    }

    #[test]
    fn test_part_2_real() {
        let input = fs::read_to_string("input").unwrap();
        let (packet, _) = parse_to_packets(&str_to_binary(&input.trim()));
        assert_eq!(compute_packet(&packet), 186189840660);
    }
}
