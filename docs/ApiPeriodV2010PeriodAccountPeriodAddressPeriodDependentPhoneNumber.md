# ApiPeriodV2010PeriodAccountPeriodAddressPeriodDependentPhoneNumber

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**sid** | Option<**String**> | The unique string that identifies the resource | [optional]
**account_sid** | Option<**String**> | The SID of the Account that created the resource | [optional]
**friendly_name** | Option<**String**> | The string that you assigned to describe the resource | [optional]
**phone_number** | Option<**String**> | The phone number in E.164 format | [optional]
**voice_url** | Option<**String**> | The URL we call when the phone number receives a call | [optional]
**voice_method** | Option<**String**> | The HTTP method used with the voice_url | [optional]
**voice_fallback_method** | Option<**String**> | The HTTP method used with voice_fallback_url | [optional]
**voice_fallback_url** | Option<**String**> | The URL we call when an error occurs in TwiML | [optional]
**voice_caller_id_lookup** | Option<**bool**> | Whether to lookup the caller's name | [optional]
**date_created** | Option<**String**> | The RFC 2822 date and time in GMT that the resource was created | [optional]
**date_updated** | Option<**String**> | The RFC 2822 date and time in GMT that the resource was last updated | [optional]
**sms_fallback_method** | Option<**String**> | The HTTP method used with sms_fallback_url | [optional]
**sms_fallback_url** | Option<**String**> | The URL that we call when an error occurs while retrieving or executing the TwiML | [optional]
**sms_method** | Option<**String**> | The HTTP method to use with sms_url | [optional]
**sms_url** | Option<**String**> | The URL we call when the phone number receives an incoming SMS message | [optional]
**address_requirements** | Option<[**crate::models::DependentPhoneNumberEnumAddressRequirement**](dependent_phone_number_enum_address_requirement.md)> |  | [optional]
**capabilities** | Option<[**serde_json::Value**](.md)> | Indicate if a phone can receive calls or messages | [optional]
**status_callback** | Option<**String**> | The URL to send status information to your application | [optional]
**status_callback_method** | Option<**String**> | The HTTP method we use to call status_callback | [optional]
**api_version** | Option<**String**> | The API version used to start a new TwiML session | [optional]
**sms_application_sid** | Option<**String**> | The SID of the application that handles SMS messages sent to the phone number | [optional]
**voice_application_sid** | Option<**String**> | The SID of the application that handles calls to the phone number | [optional]
**trunk_sid** | Option<**String**> | The SID of the Trunk that handles calls to the phone number | [optional]
**emergency_status** | Option<[**crate::models::DependentPhoneNumberEnumEmergencyStatus**](dependent_phone_number_enum_emergency_status.md)> |  | [optional]
**emergency_address_sid** | Option<**String**> | The emergency address configuration to use for emergency calling | [optional]
**uri** | Option<**String**> | The URI of the resource, relative to `https://api.twilio.com` | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


