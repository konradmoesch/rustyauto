pub trait Payload {
    fn as_slice(&self) -> &[u8];
    fn from_slice(bytes: &[u8]) -> Self where Self: Sized;
}

struct PlainPayload {
    bytes: Vec<u8>,
}

impl PlainPayload {
    fn as_vec(&self) -> Vec<u8> {
        let bytes_to_return = self.bytes.to_vec();
        return bytes_to_return;
    }
    fn get_message_id(&self) -> u16 {
        return u16::from_be_bytes([self.bytes.as_slice()[0], self.bytes.as_slice()[1]]);
    }
    fn get_payload(&self) -> &[u8] {
        return &self.bytes.as_slice()[2..];
    }
    fn from_parts(message_id: u16, payload: &[u8]) -> Self {
        let mut vector = vec![message_id.to_be_bytes()[0], message_id.to_be_bytes()[1]];
        vector.extend_from_slice(payload);
        Self {
            bytes: vector
        }
    }
}

impl Payload for PlainPayload {
    fn as_slice(&self) -> &[u8] {
        return self.bytes.as_slice();
    }

    fn from_slice(bytes: &[u8]) -> Self {
        Self {
            bytes: bytes.to_vec(),
        }
    }
}

struct EncryptedPayload {
    encrypted_bytes: Vec<u8>,
}

impl Payload for EncryptedPayload {
    fn as_slice(&self) -> &[u8] {
        self.encrypted_bytes.as_slice()
    }

    fn from_slice(bytes: &[u8]) -> Self {
        Self {
            encrypted_bytes: bytes.to_vec()
        }
    }
}
