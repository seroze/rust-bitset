

/**
 * Why do we want a trait here, because in future we should be able to swap it with something more performant
 * 
 */
pub trait BitSet{

    // gets the bit at pos
    fn get_bit(&self, pos: usize) -> u64;

    // sets the bit at pos
    fn set_bit(&mut self, pos: usize);

    // resets all the bits to 0
    fn reset_bits(&mut self); 

    // Produces bitstring representation 
    fn to_string(&self) -> String;

}