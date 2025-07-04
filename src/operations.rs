use borsh::{BorshDeserialize, BorshSerialize};

/**
 * Challenges (and todos):
 * 1. Need to support both encrypted and unencrypted messages on operations.
 *    not sure what to do yet, can be sets in advance as a field (encrypted: true|false) or is it guessable at runtime?
 * 2. I should be able to send a copy to myself or not, when i send a copy to myself, it is to be retreived on other devices
 *      => is it included in the same operation or is it a separate operation?
 * 3. All operation except annoucements (? to be confirmed) can either be included in a transaction:
 *     * send to myself
 *     * send to a particular address
 */

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub enum Target {
    // opened targets
    Everyone,
    PubKey(Vec<u8>),
    Group(Vec<u8>),
    // hidden targets, agnostic to the plurality of the target (goup or not)
    Alias(Vec<u8>),
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct Message {
    pub operation_version: u8,
    pub target: Target,
    // message payload can be sealed or not (sealed = encrypted)
    pub payload: Vec<u8>,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
/**
 * Formely "Handshake"
 */
pub struct Announcement {
    pub operation_version: u8,
    // announcement payload can be sealed or not (sealed = encrypted)
    pub paylaod: Vec<u8>,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
/**
 * Extension is a way to extend the protocol with new features
 *   => Payment is now an extension (not a mandatory operation to implement)
 */
pub struct Extension {
    // allow extension message shape to change over time
    pub operation_version: u8,
    // allow extension payload shape to change over time
    pub extension_version: u8,
    pub category: String,
    pub name: String,
    pub target: Target,
    // extension payload can be sealed or not (sealed = encrypted)
    pub payload: Vec<u8>,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum Operation {
    Message(Message),
    Announcement(Announcement),
    Extension(Extension),
}

impl PartialEq for Operation {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Operation::Message(a), Operation::Message(b)) => a == b,
            (Operation::Announcement(a), Operation::Announcement(b)) => a == b,
            (Operation::Extension(a), Operation::Extension(b)) => a == b,
            _ => false,
        }
    }
}
