#![no_std]

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

mod bump_allocator;
pub use bump_allocator::BumpAllocator;

#[global_allocator]
static ALLOCATOR: BumpAllocator = BumpAllocator::new(0x100000, 0x200000); // Example heap range.

#[alloc_error_handler]
fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
    panic!("Allocation error: {:?}", layout);
}
