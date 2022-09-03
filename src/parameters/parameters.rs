use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum SMBParameters {
    Negotiate(Vec<String>)
}