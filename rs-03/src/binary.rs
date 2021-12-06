use std::fmt::{Display, Error, Formatter};
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct Binary {
    pub bits: Vec<bool>,
}

impl Binary {
    pub fn value_at(&self, index: usize) -> usize {
        self.bits[index] as usize
    }

    pub fn to_decimal(&self) -> i32 {
        let mut result = 0;
        for (i, bit) in self.bits.iter().rev().enumerate() {
            if *bit {
                result += 1 << i;
            }
        }
        result
    }
}

impl FromStr for Binary {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            bits: input
                .chars()
                .map(|c| match c {
                    '0' => false,
                    '1' => true,
                    _ => panic!("Invalid character"),
                })
                .collect(),
        })
    }
}

impl Display for Binary {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut result = String::with_capacity(self.bits.len());
        for bit in self.bits.iter() {
            result.push_str(if *bit { "1" } else { "0" });
        }
        write!(f, "{}", result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let binary = Binary::from_str("11111111").unwrap();
        assert_eq!(binary.to_decimal(), 255);
    }

    #[test]
    fn test_binary_value_at() {
        let binary: Binary = "001011".parse().unwrap();
        assert_eq!(binary.value_at(0), 0);
        assert_eq!(binary.value_at(1), 0);
        assert_eq!(binary.value_at(2), 1);
        assert_eq!(binary.value_at(3), 0);
        assert_eq!(binary.value_at(4), 1);
        assert_eq!(binary.value_at(5), 1);
    }

    #[test]
    fn test_binary_to_decimal() {
        let binary: Binary = "001011".parse().unwrap();
        assert_eq!(binary.to_decimal(), 11);
    }

    #[test]
    fn test_binary_to_string() {
        let binary: Binary = "001111".parse().unwrap();
        assert_eq!(format!("{}", binary), "001111");
    }
}
