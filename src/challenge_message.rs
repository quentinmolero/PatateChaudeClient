use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub(crate) struct MD5HashCashInput {
    pub(crate) complexity: u32,
    pub(crate) message: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub(crate) struct MD5HashCashOutput {
    pub(crate) seed: u64,
    pub(crate) hashcode: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub(crate) struct RecoverSecretInput {
    pub(crate) word_count: usize,
    pub(crate) letters: String,
    pub(crate) tuple_sizes: Vec<usize>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub(crate) struct RecoverSecretOutput {
    pub(crate) secret_sentence: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub(crate) struct MonstrousMazeInput {
    pub grid: String,
    pub endurance: u8,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub(crate) struct MonstrousMazeOutput {
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub(crate) enum Challenge {
    MD5HashCash(MD5HashCashInput),
    RecoverSecret(RecoverSecretInput),
    MonstrousMaze(MonstrousMazeInput),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub(crate) enum ChallengeOutput {
    MD5HashCash(MD5HashCashOutput),
    RecoverSecret(RecoverSecretOutput),
    MonstrousMaze(MonstrousMazeOutput),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub(crate) struct ChallengeResult {
    pub(crate) answer: ChallengeOutput,
    pub(crate) next_target: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub(crate) struct ChallengeValueResult {
    pub(crate) used_time: f64,
    pub(crate) next_target: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub(crate) enum ChallengeValue {
    Unreachable,
    Timeout,
    BadResult(ChallengeValueResult),
    Ok(ChallengeValueResult),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub(crate) struct ReportedChallengeResult {
    pub(crate) name: String,
    pub(crate) value: ChallengeValue,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub(crate) enum ChallengeMessage {
    ChallengeResult(ChallengeResult),
}
