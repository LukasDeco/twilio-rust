# ApiPeriodV2010PeriodAccountPeriodShortCode

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_sid** | Option<**String**> | The SID of the Account that created this resource | [optional]
**api_version** | Option<**String**> | The API version used to start a new TwiML session | [optional]
**date_created** | Option<**String**> | The RFC 2822 date and time in GMT that this resource was created | [optional]
**date_updated** | Option<**String**> | The RFC 2822 date and time in GMT that this resource was last updated | [optional]
**friendly_name** | Option<**String**> | A string that you assigned to describe this resource | [optional]
**short_code** | Option<**String**> | The short code. e.g., 894546. | [optional]
**sid** | Option<**String**> | The unique string that identifies this resource | [optional]
**sms_fallback_method** | Option<**String**> | HTTP method we use to call the sms_fallback_url | [optional]
**sms_fallback_url** | Option<**String**> | URL Twilio will request if an error occurs in executing TwiML | [optional]
**sms_method** | Option<**String**> | HTTP method to use when requesting the sms url | [optional]
**sms_url** | Option<**String**> | URL we call when receiving an incoming SMS message to this short code | [optional]
**uri** | Option<**String**> | The URI of this resource, relative to `https://api.twilio.com` | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


