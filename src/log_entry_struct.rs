use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Metadata {
    parentResourceId: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LogEntry {
    level: String,
    message: String,
    resourceId: String,
    timestamp: String,
    traceId: String,
    spanId: String,
    commit: String,
    metadata: Metadata,
}

