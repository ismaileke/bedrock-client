use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct BitSet {
    length: usize,
    parts: Vec<u64>,
}

impl BitSet {
    const INT_BITS: usize = size_of::<u64>() * 8;
    const SHIFT: usize = 7;

    pub fn new(length: usize, mut parts: Vec<u64>) -> Self {
        let expected_parts = Self::get_expected_parts_count(length);
        if parts.len() > expected_parts {
            panic!("Too many parts");
        } else if parts.len() < expected_parts {
            parts.resize(expected_parts, 0);
        }
        Self { length, parts }
    }

    pub fn get(&self, index: usize) -> bool {
        let (part_index, bit_index) = self.get_part_index(index);
        (self.parts[part_index] & (1 << bit_index)) != 0
    }

    pub fn set(&mut self, index: usize, value: bool) {
        let (part_index, bit_index) = self.get_part_index(index);
        if value {
            self.parts[part_index] |= 1 << bit_index;
        } else {
            self.parts[part_index] &= !(1 << bit_index);
        }
    }

    fn get_part_index(&self, index: usize) -> (usize, usize) {
        if index >= self.length {
            panic!("Index out of bounds");
        }
        (index / Self::INT_BITS, index % Self::INT_BITS)
    }

    pub fn get_parts_count(&self) -> usize {
        self.parts.len()
    }

    fn get_expected_parts_count(length: usize) -> usize {
        (length + Self::INT_BITS - 1) / Self::INT_BITS
    }

    pub fn read(stream: &mut Stream, length: usize) -> Self {
        let mut result: Vec<u64> = vec![0];
        let mut current_index = 0;
        let mut current_shift = 0;

        let mut i = 0;
        while i < length {
            let b = stream.get_byte() as u64;
            let bits = b & 0x7f;

            result[current_index] |= bits << current_shift;
            let mut next_shift = current_shift + Self::SHIFT;

            if next_shift >= Self::INT_BITS {
                next_shift -= Self::INT_BITS;
                let right_shift = Self::SHIFT - next_shift;
                if result.len() <= current_index + 1 {
                    result.push(0);
                }
                result[current_index + 1] = bits >> right_shift;
                current_index += 1;
            }

            current_shift = next_shift;

            if (b & 0x80) == 0 {
                return Self::new(
                    length,
                    result[..Self::get_expected_parts_count(length)].to_vec(),
                );
            }
            i += Self::SHIFT;
        }

        Self::new(
            length,
            result[..Self::get_expected_parts_count(length)].to_vec(),
        )
    }

    pub fn write(&self, stream: &mut Stream) {
        let mut current_index = 0;
        let mut current_shift = 0;

        let mut i = 0;
        while i < self.length {
            let mut bits = self.parts[current_index] >> current_shift;
            let mut next_shift = current_shift + Self::SHIFT;

            if next_shift >= Self::INT_BITS {
                next_shift -= Self::INT_BITS;
                let extra = (self.parts.get(current_index + 1).copied().unwrap_or(0))
                    << (Self::SHIFT - next_shift);
                bits |= extra;
                current_index += 1;
            }

            current_shift = next_shift;

            let last = i + Self::SHIFT >= self.length;
            if !last {
                bits |= 0x80;
            }

            stream.put_byte(bits as u8);
            if last {
                break;
            }

            i += Self::SHIFT;
        }
    }

    pub fn get_length(&self) -> usize {
        self.length
    }

    pub fn equals(&self, other: &BitSet) -> bool {
        self.length == other.length && self.parts == other.parts
    }
}
