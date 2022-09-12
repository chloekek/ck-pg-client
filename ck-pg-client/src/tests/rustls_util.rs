use {
    rustls::{
        Certificate,
        ClientConfig,
        Error,
        ServerName,
        client::{ServerCertVerified, ServerCertVerifier},
    },
    std::{sync::Arc, time::SystemTime},
};

/// Create Rustls client configuration.
pub fn rustls_config() -> Arc<ClientConfig>
{
    let server_cert_verifier = Arc::new(NoServerCertVerification);
    Arc::new(
        ClientConfig::builder()
        .with_safe_defaults()
        .with_custom_certificate_verifier(server_cert_verifier)
        .with_no_client_auth()
    )
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
