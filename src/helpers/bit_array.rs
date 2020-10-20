/// This structure is able to store numbers with a specified custom bit-count
#[derive(Clone, Debug)]
pub struct BitArray {
    len: usize,
    item_width: u8,
    mask: u64,
    data: Vec<u64>,
}

impl BitArray {
    pub const MIN_BITS_PER_ITEM: u8 = 4;
    pub const MAX_BITS_PER_ITEM: u8 = 64;
    pub fn new(item_width: u8, capacity: usize) -> Self {
        assert!(
            item_width <= Self::MAX_BITS_PER_ITEM,
            "Item width is limited to {} bits (got {})",
            Self::MAX_BITS_PER_ITEM,
            item_width
        );
        assert!(
            item_width >= Self::MIN_BITS_PER_ITEM,
            "Item width must be larger than {} (got {})",
            Self::MIN_BITS_PER_ITEM,
            item_width
        );

        let data = vec![0; (((capacity * (item_width as usize)) as f64) / 64.0).ceil() as usize];

        Self {
            len: capacity,
            item_width: item_width,
            mask: (1 << (item_width as u64)) - 1,
            data: data,
        }
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn bits_per_item(&self) -> u8 {
        self.item_width
    }
    pub fn get(&self, index: usize) -> u64 {
        assert!(index < self.len, "Index out of bounds");

        let bit_index = index * (self.item_width as usize);
        let start_long_index = bit_index / 64;
        let start_long = self.data[start_long_index];
        let index_in_start_long = (bit_index % 64) as u64;
        let mut result = start_long >> index_in_start_long;

        let end_bit_offset = index_in_start_long + self.item_width as u64;
        if end_bit_offset > 64 {
            // value is between two u64 values
            let end_long = self.data[start_long_index + 1];
            result |= end_long << (64 - index_in_start_long);
        }

        result & self.mask
    }
    pub fn get_at_pos(&self, x: u8, y: u8, z: u8) -> u64 {
        assert!(x < 16);
        assert!(y < 16);
        assert!(z < 16);
        self.get(((y as usize) << 8) | ((z as usize) << 4) | x as usize)
    }
    pub fn set(&mut self, index: usize, val: u64) {
        assert!(index < self.len, "Index out of bounds");
        assert!(val <= self.mask, "Value is too large");

        let bit_index = index * (self.item_width as usize);
        let start_long_index = bit_index / 64;
        let index_in_start_long = (bit_index % 64) as u64;

        self.data[start_long_index] = (self.data[start_long_index]
            & !(self.mask << index_in_start_long))
            | ((val & self.mask) << index_in_start_long);

        let end_bit_offset = index_in_start_long + self.item_width as u64;
        if end_bit_offset > 64 {
            // value is between two u64 values
            let a = start_long_index + 1;
            self.data[a] = (self.data[a] & !((1 << (end_bit_offset - 64)) - 1))
                | (val >> (64 - index_in_start_long));
        }

        debug_assert_eq!(self.get(index), val);
    }
    pub fn set_at_pos(&mut self, x: u8, y: u8, z: u8, val: u64) {
        assert!(x < 16);
        assert!(y < 16);
        assert!(z < 16);
        self.set(((y as usize) << 8) | ((z as usize) << 4) | x as usize, val);
    }
    pub fn resize_to(&self, new_item_width: u8) -> Result<BitArray, ()> {
        assert!(
            new_item_width <= Self::MAX_BITS_PER_ITEM,
            "Item width is limited to {} bits (got {})",
            Self::MAX_BITS_PER_ITEM,
            new_item_width
        );
        assert!(
            new_item_width >= Self::MIN_BITS_PER_ITEM,
            "Item width must be larger than {} (got {})",
            Self::MIN_BITS_PER_ITEM,
            new_item_width
        );

        let mut new_arr = BitArray::new(new_item_width, self.len);

        for i in 0..self.len {
            let val = self.get(i);
            if needed_bits(val) > new_item_width {
                return Err(());
            }

            new_arr.set(i, val);
            debug_assert_eq!(new_arr.get(i), val);
        }

        Ok(new_arr)
    }
}

impl From<BitArray> for Vec<u64> {
    fn from(bit_array: BitArray) -> Self {
        bit_array.data
    }
}

impl AsRef<[u64]> for BitArray {
    fn as_ref(&self) -> &[u64] {
        &self.data
    }
}

fn needed_bits(n: u64) -> u8 {
    (64 - n.leading_zeros()) as u8
}
