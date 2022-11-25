# ApiPeriodV2010PeriodAccountPeriodConferencePeriodParticipant

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_sid** | Option<**String**> | The SID of the Account that created the resource | [optional]
**call_sid** | Option<**String**> | The SID of the Call the resource is associated with | [optional]
**label** | Option<**String**> | The label of this participant | [optional]
**call_sid_to_coach** | Option<**String**> | The SID of the participant who is being `coached` | [optional]
**coaching** | Option<**bool**> | Indicates if the participant changed to coach | [optional]
**conference_sid** | Option<**String**> | The SID of the conference the participant is in | [optional]
**date_created** | Option<**String**> | The RFC 2822 date and time in GMT that the resource was created | [optional]
**date_updated** | Option<**String**> | The RFC 2822 date and time in GMT that the resource was last updated | [optional]
**end_conference_on_exit** | Option<**bool**> | Whether the conference ends when the participant leaves | [optional]
**muted** | Option<**bool**> | Whether the participant is muted | [optional]
**hold** | Option<**bool**> | Whether the participant is on hold | [optional]
**start_conference_on_enter** | Option<**bool**> | Whether the conference starts when the participant joins the conference | [optional]
**status** | Option<[**crate::models::ParticipantEnumStatus**](participant_enum_status.md)> |  | [optional]
**uri** | Option<**String**> | The URI of the resource, relative to `https://api.twilio.com` | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


