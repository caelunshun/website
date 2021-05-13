use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct PushAction {
    pub r#ref: String,
}
