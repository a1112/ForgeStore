use bytes::BytesMut;
use std::panic::{catch_unwind, AssertUnwindSafe};

// Run in release mode: older bytes versions panic only with debug overflow checks.
// Do not call put_u8 or spare_capacity_mut after reserve; the vulnerable version
// can corrupt the capacity, so those APIs would turn this regression into UB.
#[test]
fn bytes_mut_rejects_capacity_overflow_with_unique_split_owner() {
    let mut original = BytesMut::from(&b"hello world"[..]);
    let mut tail = original.split_off(5);
    drop(original);

    let result = catch_unwind(AssertUnwindSafe(|| tail.reserve(usize::MAX - 6)));
    assert!(result.is_err(), "overflowing reserve must be rejected");
}
