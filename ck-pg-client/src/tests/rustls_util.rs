use {
    rustls::{
        Certificate,
        ClientConfig,
        ClientConnection,
        Error,
        ServerName,
        client::{ServerCertVerified, ServerCertVerifier},
    },
    std::{sync::Arc, time::SystemTime},
};

/// Configure a client connection for use in tests.
pub fn create_client_connection() -> ClientConnection
{
    let server_cert_verifier = Arc::new(NoServerCertVerification);
    let client_config = Arc::new(
        ClientConfig::builder()
        .with_safe_defaults()
        .with_custom_certificate_verifier(server_cert_verifier)
        .with_no_client_auth()
    );
    let server_name = "localhost".try_into().unwrap();
    ClientConnection::new(client_config, server_name).unwrap()
}

/// Implementation of [`ServerCertVerifier`] that skips verification.
///
/// This is only for use in tests. Do not use in production.
pub struct NoServerCertVerification;

impl ServerCertVerifier for NoServerCertVerification
{
    fn verify_server_cert(
        &self,
        _end_entity: &Certificate,
        _intermediates: &[Certificate],
        _server_name: &ServerName,
        _scts: &mut dyn Iterator<Item = &[u8]>,
        _ocsp_response: &[u8],
        _now: SystemTime,
    ) -> Result<ServerCertVerified, Error>
    {
        assert!(cfg!(test),
            "Do not use NoServerCertVerification in production!");
        Ok(ServerCertVerified::assertion())
    }
}
