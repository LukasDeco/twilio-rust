# ApiPeriodV2010PeriodAccountPeriodConnectApp

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_sid** | Option<**String**> | The SID of the Account that created the resource | [optional]
**authorize_redirect_url** | Option<**String**> | The URL to redirect the user to after authorization | [optional]
**company_name** | Option<**String**> | The company name set for the Connect App | [optional]
**deauthorize_callback_method** | Option<**String**> | The HTTP method we use to call deauthorize_callback_url | [optional]
**deauthorize_callback_url** | Option<**String**> | The URL we call to de-authorize the Connect App | [optional]
**description** | Option<**String**> | The description of the Connect App | [optional]
**friendly_name** | Option<**String**> | The string that you assigned to describe the resource | [optional]
**homepage_url** | Option<**String**> | The URL users can obtain more information | [optional]
**permissions** | Option<[**Vec<crate::models::ConnectAppEnumPermission>**](connect_app_enum_permission.md)> | The set of permissions that your ConnectApp requests | [optional]
**sid** | Option<**String**> | The unique string that identifies the resource | [optional]
**uri** | Option<**String**> | The URI of the resource, relative to `https://api.twilio.com` | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


