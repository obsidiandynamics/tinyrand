use super::*;

#[test]
fn with_thread_local() {
    let mut rand = thread_rand();
    assert_ne!(0, rand.next_u16());
    assert_ne!(0, rand.next_u32());
    assert_ne!(0, rand.next_u64());
    assert_ne!(0, rand.next_u128());
    assert_ne!(0, rand.next_usize());
    assert_ne!(rand.next_u16() as u32, rand.next_u32());
    assert_ne!(rand.next_u16() as u64, rand.next_u64());
    assert_ne!(rand.next_u16() as u128, rand.next_u128());
    assert_ne!(rand.next_u32() as u64, rand.next_u64());
    assert_ne!(rand.next_u32() as u128, rand.next_u128());
    assert_ne!(rand.next_u64() as u128, rand.next_u128());
    assert_ne!(rand.next_usize() as u128, rand.next_u128());
}