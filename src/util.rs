#[allow(clippy::trivially_copy_pass_by_ref)] // needs to match signature for use in serde attribute
#[inline]
pub const fn is_false(v: &bool) -> bool {
    !(*v)
}

#[allow(clippy::trivially_copy_pass_by_ref)] // needs to match signature for use in serde attribute
#[inline]
pub const fn is_true(v: &bool) -> bool {
    *v
}

// https://github.com/serde-rs/serde/issues/1030#issuecomment-522278006
pub fn default_as_true() -> bool {
    true
}
