/*
 * Twilio - Api
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.37.3
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ApiPeriodV2010PeriodAccountPeriodSipPeriodSipDomainPeriodSipAuthPeriodSipAuthCallsPeriodSipAuthCallsIpAccessControlListMapping {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<Option<String>>,
    /// The RFC 2822 date and time in GMT that the resource was created
    #[serde(rename = "date_created", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<Option<String>>,
    /// The RFC 2822 date and time in GMT that the resource was last updated
    #[serde(rename = "date_updated", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<Option<String>>,
    /// The string that you assigned to describe the resource
    #[serde(rename = "friendly_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<Option<String>>,
    /// The unique string that identifies the resource
    #[serde(rename = "sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sid: Option<Option<String>>,
}

impl ApiPeriodV2010PeriodAccountPeriodSipPeriodSipDomainPeriodSipAuthPeriodSipAuthCallsPeriodSipAuthCallsIpAccessControlListMapping {
    pub fn new() -> ApiPeriodV2010PeriodAccountPeriodSipPeriodSipDomainPeriodSipAuthPeriodSipAuthCallsPeriodSipAuthCallsIpAccessControlListMapping {
        ApiPeriodV2010PeriodAccountPeriodSipPeriodSipDomainPeriodSipAuthPeriodSipAuthCallsPeriodSipAuthCallsIpAccessControlListMapping {
            account_sid: None,
            date_created: None,
            date_updated: None,
            friendly_name: None,
            sid: None,
        }
    }
}


