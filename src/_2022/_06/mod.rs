use std::cmp::PartialEq;
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

pub fn part1(input: String) -> String {
    decode(4, input)
        .map(|code| code.to_string())
        .unwrap_or("Unknown code".to_string())
}

pub fn part2(input: String) -> String {
    decode(14, input)
        .map(|code| code.to_string())
        .unwrap_or("Unknown code".to_string())
}

fn decode(packet_size: usize, input: String) -> Option<usize> {
    let mut buffer = Buffer {
        capacity: packet_size,
        elements: VecDeque::new(),
    };

    for (index, char) in input.chars().enumerate() {
        buffer.push(char);

        if buffer.is_heterogeneous() {
            return Some(index + 1);
        }
    }

    return None;
}

struct Buffer<T> {
    capacity: usize,
    elements: VecDeque<T>,
}

impl<T: PartialEq + Hash + Eq> Buffer<T> {
    fn push(&mut self, element: T) {
        self.elements.push_back(element);

        if self.elements.len() > self.capacity {
            self.elements.pop_front();
        }
    }

    fn is_heterogeneous(&self) -> bool {
        if self.capacity == self.elements.len() {
            let mut element_map = HashMap::new();

            return self.elements.iter().all(|element| {
                if element_map.contains_key(element) {
                    return false;
                }

                element_map.insert(element, true);
                return true;
            });
        }

        false
    }
}
