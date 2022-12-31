use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Question {
    pub qno: usize,
    pub qn: String,
    pub opt: [String; 4],
    pub ans: usize,
}
