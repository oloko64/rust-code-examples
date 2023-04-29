use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonRpcRequest {
    jsonrpc: String,

    // https://serde.rs/attr-flatten.html
    #[serde(flatten)]
    method: Method,
}

// https://serde.rs/enum-representations.html#internally-tagged
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "method")]
enum Method {
    #[serde(rename = "hello")]
    Hello { params: HelloParams },

    #[serde(other)]
    NotImplemented,
}

#[derive(Debug, Deserialize, Serialize)]
struct HelloParams {
    hello: String,
    value: u32,
}
