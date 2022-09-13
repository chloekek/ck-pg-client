use super::Md5;

/// Implementation of the [`Md5`] trait that unconditionally fails to hash.
pub struct Md5Unavailable;

impl Md5 for Md5Unavailable
{
    fn md5(&self, _plaintext: &[u8]) -> Option<[u8; 16]>
    {
        None
    }
}
