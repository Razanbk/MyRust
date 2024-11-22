#![no_std]
extern crate alloc;

use alloc::vec::Vec;

#[test]
fn test_allocation() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    assert_eq!(vec, [1, 2]);
}
