# ApiPeriodV2010PeriodAccountPeriodMessage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**body** | Option<**String**> | The message text | [optional]
**num_segments** | Option<**String**> | The number of messages used to deliver the message body | [optional]
**direction** | Option<[**crate::models::MessageEnumDirection**](message_enum_direction.md)> |  | [optional]
**from** | Option<**String**> | The phone number that initiated the message | [optional]
**to** | Option<**String**> | The phone number that received the message | [optional]
**date_updated** | Option<**String**> | The RFC 2822 date and time in GMT that the resource was last updated | [optional]
**price** | Option<**String**> | The amount billed for the message | [optional]
**error_message** | Option<**String**> | The description of the error_code | [optional]
**uri** | Option<**String**> | The URI of the resource, relative to `https://api.twilio.com` | [optional]
**account_sid** | Option<**String**> | The SID of the Account that created the resource | [optional]
**num_media** | Option<**String**> | The number of media files associated with the message | [optional]
**status** | Option<[**crate::models::MessageEnumStatus**](message_enum_status.md)> |  | [optional]
**messaging_service_sid** | Option<**String**> | The SID of the Messaging Service used with the message. | [optional]
**sid** | Option<**String**> | The unique string that identifies the resource | [optional]
**date_sent** | Option<**String**> | The RFC 2822 date and time in GMT when the message was sent | [optional]
**date_created** | Option<**String**> | The RFC 2822 date and time in GMT that the resource was created | [optional]
**error_code** | Option<**i32**> | The error code associated with the message | [optional]
**price_unit** | Option<**String**> | The currency in which price is measured | [optional]
**api_version** | Option<**String**> | The API version used to process the message | [optional]
**subresource_uris** | Option<[**serde_json::Value**](.md)> | A list of related resources identified by their relative URIs | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


