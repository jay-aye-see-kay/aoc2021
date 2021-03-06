use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}

fn part_1(input: &str) -> i64 {
    let binary_str = str_to_binary(input.trim());
    let (packet, _) = parse_to_packets(&binary_str);
    sum_version_numbers(&packet)
}

fn part_2(input: &str) -> i64 {
    let binary_str = str_to_binary(input.trim());
    let (packet, _) = parse_to_packets(&binary_str);
    compute_packet(&packet)
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
    BitLength(usize),
    PacketCount(usize),
}

impl ExitCondition {
    fn satisfied(&self, length_consumed: usize, packets_consumed: usize) -> bool {
        match self {
            ExitCondition::BitLength(exit_value) => length_consumed >= *exit_value,
            ExitCondition::PacketCount(exit_value) => packets_consumed >= *exit_value,
        }
    }
}

/// interprets each character as a hex value and converts that to a 4 wide binary string
fn str_to_binary(input: &str) -> String {
    input
        .chars()
        .map(|c| c.to_digit(16).unwrap())
        .map(|d| format!("{:04b}", d))
        .collect()
}

/// takes a binary value stored as a string and converts in to a decimal
fn binary_str_to_int(binary_str: &str) -> i64 {
    let bits: Vec<_> = binary_str
        .chars()
        .map(|c| match c {
            '0' => false,
            '1' => true,
            _ => panic!("Invalid character"),
        })
        .collect();
    let mut result = 0;
    for (i, bit) in bits.iter().rev().enumerate() {
        if *bit {
            result += 1 << i;
        }
    }
    result
}

fn parse_to_packets(binary_str: &str) -> (Packet, usize) {
    let packet_version = binary_str_to_int(&binary_str[..3]);
    let packet_type = binary_str_to_int(&binary_str[3..6]);

    // special case: literal packet type
    if packet_type == 4 {
        let increment: usize = 5;
        let mut index: usize = 6;
        let mut digit_str = "".to_string();
        loop {
            let should_continue = &binary_str[index..index + 1] == "1";
            let literal_str = &binary_str[index + 1..index + increment];
            digit_str.push_str(literal_str);

            index += increment;
            if !should_continue {
                break;
            }
        }
        let packet = Packet::Literal(LiteralPacket {
            packet_version,
            value: binary_str_to_int(&digit_str),
        });
        return (packet, index);
    }

    let exit_cond_str_end;
    let length_type_id = &binary_str[6..7];
    let exit_condition = match length_type_id {
        "0" => {
            exit_cond_str_end = 22;
            let str_range = 7..exit_cond_str_end;
            let exit_value = binary_str_to_int(&binary_str[str_range]);
            ExitCondition::BitLength(exit_value as usize)
        }
        "1" => {
            exit_cond_str_end = 18;
            let str_range = 7..exit_cond_str_end;
            let exit_value = binary_str_to_int(&binary_str[str_range]);
            ExitCondition::PacketCount(exit_value as usize)
        }
        _ => panic!(),
    };

    let mut packets_consumed = 0;
    let mut index = exit_cond_str_end;
    let mut packets = vec![];
    while !exit_condition.satisfied(index - exit_cond_str_end, packets_consumed) {
        let (sub_packet, consumed) = parse_to_packets(&binary_str[index..]);
        packets.push(sub_packet);
        index += consumed;
        packets_consumed += 1;
    }

    let packet = Packet::Operator(OperatorPacket {
        packet_version,
        packet_type,
        packets,
    });

    (packet, index)
}

fn sum_version_numbers(packet: &Packet) -> i64 {
    match packet {
        Packet::Literal(packet) => packet.packet_version,
        Packet::Operator(packet) => {
            let sum: i64 = packet.packets.iter().map(sum_version_numbers).sum();
            sum + packet.packet_version
        }
    }
}

fn compute_packet(packet: &Packet) -> i64 {
    match packet {
        Packet::Literal(packet) => packet.value,
        Packet::Operator(packet) => {
            let sub_packet_iter = packet.packets.iter().map(compute_packet);
            let sub_packets: Vec<_> = sub_packet_iter.clone().collect();
            match packet.packet_type {
                0 => sub_packet_iter.sum(),
                1 => sub_packet_iter.product(),
                2 => sub_packet_iter.min().unwrap(),
                3 => sub_packet_iter.max().unwrap(),
                5 => (sub_packets[0] > sub_packets[1]) as i64,
                6 => (sub_packets[0] < sub_packets[1]) as i64,
                7 => (sub_packets[0] == sub_packets[1]) as i64,
                _ => panic!("unknown packet type: {}", packet.packet_type),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_str_to_binary() {
        assert_eq!(str_to_binary("1"), "0001");
        assert_eq!(str_to_binary("F"), "1111");
        assert_eq!(str_to_binary("D2FE28"), "110100101111111000101000");
    }

    #[test]
    fn test_parse_to_packets_literal() {
        let (packet, consumed) = parse_to_packets("110100101111111000101000");
        assert_eq!(consumed, 21);
        assert_eq!(
            packet,
            Packet::Literal(LiteralPacket {
                packet_version: 6,
                value: 2021,
            })
        );
    }

    #[test]
    fn test_exit_condition_satified() {
        assert_eq!(ExitCondition::BitLength(10).satisfied(5, 0), false);
        assert_eq!(ExitCondition::BitLength(10).satisfied(10, 0), true);
        assert_eq!(ExitCondition::PacketCount(10).satisfied(0, 5), false);
        assert_eq!(ExitCondition::PacketCount(10).satisfied(0, 10), true);
    }

    #[test]
    fn test_parse_to_packets_two_sub_packets() {
        let (packet, consusumed) =
            parse_to_packets("00111000000000000110111101000101001010010001001000000000");

        assert_eq!(consusumed, 49);

        match packet {
            Packet::Literal(_) => assert!(false),
            Packet::Operator(packet) => {
                assert_eq!(packet.packet_version, 1);
                assert_eq!(packet.packet_type, 6);
                assert_eq!(packet.packets.len(), 2);
                assert_eq!(
                    packet.packets[0],
                    Packet::Literal(LiteralPacket {
                        packet_version: 6,
                        value: 10,
                    }),
                );
                assert_eq!(
                    packet.packets[1],
                    Packet::Literal(LiteralPacket {
                        packet_version: 2,
                        value: 20,
                    }),
                );
            }
        };
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
                        packet_version: 2,
                        value: 1,
                    }),
                );
                assert_eq!(
                    packet.packets[1],
                    Packet::Literal(LiteralPacket {
                        packet_version: 4,
                        value: 2,
                    }),
                );
                assert_eq!(
                    packet.packets[2],
                    Packet::Literal(LiteralPacket {
                        packet_version: 1,
                        value: 3,
                    }),
                );
            }
        }
    }

    #[test]
    fn test_part_1_sample() {
        assert_eq!(part_1("8A004A801A8002F478"), 16);
        assert_eq!(part_1("620080001611562C8802118E34"), 12);
        assert_eq!(part_1("C0015000016115A2E0802F182340"), 23);
        assert_eq!(part_1("A0016C880162017C3686B18A3D4780"), 31);
    }

    #[test]
    fn test_part_1_real() {
        let input = fs::read_to_string("input").unwrap();
        assert_eq!(part_1(&input), 854);
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
        assert_eq!(part_2(&input), 186189840660);
    }
}
