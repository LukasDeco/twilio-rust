# ApiPeriodV2010PeriodAccountPeriodCall

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**sid** | Option<**String**> | The unique string that identifies this resource | [optional]
**date_created** | Option<**String**> | The RFC 2822 date and time in GMT that this resource was created | [optional]
**date_updated** | Option<**String**> | The RFC 2822 date and time in GMT that this resource was last updated | [optional]
**parent_call_sid** | Option<**String**> | The SID that identifies the call that created this leg. | [optional]
**account_sid** | Option<**String**> | The SID of the Account that created this resource | [optional]
**to** | Option<**String**> | The phone number, SIP address or Client identifier that received this call. Phone numbers are in E.164 format (e.g., +16175551212). SIP addresses are formatted as `name@company.com`. Client identifiers are formatted `client:name`. | [optional]
**to_formatted** | Option<**String**> | The phone number, SIP address or Client identifier that received this call. Formatted for display. | [optional]
**from** | Option<**String**> | The phone number, SIP address or Client identifier that made this call. Phone numbers are in E.164 format (e.g., +16175551212). SIP addresses are formatted as `name@company.com`. Client identifiers are formatted `client:name`. | [optional]
**from_formatted** | Option<**String**> | The calling phone number, SIP address, or Client identifier formatted for display. | [optional]
**phone_number_sid** | Option<**String**> | If the call was inbound, this is the SID of the IncomingPhoneNumber resource that received the call. If the call was outbound, it is the SID of the OutgoingCallerId resource from which the call was placed. | [optional]
**status** | Option<[**crate::models::CallEnumStatus**](call_enum_status.md)> |  | [optional]
**start_time** | Option<**String**> | The start time of the call. Null if the call has not yet been dialed. | [optional]
**end_time** | Option<**String**> | The end time of the call. Null if the call did not complete successfully. | [optional]
**duration** | Option<**String**> | The length of the call in seconds. | [optional]
**price** | Option<**String**> | The charge for this call, in the currency associated with the account. Populated after the call is completed. May not be immediately available. | [optional]
**price_unit** | Option<**String**> | The currency in which `Price` is measured. | [optional]
**direction** | Option<**String**> | A string describing the direction of the call. `inbound` for inbound calls, `outbound-api` for calls initiated via the REST API or `outbound-dial` for calls initiated by a `Dial` verb. | [optional]
**answered_by** | Option<**String**> | Either `human` or `machine` if this call was initiated with answering machine detection. Empty otherwise. | [optional]
**api_version** | Option<**String**> | The API Version used to create the call | [optional]
**forwarded_from** | Option<**String**> | The forwarding phone number if this call was an incoming call forwarded from another number (depends on carrier supporting forwarding). Otherwise, empty. | [optional]
**group_sid** | Option<**String**> | The Group SID associated with this call. If no Group is associated with the call, the field is empty. | [optional]
**caller_name** | Option<**String**> | The caller's name if this call was an incoming call to a phone number with caller ID Lookup enabled. Otherwise, empty. | [optional]
**queue_time** | Option<**String**> | The wait time in milliseconds before the call is placed. | [optional]
**trunk_sid** | Option<**String**> | The (optional) unique identifier of the trunk resource that was used for this call. | [optional]
**uri** | Option<**String**> | The URI of this resource, relative to `https://api.twilio.com` | [optional]
**subresource_uris** | Option<[**serde_json::Value**](.md)> | A list of related subresources identified by their relative URIs | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


