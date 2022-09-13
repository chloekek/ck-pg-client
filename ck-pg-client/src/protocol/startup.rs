use {
    crate::{
        Error,
        Result,
        capabilities::Md5,
        protocol::{
            BackendMessage,
            Receiver,
            write_int32_u32,
            write_string_slice,
        },
    },
    std::{collections::HashMap, ffi::CString, io::{Read, Write}},
};

/// Information discovered during the start-up flow.
pub struct StartupInfo
{
    pub parameter_statuses: HashMap<CString, CString>,
    pub backend_process_id: u32,
    pub backend_secret_key: u32,
}

/// Implementation of the [_Start-up_][spec] flow.
///
/// No data must be sent on the stream prior to calling this function.
/// The `parameters` argument specifies `StartupMessage` parameters.
///
#[doc = crate::pgdoc::startup!("spec")]
pub fn startup<M, S, I, N, V>(
    md5: &M,
    receiver: &mut Receiver,
    stream: &mut S,
    parameters: I,
) -> Result<StartupInfo>
    where M: Md5
        , S: Read + Write
        , I: IntoIterator<Item = (N, V)>
        , N: AsRef<[u8]>
        , V: AsRef<[u8]>
{
    let startup_message = build_startup_message(parameters)?;
    stream.write_all(&startup_message)?;
    drop(startup_message);

    handle_authentication(md5, receiver, stream)?;

    handle_info(receiver, stream)
}

fn build_startup_message<I, N, V>(parameters: I) -> Result<Vec<u8>>
    where I: IntoIterator<Item = (N, V)>, N: AsRef<[u8]>, V: AsRef<[u8]>
{
    let mut buf = vec![0, 0, 0, 0];

    write_int32_u32(&mut buf, 196608);

    for (name, value) in parameters {
        write_string_slice(&mut buf, name.as_ref())?;
        write_string_slice(&mut buf, value.as_ref())?;
    }

    buf.push(0);

    let length = u32::try_from(buf.len()).unwrap();
    buf[0 .. 4].copy_from_slice(&length.to_be_bytes());

    Ok(buf)
}

fn handle_authentication<M, S>(
    md5: &M,
    receiver: &mut Receiver,
    stream: &mut S,
) -> Result<()>
    where M: Md5, S: Read
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
            Err(Error::BackendMessageUnexpected),
    }
}

fn handle_info<S>(receiver: &mut Receiver, stream: &mut S)
    -> Result<StartupInfo>
    where S: Read
{
    let mut info = StartupInfo{
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
                return Err(Error::BackendMessageUnexpected),
        }
    }

    Ok(info)
}
