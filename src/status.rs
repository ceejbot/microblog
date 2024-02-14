//! Our microblog status structure and useful functions for it.

use chrono::{DateTime, Utc};
use nanoid::nanoid;
use serde::Serialize;

/// Microblogs are quite small. Surely we'll never need additional fields.
pub struct StatusRecord {
    id: String,
    body: String,
    created: DateTime<Utc>,
    modified: DateTime<Utc>,
    deleted: Option<DateTime<Utc>>,
}

impl StatusRecord {
    /// Create a microblog status update given a body.
    pub fn new(body: String) -> Self {
        let id = nanoid!();
        let now: DateTime<Utc> = Utc::now();
        Self {
            id,
            body,
            created: now.clone(),
            modified: now,
            deleted: None,
        }
    }
}

pub struct StatusPublic {
    pub id: String,
    pub body: String,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
}

// This is the implementation of a standard conversion trait that lets us
// go from a Status struct to the StatusPublic struct (and vice versa!).
impl From<StatusRecord> for StatusPublic {
    fn from(value: StatusRecord) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn synchronous_test() {
        assert!(true, "true is truthful; write more tests");
    }

    #[tokio::test]
    async fn asynchronous_test() {
        assert!(!false, "false is false; write more tests");
    }
}
