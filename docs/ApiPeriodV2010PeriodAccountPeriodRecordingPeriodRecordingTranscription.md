# ApiPeriodV2010PeriodAccountPeriodRecordingPeriodRecordingTranscription

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_sid** | Option<**String**> | The SID of the Account that created the resource | [optional]
**api_version** | Option<**String**> | The API version used to create the transcription | [optional]
**date_created** | Option<**String**> | The RFC 2822 date and time in GMT that the resource was created | [optional]
**date_updated** | Option<**String**> | The RFC 2822 date and time in GMT that the resource was last updated | [optional]
**duration** | Option<**String**> | The duration of the transcribed audio in seconds. | [optional]
**price** | Option<**f32**> | The charge for the transcription | [optional]
**price_unit** | Option<**String**> | The currency in which price is measured | [optional]
**recording_sid** | Option<**String**> | The SID that identifies the transcription's recording | [optional]
**sid** | Option<**String**> | The unique string that identifies the resource | [optional]
**status** | Option<[**crate::models::RecordingTranscriptionEnumStatus**](recording_transcription_enum_status.md)> |  | [optional]
**transcription_text** | Option<**String**> | The text content of the transcription. | [optional]
**r#type** | Option<**String**> | The transcription type | [optional]
**uri** | Option<**String**> | The URI of the resource, relative to `https://api.twilio.com` | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


