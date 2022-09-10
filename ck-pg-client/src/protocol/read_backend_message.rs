use std::io::{self, Read};

/// Synchronously read a backend message from a stream.
///
/// The provided buffer is replaced with the entire message.
/// Reusing the buffer across reads reduces the number of allocations.
/// If reading fails, the contents of the buffer are unspecified.
pub fn read_backend_message<R>(r: &mut R, buf: &mut Vec<u8>) -> io::Result<()>
    where R: Read
{
    buf.clear();

    read_exact(r, buf, 5)?;

    let length = u32::from_be_bytes([buf[1], buf[2], buf[3], buf[4]]);
    let length = length.checked_sub(4).ok_or_else(make_length_error)?;

    read_exact(r, buf, length)?;

    Ok(())
}

/// Similar to [`Read::read_exact`], but appends onto a given buffer.
fn read_exact<R>(r: &mut R, buf: &mut Vec<u8>, size: u32) -> io::Result<()>
    where R: Read
{
    let size_u64 = u64::from(size);

    // Using Read::take with Read::read_to_end is a nice way to
    // safely read N bytes into an uninitialized buffer.
    let actual = r.take(size_u64).read_to_end(buf)?;

    // This doesn't overflow, because Read::take doesn't read more
    // than size_u64, and size_u64 originates from u32.
    let actual_u32 = actual as u32;

    if actual_u32 != size {
        Err(io::ErrorKind::UnexpectedEof.into())
    } else {
        Ok(())
    }
}

/// Construct an error about the message length being too short.
///
/// Message lengths are always at least 4,
/// because they include the length field itself.
#[cold]
fn make_length_error() -> io::Error
{
    let message = "PostgreSQL backend message length is too small";
    io::Error::new(io::ErrorKind::Other, message)
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn example()
    {
        let mut buf = &[b'R', 0, 0, 0, 12, 0, 0, 0, 5, 9, 8, 7, 6, 0xFF][..];
        let mut message = Vec::new();
        read_backend_message(&mut buf, &mut message).unwrap();
        assert_eq!(message, [b'R', 0, 0, 0, 12, 0, 0, 0, 5, 9, 8, 7, 6]);
        assert_eq!(buf, [0xFF], "read_backend_message read too much");
    }

    #[test]
    fn bad_length()
    {
        for i in 0 .. 4 {
            let mut buf = &[b'R', 0, 0, 0, i][..];
            let result = read_backend_message(&mut buf, &mut Vec::new());
            assert_eq!(
                result.unwrap_err().to_string(),
                make_length_error().to_string(),
            );
        }
    }

    #[test]
    fn unexpected_eof()
    {
        let mut buf = &[b'R', 0, 0, 0, 12, 0, 0, 0][..];
        let result = read_backend_message(&mut buf, &mut Vec::new());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::UnexpectedEof);
    }
}
