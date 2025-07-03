use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProtocolError {
    #[error("Invalid hex string")]
    InvalidHexString(#[from] hex::FromHexError),
    #[error("Payload doesn't contain prefix")]
    PayloadDoesntContainPrefix(),
    
    #[error("Borsh serialization error: {0}")]
    BorshSerializationError(#[from] borsh::io::Error),
}
