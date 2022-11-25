# ApiPeriodV2010PeriodAccountPeriodUsagePeriodUsageRecordPeriodUsageRecordThisMonth

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_sid** | Option<**String**> | The SID of the Account accrued the usage | [optional]
**api_version** | Option<**String**> | The API version used to create the resource | [optional]
**as_of** | Option<**String**> | Usage records up to date as of this timestamp | [optional]
**category** | Option<[**crate::models::UsageRecordThisMonthEnumCategory**](usage_record_this_month_enum_category.md)> |  | [optional]
**count** | Option<**String**> | The number of usage events | [optional]
**count_unit** | Option<**String**> | The units in which count is measured | [optional]
**description** | Option<**String**> | A plain-language description of the usage category | [optional]
**end_date** | Option<[**String**](string.md)> | The last date for which usage is included in the UsageRecord | [optional]
**price** | Option<**f32**> | The total price of the usage | [optional]
**price_unit** | Option<**String**> | The currency in which `price` is measured | [optional]
**start_date** | Option<[**String**](string.md)> | The first date for which usage is included in this UsageRecord | [optional]
**subresource_uris** | Option<[**serde_json::Value**](.md)> | A list of related resources identified by their relative URIs | [optional]
**uri** | Option<**String**> | The URI of the resource, relative to `https://api.twilio.com` | [optional]
**usage** | Option<**String**> | The amount of usage | [optional]
**usage_unit** | Option<**String**> | The units in which usage is measured | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


