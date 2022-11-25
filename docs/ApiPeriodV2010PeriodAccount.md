# ApiPeriodV2010PeriodAccount

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**auth_token** | Option<**String**> | The authorization token for this account | [optional]
**date_created** | Option<**String**> | The date this account was created | [optional]
**date_updated** | Option<**String**> | The date this account was last updated | [optional]
**friendly_name** | Option<**String**> | A human readable description of this account | [optional]
**owner_account_sid** | Option<**String**> | The unique 34 character id representing the parent of this account | [optional]
**sid** | Option<**String**> | A 34 character string that uniquely identifies this resource. | [optional]
**status** | Option<[**crate::models::AccountEnumStatus**](account_enum_status.md)> |  | [optional]
**subresource_uris** | Option<[**serde_json::Value**](.md)> | Account Instance Subresources | [optional]
**r#type** | Option<[**crate::models::AccountEnumType**](account_enum_type.md)> |  | [optional]
**uri** | Option<**String**> | The URI for this resource, relative to `https://api.twilio.com` | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


