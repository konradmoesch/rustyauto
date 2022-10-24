use std::io::Cursor;

use openssl::ssl::{SslContextBuilder, SslMethod};

use crate::messenger::message::EncryptionType;

pub struct Cryptor {
    ssl_context: openssl::ssl::SslContext,
    ssl_stream: openssl::ssl::SslStream<Cursor<Vec<u8>>>,
    old_position: usize,
}

impl Cryptor {
    pub fn init() -> Self {
        log::info!("Initializing cryptor");
        log::info!("using openssl version {}", openssl::version::version());
        openssl::init();
        let cert = openssl::x509::X509::from_pem(CERTIFICATE.as_bytes()).unwrap();
        let pkey = openssl::pkey::PKey::private_key_from_pem(PRIVATE_KEY.as_bytes()).unwrap();
        let method = SslMethod::tls_client();
        let mut ssl_context_builder = SslContextBuilder::new(method).unwrap();
        ssl_context_builder.set_certificate(&cert).expect("Setting certificate failed");
        ssl_context_builder.set_private_key(&pkey).expect("Setting privkey failed");
        let ssl_context = ssl_context_builder.build();
        let mut ssl = openssl::ssl::Ssl::new(&ssl_context).unwrap();
        /*let read_bio = openssl_sys::BIO_new(openssl_sys::BIO_s_mem());
        let write_bio = openssl_sys::BIO_new(openssl_sys::BIO_s_mem());
        let bios = (write_bio, read_bio);*/
        ssl.set_connect_state();
        dbg!(ssl.version_str());
        ssl.set_verify(openssl::ssl::SslVerifyMode::NONE);
        let buff = Cursor::new(Vec::<u8>::new());
        let ssl_stream = openssl::ssl::SslStream::new(ssl, buff).unwrap();

        Self { ssl_context, ssl_stream, old_position: 0 }
    }

    pub fn do_handshake(&mut self) {
        log::info!("Doing SSL handshake");
        match self.ssl_stream.do_handshake() {
            Ok(_) => { log::info!("Successfully did handshake"); }
            Err(e) => {
                log::error!("ssl handshake error: {:?}", e);
            }
        };
    }

    pub fn read_handshake_buffer(&mut self) -> Vec<u8> {
        log::info!("Reading handshake buffer");
        let memory_stream_ref = self.ssl_stream.get_ref();
        log::debug!("{:?}", memory_stream_ref);
        //vector.extend_from_slice(&memory_stream_ref.data.as_slices().0);
        //let tcp_stream = tcp_stream_ref.try_clone().unwrap();

        //self.ssl_stream.read(vector.as_mut_slice()).expect("unable to read from stream");
        //self.ssl_stream.read(&mut buffer[..]).expect("unable to read from stream");
        //vector
        let value = &memory_stream_ref.get_ref()[self.old_position..];
        log::error!("{:?}",value.to_vec());
        let to_return = memory_stream_ref.get_ref().clone();
        let to_return = value.to_vec();
        to_return
    }

    pub fn write_handshake_buffer(&mut self, to_write: &[u8]) {
        log::info!("Writing handshake buffer");

        let mut memory_stream_ref = self.ssl_stream.get_mut();
        &memory_stream_ref.get_mut().clear();
        memory_stream_ref.set_position(0);
        &memory_stream_ref.get_mut().extend(to_write);
        log::info!("{:?}", &memory_stream_ref);
        self.old_position = to_write.len();
    }

    pub fn decrypt_buffer(&mut self, to_write: &[u8]) -> Vec<u8> {
        //log::debug!("Decrypting payload {:?}", to_write);
        let mut memory_stream_ref = self.ssl_stream.get_mut();
        &memory_stream_ref.get_mut().clear();
        memory_stream_ref.set_position(0);
        &memory_stream_ref.get_mut().extend(to_write);

        let mut buffer = vec![0u8; 1000];
        let size = self.ssl_stream.ssl_read(buffer.as_mut_slice()).expect("Failed to ssl stream");
        //log::debug!("Decrypted size: {}", size);
        buffer.as_slice()[0..size].to_vec()
    }

    pub fn encrypt_buffer(&mut self, to_encrypt: &[u8]) -> Vec<u8> {
        //log::debug!("Encrypting payload {:?}", to_encrypt);
        let mut memory_stream_ref = self.ssl_stream.get_mut();
        &memory_stream_ref.get_mut().clear();
        memory_stream_ref.set_position(0);
        &memory_stream_ref.get_mut().extend(to_encrypt);

        let size = self.ssl_stream.ssl_write(to_encrypt).expect("Failed to ssl stream");
        //log::debug!("Encrypted size: {}", size);
        self.ssl_stream.get_ref().get_ref().clone()
    }

    pub fn encrypt_message(&mut self, message_to_encrypt: &mut crate::messenger::message::Message) {
        match message_to_encrypt.frame_header.encryption_type {
            EncryptionType::Plain => log::warn!("Message is plain text, nothing to do!"),
            EncryptionType::Encrypted => {
                //log::debug!("Encrypting message");
                message_to_encrypt.payload = self.encrypt_buffer(message_to_encrypt.payload.as_slice());
                //log::debug!("Encrypted: {:?}", message_to_encrypt);
            }
        }
    }

    pub fn decrypt_message(&mut self, message_to_decrypt: &mut crate::messenger::message::Message) {
        match message_to_decrypt.frame_header.encryption_type {
            EncryptionType::Plain => log::warn!("Message is plain text, nothing to do!"),
            EncryptionType::Encrypted => {
                //log::debug!("Decrypting message");
                message_to_decrypt.payload = self.decrypt_buffer(message_to_decrypt.payload.as_slice());
                //log::debug!("Decrypted: {:?}", message_to_decrypt);
            }
        }
    }
}

const CERTIFICATE: &str = "-----BEGIN CERTIFICATE-----\n\
MIIDKjCCAhICARswDQYJKoZIhvcNAQELBQAwWzELMAkGA1UEBhMCVVMxEzARBgNV\n\
BAgMCkNhbGlmb3JuaWExFjAUBgNVBAcMDU1vdW50YWluIFZpZXcxHzAdBgNVBAoM\n\
Fkdvb2dsZSBBdXRvbW90aXZlIExpbmswJhcRMTQwNzA0MDAwMDAwLTA3MDAXETQ1\n\
MDQyOTE0MjgzOC0wNzAwMFMxCzAJBgNVBAYTAkpQMQ4wDAYDVQQIDAVUb2t5bzER\n\
MA8GA1UEBwwISGFjaGlvamkxFDASBgNVBAoMC0pWQyBLZW53b29kMQswCQYDVQQL\n\
DAIwMTCCASIwDQYJKoZIhvcNAQEBBQADggEPADCCAQoCggEBAM911mNnUfx+WJtx\n\
uk06GO7kXRW/gXUVNQBkbAFZmVdVNvLoEQNthi2X8WCOwX6n6oMPxU2MGJnvicP3\n\
6kBqfHhfQ2Fvqlf7YjjhgBHh0lqKShVPxIvdatBjVQ76aym5H3GpkigLGkmeyiVo\n\
VO8oc3cJ1bO96wFRmk7kJbYcEjQyakODPDu4QgWUTwp1Z8Dn41ARMG5OFh6otITL\n\
XBzj9REkUPkxfS03dBXGr5/LIqvSsnxib1hJ47xnYJXROUsBy3e6T+fYZEEzZa7y\n\
7tFioHIQ8G/TziPmvFzmQpaWMGiYfoIgX8WoR3GD1diYW+wBaZTW+4SFUZJmRKgq\n\
TbMNFkMCAwEAATANBgkqhkiG9w0BAQsFAAOCAQEAsGdH5VFn78WsBElMXaMziqFC\n\
zmilkvr85/QpGCIztI0FdF6xyMBJk/gYs2thwvF+tCCpXoO8mjgJuvJZlwr6fHzK\n\
Ox5hNUb06AeMtsUzUfFjSZXKrSR+XmclVd+Z6/ie33VhGePOPTKYmJ/PPfTT9wvT\n\
93qswcxhA+oX5yqLbU3uDPF1ZnJaEeD/YN45K/4eEA4/0SDXaWW14OScdS2LV0Bc\n\
YmsbkPVNYZn37FlY7e2Z4FUphh0A7yME2Eh/e57QxWrJ1wubdzGnX8mrABc67ADU\n\
U5r9tlTRqMs7FGOk6QS2Cxp4pqeVQsrPts4OEwyPUyb3LfFNo3+sP111D9zEow==\n\
-----END CERTIFICATE-----\n";

const PRIVATE_KEY: &str = "-----BEGIN RSA PRIVATE KEY-----\n\
MIIEowIBAAKCAQEAz3XWY2dR/H5Ym3G6TToY7uRdFb+BdRU1AGRsAVmZV1U28ugR\n\
A22GLZfxYI7Bfqfqgw/FTYwYme+Jw/fqQGp8eF9DYW+qV/tiOOGAEeHSWopKFU/E\n\
i91q0GNVDvprKbkfcamSKAsaSZ7KJWhU7yhzdwnVs73rAVGaTuQlthwSNDJqQ4M8\n\
O7hCBZRPCnVnwOfjUBEwbk4WHqi0hMtcHOP1ESRQ+TF9LTd0Fcavn8siq9KyfGJv\n\
WEnjvGdgldE5SwHLd7pP59hkQTNlrvLu0WKgchDwb9POI+a8XOZClpYwaJh+giBf\n\
xahHcYPV2Jhb7AFplNb7hIVRkmZEqCpNsw0WQwIDAQABAoIBAB2u7ZLheKCY71Km\n\
bhKYqnKb6BmxgfNfqmq4858p07/kKG2O+Mg1xooFgHrhUhwuKGbCPee/kNGNrXeF\n\
pFW9JrwOXVS2pnfaNw6ObUWhuvhLaxgrhqLAdoUEgWoYOHcKzs3zhj8Gf6di+edq\n\
SyTA8+xnUtVZ6iMRKvP4vtCUqaIgBnXdmQbGINP+/4Qhb5R7XzMt/xPe6uMyAIyC\n\
y5Fm9HnvekaepaeFEf3bh4NV1iN/R8px6cFc6ELYxIZc/4Xbm91WGqSdB0iSriaZ\n\
TjgrmaFjSO40tkCaxI9N6DGzJpmpnMn07ifhl2VjnGOYwtyuh6MKEnyLqTrTg9x0\n\
i3mMwskCgYEA9IyljPRerXxHUAJt+cKOayuXyNt80q9PIcGbyRNvn7qIY6tr5ut+\n\
ZbaFgfgHdSJ/4nICRq02HpeDJ8oj9BmhTAhcX6c1irH5ICjRlt40qbPwemIcpybt\n\
mb+DoNYbI8O4dUNGH9IPfGK8dRpOok2m+ftfk94GmykWbZF5CnOKIp8CgYEA2Syc\n\
5xlKB5Qk2ZkwXIzxbzozSfunHhWWdg4lAbyInwa6Y5GB35UNdNWI8TAKZsN2fKvX\n\
RFgCjbPreUbREJaM3oZ92o5X4nFxgjvAE1tyRqcPVbdKbYZgtcqqJX06sW/g3r/3\n\
RH0XPj2SgJIHew9sMzjGWDViMHXLmntI8rVA7d0CgYBOr36JFwvrqERN0ypNpbMr\n\
epBRGYZVSAEfLGuSzEUrUNqXr019tKIr2gmlIwhLQTmCxApFcXArcbbKs7jTzvde\n\
PoZyZJvOr6soFNozP/YT8Ijc5/quMdFbmgqhUqLS5CPS3z2N+YnwDNj0mO1aPcAP\n\
STmcm2DmxdaolJksqrZ0owKBgQCD0KJDWoQmaXKcaHCEHEAGhMrQot/iULQMX7Vy\n\
gl5iN5E2EgFEFZIfUeRWkBQgH49xSFPWdZzHKWdJKwSGDvrdrcABwdfx520/4MhK\n\
d3y7CXczTZbtN1zHuoTfUE0pmYBhcx7AATT0YCblxrynosrHpDQvIefBBh5YW3AB\n\
cKZCOQKBgEM/ixzI/OVSZ0Py2g+XV8+uGQyC5XjQ6cxkVTX3Gs0ZXbemgUOnX8co\n\
eCXS4VrhEf4/HYMWP7GB5MFUOEVtlLiLM05ruUL7CrphdfgayDXVcTPfk75lLhmu\n\
KAwp3tIHPoJOQiKNQ3/qks5km/9dujUGU2ARiU3qmxLMdgegFz8e\n\
-----END RSA PRIVATE KEY-----\n";
