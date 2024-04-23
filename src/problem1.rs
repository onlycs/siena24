use crate::common::*;

pub fn main() {
    /*
     * if n is divisible by two, its binary representation ends in a zero
     * if n is divided by zero, it gets bit-shifted right 1, i.e. 0b1010 ==> 0b0101
     * therefore, the number of times n can be divided by two is the number of
     * trailing zeros in the binary representation of n. By extension, K is that
     * number.
     */
    println!("{}", read_line::<usize>().trailing_zeros());
}
