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
pub struct ApiPeriodV2010PeriodAccountPeriodConferencePeriodConferenceRecording {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<Option<String>>,
    /// The API version used to create the recording
    #[serde(rename = "api_version", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<Option<String>>,
    /// The SID of the Call the resource is associated with
    #[serde(rename = "call_sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub call_sid: Option<Option<String>>,
    /// The Conference SID that identifies the conference associated with the recording
    #[serde(rename = "conference_sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub conference_sid: Option<Option<String>>,
    /// The RFC 2822 date and time in GMT that the resource was created
    #[serde(rename = "date_created", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<Option<String>>,
    /// The RFC 2822 date and time in GMT that the resource was last updated
    #[serde(rename = "date_updated", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<Option<String>>,
    /// The start time of the recording, given in RFC 2822 format
    #[serde(rename = "start_time", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<Option<String>>,
    /// The length of the recording in seconds
    #[serde(rename = "duration", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub duration: Option<Option<String>>,
    /// The unique string that identifies the resource
    #[serde(rename = "sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sid: Option<Option<String>>,
    /// The one-time cost of creating the recording.
    #[serde(rename = "price", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub price: Option<Option<String>>,
    /// The currency used in the price property.
    #[serde(rename = "price_unit", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub price_unit: Option<Option<String>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::ConferenceRecordingEnumStatus>,
    /// The number of channels in the final recording file as an integer
    #[serde(rename = "channels", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub channels: Option<Option<i32>>,
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<crate::models::ConferenceRecordingEnumSource>,
    /// More information about why the recording is missing, if status is `absent`.
    #[serde(rename = "error_code", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub error_code: Option<Option<i32>>,
    /// How to decrypt the recording.
    #[serde(rename = "encryption_details", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub encryption_details: Option<Option<serde_json::Value>>,
    /// The URI of the resource, relative to `https://api.twilio.com`
    #[serde(rename = "uri", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub uri: Option<Option<String>>,
}

impl ApiPeriodV2010PeriodAccountPeriodConferencePeriodConferenceRecording {
    pub fn new() -> ApiPeriodV2010PeriodAccountPeriodConferencePeriodConferenceRecording {
        ApiPeriodV2010PeriodAccountPeriodConferencePeriodConferenceRecording {
            account_sid: None,
            api_version: None,
            call_sid: None,
            conference_sid: None,
            date_created: None,
            date_updated: None,
            start_time: None,
            duration: None,
            sid: None,
            price: None,
            price_unit: None,
            status: None,
            channels: None,
            source: None,
            error_code: None,
            encryption_details: None,
            uri: None,
        }
    }
}


