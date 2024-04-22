use std::{char, collections::VecDeque};

pub fn roman_to_int(s: String) -> i32 {
    let mut int = 0;
    let mut chars = s.chars().collect::<VecDeque<char>>();
    while !chars.is_empty() {
        match chars[0] {
            'I' => {
                if chars.len() > 1 {
                    match chars[1] {
                        'V' => {
                            int += 4;
                            chars.pop_front();
                            chars.pop_front();
                            continue;
                        }
                        'X' => {
                            int += 9;
                            chars.pop_front();
                            chars.pop_front();
                            continue;
                        }
                        _ => {}
                    }
                }
                chars.remove(0);
                int += 1;
                continue;
            }
            'V' => {
                chars.remove(0);
                int += 5;
                continue;
            }
            'X' => {
                if chars.len() > 1 {
                    match chars[1] {
                        'L' => {
                            int += 40;
                            chars.pop_front();
                            chars.pop_front();
                            continue;
                        }
                        'C' => {
                            int += 90;
                            chars.pop_front();
                            chars.pop_front();
                            continue;
                        }
                        _ => {}
                    }
                }
                chars.remove(0);
                int += 10;
                continue;
            }
            'L' => {
                chars.remove(0);
                int += 50;
                continue;
            }
            'C' => {
                if chars.len() > 1 {
                    match chars[1] {
                        'D' => {
                            int += 400;
                            chars.pop_front();
                            chars.pop_front();
                            continue;
                        }
                        'M' => {
                            int += 900;
                            chars.pop_front();
                            chars.pop_front();
                            continue;
                        }
                        _ => {}
                    }
                }
                chars.remove(0);
                int += 100;
                continue;
            }
            'D' => {
                chars.remove(0);
                int += 500;
                continue;
            }
            'M' => {
                chars.remove(0);
                int += 1000;
                continue;
            }
            _ => {}
        }
    }
    int
}

mod test {
    use crate::algos::leetcode::roman_to_int;
    #[test]
    fn roman_to_int_1() {
        assert_eq!(roman_to_int("I".to_string()), 1);
        assert_eq!(roman_to_int("XII".to_string()), 12);
        assert_eq!(roman_to_int("XXVII".to_string()), 27);
        assert_eq!(roman_to_int("IV".to_string()), 4);
        assert_eq!(roman_to_int("IX".to_string()), 9);
        assert_eq!(roman_to_int("XL".to_string()), 40);
        assert_eq!(roman_to_int("XC".to_string()), 90);
        assert_eq!(roman_to_int("CD".to_string()), 400);
        assert_eq!(roman_to_int("CM".to_string()), 900);
        assert_eq!(roman_to_int("MCMXCIV".to_string()), 1994);
    }
}
