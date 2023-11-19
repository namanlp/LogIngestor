use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Metadata {
    parentResourceId: String,
}

impl Metadata{
    pub fn into_tuple(self) -> String{
        self.parentResourceId
    }
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


impl LogEntry {
    // Method to return a tuple of all fields
    pub fn into_tuple(self) -> (String, String, String, String, String, String, String, String) {
        (
            self.level,
            self.message,
            self.resourceId,
            self.timestamp,
            self.traceId,
            self.spanId,
            self.commit,
            self.metadata.parentResourceId
        )
    }
}
