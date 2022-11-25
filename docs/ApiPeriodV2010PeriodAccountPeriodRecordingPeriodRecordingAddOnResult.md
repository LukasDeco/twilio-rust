# ApiPeriodV2010PeriodAccountPeriodRecordingPeriodRecordingAddOnResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**sid** | Option<**String**> | The unique string that identifies the resource | [optional]
**account_sid** | Option<**String**> | The SID of the Account that created the resource | [optional]
**status** | Option<[**crate::models::RecordingAddOnResultEnumStatus**](recording_add_on_result_enum_status.md)> |  | [optional]
**add_on_sid** | Option<**String**> | The SID of the Add-on to which the result belongs | [optional]
**add_on_configuration_sid** | Option<**String**> | The SID of the Add-on configuration | [optional]
**date_created** | Option<**String**> | The RFC 2822 date and time in GMT that the resource was created | [optional]
**date_updated** | Option<**String**> | The RFC 2822 date and time in GMT that the resource was last updated | [optional]
**date_completed** | Option<**String**> | The date and time in GMT that the result was completed | [optional]
**reference_sid** | Option<**String**> | The SID of the recording to which the AddOnResult resource belongs | [optional]
**subresource_uris** | Option<[**serde_json::Value**](.md)> | A list of related resources identified by their relative URIs | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


