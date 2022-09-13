use {
    crate::{Error, Result},
    super::{BackendMessage, ErrorNoticeFieldArray},
    std::io::{self, Read},
};

/// Utility for receiving backend messages.
///
/// The receiver contains a buffer for reading backend messages into.
/// This buffer is reused across receives, reducing the number of allocations.
///
/// The receiver also contains a function called on notice response messages.
/// When a notice response is received, the function is called.
/// The receive is then retried until the message isn't a notice response.
/// This saves the caller from worrying about notice responses.
pub struct Receiver
{
    buf: Vec<u8>,
    on_notice: Box<dyn FnMut(ErrorNoticeFieldArray) + Send>,
}

impl Receiver
{
    /// Create a receiver with a notice handler.
    pub fn new<F>(on_notice: F) -> Self
        where F: 'static + FnMut(ErrorNoticeFieldArray) + Send
    {
        Self{buf: Vec::new(), on_notice: Box::new(on_notice)}
    }

    /// Synchronously read and parse a backend message from a stream.
    ///
    /// If the message cannot be parsed, an error is returned.
    pub fn receive<'a, R>(&'a mut self, r: &mut R)
        -> Result<BackendMessage<'a>>
        where R: Read
    {
        let buf = &mut self.buf;
        loop {
            read_backend_message(r, buf)?;
            // We have to duplicate the parse in both branches because of [1].
            // Once Polonius arrives, we can move the parse out of the `if`
            // and replace `unreachable!()` with `break Ok(message);`.
            // [1]: https://github.com/rust-lang/rust/issues/54663
            if buf.starts_with(b"N") {
                let message = BackendMessage::parse(buf)
                    .ok_or(Error::BackendMessageParse)?;
                if let BackendMessage::NoticeResponse{fields} = message {
                    (self.on_notice)(fields);
                } else {
                    unreachable!();
                }
            } else {
                break BackendMessage::parse(buf)
                    .ok_or(Error::BackendMessageParse);
            }
        }
    }
}

/// Synchronously read a backend message from a stream.
///
/// The provided buffer is replaced with the entire message.
/// Reusing the buffer across reads reduces the number of allocations.
/// If reading fails, the contents of the buffer are unspecified.
pub fn read_backend_message<R>(r: &mut R, buf: &mut Vec<u8>) -> Result<()>
    where R: Read
{
    buf.clear();

    read_exact(r, buf, 5)?;

    let length = u32::from_be_bytes([buf[1], buf[2], buf[3], buf[4]]);
    let length = length.checked_sub(4).ok_or(Error::BackendMessageParse)?;

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

#[cfg(test)]
mod tests
{
    use {super::*, std::assert_matches::assert_matches};

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
            assert_matches!(result.unwrap_err(), Error::BackendMessageParse);
        }
    }

    #[test]
    fn unexpected_eof()
    {
        let mut buf = &[b'R', 0, 0, 0, 12, 0, 0, 0][..];
        let result = read_backend_message(&mut buf, &mut Vec::new());
        assert_matches!(
            result.unwrap_err(),
            Error::Io(err) if err.kind() == io::ErrorKind::UnexpectedEof,
        );
    }
}
