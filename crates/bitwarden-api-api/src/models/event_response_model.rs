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
pub struct EventResponseModel {
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<crate::models::EventType>,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "organizationId", skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    #[serde(rename = "providerId", skip_serializing_if = "Option::is_none")]
    pub provider_id: Option<String>,
    #[serde(rename = "cipherId", skip_serializing_if = "Option::is_none")]
    pub cipher_id: Option<String>,
    #[serde(rename = "collectionId", skip_serializing_if = "Option::is_none")]
    pub collection_id: Option<String>,
    #[serde(rename = "groupId", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(rename = "policyId", skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    #[serde(rename = "organizationUserId", skip_serializing_if = "Option::is_none")]
    pub organization_user_id: Option<String>,
    #[serde(rename = "providerUserId", skip_serializing_if = "Option::is_none")]
    pub provider_user_id: Option<String>,
    #[serde(
        rename = "providerOrganizationId",
        skip_serializing_if = "Option::is_none"
    )]
    pub provider_organization_id: Option<String>,
    #[serde(rename = "actingUserId", skip_serializing_if = "Option::is_none")]
    pub acting_user_id: Option<String>,
    #[serde(rename = "installationId", skip_serializing_if = "Option::is_none")]
    pub installation_id: Option<String>,
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(rename = "deviceType", skip_serializing_if = "Option::is_none")]
    pub device_type: Option<crate::models::DeviceType>,
    #[serde(rename = "ipAddress", skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(rename = "systemUser", skip_serializing_if = "Option::is_none")]
    pub system_user: Option<crate::models::EventSystemUser>,
    #[serde(rename = "domainName", skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
}

impl EventResponseModel {
    pub fn new() -> EventResponseModel {
        EventResponseModel {
            object: None,
            _type: None,
            user_id: None,
            organization_id: None,
            provider_id: None,
            cipher_id: None,
            collection_id: None,
            group_id: None,
            policy_id: None,
            organization_user_id: None,
            provider_user_id: None,
            provider_organization_id: None,
            acting_user_id: None,
            installation_id: None,
            date: None,
            device_type: None,
            ip_address: None,
            system_user: None,
            domain_name: None,
        }
    }
}