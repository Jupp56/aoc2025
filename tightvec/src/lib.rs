pub const ITEM_SIZE: usize = 64;
pub type StorageItem = u64;

#[derive(Default, Clone, Debug)]
pub struct TightVec {
    inner: Vec<StorageItem>,
    len: usize,
}

impl TightVec {
    pub fn with_len_and_value(len: usize, value: bool) -> Self {
        let mut v = Self::default();

        if value {
            v.inner = vec![StorageItem::MAX; len / ITEM_SIZE];
        } else {
            v.inner = vec![0; len / ITEM_SIZE];
        }

        let remainder = len % ITEM_SIZE;

        v.len = len - remainder;

        for _ in 0..remainder {
            v.push(value);
        }

        v
    }

    pub fn get_raw(&self) -> &[StorageItem] {
        &self.inner
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn index(&self, index: usize) -> bool {
        let mask = 1 << Self::rem(index);

        self.inner_index(index) & mask == mask
    }

    pub fn try_index(&self, index: usize) -> Option<bool> {
        if index > self.len - 1 {
            return None;
        }

        Some(self.index(index))
    }

    pub fn set(&mut self, index: usize, value: bool) {
        let mask = Self::mask_for_index(index);

        if !value {
            let inverted_mask = u64::MAX - mask;
            *self.inner_index_mut(index) &= inverted_mask;
        } else {
            *self.inner_index_mut(index) |= mask;
        }
    }

    pub fn push(&mut self, value: bool) {
        let new_index = self.len;
        self.len += 1;

        if self.len == 0 || (self.len / ITEM_SIZE) >= self.inner.len() {
            self.inner.push(0);
        }

        self.set(new_index, value);
    }

    /// Fills multiple consecutive entries with the same value
    pub fn fill_multiple(&mut self, start_index: usize, end_index_inclusive: usize, value: bool) {
        let mut current_index = start_index;

        while !current_index.is_multiple_of(ITEM_SIZE) && current_index < end_index_inclusive {
            self.set(current_index, value);
            current_index += 1;
        }

        let fill_value = match value {
            true => StorageItem::MAX,
            false => 0,
        };

        while end_index_inclusive - current_index > ITEM_SIZE {
            *self.inner_index_mut(current_index) = fill_value;
            current_index += 64;
        }

        while current_index <= end_index_inclusive {
            self.set(current_index, value);
            current_index += 1;
        }
    }

    /// reference to the inner store containing the value for the given external index
    fn inner_index(&self, index: usize) -> &u64 {
        &self.inner[index / ITEM_SIZE]
    }

    /// mutable reference to the inner store containing the value for the given external index
    fn inner_index_mut(&mut self, index: usize) -> &mut u64 {
        &mut self.inner[index / ITEM_SIZE]
    }

    /// remainder when converting external -> internal index
    fn rem(index: usize) -> usize {
        index % ITEM_SIZE
    }

    fn mask_for_index(index: usize) -> StorageItem {
        1 << Self::rem(index)
    }
}

#[cfg(test)]
mod test {
    use crate::TightVec;

    #[test]
    fn push_1() {
        let mut t: TightVec = TightVec::default();

        t.push(true);
        assert!(t.index(0));
    }

    #[test]
    fn push_false() {
        let mut t: TightVec = TightVec::default();

        t.push(false);
        assert!(!t.index(0));
    }

    #[test]
    fn push_multiple() {
        let mut t: TightVec = TightVec::default();

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
        let mut t: TightVec = TightVec::default();

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
        let mut t: TightVec = TightVec::default();
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

    #[test]
    fn with_len_and_val() {
        let x: TightVec = TightVec::with_len_and_value(10, true);

        assert_eq!(x.len, 10);

        assert_eq!(x.inner.len(), 1);

        for i in 0..10 {
            assert!(x.index(i));
        }

        assert_eq!(x.try_index(11), None);
    }

    #[test]
    fn try_index() {
        let mut v = TightVec::default();
        v.push(false);
        v.push(true);

        assert_eq!(v.try_index(0), Some(false));
        assert_eq!(v.try_index(1), Some(true));
        assert_eq!(v.try_index(2), None);
    }

    #[test]
    fn fill_stretch() {
        let mut v = TightVec::with_len_and_value(20, false);

        v.fill_multiple(5, 18, true);

        for i in 0..=4 {
            assert!(!v.index(i))
        }

        for i in 5..=18 {
            assert!(v.index(i));
        }

        assert!(!v.index(19));
        assert_eq!(v.try_index(20), None);
    }
}
