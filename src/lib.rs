use fasthash::{xx, murmur, murmur2, murmur3};
use bitvec::vec::BitVec;

#[derive(Debug)]
pub struct Bloomfilter {
    size: usize,
    pub bit_vector : BitVec,
    hashes : i8,
}

impl Bloomfilter {
    pub fn new(size : usize, hash_num : i8) -> Bloomfilter {
        let mut bitvector : BitVec = BitVec::with_capacity(size);
        bitvector.resize(size, false);
        Bloomfilter {
            size: size,
            bit_vector : bitvector,
            hashes : hash_num
        }
    }

    pub fn insert(&mut self, value : &[u8]) {
        self.bit_vector.set(xx::hash64(value) as usize % self.size, true);

        if self.hashes > 1 {
            self.bit_vector.set(xx::hash32(value) as usize % self.size, true);
        }
        if self.hashes > 2 {
            self.bit_vector.set(murmur::hash32(value) as usize % self.size, true);
        }
        if self.hashes > 3 {
            self.bit_vector.set(murmur2::hash32(value) as usize % self.size, true);
        }
        if self.hashes > 4 {
            self.bit_vector.set(murmur2::hash64(value) as usize % self.size, true);
        }
        if self.hashes > 5 {
            self.bit_vector.set(murmur3::hash32(value) as usize % self.size, true);
        }
        if self.hashes > 6 {
            self.bit_vector.set(murmur3::hash128(value) as usize % self.size, true);
        }
    }

    pub fn lookup(&self, value : &[u8]) -> bool {

        if self.bit_vector[xx::hash64(value) as usize % self.size] == false {
            return false
        }

        if self.hashes > 1 && self.bit_vector[xx::hash32(value) as usize % self.size] == false {
            return false
        }

        if self.hashes > 2 && self.bit_vector[murmur::hash32(value) as usize % self.size] == false {
            return false
        }
        if self.hashes > 3 && self.bit_vector[murmur2::hash32(value) as usize % self.size] == false {
            return false
        }
        if self.hashes > 4 && self.bit_vector[murmur2::hash64(value) as usize % self.size] == false {
            return false
        }
        if self.hashes > 5 && self.bit_vector[murmur3::hash32(value) as usize % self.size] == false {
            return false
        }
        if self.hashes > 6 && self.bit_vector[murmur3::hash128(value) as usize % self.size] == false {
            return false
        }


        return true
    }
}
