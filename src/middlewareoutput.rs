use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::collections::HashMap;

// Define a struct to hold the output of middleware execution
#[derive(Serialize, Deserialize, Debug)]
pub struct MiddlewareOutput {
    pub status: u16,        // HTTP status code
    pub headers: HashMap<String, String>, // Response headers
    pub state: Value,       // Updated state information
    pub terminate: bool,    // Flag to indicate if middleware execution should terminate
}