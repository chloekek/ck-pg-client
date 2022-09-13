use crate::{Error, Result};

pub fn write_int32_u32(buf: &mut Vec<u8>, value: u32)
{
    buf.extend_from_slice(&value.to_be_bytes());
}

pub fn write_string_slice(buf: &mut Vec<u8>, value: &[u8]) -> Result<()>
{
    if value.contains(&0) {
        Err(Error::Nul)
    } else {
        buf.extend_from_slice(value);
        buf.push(0);
        Ok(())
    }
}
