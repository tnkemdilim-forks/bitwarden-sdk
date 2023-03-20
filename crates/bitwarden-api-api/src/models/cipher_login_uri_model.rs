/*
 * Bitwarden Internal API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: latest
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CipherLoginUriModel {
    #[serde(rename = "uri", skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    pub _match: Option<crate::models::UriMatchType>,
}

impl CipherLoginUriModel {
    pub fn new() -> CipherLoginUriModel {
        CipherLoginUriModel {
            uri: None,
            _match: None,
        }
    }
}