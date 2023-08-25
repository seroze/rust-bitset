

use crate::bitset::BitSet; // should we be using file names here ? in rust it's not like java where you follow class name and file name to match 

pub struct Vec64Bitset{
    bit_arr: Vec<u64>,
    size: usize,
}

impl Vec64Bitset{

    pub fn new(size: usize) -> Vec64Bitset {

        // let's say 
        let mut sz = size/ 64; 
        if size%64!=0 {sz+=1;}

        // println!("sz= {}, size = {}", sz, size); 
        // let bit_arr: Vec<u64> = Vec::with_capacity(sz);
        let bit_arr: Vec<u64> = vec![0u64; sz];

        // println!("bit_arr = {:?}", bit_arr);
        
        // println!("Bitset is created succesfully with len {}", bit_arr.len());

        Self{
            bit_arr, 
            size: size,
        }
    }

    pub fn get_size(&self) -> usize {
        return self.size; 
    }

}

impl BitSet for Vec64Bitset{

    fn get_bit(&self, pos: usize) -> u64 {

        let block_id = pos/64;
        let rem = pos%64;

        self.bit_arr[block_id+rem]
    }

    // assumes pos is 0-indexed
    fn set_bit(&mut self, pos: usize) {

        let block_id = pos/64;
        let rem = pos%64;

        // println!("block_id = {}, rem = {}", block_id, rem);

        self.bit_arr[block_id] |= 1<<rem; 

        println!("Bit set succesfully"); 
        
    }

    fn reset_bits(&mut self) {
        
        let n = self.bit_arr.len(); 
        for i in 0..n{
            self.bit_arr[i] = 0; 
        }
    }

    fn to_string(&self) -> String {
        //convert underlying bit_arr to String 
        let mut v = Vec::<u8>::new(); 

        // remember the size 
        for i in 0..(self.size){

            let block_id = i/64;
            let rem = i%64;

            v.push(((self.bit_arr[block_id]>>rem)&1) as u8);
        }

        println!("bit_arr= {:?}", self.bit_arr);
        println!("v= {:?}", v);
        // convert vec<u8> into binary string 
        v.into_iter().map(|x| ('0' as u8 + x as u8) as char).rev().collect::<String>()
        // "0".to_string()
    }


}

// how does left << on a bitset work ? how to make it fast 