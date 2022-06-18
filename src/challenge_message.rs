use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum ChallengeName {
    HashCash,
    RecoverSecret,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum ChallengeOutput {
    HashCash(String),
    RecoverSecret(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum ChallengeAnswer {
    ChallengeName(ChallengeOutput),
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ChallengeResult {
    pub(crate) name: ChallengeAnswer,
    pub(crate) next_target: String,
}
