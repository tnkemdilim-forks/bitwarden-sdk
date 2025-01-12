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
pub struct AuthRequestResponseModel {
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "publicKey", skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    #[serde(rename = "requestDeviceType", skip_serializing_if = "Option::is_none")]
    pub request_device_type: Option<String>,
    #[serde(rename = "requestIpAddress", skip_serializing_if = "Option::is_none")]
    pub request_ip_address: Option<String>,
    #[serde(rename = "requestFingerprint", skip_serializing_if = "Option::is_none")]
    pub request_fingerprint: Option<String>,
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "masterPasswordHash", skip_serializing_if = "Option::is_none")]
    pub master_password_hash: Option<String>,
    #[serde(rename = "creationDate", skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "responseDate", skip_serializing_if = "Option::is_none")]
    pub response_date: Option<String>,
    #[serde(rename = "requestApproved", skip_serializing_if = "Option::is_none")]
    pub request_approved: Option<bool>,
    #[serde(rename = "origin", skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}

impl AuthRequestResponseModel {
    pub fn new() -> AuthRequestResponseModel {
        AuthRequestResponseModel {
            object: None,
            id: None,
            public_key: None,
            request_device_type: None,
            request_ip_address: None,
            request_fingerprint: None,
            key: None,
            master_password_hash: None,
            creation_date: None,
            response_date: None,
            request_approved: None,
            origin: None,
        }
    }
}
