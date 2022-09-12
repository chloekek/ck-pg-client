use super::Md5;

/// Implementation of the [`Md5`] trait using the [`md5`] crate.
pub struct Md5Md5;

impl Md5 for Md5Md5
{
    fn md5(&self, plaintext: &[u8]) -> Option<[u8; 16]>
    {
        Some(md5::compute(plaintext).0)
    }
}
