use std::{ops::Shl, u8};

#[derive(Default, Clone, Debug)]
pub struct TightVec {
    inner: Vec<u8>,
    len: usize,
}

impl TightVec {
    pub fn index(&self, index: usize) -> bool {
        let index_inner = index / 8;
        let index_remainder = index % 8;

        let mask = 1_u8.shl(index_remainder);

        self.inner[index_inner] & mask == mask
    }

    pub fn set(&mut self, index: usize, value: bool) {
        let index_inner = index / 8;
        let index_remainder = index % 8;

        let mask = 1_u8.shl(index_remainder);

        if !value {
            self.inner[index_inner] ^= mask;
        } else {
            self.inner[index_inner] |= mask;
        }
    }

    pub fn push(&mut self, b: bool) {
        self.len += 1;

        if self.len == 0 || (self.len / 8) >= self.inner.len() {
            self.inner.push(0);
        }

        let arr_index = (self.len - 1) / 8;

        let inner_index = (self.len - 1) % 8;

        let new_value = (b as u8).shl(inner_index);
        let current_value = self.inner[arr_index];

        let bit_value = current_value | new_value;

        self.inner[arr_index] = bit_value;
    }

    pub fn with_len_and_value(number: usize, value: bool) -> Self {
        let mut v = Self::new();
        if value {
            v.inner = vec![u8::MAX; number / 8];
        } else {
            v.inner = vec![0; number / 8];
        }

        v.len = number - number % 8;

        for _ in 0..=number % 8 {
            v.push(value);
        }

        v
    }

    pub fn fill_stretch(&mut self, index: usize, end_inclusive: usize, value: bool) {
        let mut current_index = index;

        while !current_index.is_multiple_of(8) {
            if current_index > end_inclusive {
                return;
            }

            self.set(current_index, value);
            current_index += 1;
        }

        while end_inclusive - current_index > 8 {
            if value {
                self.inner[current_index / 8] = u8::MAX;
            } else {
                self.inner[current_index / 8] = 0;
            }

            current_index += 8;
        }

        while current_index <= end_inclusive {
            self.set(current_index, value);
            current_index += 1;
        }
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_raw(&self) -> &[u8] {
        &self.inner
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

#[cfg(test)]
mod test {
    use crate::TightVec;

    #[test]
    fn push_1() {
        let mut t: TightVec = TightVec::new();

        t.push(true);
        assert!(t.index(0));
    }

    #[test]
    fn push_false() {
        let mut t: TightVec = TightVec::new();

        t.push(false);
        assert!(!t.index(0));
    }

    #[test]
    fn push_multiple() {
        let mut t: TightVec = TightVec::new();

        t.push(false);
        t.push(true);
        t.push(true);
        t.push(false);
        assert!(!t.index(0));
        assert!(t.index(1));
        assert!(t.index(2));
        assert!(!t.index(3));
    }

    #[test]
    fn push_over_8() {
        let mut t: TightVec = TightVec::new();

        t.push(false);
        t.push(true);
        t.push(true);
        t.push(false);
        t.push(true);
        t.push(false);
        t.push(true);
        t.push(false);
        t.push(true);
        t.push(false);
        assert!(!t.index(0));
        assert!(t.index(1));
        assert!(t.index(2));
        assert!(!t.index(3));
        assert!(t.index(4));
        assert!(!t.index(5));
        assert!(t.index(6));
        assert!(!t.index(7));
        assert!(t.index(8));
        assert!(!t.index(9));
    }

    #[test]
    fn set_index() {
        let mut t: TightVec = TightVec::new();
        t.push(false);
        t.push(true);
        t.push(true);
        t.push(false);

        t.set(2, false);

        assert!(!t.index(0));
        assert!(t.index(1));
        assert!(!t.index(2));
        assert!(!t.index(3));
    }
}
