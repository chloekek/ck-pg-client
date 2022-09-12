use {
    crate::{capabilities::Md5, protocol::{BackendMessage, Receiver}},
    std::{
        borrow::Cow,
        collections::HashMap,
        ffi::CString,
        io::{self, Read, Write},
    },
    thiserror::Error,
};

/// Information discovered during the start-up flow.
pub struct StartUpInfo
{
    pub parameter_statuses: HashMap<CString, CString>,
    pub backend_process_id: u32,
    pub backend_secret_key: u32,
}

/// Error returned by [`start_up()`].
#[allow(missing_docs)]
#[derive(Debug, Error)]
pub enum StartUpError
{
    #[error("{0}")]
    Io(#[from] io::Error),

    #[error("received unexpected backend message")]
    UnexpectedBackendMessage,
}

/// Initiate a database connection given a stream.
///
/// This function will perform the [_Start-up_][spec] flow.
/// No data must be sent on the stream prior to calling this function.
/// The `parameters` argument specifies `StartupMessage` parameters.
/// The `authenticator` argument is used to solve authentication challenges.
///
#[doc = crate::pgdoc::start_up!("spec")]
pub fn start_up<S, I, N, V, M>(
    receiver: &mut Receiver,
    stream: &mut S,
    parameters: I,
    md5: &M,
) -> Result<StartUpInfo, StartUpError>
    where S: Read + Write
        , I: IntoIterator<Item = (N, V)>
        , N: AsRef<str>
        , V: AsRef<str>
        , M: Md5
{
    let startup_message = build_startup_message(parameters);
    stream.write_all(&startup_message)?;
    drop(startup_message);

    handle_authentication(receiver, stream, md5)?;

    handle_info(receiver, stream)
}

fn build_startup_message<I, N, V>(parameters: I) -> Vec<u8>
    where I: IntoIterator<Item = (N, V)>, N: AsRef<str>, V: AsRef<str>
{
    let mut buf = vec![0, 0, 0, 0];

    buf.extend_from_slice(&196608u32.to_be_bytes());

    for (name, value) in parameters {
        // XXX: What do we do about interior nuls?
        buf.extend_from_slice(name.as_ref().as_bytes());
        buf.push(0);
        buf.extend_from_slice(value.as_ref().as_bytes());
        buf.push(0);
    }

    buf.push(0);

    let length = u32::try_from(buf.len()).unwrap();
    buf[0 .. 4].copy_from_slice(&length.to_be_bytes());

    buf
}

fn handle_authentication<S, M>(
    receiver: &mut Receiver,
    stream: &mut S,
    md5: &M,
) -> Result<(), StartUpError>
    where S: Read, M: Md5
{
    let message = receiver.receive(stream)?;
    match message {
        BackendMessage::AuthenticationOk =>
            Ok(()),
        // TODO: Handle Authentication*.
        // TODO: Handle NegotiateProtocolVersion.
        BackendMessage::ErrorResponse{..} =>
            todo!("{message:?}"),
        _ =>
            Err(StartUpError::UnexpectedBackendMessage),
    }
}

fn handle_info<S>(receiver: &mut Receiver, stream: &mut S)
    -> Result<StartUpInfo, StartUpError>
    where S: Read
{
    let mut info = StartUpInfo{
        parameter_statuses: HashMap::new(),
        backend_process_id: 0,
        backend_secret_key: 0,
    };

    loop {
        let message = receiver.receive(stream)?;
        match message {
            BackendMessage::BackendKeyData{
                this_backend_process_id,
                this_backend_secret_key,
            } => {
                info.backend_process_id = this_backend_process_id;
                info.backend_secret_key = this_backend_secret_key;
            },
            BackendMessage::ParameterStatus{name, current_value} => {
                let name = name.to_owned();
                let current_value = current_value.to_owned();
                info.parameter_statuses.insert(name, current_value);
            },
            BackendMessage::ReadyForQuery{transaction_status_indicator} =>
                break,
            BackendMessage::ErrorResponse{..} =>
                todo!("{message:?}"),
            _ =>
                return Err(StartUpError::UnexpectedBackendMessage),
        }
    }

    Ok(info)
}
