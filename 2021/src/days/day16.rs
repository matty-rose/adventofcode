use crate::registry::{DayCommand, Function};
use crate::utils;

#[derive(Debug)]
struct Packet {
    version: isize,
    type_id: isize,
    literal: Option<isize>,
    subpackets: Option<Vec<Packet>>,
}

impl Packet {
    fn version_sum(&self) -> usize {
        match &self.subpackets {
            None => self.version as usize,
            Some(subpackets) => {
                subpackets.iter().map(|sp| sp.version_sum()).sum::<usize>() + self.version as usize
            }
        }
    }

    fn value(&self) -> isize {
        if self.type_id == 4 {
            return self.literal.unwrap();
        }

        println!("{:?}", self.subpackets);
        let mut subpacket_values = self
            .subpackets
            .as_ref()
            .unwrap()
            .iter()
            .map(|sp| sp.value());
        match self.type_id {
            0 => subpacket_values.sum(),
            1 => subpacket_values.product(),
            2 => subpacket_values.min().unwrap(),
            3 => subpacket_values.max().unwrap(),
            5 => {
                let first = subpacket_values.next();
                let second = subpacket_values.next();
                if first > second {
                    1
                } else {
                    0
                }
            }
            6 => {
                let first = subpacket_values.next();
                let second = subpacket_values.next();
                if first < second {
                    1
                } else {
                    0
                }
            }
            7 => {
                let first = subpacket_values.next();
                let second = subpacket_values.next();
                if first == second {
                    1
                } else {
                    0
                }
            }
            _ => unreachable!(),
        }
    }
}

type RemainingBits = Vec<char>;

fn hex_to_str(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => unreachable!(),
    }
}

fn binchars(s: &[char]) -> isize {
    isize::from_str_radix(&s.iter().collect::<String>(), 2).unwrap()
}

fn decode_transmission(transmission: &str) -> String {
    transmission.chars().map(hex_to_str).collect()
}

fn decode_packet(packet: &str) -> (Packet, RemainingBits) {
    let chars: Vec<char> = packet.chars().collect();
    let mut idx = 0;

    let version_str: String = chars[idx..idx + 3].iter().collect();
    idx += 3;
    let type_id_str: String = chars[idx..idx + 3].iter().collect();
    idx += 3;
    let version = isize::from_str_radix(&version_str, 2).unwrap();
    let type_id = isize::from_str_radix(&type_id_str, 2).unwrap();

    let (literal_num, subpackets) = match type_id {
        4 => {
            let mut is_last = false;
            let mut num_chars = Vec::new();
            while !is_last {
                let bits = &chars[idx..idx + 5];
                if bits[0] == '0' {
                    is_last = true
                }
                num_chars.append(&mut bits[1..].to_vec());
                idx += 5
            }
            (Some(binchars(&num_chars)), None)
        }
        _ => {
            let mut subpackets: Vec<Packet> = Vec::new();
            match chars[idx] {
                '0' => {
                    idx += 1;
                    let mut length = binchars(&chars[idx..idx + 15]);
                    idx += 15;
                    let bits = &chars[idx..];
                    let mut prev_len = chars[idx..].len();
                    loop {
                        if length <= 0 || idx > bits.len() {
                            break;
                        }
                        let remaining_bits: String = chars[idx..].iter().collect();
                        println!("length {}, remaining_bits: {:?}", length, remaining_bits);
                        let (subpacket, tmp_bits) = decode_packet(&remaining_bits);
                        let diff = prev_len - tmp_bits.len();
                        length -= diff as isize;
                        idx += diff;
                        prev_len = tmp_bits.len();
                        subpackets.push(subpacket);
                    }
                }
                '1' => {
                    idx += 1;
                    let mut num_packets = binchars(&chars[idx..idx + 11]);
                    idx += 11;
                    let mut prev_len = chars[idx..].len();
                    while num_packets != 0 {
                        if num_packets == 0 {
                            break;
                        }
                        let remaining_bits: String = chars[idx..].iter().collect();
                        let (subpacket, tmp_bits) = decode_packet(&remaining_bits);
                        num_packets -= 1;
                        idx += prev_len - tmp_bits.len();
                        prev_len = tmp_bits.len();
                        subpackets.push(subpacket);
                    }
                }
                _ => unreachable!(),
            }
            (None, Some(subpackets))
        }
    };

    let final_remaining = if idx > chars.len() {
        Vec::new()
    } else {
        chars[idx..].to_vec()
    };
    (
        Packet {
            version,
            type_id,
            literal: literal_num,
            subpackets,
        },
        final_remaining,
    )
}

fn part1(transmission: &str) {
    let (packet, _) = decode_packet(transmission);
    println!("packet: {:?}", packet);
    let version_sum = packet.version_sum();
    println!("sum of versions: {:?}", version_sum);
}

fn part2(transmission: &str) {
    let (packet, _) = decode_packet(transmission);
    println!("packet value: {:?}", packet.value());
}

fn main(part: &str, file: Option<&str>) -> Option<()> {
    let filename = file.expect("expected a file for this problem");
    let lines: Vec<String> = utils::read_lines(filename).expect("could not load lines");
    let transmission = decode_transmission(&lines[0]);
    match part {
        "1" => part1(&transmission),
        "2" => part2(&transmission),
        _ => (),
    }
    None
}

inventory::submit! {
    DayCommand::new("day16", Function{func: main})
}
