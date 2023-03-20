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
pub struct SecretCreateRequestModel {
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "note")]
    pub note: String,
    #[serde(rename = "projectIds", skip_serializing_if = "Option::is_none")]
    pub project_ids: Option<Vec<String>>,
}

impl SecretCreateRequestModel {
    pub fn new(key: String, value: String, note: String) -> SecretCreateRequestModel {
        SecretCreateRequestModel {
            key,
            value,
            note,
            project_ids: None,
        }
    }
}