use serde::{Deserialize, Serialize};

use crate::CoreError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LedgerEntry {
    pub seq: u64,
    pub digest: String,
    pub prev_digest: String,
    pub kind: String,
    pub payload_digest: String,
}

pub fn verify_ledger_head(entries: &[LedgerEntry], head: &str) -> Result<(), CoreError> {
    if entries.is_empty() {
        return Err(CoreError::AuditFailure("empty ledger".to_string()));
    }
    for idx in 1..entries.len() {
        if entries[idx].prev_digest != entries[idx - 1].digest {
            return Err(CoreError::AuditFailure(format!(
                "broken chain at seq {}",
                entries[idx].seq
            )));
        }
    }
    let last = entries.last().expect("non-empty");
    if last.digest != head {
        return Err(CoreError::AuditFailure(
            "head digest does not match ledger tip".to_string(),
        ));
    }
    Ok(())
}
