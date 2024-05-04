use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_json::Value;

// Define a struct to hold metadata about the incoming request
#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    pub method: String,     // HTTP method (GET, POST, etc.)
    pub path: String,       // Request path
    pub query: Value,       // Query string parameters
    pub req_body: String,   // Path to the request body file
    pub res_body: String,   // Path to the response body file
    pub headers: HashMap<String, String>, // Request headers
    pub state: Value,       // State information (initially empty)
}