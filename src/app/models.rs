use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Question {
    qn: String,
    opt: [String; 4],
    ans: usize,
}
