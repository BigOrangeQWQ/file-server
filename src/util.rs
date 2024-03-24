use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use chrono::{Days, Utc};

pub fn random_chars(len: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut rng = thread_rng();

    (0..len)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

pub fn next_day_time() -> i64 {
    let now = Utc::now();
    now.checked_add_days(Days::new(1)).unwrap().timestamp()
}

enum Status {
    Ok,
    Error,
}

impl Status {
    fn to_string(&self) -> String {
        match self {
            Status::Ok => "ok".to_string(),
            Status::Error => "error".to_string(),
        }
    }
}

impl Serialize for Status {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Status {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "ok" => Ok(Status::Ok),
            "error" => Ok(Status::Error),
            _ => Err(serde::de::Error::custom("unexpected status")),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct RespData {
    status: Status,
    data: Vec<String>,
}

impl RespData {
    fn new(status: Status, data: Vec<String>) -> Self {
        Self { status, data }
    }

    pub fn ok(data: Vec<String>) -> Self {
        Self::new(Status::Ok, data)
    }

    // pub fn error(data: Vec<String>) -> Self {
    //     Self::new(Status::Error, data)
    // }

    pub fn error_str(data: impl ToString) -> Self {
        Self::new(Status::Error, vec![data.to_string()])
    }
}
