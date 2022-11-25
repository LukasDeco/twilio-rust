# ApiPeriodV2010PeriodAccountPeriodApplication

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_sid** | Option<**String**> | The SID of the Account that created the resource | [optional]
**api_version** | Option<**String**> | The API version used to start a new TwiML session | [optional]
**date_created** | Option<**String**> | The RFC 2822 date and time in GMT that the resource was created | [optional]
**date_updated** | Option<**String**> | The RFC 2822 date and time in GMT that the resource was last updated | [optional]
**friendly_name** | Option<**String**> | The string that you assigned to describe the resource | [optional]
**message_status_callback** | Option<**String**> | The URL to send message status information to your application | [optional]
**sid** | Option<**String**> | The unique string that identifies the resource | [optional]
**sms_fallback_method** | Option<**String**> | The HTTP method used with sms_fallback_url | [optional]
**sms_fallback_url** | Option<**String**> | The URL that we call when an error occurs while retrieving or executing the TwiML | [optional]
**sms_method** | Option<**String**> | The HTTP method to use with sms_url | [optional]
**sms_status_callback** | Option<**String**> | The URL to send status information to your application | [optional]
**sms_url** | Option<**String**> | The URL we call when the phone number receives an incoming SMS message | [optional]
**status_callback** | Option<**String**> | The URL to send status information to your application | [optional]
**status_callback_method** | Option<**String**> | The HTTP method we use to call status_callback | [optional]
**uri** | Option<**String**> | The URI of the resource, relative to `https://api.twilio.com` | [optional]
**voice_caller_id_lookup** | Option<**bool**> | Whether to lookup the caller's name | [optional]
**voice_fallback_method** | Option<**String**> | The HTTP method used with voice_fallback_url | [optional]
**voice_fallback_url** | Option<**String**> | The URL we call when a TwiML error occurs | [optional]
**voice_method** | Option<**String**> | The HTTP method used with the voice_url | [optional]
**voice_url** | Option<**String**> | The URL we call when the phone number receives a call | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


