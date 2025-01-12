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
pub struct OrganizationUserBulkConfirmRequestModelEntry {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "key")]
    pub key: String,
}

impl OrganizationUserBulkConfirmRequestModelEntry {
    pub fn new(id: String, key: String) -> OrganizationUserBulkConfirmRequestModelEntry {
        OrganizationUserBulkConfirmRequestModelEntry { id, key }
    }
}
