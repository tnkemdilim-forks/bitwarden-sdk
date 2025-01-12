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
pub struct SendResponseModel {
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "accessId", skip_serializing_if = "Option::is_none")]
    pub access_id: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<crate::models::SendType>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "notes", skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "file", skip_serializing_if = "Option::is_none")]
    pub file: Option<Box<crate::models::SendFileModel>>,
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<Box<crate::models::SendTextModel>>,
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "maxAccessCount", skip_serializing_if = "Option::is_none")]
    pub max_access_count: Option<i32>,
    #[serde(rename = "accessCount", skip_serializing_if = "Option::is_none")]
    pub access_count: Option<i32>,
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "disabled", skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(rename = "revisionDate", skip_serializing_if = "Option::is_none")]
    pub revision_date: Option<String>,
    #[serde(rename = "expirationDate", skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    #[serde(rename = "deletionDate", skip_serializing_if = "Option::is_none")]
    pub deletion_date: Option<String>,
    #[serde(rename = "hideEmail", skip_serializing_if = "Option::is_none")]
    pub hide_email: Option<bool>,
}

impl SendResponseModel {
    pub fn new() -> SendResponseModel {
        SendResponseModel {
            object: None,
            id: None,
            access_id: None,
            _type: None,
            name: None,
            notes: None,
            file: None,
            text: None,
            key: None,
            max_access_count: None,
            access_count: None,
            password: None,
            disabled: None,
            revision_date: None,
            expiration_date: None,
            deletion_date: None,
            hide_email: None,
        }
    }
}
