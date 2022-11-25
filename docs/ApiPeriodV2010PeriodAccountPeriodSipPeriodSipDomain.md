# ApiPeriodV2010PeriodAccountPeriodSipPeriodSipDomain

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_sid** | Option<**String**> | The SID of the Account that created the resource | [optional]
**api_version** | Option<**String**> | The API version used to process the call | [optional]
**auth_type** | Option<**String**> | The types of authentication mapped to the domain | [optional]
**date_created** | Option<**String**> | The RFC 2822 date and time in GMT that the resource was created | [optional]
**date_updated** | Option<**String**> | The RFC 2822 date and time in GMT that the resource was last updated | [optional]
**domain_name** | Option<**String**> | The unique address on Twilio to route SIP traffic | [optional]
**friendly_name** | Option<**String**> | The string that you assigned to describe the resource | [optional]
**sid** | Option<**String**> | The unique string that identifies the resource | [optional]
**uri** | Option<**String**> | The URI of the resource, relative to `https://api.twilio.com` | [optional]
**voice_fallback_method** | Option<**String**> | The HTTP method used with voice_fallback_url | [optional]
**voice_fallback_url** | Option<**String**> | The URL we call when an error occurs while executing TwiML | [optional]
**voice_method** | Option<**String**> | The HTTP method to use with voice_url | [optional]
**voice_status_callback_method** | Option<**String**> | The HTTP method we use to call voice_status_callback_url | [optional]
**voice_status_callback_url** | Option<**String**> | The URL that we call with status updates | [optional]
**voice_url** | Option<**String**> | The URL we call when receiving a call | [optional]
**subresource_uris** | Option<[**serde_json::Value**](.md)> | A list mapping resources associated with the SIP Domain resource | [optional]
**sip_registration** | Option<**bool**> | Whether SIP registration is allowed | [optional]
**emergency_calling_enabled** | Option<**bool**> | Whether emergency calling is enabled for the domain. | [optional]
**secure** | Option<**bool**> | Whether secure SIP is enabled for the domain | [optional]
**byoc_trunk_sid** | Option<**String**> | The SID of the BYOC Trunk resource. | [optional]
**emergency_caller_sid** | Option<**String**> | Whether an emergency caller sid is configured for the domain. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


