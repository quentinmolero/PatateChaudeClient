use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Welcome {
    pub(crate) version: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum SubscriptionError {
    AlreadyRegistered,
    InvalidName
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct SubscribeError {
    pub(crate) subscription_error: SubscriptionError,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum Result {
    Ok,
    SubscribeError(SubscribeError),
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct SubscribeResult {
    pub(crate) result: Result,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct PublicPlayer {
    pub(crate) name: String,
    pub(crate) stream_id: String,
    pub(crate) score: i32,
    pub(crate) steps: u32,
    pub(crate) is_active: bool,
    pub(crate) total_used_time: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Challenge {
    pub(crate) challenge: String,
    pub(crate) chain: Vec<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum ServerMessage {
    Welcome(Welcome),
    SubscribeResult(SubscribeResult),
    PublicLeaderBoard(Vec<PublicPlayer>),
    Challenge(Challenge),
}
