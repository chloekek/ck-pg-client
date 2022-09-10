use {
    crate::usize_conversions::u32_to_usize,
    std::{ffi::CStr, fmt, num::{NonZeroU8, NonZeroU16, NonZeroU32}},
};

/// Backend message.
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub enum BackendMessage<'a>
{
    AuthenticationOk,
    AuthenticationKerberosV5,
    AuthenticationCleartextPassword,
    AuthenticationMD5Password{
        salt: [u8; 4],
    },
    AuthenticationSCMCredential,
    AuthenticationGSS,
    AuthenticationGSSContinue{
        authentication_data: &'a [u8],
    },
    AuthenticationSSPI,
    AuthenticationSASL{
        authentication_mechanism_names: StringArray<'a>,
    },
    AuthenticationSASLContinue{
        data: &'a [u8],
    },
    AuthenticationSASLFinal{
        additional_data: &'a [u8],
    },
    BackendKeyData{
        this_backend_process_id: u32,
        this_backend_secret_key: u32,
    },
    BindComplete,
    CloseComplete,
    CommandComplete{
        command_tag: &'a CStr,
    },
    CopyData{
        data: &'a [u8],
    },
    CopyDone,
    CopyInResponse{
        overall_copy_format: u8,
        column_format_codes: Int16Array<'a>,
    },
    CopyOutResponse{
        overall_copy_format: u8,
        column_format_codes: Int16Array<'a>,
    },
    CopyBothResponse{
        overall_copy_format: u8,
        column_format_codes: Int16Array<'a>,
    },
    DataRow{
        column_values: DataRowColumnValueArray<'a>,
    },
    EmptyQueryResponse,
    ErrorResponse{
        fields: ErrorNoticeFieldArray<'a>,
    },
    FunctionCallResponse{
        result_value: Option<&'a [u8]>,
    },
    NegotiateProtocolVersion{
        newest_minor_protocol_version: u32,
        unrecognized_option_names: StringArray<'a>,
    },
    NoData,
    NoticeResponse{
        fields: ErrorNoticeFieldArray<'a>,
    },
    NotificationResponse{
        notifying_backend_process_id: u32,
        channel_name: &'a CStr,
        payload: &'a CStr,
    },
    ParameterDescription{
        data_type_oids: Int32Array<'a>,
    },
    ParameterStatus{
        name: &'a CStr,
        current_value: &'a CStr,
    },
    ParseComplete,
    PortalSuspended,
    ReadyForQuery{
        transaction_status_indicator: u8,
    },
    RowDescription{
        fields: RowDescriptionFieldArray<'a>,
    },
}

impl<'a> BackendMessage<'a>
{
    /// Parse a backend message.
    ///
    /// The expectation is that all backend messages are well-formed.
    /// An ill-formed backend message may either result in returning [`None`]
    /// or returning a message that contains an iterator that stops early.
    /// However, in no circumstance will this function panic.
    pub fn parse(mut input: &'a [u8]) -> Option<Self>
    {
        let [identifier] = parse_bytes_array(&mut input)?;
        let _length = parse_int32_u32(&mut input)?;
        match identifier {
            b'R' => Self::parse_authentication(input),
            b'K' => Self::parse_backend_key_data(input),
            b'2' => Self::parse_bind_complete(input),
            b'3' => Self::parse_close_complete(input),
            b'C' => Self::parse_command_complete(input),
            b'd' => Self::parse_copy_data(input),
            b'c' => Self::parse_copy_done(input),
            b'G' => Self::parse_copy_in_response(input),
            b'H' => Self::parse_copy_out_response(input),
            b'W' => Self::parse_copy_both_response(input),
            b'D' => Self::parse_data_row(input),
            b'I' => Self::parse_empty_query_response(input),
            b'E' => Self::parse_error_response(input),
            b'V' => Self::parse_function_call_response(input),
            b'v' => Self::parse_negotiate_protocol_version(input),
            b'n' => Self::parse_no_data(input),
            b'N' => Self::parse_notice_response(input),
            b'A' => Self::parse_notification_response(input),
            b't' => Self::parse_parameter_description(input),
            b'S' => Self::parse_parameter_status(input),
            b'1' => Self::parse_parse_complete(input),
            b's' => Self::parse_portal_suspended(input),
            b'Z' => Self::parse_ready_for_query(input),
            b'T' => Self::parse_row_description(input),
            _ => None,
        }
    }

    fn parse_authentication(mut input: &'a [u8]) -> Option<Self>
    {
        let challenge = parse_int32_u32(&mut input)?;
        match challenge {
             0 => Self::parse_authentication_ok(input),
             2 => Self::parse_authentication_kerberos_v5(input),
             3 => Self::parse_authentication_cleartext_password(input),
             5 => Self::parse_authentication_md5_password(input),
             6 => Self::parse_authentication_scm_credential(input),
             7 => Self::parse_authentication_gss(input),
             8 => Self::parse_authentication_gss_continue(input),
             9 => Self::parse_authentication_sspi(input),
            10 => Self::parse_authentication_sasl(input),
            11 => Self::parse_authentication_sasl_continue(input),
            12 => Self::parse_authentication_sasl_final(input),
            _ => None,
        }
    }

    fn parse_authentication_ok(_input: &'a [u8]) -> Option<Self>
    {
        Some(Self::AuthenticationOk)
    }

    fn parse_authentication_kerberos_v5(mut input: &'a [u8]) -> Option<Self>
    {
        input = input; todo!("{:?}", input)
    }

    fn parse_authentication_cleartext_password(mut input: &'a [u8])
        -> Option<Self>
    {
        input = input; todo!("{:?}", input)
    }

    fn parse_authentication_md5_password(mut input: &'a [u8]) -> Option<Self>
    {
        input = input; todo!("{:?}", input)
    }

    fn parse_authentication_scm_credential(mut input: &'a [u8]) -> Option<Self>
    {
        input = input; todo!("{:?}", input)
    }

    fn parse_authentication_gss(mut input: &'a [u8]) -> Option<Self>
    {
        input = input; todo!("{:?}", input)
    }

    fn parse_authentication_gss_continue(mut input: &'a [u8]) -> Option<Self>
    {
        input = input; todo!("{:?}", input)
    }

    fn parse_authentication_sspi(mut input: &'a [u8]) -> Option<Self>
    {
        input = input; todo!("{:?}", input)
    }

    fn parse_authentication_sasl(mut input: &'a [u8]) -> Option<Self>
    {
        input = input; todo!("{:?}", input)
    }

    fn parse_authentication_sasl_continue(mut input: &'a [u8]) -> Option<Self>
    {
        input = input; todo!("{:?}", input)
    }

    fn parse_authentication_sasl_final(mut input: &'a [u8]) -> Option<Self>
    {
        input = input; todo!("{:?}", input)
    }

    fn parse_backend_key_data(mut input: &'a [u8]) -> Option<Self>
    {
        let this_backend_process_id = parse_int32_u32(&mut input)?;
        let this_backend_secret_key = parse_int32_u32(&mut input)?;

        let message = Self::BackendKeyData{
            this_backend_process_id,
            this_backend_secret_key,
        };
        Some(message)
    }

    fn parse_bind_complete(_input: &'a [u8]) -> Option<Self>
    {
        Some(Self::BindComplete)
    }

    fn parse_close_complete(_input: &'a [u8]) -> Option<Self>
    {
        Some(Self::CloseComplete)
    }

    fn parse_command_complete(mut input: &'a [u8]) -> Option<Self>
    {
        let command_tag = parse_string_cstr(&mut input)?;
        Some(Self::CommandComplete{command_tag})
    }

    fn parse_copy_data(input: &'a [u8]) -> Option<Self>
    {
        let data = input;
        Some(Self::CopyData{data})
    }

    fn parse_copy_done(_input: &'a [u8]) -> Option<Self>
    {
        Some(Self::CopyDone)
    }

    fn parse_copy_in_response(mut input: &'a [u8]) -> Option<Self>
    {
        let overall_copy_format = parse_int8_u8(&mut input)?;
        let _column_count = parse_int16_u16(&mut input)?;
        let column_format_codes = Int16Array(input);
        Some(Self::CopyInResponse{overall_copy_format, column_format_codes})
    }

    fn parse_copy_out_response(mut input: &'a [u8]) -> Option<Self>
    {
        let overall_copy_format = parse_int8_u8(&mut input)?;
        let _column_count = parse_int16_u16(&mut input)?;
        let column_format_codes = Int16Array(input);
        Some(Self::CopyOutResponse{overall_copy_format, column_format_codes})
    }

    fn parse_copy_both_response(mut input: &'a [u8]) -> Option<Self>
    {
        let overall_copy_format = parse_int8_u8(&mut input)?;
        let _column_count = parse_int16_u16(&mut input)?;
        let column_format_codes = Int16Array(input);
        Some(Self::CopyBothResponse{overall_copy_format, column_format_codes})
    }

    fn parse_data_row(mut input: &'a [u8]) -> Option<Self>
    {
        let _column_count = parse_int16_u16(&mut input)?;
        let column_values = DataRowColumnValueArray(input);
        Some(Self::DataRow{column_values})
    }

    fn parse_empty_query_response(_input: &'a [u8]) -> Option<Self>
    {
        Some(Self::EmptyQueryResponse)
    }

    fn parse_error_response(mut input: &'a [u8]) -> Option<Self>
    {
        let _trailing_zero;
        (_trailing_zero, input) = input.split_last()?;
        let fields = ErrorNoticeFieldArray(input);
        Some(Self::ErrorResponse{fields})
    }

    fn parse_function_call_response(mut input: &'a [u8]) -> Option<Self>
    {
        let result_value_length = parse_int32_u32(&mut input)?;

        let result_value =
            if result_value_length == u32::MAX {
                None
            } else {
                let length = u32_to_usize(result_value_length);
                let result_value = parse_bytes_slice(&mut input, length)?;
                Some(result_value)
            };

        Some(Self::FunctionCallResponse{result_value})
    }

    fn parse_negotiate_protocol_version(mut input: &'a [u8]) -> Option<Self>
    {
        let newest_minor_protocol_version = parse_int32_u32(&mut input)?;
        let _unrecognized_option_count = parse_int32_u32(&mut input)?;
        let unrecognized_option_names = StringArray(input);
        let message = Self::NegotiateProtocolVersion{
            newest_minor_protocol_version,
            unrecognized_option_names,
        };
        Some(message)
    }

    fn parse_no_data(_input: &'a [u8]) -> Option<Self>
    {
        Some(Self::NoData)
    }

    fn parse_notice_response(mut input: &'a [u8]) -> Option<Self>
    {
        let _trailing_zero;
        (_trailing_zero, input) = input.split_last()?;
        let fields = ErrorNoticeFieldArray(input);
        Some(Self::NoticeResponse{fields})
    }

    fn parse_notification_response(mut input: &'a [u8]) -> Option<Self>
    {
        let notifying_backend_process_id = parse_int32_u32(&mut input)?;
        let channel_name = parse_string_cstr(&mut input)?;
        let payload = parse_string_cstr(&mut input)?;
        let message = Self::NotificationResponse{
            notifying_backend_process_id,
            channel_name,
            payload,
        };
        Some(message)
    }

    fn parse_parameter_description(mut input: &'a [u8]) -> Option<Self>
    {
        let _parameter_count = parse_int16_u16(&mut input)?;
        let data_type_oids = Int32Array(input);
        Some(Self::ParameterDescription{data_type_oids})
    }

    fn parse_parameter_status(mut input: &'a [u8]) -> Option<Self>
    {
        let name = parse_string_cstr(&mut input)?;
        let current_value = parse_string_cstr(&mut input)?;
        Some(Self::ParameterStatus{name, current_value})
    }

    fn parse_parse_complete(_input: &'a [u8]) -> Option<Self>
    {
        Some(Self::ParseComplete)
    }

    fn parse_portal_suspended(_input: &'a [u8]) -> Option<Self>
    {
        Some(Self::PortalSuspended)
    }

    fn parse_ready_for_query(mut input: &'a [u8]) -> Option<Self>
    {
        let [transaction_status_indicator] = parse_bytes_array(&mut input)?;
        Some(Self::ReadyForQuery{transaction_status_indicator})
    }

    fn parse_row_description(mut input: &'a [u8]) -> Option<Self>
    {
        let _field_count = parse_int16_u16(&mut input)?;
        let fields = RowDescriptionFieldArray(input);
        Some(Self::RowDescription{fields})
    }
}

/* ------------- Iterators over array fields of backend messages ------------ */

/// Iterator over an array of `Int16`s.
#[derive(Clone)]
pub struct Int16Array<'a>(&'a [u8]);

impl<'a> Iterator for Int16Array<'a>
{
    type Item = u16;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item>
    {
        parse_int16_u16(&mut self.0)
    }
}

/// Iterator over an array of `Int32`s.
#[derive(Clone)]
pub struct Int32Array<'a>(&'a [u8]);

impl<'a> Iterator for Int32Array<'a>
{
    type Item = u32;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item>
    {
        parse_int32_u32(&mut self.0)
    }
}

/// Iterator over an array of `String`s.
#[derive(Clone)]
pub struct StringArray<'a>(&'a [u8]);

impl<'a> Iterator for StringArray<'a>
{
    type Item = &'a CStr;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item>
    {
        parse_string_cstr(&mut self.0)
    }
}

/// Iterator over an array of fields of `ErrorResponse` and `NoticeResponse`.
#[derive(Clone)]
pub struct ErrorNoticeFieldArray<'a>(&'a [u8]);

/// Subordinate fields of an `ErrorResponse` or `NoticeResponse` field.
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub struct ErrorNoticeField<'a>
{
    pub type_code: NonZeroU8,
    pub value: &'a CStr,
}

impl<'a> Iterator for ErrorNoticeFieldArray<'a>
{
    type Item = ErrorNoticeField<'a>;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item>
    {
        let mut buf = self.0;

        let [type_code] = parse_bytes_array(&mut buf)?;
        let type_code = NonZeroU8::new(type_code)?;

        let value = parse_string_cstr(&mut buf)?;

        self.0 = buf;

        Some(ErrorNoticeField{type_code, value})
    }
}

/// Iterator over an array of `DataRow` column values.
#[derive(Clone)]
pub struct DataRowColumnValueArray<'a>(&'a [u8]);

impl<'a> Iterator for DataRowColumnValueArray<'a>
{
    type Item = Option<&'a [u8]>;

    fn next(&mut self) -> Option<Self::Item>
    {
        let mut buf = self.0;

        let length = parse_int32_u32(&mut buf)?;

        let value =
            if length == u32::MAX {
                None
            } else {
                let length = u32_to_usize(length);
                let value = parse_bytes_slice(&mut buf, length)?;
                Some(value)
            };

        self.0 = buf;

        Some(value)
    }
}

/// Iterator over an array of `RowDescription` fields.
#[derive(Clone)]
pub struct RowDescriptionFieldArray<'a>(&'a [u8]);

/// Subordinate fields of a `RowDescription` field.
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub struct RowDescriptionField<'a>
{
    pub name: &'a CStr,
    pub table_oid: Option<NonZeroU32>,
    pub attribute_number: Option<NonZeroU16>,
    pub data_type_oid: u32,
    pub data_type_size: i16,
    pub data_type_modifier: u32,
    pub format_code: u16,
}

impl<'a> Iterator for RowDescriptionFieldArray<'a>
{
    type Item = RowDescriptionField<'a>;

    fn next(&mut self) -> Option<Self::Item>
    {
        let mut buf = self.0;

        let name = parse_string_cstr(&mut buf)?;
        let table_oid = parse_int32_u32(&mut buf)?;
        let attribute_number = parse_int16_u16(&mut buf)?;
        let data_type_oid = parse_int32_u32(&mut buf)?;
        let data_type_size = parse_int16_i16(&mut buf)?;
        let data_type_modifier = parse_int32_u32(&mut buf)?;
        let format_code = parse_int16_u16(&mut buf)?;

        let table_oid = NonZeroU32::new(table_oid);
        let attribute_number = NonZeroU16::new(attribute_number);

        self.0 = buf;

        let item = RowDescriptionField{
            name, table_oid, attribute_number, data_type_oid,
            data_type_size, data_type_modifier, format_code,
        };
        Some(item)
    }
}

/* ----------------- Implementing Debug for those iterators ----------------- */

macro_rules! iterator_debug
{
    ($name:ident) => {
        impl<'a> fmt::Debug for $name<'a>
        {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
            {
                f.debug_list()
                    .entries(self.clone())
                    .finish()
            }
        }
    };
}

iterator_debug!(Int16Array);
iterator_debug!(Int32Array);
iterator_debug!(StringArray);
iterator_debug!(DataRowColumnValueArray);
iterator_debug!(ErrorNoticeFieldArray);
iterator_debug!(RowDescriptionFieldArray);

/* ------------------------- Low-level field parsing ------------------------ */

// These functions parse the PostgreSQL message data types.
// The functions are named `parse_<pgty>_<rsty>` where `<pgty>`
// is the message data type and `<rsty>` is the corresponding Rust type.

#[inline(always)]
fn parse_int8_u8(input: &mut &[u8]) -> Option<u8>
{
    parse_bytes_array(input).map(u8::from_be_bytes)
}

#[inline(always)]
fn parse_int16_i16(input: &mut &[u8]) -> Option<i16>
{
    parse_bytes_array(input).map(i16::from_be_bytes)
}

#[inline(always)]
fn parse_int16_u16(input: &mut &[u8]) -> Option<u16>
{
    parse_bytes_array(input).map(u16::from_be_bytes)
}

#[inline(always)]
fn parse_int32_u32(input: &mut &[u8]) -> Option<u32>
{
    parse_bytes_array(input).map(u32::from_be_bytes)
}

#[inline(always)]
fn parse_string_cstr<'a>(input: &mut &'a [u8]) -> Option<&'a CStr>
{
    let value = CStr::from_bytes_until_nul(input).ok()?;
    *input = &input[value.to_bytes_with_nul().len() ..];
    Some(value)
}

#[inline(always)]
fn parse_bytes_slice<'a>(input: &mut &'a [u8], len: usize) -> Option<&'a [u8]>
{
    if input.len() < len {
        None
    } else {
        let (prefix, suffix) = input.split_at(len);
        *input = suffix;
        Some(prefix)
    }
}

#[inline(always)]
fn parse_bytes_array<const N: usize>(input: &mut &[u8]) -> Option<[u8; N]>
{
    let slice = parse_bytes_slice(input, N)?;
    let array = <[u8; N]>::try_from(slice).unwrap();
    Some(array)
}
