use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::sdk::auth::response::two_factor_login_response::{
    authenticator::Authenticator, duo::Duo, email::Email, remember::Remember, web_authn::WebAuthn,
    yubi_key::YubiKey,
};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TwoFactorProviders {
    pub authenticator: Option<Authenticator>,
    /// Email 2fa
    pub email: Option<Email>,
    /// Duo-backed 2fa
    pub duo: Option<Duo>,
    /// Duo-backed 2fa operated by an organization the user is a member of
    pub organization_duo: Option<Duo>,
    /// Yubikey-backed 2fa
    pub yubi_key: Option<YubiKey>,
    /// Presence indicates the user has stored this device as bypassing 2fa
    pub remember: Option<Remember>,
    /// WebAuthn-backed 2fa
    pub web_authn: Option<WebAuthn>,
}

impl From<crate::api::response::TwoFactorProviders> for TwoFactorProviders {
    fn from(api: crate::api::response::TwoFactorProviders) -> Self {
        Self {
            authenticator: api.authenticator.map(Into::into),
            email: api.email.map(Into::into),
            duo: api.duo.map(Into::into),
            organization_duo: api.organization_duo.map(Into::into),
            yubi_key: api.yubi_key.map(Into::into),
            remember: api.remember.map(Into::into),
            web_authn: api.web_authn.map(Into::into),
        }
    }
}
