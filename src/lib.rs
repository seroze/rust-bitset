// declare all the available classes here 
mod bitset; 
mod vec64impl;

pub use crate::vec64impl::Vec64Bitset; 

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::bitset::BitSet;

    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // let's write some tests
    #[test]
    fn bitset_print(){

        let mut a = Vec64Bitset::new(10);
        // let mut b = Vec64Bitset::new(10);
        println!("before a = {}", a.to_string());

        a.set_bit(1); 
        a.set_bit(2); 
        a.set_bit(5); 
        a.set_bit(7);

        // b.set_bit(1); 
        // b.set_bit(2); 
        // b.set_bit(4); 
        // b.set_bit(7);

        println!("after a = {}", a.to_string());

    }
}
