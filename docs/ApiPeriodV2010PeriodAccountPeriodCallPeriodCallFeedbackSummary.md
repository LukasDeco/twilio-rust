# ApiPeriodV2010PeriodAccountPeriodCallPeriodCallFeedbackSummary

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_sid** | Option<**String**> | The unique sid that identifies this account | [optional]
**call_count** | Option<**i32**> | The total number of calls | [optional]
**call_feedback_count** | Option<**i32**> | The total number of calls with a feedback entry | [optional]
**date_created** | Option<**String**> | The date this resource was created | [optional]
**date_updated** | Option<**String**> | The date this resource was last updated | [optional]
**end_date** | Option<[**String**](string.md)> | The latest feedback entry date in the summary | [optional]
**include_subaccounts** | Option<**bool**> | Whether the feedback summary includes subaccounts | [optional]
**issues** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | Issues experienced during the call | [optional]
**quality_score_average** | Option<**f32**> | The average QualityScore of the feedback entries | [optional]
**quality_score_median** | Option<**f32**> | The median QualityScore of the feedback entries | [optional]
**quality_score_standard_deviation** | Option<**f32**> | The standard deviation of the quality scores | [optional]
**sid** | Option<**String**> | A string that uniquely identifies this feedback entry | [optional]
**start_date** | Option<[**String**](string.md)> | The earliest feedback entry date in the summary | [optional]
**status** | Option<[**crate::models::CallFeedbackSummaryEnumStatus**](call_feedback_summary_enum_status.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


