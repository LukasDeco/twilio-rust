# ApiPeriodV2010PeriodAccountPeriodToken

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_sid** | Option<**String**> | The SID of the Account that created the resource | [optional]
**date_created** | Option<**String**> | The RFC 2822 date and time in GMT that the resource was created | [optional]
**date_updated** | Option<**String**> | The RFC 2822 date and time in GMT that the resource was last updated | [optional]
**ice_servers** | Option<[**Vec<crate::models::ApiV2010AccountTokenIceServersInner>**](api_v2010_account_token_ice_servers_inner.md)> | An array representing the ephemeral credentials | [optional]
**password** | Option<**String**> | The temporary password used for authenticating | [optional]
**ttl** | Option<**String**> | The duration in seconds the credentials are valid | [optional]
**username** | Option<**String**> | The temporary username that uniquely identifies a Token | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


