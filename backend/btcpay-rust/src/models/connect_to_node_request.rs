/*
 * BTCPay Greenfield API
 *
 * A full API to use your BTCPay Server
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ConnectToNodeRequest {
    /// Node URI in the form `pubkey@endpoint[:port]`
    #[serde(rename = "nodeURI", skip_serializing_if = "Option::is_none")]
    pub node_uri: Option<String>,
}

impl ConnectToNodeRequest {
    pub fn new() -> ConnectToNodeRequest {
        ConnectToNodeRequest {
            node_uri: None,
        }
    }
}

