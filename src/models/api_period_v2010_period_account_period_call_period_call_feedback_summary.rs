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
pub struct ApiPeriodV2010PeriodAccountPeriodCallPeriodCallFeedbackSummary {
    /// The unique sid that identifies this account
    #[serde(rename = "account_sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<Option<String>>,
    /// The total number of calls
    #[serde(rename = "call_count", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub call_count: Option<Option<i32>>,
    /// The total number of calls with a feedback entry
    #[serde(rename = "call_feedback_count", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub call_feedback_count: Option<Option<i32>>,
    /// The date this resource was created
    #[serde(rename = "date_created", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<Option<String>>,
    /// The date this resource was last updated
    #[serde(rename = "date_updated", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<Option<String>>,
    /// The latest feedback entry date in the summary
    #[serde(rename = "end_date", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<Option<String>>,
    /// Whether the feedback summary includes subaccounts
    #[serde(rename = "include_subaccounts", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub include_subaccounts: Option<Option<bool>>,
    /// Issues experienced during the call
    #[serde(rename = "issues", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub issues: Option<Option<Vec<serde_json::Value>>>,
    /// The average QualityScore of the feedback entries
    #[serde(rename = "quality_score_average", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub quality_score_average: Option<Option<f32>>,
    /// The median QualityScore of the feedback entries
    #[serde(rename = "quality_score_median", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub quality_score_median: Option<Option<f32>>,
    /// The standard deviation of the quality scores
    #[serde(rename = "quality_score_standard_deviation", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub quality_score_standard_deviation: Option<Option<f32>>,
    /// A string that uniquely identifies this feedback entry
    #[serde(rename = "sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sid: Option<Option<String>>,
    /// The earliest feedback entry date in the summary
    #[serde(rename = "start_date", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<Option<String>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::CallFeedbackSummaryEnumStatus>,
}

impl ApiPeriodV2010PeriodAccountPeriodCallPeriodCallFeedbackSummary {
    pub fn new() -> ApiPeriodV2010PeriodAccountPeriodCallPeriodCallFeedbackSummary {
        ApiPeriodV2010PeriodAccountPeriodCallPeriodCallFeedbackSummary {
            account_sid: None,
            call_count: None,
            call_feedback_count: None,
            date_created: None,
            date_updated: None,
            end_date: None,
            include_subaccounts: None,
            issues: None,
            quality_score_average: None,
            quality_score_median: None,
            quality_score_standard_deviation: None,
            sid: None,
            start_date: None,
            status: None,
        }
    }
}

