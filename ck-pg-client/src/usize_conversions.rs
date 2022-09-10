//! Infallible conversions to and from `usize`.
//!
//! Rust theoretically supports 16-bit targets, but ck-pg-client doesn't.
//! Using PostgreSQL on 16-bit targets makes little sense
//! given PostgreSQL reports many lengths as 32-bit integers.

#[cfg(not(any(target_pointer_width = "32", target_pointer_width = "64")))]
compile_error!("ck-pg-client is only supported on 32-bit and 64-bit targets");

pub fn u32_to_usize(value: u32) -> usize
{
    value as usize
}
