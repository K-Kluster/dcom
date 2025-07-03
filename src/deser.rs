use borsh::BorshDeserialize;
use hex::{decode, encode};

use crate::{errors::ProtocolError, operations::Operation, result::Result};

const PROTOCOL_ENCODED_MESSAGE_PREFIX: &str = "64636f6d";

/**
 * The length of the encoded prefix in bytes.
 * dcom = 64636f6d in hex, is 8 bytes long.
 */
const PROTOCOL_ENCODED_MESSAGE_PREFIX_LENGTH: usize = PROTOCOL_ENCODED_MESSAGE_PREFIX.len();

pub fn encode_operation(operation: Operation) -> Result<Vec<u8>> {
    let encoded_operation = borsh::to_vec(&operation)?;
    let encoded_message = format!(
        "{}{}",
        PROTOCOL_ENCODED_MESSAGE_PREFIX,
        encode(encoded_operation)
    );
    Ok(encoded_message.into_bytes())
}

pub fn decode_operation(encoded_message: &[u8]) -> Result<Operation> {
    if encoded_message.len() <= PROTOCOL_ENCODED_MESSAGE_PREFIX_LENGTH {
        return Err(ProtocolError::PayloadDoesntContainPrefix());
    }

    let hex_payload = &encoded_message[PROTOCOL_ENCODED_MESSAGE_PREFIX_LENGTH..];
    let decoded_payload = decode(hex_payload)?;
    let operation_struct: Operation = BorshDeserialize::try_from_slice(&decoded_payload)?;
    Ok(operation_struct)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::operations::*;

    #[test]
    fn test_decode_empty_message() {
        assert!(decode_operation(&[]).is_err());
    }

    #[test]
    fn test_encode_decode_message() {
        let message = Message {
            operation_version: 1,
            target: "target".to_string(),
            content: "content".to_string(),
        };
        let operation = Operation::Message(message.clone());
        let encoded = encode_operation(operation).unwrap();
        let decoded = decode_operation(&encoded).unwrap();
        if let Operation::Message(decoded_message) = decoded {
            assert_eq!(decoded_message, message);
        } else {
            panic!("Expected Message operation");
        }
    }

    #[test]
    fn test_encode_decode_announcement() {
        let announcement = Announcement {
            operation_version: 1,
            paylaod: vec![1, 2, 3],
        };
        let operation = Operation::Announcement(announcement.clone());
        let encoded = encode_operation(operation).unwrap();
        let decoded = decode_operation(&encoded).unwrap();
        if let Operation::Announcement(decoded_announcement) = decoded {
            assert_eq!(decoded_announcement, announcement);
        } else {
            panic!("Expected Announcement operation");
        }
    }

    #[test]
    fn test_encode_decode_extension() {
        let extension = Extension {
            operation_version: 1,
            extension_version: 1,
            category: "category".to_string(),
            name: "name".to_string(),
            target: "target".to_string(),
            payload: vec![1, 2, 3],
        };
        let operation = Operation::Extension(extension.clone());
        let encoded = encode_operation(operation).unwrap();
        let decoded = decode_operation(&encoded).unwrap();
        if let Operation::Extension(decoded_extension) = decoded {
            assert_eq!(decoded_extension, extension);
        } else {
            panic!("Expected Extension operation");
        }
    }
}
