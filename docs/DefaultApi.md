# \DefaultApi

All URIs are relative to *https://api.twilio.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_account**](DefaultApi.md#create_account) | **POST** /2010-04-01/Accounts.json | 
[**create_address**](DefaultApi.md#create_address) | **POST** /2010-04-01/Accounts/{AccountSid}/Addresses.json | 
[**create_application**](DefaultApi.md#create_application) | **POST** /2010-04-01/Accounts/{AccountSid}/Applications.json | 
[**create_call**](DefaultApi.md#create_call) | **POST** /2010-04-01/Accounts/{AccountSid}/Calls.json | 
[**create_call_feedback_summary**](DefaultApi.md#create_call_feedback_summary) | **POST** /2010-04-01/Accounts/{AccountSid}/Calls/FeedbackSummary.json | 
[**create_call_recording**](DefaultApi.md#create_call_recording) | **POST** /2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Recordings.json | 
[**create_incoming_phone_number**](DefaultApi.md#create_incoming_phone_number) | **POST** /2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers.json | 
[**create_incoming_phone_number_assigned_add_on**](DefaultApi.md#create_incoming_phone_number_assigned_add_on) | **POST** /2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/{ResourceSid}/AssignedAddOns.json | 
[**create_incoming_phone_number_local**](DefaultApi.md#create_incoming_phone_number_local) | **POST** /2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/Local.json | 
[**create_incoming_phone_number_mobile**](DefaultApi.md#create_incoming_phone_number_mobile) | **POST** /2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/Mobile.json | 
[**create_incoming_phone_number_toll_free**](DefaultApi.md#create_incoming_phone_number_toll_free) | **POST** /2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/TollFree.json | 
[**create_message**](DefaultApi.md#create_message) | **POST** /2010-04-01/Accounts/{AccountSid}/Messages.json | 
[**create_message_feedback**](DefaultApi.md#create_message_feedback) | **POST** /2010-04-01/Accounts/{AccountSid}/Messages/{MessageSid}/Feedback.json | 
[**create_new_key**](DefaultApi.md#create_new_key) | **POST** /2010-04-01/Accounts/{AccountSid}/Keys.json | 
[**create_new_signing_key**](DefaultApi.md#create_new_signing_key) | **POST** /2010-04-01/Accounts/{AccountSid}/SigningKeys.json | 
[**create_participant**](DefaultApi.md#create_participant) | **POST** /2010-04-01/Accounts/{AccountSid}/Conferences/{ConferenceSid}/Participants.json | 
[**create_payments**](DefaultApi.md#create_payments) | **POST** /2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Payments.json | 
[**create_queue**](DefaultApi.md#create_queue) | **POST** /2010-04-01/Accounts/{AccountSid}/Queues.json | 
[**create_sip_auth_calls_credential_list_mapping**](DefaultApi.md#create_sip_auth_calls_credential_list_mapping) | **POST** /2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/Auth/Calls/CredentialListMappings.json | 
[**create_sip_auth_calls_ip_access_control_list_mapping**](DefaultApi.md#create_sip_auth_calls_ip_access_control_list_mapping) | **POST** /2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/Auth/Calls/IpAccessControlListMappings.json | 
[**create_sip_auth_registrations_credential_list_mapping**](DefaultApi.md#create_sip_auth_registrations_credential_list_mapping) | **POST** /2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/Auth/Registrations/CredentialListMappings.json | 
[**create_sip_credential**](DefaultApi.md#create_sip_credential) | **POST** /2010-04-01/Accounts/{AccountSid}/SIP/CredentialLists/{CredentialListSid}/Credentials.json | 
[**create_sip_credential_list**](DefaultApi.md#create_sip_credential_list) | **POST** /2010-04-01/Accounts/{AccountSid}/SIP/CredentialLists.json | 
[**create_sip_credential_list_mapping**](DefaultApi.md#create_sip_credential_list_mapping) | **POST** /2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/CredentialListMappings.json | 
[**create_sip_domain**](DefaultApi.md#create_sip_domain) | **POST** /2010-04-01/Accounts/{AccountSid}/SIP/Domains.json | 
[**create_sip_ip_access_control_list**](DefaultApi.md#create_sip_ip_access_control_list) | **POST** /2010-04-01/Accounts/{AccountSid}/SIP/IpAccessControlLists.json | 
[**create_sip_ip_access_control_list_mapping**](DefaultApi.md#create_sip_ip_access_control_list_mapping) | **POST** /2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/IpAccessControlListMappings.json | 
[**create_sip_ip_address**](DefaultApi.md#create_sip_ip_address) | **POST** /2010-04-01/Accounts/{AccountSid}/SIP/IpAccessControlLists/{IpAccessControlListSid}/IpAddresses.json | 
[**create_siprec**](DefaultApi.md#create_siprec) | **POST** /2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Siprec.json | 
[**create_stream**](DefaultApi.md#create_stream) | **POST** /2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Streams.json | 
[**create_token**](DefaultApi.md#create_token) | **POST** /2010-04-01/Accounts/{AccountSid}/Tokens.json | 
[**create_usage_trigger**](DefaultApi.md#create_usage_trigger) | **POST** /2010-04-01/Accounts/{AccountSid}/Usage/Triggers.json | 
[**create_user_defined_message**](DefaultApi.md#create_user_defined_message) | **POST** /2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/UserDefinedMessages.json | 
[**create_user_defined_message_subscription**](DefaultApi.md#create_user_defined_message_subscription) | **POST** /2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/UserDefinedMessageSubscriptions.json | 
[**create_validation_request**](DefaultApi.md#create_validation_request) | **POST** /2010-04-01/Accounts/{AccountSid}/OutgoingCallerIds.json | 
[**delete_address**](DefaultApi.md#delete_address) | **DELETE** /2010-04-01/Accounts/{AccountSid}/Addresses/{Sid}.json | 
[**delete_application**](DefaultApi.md#delete_application) | **DELETE** /2010-04-01/Accounts/{AccountSid}/Applications/{Sid}.json | 
[**delete_call**](DefaultApi.md#delete_call) | **DELETE** /2010-04-01/Accounts/{AccountSid}/Calls/{Sid}.json | 
[**delete_call_feedback_summary**](DefaultApi.md#delete_call_feedback_summary) | **DELETE** /2010-04-01/Accounts/{AccountSid}/Calls/FeedbackSummary/{Sid}.json | 
[**delete_call_recording**](DefaultApi.md#delete_call_recording) | **DELETE** /2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Recordings/{Sid}.json | 
[**delete_conference_recording**](DefaultApi.md#delete_conference_recording) | **DELETE** /2010-04-01/Accounts/{AccountSid}/Conferences/{ConferenceSid}/Recordings/{Sid}.json | 
[**delete_connect_app**](DefaultApi.md#delete_connect_app) | **DELETE** /2010-04-01/Accounts/{AccountSid}/ConnectApps/{Sid}.json | 
[**delete_incoming_phone_number**](DefaultApi.md#delete_incoming_phone_number) | **DELETE** /2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/{Sid}.json | 
[**delete_incoming_phone_number_assigned_add_on**](DefaultApi.md#delete_incoming_phone_number_assigned_add_on) | **DELETE** /2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/{ResourceSid}/AssignedAddOns/{Sid}.json | 
[**delete_key**](DefaultApi.md#delete_key) | **DELETE** /2010-04-01/Accounts/{AccountSid}/Keys/{Sid}.json | 
[**delete_media**](DefaultApi.md#delete_media) | **DELETE** /2010-04-01/Accounts/{AccountSid}/Messages/{MessageSid}/Media/{Sid}.json | 
[**delete_message**](DefaultApi.md#delete_message) | **DELETE** /2010-04-01/Accounts/{AccountSid}/Messages/{Sid}.json | 
[**delete_outgoing_caller_id**](DefaultApi.md#delete_outgoing_caller_id) | **DELETE** /2010-04-01/Accounts/{AccountSid}/OutgoingCallerIds/{Sid}.json | 
[**delete_participant**](DefaultApi.md#delete_participant) | **DELETE** /2010-04-01/Accounts/{AccountSid}/Conferences/{ConferenceSid}/Participants/{CallSid}.json | 
[**delete_queue**](DefaultApi.md#delete_queue) | **DELETE** /2010-04-01/Accounts/{AccountSid}/Queues/{Sid}.json | 
[**delete_recording**](DefaultApi.md#delete_recording) | **DELETE** /2010-04-01/Accounts/{AccountSid}/Recordings/{Sid}.json | 
[**delete_recording_add_on_result**](DefaultApi.md#delete_recording_add_on_result) | **DELETE** /2010-04-01/Accounts/{AccountSid}/Recordings/{ReferenceSid}/AddOnResults/{Sid}.json | 
[**delete_recording_add_on_result_payload**](DefaultApi.md#delete_recording_add_on_result_payload) | **DELETE** /2010-04-01/Accounts/{AccountSid}/Recordings/{ReferenceSid}/AddOnResults/{AddOnResultSid}/Payloads/{Sid}.json | 
[**delete_recording_transcription**](DefaultApi.md#delete_recording_transcription) | **DELETE** /2010-04-01/Accounts/{AccountSid}/Recordings/{RecordingSid}/Transcriptions/{Sid}.json | 
[**delete_signing_key**](DefaultApi.md#delete_signing_key) | **DELETE** /2010-04-01/Accounts/{AccountSid}/SigningKeys/{Sid}.json | 
[**delete_sip_auth_calls_credential_list_mapping**](DefaultApi.md#delete_sip_auth_calls_credential_list_mapping) | **DELETE** /2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/Auth/Calls/CredentialListMappings/{Sid}.json | 
[**delete_sip_auth_calls_ip_access_control_list_mapping**](DefaultApi.md#delete_sip_auth_calls_ip_access_control_list_mapping) | **DELETE** /2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/Auth/Calls/IpAccessControlListMappings/{Sid}.json | 
[**delete_sip_auth_registrations_credential_list_mapping**](DefaultApi.md#delete_sip_auth_registrations_credential_list_mapping) | **DELETE** /2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/Auth/Registrations/CredentialListMappings/{Sid}.json | 
[**delete_sip_credential**](DefaultApi.md#delete_sip_credential) | **DELETE** /2010-04-01/Accounts/{AccountSid}/SIP/CredentialLists/{CredentialListSid}/Credentials/{Sid}.json | 
[**delete_sip_credential_list**](DefaultApi.md#delete_sip_credential_list) | **DELETE** /2010-04-01/Accounts/{AccountSid}/SIP/CredentialLists/{Sid}.json | 
[**delete_sip_credential_list_mapping**](DefaultApi.md#delete_sip_credential_list_mapping) | **DELETE** /2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/CredentialListMappings/{Sid}.json | 
[**delete_sip_domain**](DefaultApi.md#delete_sip_domain) | **DELETE** /2010-04-01/Accounts/{AccountSid}/SIP/Domains/{Sid}.json | 
[**delete_sip_ip_access_control_list**](DefaultApi.md#delete_sip_ip_access_control_list) | **DELETE** /2010-04-01/Accounts/{AccountSid}/SIP/IpAccessControlLists/{Sid}.json | 
[**delete_sip_ip_access_control_list_mapping**](DefaultApi.md#delete_sip_ip_access_control_list_mapping) | **DELETE** /2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/IpAccessControlListMappings/{Sid}.json | 
[**delete_sip_ip_address**](DefaultApi.md#delete_sip_ip_address) | **DELETE** /2010-04-01/Accounts/{AccountSid}/SIP/IpAccessControlLists/{IpAccessControlListSid}/IpAddresses/{Sid}.json | 
[**delete_transcription**](DefaultApi.md#delete_transcription) | **DELETE** /2010-04-01/Accounts/{AccountSid}/Transcriptions/{Sid}.json | 
[**delete_usage_trigger**](DefaultApi.md#delete_usage_trigger) | **DELETE** /2010-04-01/Accounts/{AccountSid}/Usage/Triggers/{Sid}.json | 
[**delete_user_defined_message_subscription**](DefaultApi.md#delete_user_defined_message_subscription) | **DELETE** /2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/UserDefinedMessageSubscriptions/{Sid}.json | 
[**fetch_account**](DefaultApi.md#fetch_account) | **GET** /2010-04-01/Accounts/{Sid}.json | 
[**fetch_address**](DefaultApi.md#fetch_address) | **GET** /2010-04-01/Accounts/{AccountSid}/Addresses/{Sid}.json | 
[**fetch_application**](DefaultApi.md#fetch_application) | **GET** /2010-04-01/Accounts/{AccountSid}/Applications/{Sid}.json | 
[**fetch_authorized_connect_app**](DefaultApi.md#fetch_authorized_connect_app) | **GET** /2010-04-01/Accounts/{AccountSid}/AuthorizedConnectApps/{ConnectAppSid}.json | 
[**fetch_available_phone_number_country**](DefaultApi.md#fetch_available_phone_number_country) | **GET** /2010-04-01/Accounts/{AccountSid}/AvailablePhoneNumbers/{CountryCode}.json | 
[**fetch_balance**](DefaultApi.md#fetch_balance) | **GET** /2010-04-01/Accounts/{AccountSid}/Balance.json | 
[**fetch_call**](DefaultApi.md#fetch_call) | **GET** /2010-04-01/Accounts/{AccountSid}/Calls/{Sid}.json | 
[**fetch_call_feedback**](DefaultApi.md#fetch_call_feedback) | **GET** /2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Feedback.json | 
[**fetch_call_feedback_summary**](DefaultApi.md#fetch_call_feedback_summary) | **GET** /2010-04-01/Accounts/{AccountSid}/Calls/FeedbackSummary/{Sid}.json | 
[**fetch_call_notification**](DefaultApi.md#fetch_call_notification) | **GET** /2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Notifications/{Sid}.json | 
[**fetch_call_recording**](DefaultApi.md#fetch_call_recording) | **GET** /2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Recordings/{Sid}.json | 
[**fetch_conference**](DefaultApi.md#fetch_conference) | **GET** /2010-04-01/Accounts/{AccountSid}/Conferences/{Sid}.json | 
[**fetch_conference_recording**](DefaultApi.md#fetch_conference_recording) | **GET** /2010-04-01/Accounts/{AccountSid}/Conferences/{ConferenceSid}/Recordings/{Sid}.json | 
[**fetch_connect_app**](DefaultApi.md#fetch_connect_app) | **GET** /2010-04-01/Accounts/{AccountSid}/ConnectApps/{Sid}.json | 
[**fetch_incoming_phone_number**](DefaultApi.md#fetch_incoming_phone_number) | **GET** /2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/{Sid}.json | 
[**fetch_incoming_phone_number_assigned_add_on**](DefaultApi.md#fetch_incoming_phone_number_assigned_add_on) | **GET** /2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/{ResourceSid}/AssignedAddOns/{Sid}.json | 
[**fetch_incoming_phone_number_assigned_add_on_extension**](DefaultApi.md#fetch_incoming_phone_number_assigned_add_on_extension) | **GET** /2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/{ResourceSid}/AssignedAddOns/{AssignedAddOnSid}/Extensions/{Sid}.json | 
[**fetch_key**](DefaultApi.md#fetch_key) | **GET** /2010-04-01/Accounts/{AccountSid}/Keys/{Sid}.json | 
[**fetch_media**](DefaultApi.md#fetch_media) | **GET** /2010-04-01/Accounts/{AccountSid}/Messages/{MessageSid}/Media/{Sid}.json | 
[**fetch_member**](DefaultApi.md#fetch_member) | **GET** /2010-04-01/Accounts/{AccountSid}/Queues/{QueueSid}/Members/{CallSid}.json | 
[**fetch_message**](DefaultApi.md#fetch_message) | **GET** /2010-04-01/Accounts/{AccountSid}/Messages/{Sid}.json | 
[**fetch_notification**](DefaultApi.md#fetch_notification) | **GET** /2010-04-01/Accounts/{AccountSid}/Notifications/{Sid}.json | 
[**fetch_outgoing_caller_id**](DefaultApi.md#fetch_outgoing_caller_id) | **GET** /2010-04-01/Accounts/{AccountSid}/OutgoingCallerIds/{Sid}.json | 
[**fetch_participant**](DefaultApi.md#fetch_participant) | **GET** /2010-04-01/Accounts/{AccountSid}/Conferences/{ConferenceSid}/Participants/{CallSid}.json | 
[**fetch_queue**](DefaultApi.md#fetch_queue) | **GET** /2010-04-01/Accounts/{AccountSid}/Queues/{Sid}.json | 
[**fetch_recording**](DefaultApi.md#fetch_recording) | **GET** /2010-04-01/Accounts/{AccountSid}/Recordings/{Sid}.json | 
[**fetch_recording_add_on_result**](DefaultApi.md#fetch_recording_add_on_result) | **GET** /2010-04-01/Accounts/{AccountSid}/Recordings/{ReferenceSid}/AddOnResults/{Sid}.json | 
[**fetch_recording_add_on_result_payload**](DefaultApi.md#fetch_recording_add_on_result_payload) | **GET** /2010-04-01/Accounts/{AccountSid}/Recordings/{ReferenceSid}/AddOnResults/{AddOnResultSid}/Payloads/{Sid}.json | 
[**fetch_recording_transcription**](DefaultApi.md#fetch_recording_transcription) | **GET** /2010-04-01/Accounts/{AccountSid}/Recordings/{RecordingSid}/Transcriptions/{Sid}.json | 
[**fetch_short_code**](DefaultApi.md#fetch_short_code) | **GET** /2010-04-01/Accounts/{AccountSid}/SMS/ShortCodes/{Sid}.json | 
[**fetch_signing_key**](DefaultApi.md#fetch_signing_key) | **GET** /2010-04-01/Accounts/{AccountSid}/SigningKeys/{Sid}.json | 
[**fetch_sip_auth_calls_credential_list_mapping**](DefaultApi.md#fetch_sip_auth_calls_credential_list_mapping) | **GET** /2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/Auth/Calls/CredentialListMappings/{Sid}.json | 
[**fetch_sip_auth_calls_ip_access_control_list_mapping**](DefaultApi.md#fetch_sip_auth_calls_ip_access_control_list_mapping) | **GET** /2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/Auth/Calls/IpAccessControlListMappings/{Sid}.json | 
[**fetch_sip_auth_registrations_credential_list_mapping**](DefaultApi.md#fetch_sip_auth_registrations_credential_list_mapping) | **GET** /2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/Auth/Registrations/CredentialListMappings/{Sid}.json | 
[**fetch_sip_credential**](DefaultApi.md#fetch_sip_credential) | **GET** /2010-04-01/Accounts/{AccountSid}/SIP/CredentialLists/{CredentialListSid}/Credentials/{Sid}.json | 
[**fetch_sip_credential_list**](DefaultApi.md#fetch_sip_credential_list) | **GET** /2010-04-01/Accounts/{AccountSid}/SIP/CredentialLists/{Sid}.json | 
[**fetch_sip_credential_list_mapping**](DefaultApi.md#fetch_sip_credential_list_mapping) | **GET** /2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/CredentialListMappings/{Sid}.json | 
[**fetch_sip_domain**](DefaultApi.md#fetch_sip_domain) | **GET** /2010-04-01/Accounts/{AccountSid}/SIP/Domains/{Sid}.json | 
[**fetch_sip_ip_access_control_list**](DefaultApi.md#fetch_sip_ip_access_control_list) | **GET** /2010-04-01/Accounts/{AccountSid}/SIP/IpAccessControlLists/{Sid}.json | 
[**fetch_sip_ip_access_control_list_mapping**](DefaultApi.md#fetch_sip_ip_access_control_list_mapping) | **GET** /2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/IpAccessControlListMappings/{Sid}.json | 
[**fetch_sip_ip_address**](DefaultApi.md#fetch_sip_ip_address) | **GET** /2010-04-01/Accounts/{AccountSid}/SIP/IpAccessControlLists/{IpAccessControlListSid}/IpAddresses/{Sid}.json | 
[**fetch_transcription**](DefaultApi.md#fetch_transcription) | **GET** /2010-04-01/Accounts/{AccountSid}/Transcriptions/{Sid}.json | 
[**fetch_usage_trigger**](DefaultApi.md#fetch_usage_trigger) | **GET** /2010-04-01/Accounts/{AccountSid}/Usage/Triggers/{Sid}.json | 
[**list_account**](DefaultApi.md#list_account) | **GET** /2010-04-01/Accounts.json | 
[**list_address**](DefaultApi.md#list_address) | **GET** /2010-04-01/Accounts/{AccountSid}/Addresses.json | 
[**list_application**](DefaultApi.md#list_application) | **GET** /2010-04-01/Accounts/{AccountSid}/Applications.json | 
[**list_authorized_connect_app**](DefaultApi.md#list_authorized_connect_app) | **GET** /2010-04-01/Accounts/{AccountSid}/AuthorizedConnectApps.json | 
[**list_available_phone_number_country**](DefaultApi.md#list_available_phone_number_country) | **GET** /2010-04-01/Accounts/{AccountSid}/AvailablePhoneNumbers.json | 
[**list_available_phone_number_local**](DefaultApi.md#list_available_phone_number_local) | **GET** /2010-04-01/Accounts/{AccountSid}/AvailablePhoneNumbers/{CountryCode}/Local.json | 
[**list_available_phone_number_machine_to_machine**](DefaultApi.md#list_available_phone_number_machine_to_machine) | **GET** /2010-04-01/Accounts/{AccountSid}/AvailablePhoneNumbers/{CountryCode}/MachineToMachine.json | 
[**list_available_phone_number_mobile**](DefaultApi.md#list_available_phone_number_mobile) | **GET** /2010-04-01/Accounts/{AccountSid}/AvailablePhoneNumbers/{CountryCode}/Mobile.json | 
[**list_available_phone_number_national**](DefaultApi.md#list_available_phone_number_national) | **GET** /2010-04-01/Accounts/{AccountSid}/AvailablePhoneNumbers/{CountryCode}/National.json | 
[**list_available_phone_number_shared_cost**](DefaultApi.md#list_available_phone_number_shared_cost) | **GET** /2010-04-01/Accounts/{AccountSid}/AvailablePhoneNumbers/{CountryCode}/SharedCost.json | 
[**list_available_phone_number_toll_free**](DefaultApi.md#list_available_phone_number_toll_free) | **GET** /2010-04-01/Accounts/{AccountSid}/AvailablePhoneNumbers/{CountryCode}/TollFree.json | 
[**list_available_phone_number_voip**](DefaultApi.md#list_available_phone_number_voip) | **GET** /2010-04-01/Accounts/{AccountSid}/AvailablePhoneNumbers/{CountryCode}/Voip.json | 
[**list_call**](DefaultApi.md#list_call) | **GET** /2010-04-01/Accounts/{AccountSid}/Calls.json | 
[**list_call_event**](DefaultApi.md#list_call_event) | **GET** /2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Events.json | 
[**list_call_notification**](DefaultApi.md#list_call_notification) | **GET** /2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Notifications.json | 
[**list_call_recording**](DefaultApi.md#list_call_recording) | **GET** /2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Recordings.json | 
[**list_conference**](DefaultApi.md#list_conference) | **GET** /2010-04-01/Accounts/{AccountSid}/Conferences.json | 
[**list_conference_recording**](DefaultApi.md#list_conference_recording) | **GET** /2010-04-01/Accounts/{AccountSid}/Conferences/{ConferenceSid}/Recordings.json | 
[**list_connect_app**](DefaultApi.md#list_connect_app) | **GET** /2010-04-01/Accounts/{AccountSid}/ConnectApps.json | 
[**list_dependent_phone_number**](DefaultApi.md#list_dependent_phone_number) | **GET** /2010-04-01/Accounts/{AccountSid}/Addresses/{AddressSid}/DependentPhoneNumbers.json | 
[**list_incoming_phone_number**](DefaultApi.md#list_incoming_phone_number) | **GET** /2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers.json | 
[**list_incoming_phone_number_assigned_add_on**](DefaultApi.md#list_incoming_phone_number_assigned_add_on) | **GET** /2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/{ResourceSid}/AssignedAddOns.json | 
[**list_incoming_phone_number_assigned_add_on_extension**](DefaultApi.md#list_incoming_phone_number_assigned_add_on_extension) | **GET** /2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/{ResourceSid}/AssignedAddOns/{AssignedAddOnSid}/Extensions.json | 
[**list_incoming_phone_number_local**](DefaultApi.md#list_incoming_phone_number_local) | **GET** /2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/Local.json | 
[**list_incoming_phone_number_mobile**](DefaultApi.md#list_incoming_phone_number_mobile) | **GET** /2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/Mobile.json | 
[**list_incoming_phone_number_toll_free**](DefaultApi.md#list_incoming_phone_number_toll_free) | **GET** /2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/TollFree.json | 
[**list_key**](DefaultApi.md#list_key) | **GET** /2010-04-01/Accounts/{AccountSid}/Keys.json | 
[**list_media**](DefaultApi.md#list_media) | **GET** /2010-04-01/Accounts/{AccountSid}/Messages/{MessageSid}/Media.json | 
[**list_member**](DefaultApi.md#list_member) | **GET** /2010-04-01/Accounts/{AccountSid}/Queues/{QueueSid}/Members.json | 
[**list_message**](DefaultApi.md#list_message) | **GET** /2010-04-01/Accounts/{AccountSid}/Messages.json | 
[**list_notification**](DefaultApi.md#list_notification) | **GET** /2010-04-01/Accounts/{AccountSid}/Notifications.json | 
[**list_outgoing_caller_id**](DefaultApi.md#list_outgoing_caller_id) | **GET** /2010-04-01/Accounts/{AccountSid}/OutgoingCallerIds.json | 
[**list_participant**](DefaultApi.md#list_participant) | **GET** /2010-04-01/Accounts/{AccountSid}/Conferences/{ConferenceSid}/Participants.json | 
[**list_queue**](DefaultApi.md#list_queue) | **GET** /2010-04-01/Accounts/{AccountSid}/Queues.json | 
[**list_recording**](DefaultApi.md#list_recording) | **GET** /2010-04-01/Accounts/{AccountSid}/Recordings.json | 
[**list_recording_add_on_result**](DefaultApi.md#list_recording_add_on_result) | **GET** /2010-04-01/Accounts/{AccountSid}/Recordings/{ReferenceSid}/AddOnResults.json | 
[**list_recording_add_on_result_payload**](DefaultApi.md#list_recording_add_on_result_payload) | **GET** /2010-04-01/Accounts/{AccountSid}/Recordings/{ReferenceSid}/AddOnResults/{AddOnResultSid}/Payloads.json | 
[**list_recording_transcription**](DefaultApi.md#list_recording_transcription) | **GET** /2010-04-01/Accounts/{AccountSid}/Recordings/{RecordingSid}/Transcriptions.json | 
[**list_short_code**](DefaultApi.md#list_short_code) | **GET** /2010-04-01/Accounts/{AccountSid}/SMS/ShortCodes.json | 
[**list_signing_key**](DefaultApi.md#list_signing_key) | **GET** /2010-04-01/Accounts/{AccountSid}/SigningKeys.json | 
[**list_sip_auth_calls_credential_list_mapping**](DefaultApi.md#list_sip_auth_calls_credential_list_mapping) | **GET** /2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/Auth/Calls/CredentialListMappings.json | 
[**list_sip_auth_calls_ip_access_control_list_mapping**](DefaultApi.md#list_sip_auth_calls_ip_access_control_list_mapping) | **GET** /2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/Auth/Calls/IpAccessControlListMappings.json | 
[**list_sip_auth_registrations_credential_list_mapping**](DefaultApi.md#list_sip_auth_registrations_credential_list_mapping) | **GET** /2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/Auth/Registrations/CredentialListMappings.json | 
[**list_sip_credential**](DefaultApi.md#list_sip_credential) | **GET** /2010-04-01/Accounts/{AccountSid}/SIP/CredentialLists/{CredentialListSid}/Credentials.json | 
[**list_sip_credential_list**](DefaultApi.md#list_sip_credential_list) | **GET** /2010-04-01/Accounts/{AccountSid}/SIP/CredentialLists.json | 
[**list_sip_credential_list_mapping**](DefaultApi.md#list_sip_credential_list_mapping) | **GET** /2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/CredentialListMappings.json | 
[**list_sip_domain**](DefaultApi.md#list_sip_domain) | **GET** /2010-04-01/Accounts/{AccountSid}/SIP/Domains.json | 
[**list_sip_ip_access_control_list**](DefaultApi.md#list_sip_ip_access_control_list) | **GET** /2010-04-01/Accounts/{AccountSid}/SIP/IpAccessControlLists.json | 
[**list_sip_ip_access_control_list_mapping**](DefaultApi.md#list_sip_ip_access_control_list_mapping) | **GET** /2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/IpAccessControlListMappings.json | 
[**list_sip_ip_address**](DefaultApi.md#list_sip_ip_address) | **GET** /2010-04-01/Accounts/{AccountSid}/SIP/IpAccessControlLists/{IpAccessControlListSid}/IpAddresses.json | 
[**list_transcription**](DefaultApi.md#list_transcription) | **GET** /2010-04-01/Accounts/{AccountSid}/Transcriptions.json | 
[**list_usage_record**](DefaultApi.md#list_usage_record) | **GET** /2010-04-01/Accounts/{AccountSid}/Usage/Records.json | 
[**list_usage_record_all_time**](DefaultApi.md#list_usage_record_all_time) | **GET** /2010-04-01/Accounts/{AccountSid}/Usage/Records/AllTime.json | 
[**list_usage_record_daily**](DefaultApi.md#list_usage_record_daily) | **GET** /2010-04-01/Accounts/{AccountSid}/Usage/Records/Daily.json | 
[**list_usage_record_last_month**](DefaultApi.md#list_usage_record_last_month) | **GET** /2010-04-01/Accounts/{AccountSid}/Usage/Records/LastMonth.json | 
[**list_usage_record_monthly**](DefaultApi.md#list_usage_record_monthly) | **GET** /2010-04-01/Accounts/{AccountSid}/Usage/Records/Monthly.json | 
[**list_usage_record_this_month**](DefaultApi.md#list_usage_record_this_month) | **GET** /2010-04-01/Accounts/{AccountSid}/Usage/Records/ThisMonth.json | 
[**list_usage_record_today**](DefaultApi.md#list_usage_record_today) | **GET** /2010-04-01/Accounts/{AccountSid}/Usage/Records/Today.json | 
[**list_usage_record_yearly**](DefaultApi.md#list_usage_record_yearly) | **GET** /2010-04-01/Accounts/{AccountSid}/Usage/Records/Yearly.json | 
[**list_usage_record_yesterday**](DefaultApi.md#list_usage_record_yesterday) | **GET** /2010-04-01/Accounts/{AccountSid}/Usage/Records/Yesterday.json | 
[**list_usage_trigger**](DefaultApi.md#list_usage_trigger) | **GET** /2010-04-01/Accounts/{AccountSid}/Usage/Triggers.json | 
[**update_account**](DefaultApi.md#update_account) | **POST** /2010-04-01/Accounts/{Sid}.json | 
[**update_address**](DefaultApi.md#update_address) | **POST** /2010-04-01/Accounts/{AccountSid}/Addresses/{Sid}.json | 
[**update_application**](DefaultApi.md#update_application) | **POST** /2010-04-01/Accounts/{AccountSid}/Applications/{Sid}.json | 
[**update_call**](DefaultApi.md#update_call) | **POST** /2010-04-01/Accounts/{AccountSid}/Calls/{Sid}.json | 
[**update_call_feedback**](DefaultApi.md#update_call_feedback) | **POST** /2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Feedback.json | 
[**update_call_recording**](DefaultApi.md#update_call_recording) | **POST** /2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Recordings/{Sid}.json | 
[**update_conference**](DefaultApi.md#update_conference) | **POST** /2010-04-01/Accounts/{AccountSid}/Conferences/{Sid}.json | 
[**update_conference_recording**](DefaultApi.md#update_conference_recording) | **POST** /2010-04-01/Accounts/{AccountSid}/Conferences/{ConferenceSid}/Recordings/{Sid}.json | 
[**update_connect_app**](DefaultApi.md#update_connect_app) | **POST** /2010-04-01/Accounts/{AccountSid}/ConnectApps/{Sid}.json | 
[**update_incoming_phone_number**](DefaultApi.md#update_incoming_phone_number) | **POST** /2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/{Sid}.json | 
[**update_key**](DefaultApi.md#update_key) | **POST** /2010-04-01/Accounts/{AccountSid}/Keys/{Sid}.json | 
[**update_member**](DefaultApi.md#update_member) | **POST** /2010-04-01/Accounts/{AccountSid}/Queues/{QueueSid}/Members/{CallSid}.json | 
[**update_message**](DefaultApi.md#update_message) | **POST** /2010-04-01/Accounts/{AccountSid}/Messages/{Sid}.json | 
[**update_outgoing_caller_id**](DefaultApi.md#update_outgoing_caller_id) | **POST** /2010-04-01/Accounts/{AccountSid}/OutgoingCallerIds/{Sid}.json | 
[**update_participant**](DefaultApi.md#update_participant) | **POST** /2010-04-01/Accounts/{AccountSid}/Conferences/{ConferenceSid}/Participants/{CallSid}.json | 
[**update_payments**](DefaultApi.md#update_payments) | **POST** /2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Payments/{Sid}.json | 
[**update_queue**](DefaultApi.md#update_queue) | **POST** /2010-04-01/Accounts/{AccountSid}/Queues/{Sid}.json | 
[**update_short_code**](DefaultApi.md#update_short_code) | **POST** /2010-04-01/Accounts/{AccountSid}/SMS/ShortCodes/{Sid}.json | 
[**update_signing_key**](DefaultApi.md#update_signing_key) | **POST** /2010-04-01/Accounts/{AccountSid}/SigningKeys/{Sid}.json | 
[**update_sip_credential**](DefaultApi.md#update_sip_credential) | **POST** /2010-04-01/Accounts/{AccountSid}/SIP/CredentialLists/{CredentialListSid}/Credentials/{Sid}.json | 
[**update_sip_credential_list**](DefaultApi.md#update_sip_credential_list) | **POST** /2010-04-01/Accounts/{AccountSid}/SIP/CredentialLists/{Sid}.json | 
[**update_sip_domain**](DefaultApi.md#update_sip_domain) | **POST** /2010-04-01/Accounts/{AccountSid}/SIP/Domains/{Sid}.json | 
[**update_sip_ip_access_control_list**](DefaultApi.md#update_sip_ip_access_control_list) | **POST** /2010-04-01/Accounts/{AccountSid}/SIP/IpAccessControlLists/{Sid}.json | 
[**update_sip_ip_address**](DefaultApi.md#update_sip_ip_address) | **POST** /2010-04-01/Accounts/{AccountSid}/SIP/IpAccessControlLists/{IpAccessControlListSid}/IpAddresses/{Sid}.json | 
[**update_siprec**](DefaultApi.md#update_siprec) | **POST** /2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Siprec/{Sid}.json | 
[**update_stream**](DefaultApi.md#update_stream) | **POST** /2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Streams/{Sid}.json | 
[**update_usage_trigger**](DefaultApi.md#update_usage_trigger) | **POST** /2010-04-01/Accounts/{AccountSid}/Usage/Triggers/{Sid}.json | 



## create_account

> crate::models::ApiPeriodV2010PeriodAccount create_account(friendly_name)


Create a new Twilio Subaccount from the account making the request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**friendly_name** | Option<**String**> | A human readable description of the account to create, defaults to `SubAccount Created at {YYYY-MM-DD HH:MM meridian}` |  |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccount**](api.v2010.account.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_address

> crate::models::ApiPeriodV2010PeriodAccountPeriodAddress create_address(account_sid, customer_name, street, city, region, postal_code, iso_country, friendly_name, emergency_enabled, auto_correct_address)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that will be responsible for the new Address resource. | [required] |
**customer_name** | **String** | The name to associate with the new address. | [required] |
**street** | **String** | The number and street address of the new address. | [required] |
**city** | **String** | The city of the new address. | [required] |
**region** | **String** | The state or region of the new address. | [required] |
**postal_code** | **String** | The postal code of the new address. | [required] |
**iso_country** | **String** | The ISO country code of the new address. | [required] |
**friendly_name** | Option<**String**> | A descriptive string that you create to describe the new address. It can be up to 64 characters long. |  |
**emergency_enabled** | Option<**bool**> | Whether to enable emergency calling on the new address. Can be: `true` or `false`. |  |
**auto_correct_address** | Option<**bool**> | Whether we should automatically correct the address. Can be: `true` or `false` and the default is `true`. If empty or `true`, we will correct the address you provide if necessary. If `false`, we won't alter the address you provide. |  |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodAddress**](api.v2010.account.address.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_application

> crate::models::ApiPeriodV2010PeriodAccountPeriodApplication create_application(account_sid, api_version, voice_url, voice_method, voice_fallback_url, voice_fallback_method, status_callback, status_callback_method, voice_caller_id_lookup, sms_url, sms_method, sms_fallback_url, sms_fallback_method, sms_status_callback, message_status_callback, friendly_name)


Create a new application within your account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that will create the resource. | [required] |
**api_version** | Option<**String**> | The API version to use to start a new TwiML session. Can be: `2010-04-01` or `2008-08-01`. The default value is the account's default API version. |  |
**voice_url** | Option<**String**> | The URL we should call when the phone number assigned to this application receives a call. |  |
**voice_method** | Option<**String**> | The HTTP method we should use to call `voice_url`. Can be: `GET` or `POST`. |  |
**voice_fallback_url** | Option<**String**> | The URL that we should call when an error occurs retrieving or executing the TwiML requested by `url`. |  |
**voice_fallback_method** | Option<**String**> | The HTTP method we should use to call `voice_fallback_url`. Can be: `GET` or `POST`. |  |
**status_callback** | Option<**String**> | The URL we should call using the `status_callback_method` to send status information to your application. |  |
**status_callback_method** | Option<**String**> | The HTTP method we should use to call `status_callback`. Can be: `GET` or `POST`. |  |
**voice_caller_id_lookup** | Option<**bool**> | Whether we should look up the caller's caller-ID name from the CNAM database (additional charges apply). Can be: `true` or `false`. |  |
**sms_url** | Option<**String**> | The URL we should call when the phone number receives an incoming SMS message. |  |
**sms_method** | Option<**String**> | The HTTP method we should use to call `sms_url`. Can be: `GET` or `POST`. |  |
**sms_fallback_url** | Option<**String**> | The URL that we should call when an error occurs while retrieving or executing the TwiML from `sms_url`. |  |
**sms_fallback_method** | Option<**String**> | The HTTP method we should use to call `sms_fallback_url`. Can be: `GET` or `POST`. |  |
**sms_status_callback** | Option<**String**> | The URL we should call using a POST method to send status information about SMS messages sent by the application. |  |
**message_status_callback** | Option<**String**> | The URL we should call using a POST method to send message status information to your application. |  |
**friendly_name** | Option<**String**> | A descriptive string that you create to describe the new application. It can be up to 64 characters long. |  |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodApplication**](api.v2010.account.application.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_call

> crate::models::ApiPeriodV2010PeriodAccountPeriodCall create_call(account_sid, to, from, method, fallback_url, fallback_method, status_callback, status_callback_event, status_callback_method, send_digits, timeout, record, recording_channels, recording_status_callback, recording_status_callback_method, sip_auth_username, sip_auth_password, machine_detection, machine_detection_timeout, recording_status_callback_event, trim, caller_id, machine_detection_speech_threshold, machine_detection_speech_end_threshold, machine_detection_silence_timeout, async_amd, async_amd_status_callback, async_amd_status_callback_method, byoc, call_reason, call_token, recording_track, time_limit, url, twiml, application_sid)


Create a new outgoing call to phones, SIP-enabled endpoints or Twilio Client connections

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that will create the resource. | [required] |
**to** | **String** | The phone number, SIP address, or client identifier to call. | [required] |
**from** | **String** | The phone number or client identifier to use as the caller id. If using a phone number, it must be a Twilio number or a Verified [outgoing caller id](https://www.twilio.com/docs/voice/api/outgoing-caller-ids) for your account. If the `to` parameter is a phone number, `From` must also be a phone number. | [required] |
**method** | Option<**String**> | The HTTP method we should use when calling the `url` parameter's value. Can be: `GET` or `POST` and the default is `POST`. If an `application_sid` parameter is present, this parameter is ignored. |  |
**fallback_url** | Option<**String**> | The URL that we call using the `fallback_method` if an error occurs when requesting or executing the TwiML at `url`. If an `application_sid` parameter is present, this parameter is ignored. |  |
**fallback_method** | Option<**String**> | The HTTP method that we should use to request the `fallback_url`. Can be: `GET` or `POST` and the default is `POST`. If an `application_sid` parameter is present, this parameter is ignored. |  |
**status_callback** | Option<**String**> | The URL we should call using the `status_callback_method` to send status information to your application. If no `status_callback_event` is specified, we will send the `completed` status. If an `application_sid` parameter is present, this parameter is ignored. URLs must contain a valid hostname (underscores are not permitted). |  |
**status_callback_event** | Option<[**Vec<String>**](String.md)> | The call progress events that we will send to the `status_callback` URL. Can be: `initiated`, `ringing`, `answered`, and `completed`. If no event is specified, we send the `completed` status. If you want to receive multiple events, specify each one in a separate `status_callback_event` parameter. See the code sample for [monitoring call progress](https://www.twilio.com/docs/voice/api/call-resource?code-sample=code-create-a-call-resource-and-specify-a-statuscallbackevent&code-sdk-version=json). If an `application_sid` is present, this parameter is ignored. |  |
**status_callback_method** | Option<**String**> | The HTTP method we should use when calling the `status_callback` URL. Can be: `GET` or `POST` and the default is `POST`. If an `application_sid` parameter is present, this parameter is ignored. |  |
**send_digits** | Option<**String**> | A string of keys to dial after connecting to the number, maximum of 32 digits. Valid digits in the string include: any digit (`0`-`9`), '`#`', '`*`' and '`w`', to insert a half second pause. For example, if you connected to a company phone number and wanted to pause for one second, and then dial extension 1234 followed by the pound key, the value of this parameter would be `ww1234#`. Remember to URL-encode this string, since the '`#`' character has special meaning in a URL. If both `SendDigits` and `MachineDetection` parameters are provided, then `MachineDetection` will be ignored. |  |
**timeout** | Option<**i32**> | The integer number of seconds that we should allow the phone to ring before assuming there is no answer. The default is `60` seconds and the maximum is `600` seconds. For some call flows, we will add a 5-second buffer to the timeout value you provide. For this reason, a timeout value of 10 seconds could result in an actual timeout closer to 15 seconds. You can set this to a short time, such as `15` seconds, to hang up before reaching an answering machine or voicemail. |  |
**record** | Option<**bool**> | Whether to record the call. Can be `true` to record the phone call, or `false` to not. The default is `false`. The `recording_url` is sent to the `status_callback` URL. |  |
**recording_channels** | Option<**String**> | The number of channels in the final recording. Can be: `mono` or `dual`. The default is `mono`. `mono` records both legs of the call in a single channel of the recording file. `dual` records each leg to a separate channel of the recording file. The first channel of a dual-channel recording contains the parent call and the second channel contains the child call. |  |
**recording_status_callback** | Option<**String**> | The URL that we call when the recording is available to be accessed. |  |
**recording_status_callback_method** | Option<**String**> | The HTTP method we should use when calling the `recording_status_callback` URL. Can be: `GET` or `POST` and the default is `POST`. |  |
**sip_auth_username** | Option<**String**> | The username used to authenticate the caller making a SIP call. |  |
**sip_auth_password** | Option<**String**> | The password required to authenticate the user account specified in `sip_auth_username`. |  |
**machine_detection** | Option<**String**> | Whether to detect if a human, answering machine, or fax has picked up the call. Can be: `Enable` or `DetectMessageEnd`. Use `Enable` if you would like us to return `AnsweredBy` as soon as the called party is identified. Use `DetectMessageEnd`, if you would like to leave a message on an answering machine. If `send_digits` is provided, this parameter is ignored. For more information, see [Answering Machine Detection](https://www.twilio.com/docs/voice/answering-machine-detection). |  |
**machine_detection_timeout** | Option<**i32**> | The number of seconds that we should attempt to detect an answering machine before timing out and sending a voice request with `AnsweredBy` of `unknown`. The default timeout is 30 seconds. |  |
**recording_status_callback_event** | Option<[**Vec<String>**](String.md)> | The recording status events that will trigger calls to the URL specified in `recording_status_callback`. Can be: `in-progress`, `completed` and `absent`. Defaults to `completed`. Separate  multiple values with a space. |  |
**trim** | Option<**String**> | Whether to trim any leading and trailing silence from the recording. Can be: `trim-silence` or `do-not-trim` and the default is `trim-silence`. |  |
**caller_id** | Option<**String**> | The phone number, SIP address, or Client identifier that made this call. Phone numbers are in [E.164 format](https://wwnw.twilio.com/docs/glossary/what-e164) (e.g., +16175551212). SIP addresses are formatted as `name@company.com`. |  |
**machine_detection_speech_threshold** | Option<**i32**> | The number of milliseconds that is used as the measuring stick for the length of the speech activity, where durations lower than this value will be interpreted as a human and longer than this value as a machine. Possible Values: 1000-6000. Default: 2400. |  |
**machine_detection_speech_end_threshold** | Option<**i32**> | The number of milliseconds of silence after speech activity at which point the speech activity is considered complete. Possible Values: 500-5000. Default: 1200. |  |
**machine_detection_silence_timeout** | Option<**i32**> | The number of milliseconds of initial silence after which an `unknown` AnsweredBy result will be returned. Possible Values: 2000-10000. Default: 5000. |  |
**async_amd** | Option<**String**> | Select whether to perform answering machine detection in the background. Default, blocks the execution of the call until Answering Machine Detection is completed. Can be: `true` or `false`. |  |
**async_amd_status_callback** | Option<**String**> | The URL that we should call using the `async_amd_status_callback_method` to notify customer application whether the call was answered by human, machine or fax. |  |
**async_amd_status_callback_method** | Option<**String**> | The HTTP method we should use when calling the `async_amd_status_callback` URL. Can be: `GET` or `POST` and the default is `POST`. |  |
**byoc** | Option<**String**> | The SID of a BYOC (Bring Your Own Carrier) trunk to route this call with. Note that `byoc` is only meaningful when `to` is a phone number; it will otherwise be ignored. (Beta) |  |
**call_reason** | Option<**String**> | The Reason for the outgoing call. Use it to specify the purpose of the call that is presented on the called party's phone. (Branded Calls Beta) |  |
**call_token** | Option<**String**> | A token string needed to invoke a forwarded call. A call_token is generated when an incoming call is received on a Twilio number. Pass an incoming call's call_token value to a forwarded call via the call_token parameter when creating a new call. A forwarded call should bear the same CallerID of the original incoming call. |  |
**recording_track** | Option<**String**> | The audio track to record for the call. Can be: `inbound`, `outbound` or `both`. The default is `both`. `inbound` records the audio that is received by Twilio. `outbound` records the audio that is generated from Twilio. `both` records the audio that is received and generated by Twilio. |  |
**time_limit** | Option<**i32**> | The maximum duration of the call in seconds. Constraints depend on account and configuration. |  |
**url** | Option<**String**> | The absolute URL that returns the TwiML instructions for the call. We will call this URL using the `method` when the call connects. For more information, see the [Url Parameter](https://www.twilio.com/docs/voice/make-calls#specify-a-url-parameter) section in [Making Calls](https://www.twilio.com/docs/voice/make-calls). |  |
**twiml** | Option<**String**> | TwiML instructions for the call Twilio will use without fetching Twiml from url parameter. If both `twiml` and `url` are provided then `twiml` parameter will be ignored. Max 4000 characters. |  |
**application_sid** | Option<**String**> | The SID of the Application resource that will handle the call, if the call will be handled by an application. |  |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodCall**](api.v2010.account.call.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_call_feedback_summary

> crate::models::ApiPeriodV2010PeriodAccountPeriodCallPeriodCallFeedbackSummary create_call_feedback_summary(account_sid, start_date, end_date, include_subaccounts, status_callback, status_callback_method)


Create a FeedbackSummary resource for a call

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The unique id of the [Account](https://www.twilio.com/docs/iam/api/account) responsible for this resource. | [required] |
**start_date** | **String** | Only include feedback given on or after this date. Format is `YYYY-MM-DD` and specified in UTC. | [required] |
**end_date** | **String** | Only include feedback given on or before this date. Format is `YYYY-MM-DD` and specified in UTC. | [required] |
**include_subaccounts** | Option<**bool**> | Whether to also include Feedback resources from all subaccounts. `true` includes feedback from all subaccounts and `false`, the default, includes feedback from only the specified account. |  |
**status_callback** | Option<**String**> | The URL that we will request when the feedback summary is complete. |  |
**status_callback_method** | Option<**String**> | The HTTP method (`GET` or `POST`) we use to make the request to the `StatusCallback` URL. |  |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodCallPeriodCallFeedbackSummary**](api.v2010.account.call.call_feedback_summary.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_call_recording

> crate::models::ApiPeriodV2010PeriodAccountPeriodCallPeriodCallRecording create_call_recording(account_sid, call_sid, recording_status_callback_event, recording_status_callback, recording_status_callback_method, trim, recording_channels, recording_track)


Create a recording for the call

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that will create the resource. | [required] |
**call_sid** | **String** | The SID of the [Call](https://www.twilio.com/docs/voice/api/call-resource) to associate the resource with. | [required] |
**recording_status_callback_event** | Option<[**Vec<String>**](String.md)> | The recording status events on which we should call the `recording_status_callback` URL. Can be: `in-progress`, `completed` and `absent` and the default is `completed`. Separate multiple event values with a space. |  |
**recording_status_callback** | Option<**String**> | The URL we should call using the `recording_status_callback_method` on each recording event specified in  `recording_status_callback_event`. For more information, see [RecordingStatusCallback parameters](https://www.twilio.com/docs/voice/api/recording#recordingstatuscallback). |  |
**recording_status_callback_method** | Option<**String**> | The HTTP method we should use to call `recording_status_callback`. Can be: `GET` or `POST` and the default is `POST`. |  |
**trim** | Option<**String**> | Whether to trim any leading and trailing silence in the recording. Can be: `trim-silence` or `do-not-trim` and the default is `do-not-trim`. `trim-silence` trims the silence from the beginning and end of the recording and `do-not-trim` does not. |  |
**recording_channels** | Option<**String**> | The number of channels used in the recording. Can be: `mono` or `dual` and the default is `mono`. `mono` records all parties of the call into one channel. `dual` records each party of a 2-party call into separate channels. |  |
**recording_track** | Option<**String**> | The audio track to record for the call. Can be: `inbound`, `outbound` or `both`. The default is `both`. `inbound` records the audio that is received by Twilio. `outbound` records the audio that is generated from Twilio. `both` records the audio that is received and generated by Twilio. |  |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodCallPeriodCallRecording**](api.v2010.account.call.call_recording.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_incoming_phone_number

> crate::models::ApiPeriodV2010PeriodAccountPeriodIncomingPhoneNumber create_incoming_phone_number(account_sid, api_version, friendly_name, sms_application_sid, sms_fallback_method, sms_fallback_url, sms_method, sms_url, status_callback, status_callback_method, voice_application_sid, voice_caller_id_lookup, voice_fallback_method, voice_fallback_url, voice_method, voice_url, emergency_status, emergency_address_sid, trunk_sid, identity_sid, address_sid, voice_receive_mode, bundle_sid, phone_number, area_code)


Purchase a phone-number for the account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that will create the resource. | [required] |
**api_version** | Option<**String**> | The API version to use for incoming calls made to the new phone number. The default is `2010-04-01`. |  |
**friendly_name** | Option<**String**> | A descriptive string that you created to describe the new phone number. It can be up to 64 characters long. By default, this is a formatted version of the new phone number. |  |
**sms_application_sid** | Option<**String**> | The SID of the application that should handle SMS messages sent to the new phone number. If an `sms_application_sid` is present, we ignore all of the `sms_*_url` urls and use those set on the application. |  |
**sms_fallback_method** | Option<**String**> | The HTTP method that we should use to call `sms_fallback_url`. Can be: `GET` or `POST` and defaults to `POST`. |  |
**sms_fallback_url** | Option<**String**> | The URL that we should call when an error occurs while requesting or executing the TwiML defined by `sms_url`. |  |
**sms_method** | Option<**String**> | The HTTP method that we should use to call `sms_url`. Can be: `GET` or `POST` and defaults to `POST`. |  |
**sms_url** | Option<**String**> | The URL we should call when the new phone number receives an incoming SMS message. |  |
**status_callback** | Option<**String**> | The URL we should call using the `status_callback_method` to send status information to your application. |  |
**status_callback_method** | Option<**String**> | The HTTP method we should use to call `status_callback`. Can be: `GET` or `POST` and defaults to `POST`. |  |
**voice_application_sid** | Option<**String**> | The SID of the application we should use to handle calls to the new phone number. If a `voice_application_sid` is present, we ignore all of the voice urls and use only those set on the application. Setting a `voice_application_sid` will automatically delete your `trunk_sid` and vice versa. |  |
**voice_caller_id_lookup** | Option<**bool**> | Whether to lookup the caller's name from the CNAM database and post it to your app. Can be: `true` or `false` and defaults to `false`. |  |
**voice_fallback_method** | Option<**String**> | The HTTP method that we should use to call `voice_fallback_url`. Can be: `GET` or `POST` and defaults to `POST`. |  |
**voice_fallback_url** | Option<**String**> | The URL that we should call when an error occurs retrieving or executing the TwiML requested by `url`. |  |
**voice_method** | Option<**String**> | The HTTP method that we should use to call `voice_url`. Can be: `GET` or `POST` and defaults to `POST`. |  |
**voice_url** | Option<**String**> | The URL that we should call to answer a call to the new phone number. The `voice_url` will not be called if a `voice_application_sid` or a `trunk_sid` is set. |  |
**emergency_status** | Option<**crate::models::IncomingPhoneNumberEnumEmergencyStatus**> |  |  |
**emergency_address_sid** | Option<**String**> | The SID of the emergency address configuration to use for emergency calling from the new phone number. |  |
**trunk_sid** | Option<**String**> | The SID of the Trunk we should use to handle calls to the new phone number. If a `trunk_sid` is present, we ignore all of the voice urls and voice applications and use only those set on the Trunk. Setting a `trunk_sid` will automatically delete your `voice_application_sid` and vice versa. |  |
**identity_sid** | Option<**String**> | The SID of the Identity resource that we should associate with the new phone number. Some regions require an identity to meet local regulations. |  |
**address_sid** | Option<**String**> | The SID of the Address resource we should associate with the new phone number. Some regions require addresses to meet local regulations. |  |
**voice_receive_mode** | Option<**crate::models::IncomingPhoneNumberEnumVoiceReceiveMode**> |  |  |
**bundle_sid** | Option<**String**> | The SID of the Bundle resource that you associate with the phone number. Some regions require a Bundle to meet local Regulations. |  |
**phone_number** | Option<**String**> | The phone number to purchase specified in [E.164](https://www.twilio.com/docs/glossary/what-e164) format.  E.164 phone numbers consist of a + followed by the country code and subscriber number without punctuation characters. For example, +14155551234. |  |
**area_code** | Option<**String**> | The desired area code for your new incoming phone number. Can be any three-digit, US or Canada area code. We will provision an available phone number within this area code for you. **You must provide an `area_code` or a `phone_number`.** (US and Canada only). |  |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodIncomingPhoneNumber**](api.v2010.account.incoming_phone_number.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_incoming_phone_number_assigned_add_on

> crate::models::ApiPeriodV2010PeriodAccountPeriodIncomingPhoneNumberPeriodIncomingPhoneNumberAssignedAddOn create_incoming_phone_number_assigned_add_on(account_sid, resource_sid, installed_add_on_sid)


Assign an Add-on installation to the Number specified.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that will create the resource. | [required] |
**resource_sid** | **String** | The SID of the Phone Number to assign the Add-on. | [required] |
**installed_add_on_sid** | **String** | The SID that identifies the Add-on installation. | [required] |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodIncomingPhoneNumberPeriodIncomingPhoneNumberAssignedAddOn**](api.v2010.account.incoming_phone_number.incoming_phone_number_assigned_add_on.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_incoming_phone_number_local

> crate::models::ApiPeriodV2010PeriodAccountPeriodIncomingPhoneNumberPeriodIncomingPhoneNumberLocal create_incoming_phone_number_local(account_sid, phone_number, api_version, friendly_name, sms_application_sid, sms_fallback_method, sms_fallback_url, sms_method, sms_url, status_callback, status_callback_method, voice_application_sid, voice_caller_id_lookup, voice_fallback_method, voice_fallback_url, voice_method, voice_url, identity_sid, address_sid, emergency_status, emergency_address_sid, trunk_sid, voice_receive_mode, bundle_sid)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that will create the resource. | [required] |
**phone_number** | **String** | The phone number to purchase specified in [E.164](https://www.twilio.com/docs/glossary/what-e164) format.  E.164 phone numbers consist of a + followed by the country code and subscriber number without punctuation characters. For example, +14155551234. | [required] |
**api_version** | Option<**String**> | The API version to use for incoming calls made to the new phone number. The default is `2010-04-01`. |  |
**friendly_name** | Option<**String**> | A descriptive string that you created to describe the new phone number. It can be up to 64 characters long. By default, this is a formatted version of the phone number. |  |
**sms_application_sid** | Option<**String**> | The SID of the application that should handle SMS messages sent to the new phone number. If an `sms_application_sid` is present, we ignore all of the `sms_*_url` urls and use those set on the application. |  |
**sms_fallback_method** | Option<**String**> | The HTTP method that we should use to call `sms_fallback_url`. Can be: `GET` or `POST` and defaults to `POST`. |  |
**sms_fallback_url** | Option<**String**> | The URL that we should call when an error occurs while requesting or executing the TwiML defined by `sms_url`. |  |
**sms_method** | Option<**String**> | The HTTP method that we should use to call `sms_url`. Can be: `GET` or `POST` and defaults to `POST`. |  |
**sms_url** | Option<**String**> | The URL we should call when the new phone number receives an incoming SMS message. |  |
**status_callback** | Option<**String**> | The URL we should call using the `status_callback_method` to send status information to your application. |  |
**status_callback_method** | Option<**String**> | The HTTP method we should use to call `status_callback`. Can be: `GET` or `POST` and defaults to `POST`. |  |
**voice_application_sid** | Option<**String**> | The SID of the application we should use to handle calls to the new phone number. If a `voice_application_sid` is present, we ignore all of the voice urls and use only those set on the application. Setting a `voice_application_sid` will automatically delete your `trunk_sid` and vice versa. |  |
**voice_caller_id_lookup** | Option<**bool**> | Whether to lookup the caller's name from the CNAM database and post it to your app. Can be: `true` or `false` and defaults to `false`. |  |
**voice_fallback_method** | Option<**String**> | The HTTP method that we should use to call `voice_fallback_url`. Can be: `GET` or `POST` and defaults to `POST`. |  |
**voice_fallback_url** | Option<**String**> | The URL that we should call when an error occurs retrieving or executing the TwiML requested by `url`. |  |
**voice_method** | Option<**String**> | The HTTP method that we should use to call `voice_url`. Can be: `GET` or `POST` and defaults to `POST`. |  |
**voice_url** | Option<**String**> | The URL that we should call to answer a call to the new phone number. The `voice_url` will not be called if a `voice_application_sid` or a `trunk_sid` is set. |  |
**identity_sid** | Option<**String**> | The SID of the Identity resource that we should associate with the new phone number. Some regions require an identity to meet local regulations. |  |
**address_sid** | Option<**String**> | The SID of the Address resource we should associate with the new phone number. Some regions require addresses to meet local regulations. |  |
**emergency_status** | Option<**crate::models::IncomingPhoneNumberLocalEnumEmergencyStatus**> |  |  |
**emergency_address_sid** | Option<**String**> | The SID of the emergency address configuration to use for emergency calling from the new phone number. |  |
**trunk_sid** | Option<**String**> | The SID of the Trunk we should use to handle calls to the new phone number. If a `trunk_sid` is present, we ignore all of the voice urls and voice applications and use only those set on the Trunk. Setting a `trunk_sid` will automatically delete your `voice_application_sid` and vice versa. |  |
**voice_receive_mode** | Option<**crate::models::IncomingPhoneNumberLocalEnumVoiceReceiveMode**> |  |  |
**bundle_sid** | Option<**String**> | The SID of the Bundle resource that you associate with the phone number. Some regions require a Bundle to meet local Regulations. |  |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodIncomingPhoneNumberPeriodIncomingPhoneNumberLocal**](api.v2010.account.incoming_phone_number.incoming_phone_number_local.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_incoming_phone_number_mobile

> crate::models::ApiPeriodV2010PeriodAccountPeriodIncomingPhoneNumberPeriodIncomingPhoneNumberMobile create_incoming_phone_number_mobile(account_sid, phone_number, api_version, friendly_name, sms_application_sid, sms_fallback_method, sms_fallback_url, sms_method, sms_url, status_callback, status_callback_method, voice_application_sid, voice_caller_id_lookup, voice_fallback_method, voice_fallback_url, voice_method, voice_url, identity_sid, address_sid, emergency_status, emergency_address_sid, trunk_sid, voice_receive_mode, bundle_sid)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that will create the resource. | [required] |
**phone_number** | **String** | The phone number to purchase specified in [E.164](https://www.twilio.com/docs/glossary/what-e164) format.  E.164 phone numbers consist of a + followed by the country code and subscriber number without punctuation characters. For example, +14155551234. | [required] |
**api_version** | Option<**String**> | The API version to use for incoming calls made to the new phone number. The default is `2010-04-01`. |  |
**friendly_name** | Option<**String**> | A descriptive string that you created to describe the new phone number. It can be up to 64 characters long. By default, the is a formatted version of the phone number. |  |
**sms_application_sid** | Option<**String**> | The SID of the application that should handle SMS messages sent to the new phone number. If an `sms_application_sid` is present, we ignore all of the `sms_*_url` urls and use those of the application. |  |
**sms_fallback_method** | Option<**String**> | The HTTP method that we should use to call `sms_fallback_url`. Can be: `GET` or `POST` and defaults to `POST`. |  |
**sms_fallback_url** | Option<**String**> | The URL that we should call when an error occurs while requesting or executing the TwiML defined by `sms_url`. |  |
**sms_method** | Option<**String**> | The HTTP method that we should use to call `sms_url`. Can be: `GET` or `POST` and defaults to `POST`. |  |
**sms_url** | Option<**String**> | The URL we should call when the new phone number receives an incoming SMS message. |  |
**status_callback** | Option<**String**> | The URL we should call using the `status_callback_method` to send status information to your application. |  |
**status_callback_method** | Option<**String**> | The HTTP method we should use to call `status_callback`. Can be: `GET` or `POST` and defaults to `POST`. |  |
**voice_application_sid** | Option<**String**> | The SID of the application we should use to handle calls to the new phone number. If a `voice_application_sid` is present, we ignore all of the voice urls and use only those set on the application. Setting a `voice_application_sid` will automatically delete your `trunk_sid` and vice versa. |  |
**voice_caller_id_lookup** | Option<**bool**> | Whether to lookup the caller's name from the CNAM database and post it to your app. Can be: `true` or `false` and defaults to `false`. |  |
**voice_fallback_method** | Option<**String**> | The HTTP method that we should use to call `voice_fallback_url`. Can be: `GET` or `POST` and defaults to `POST`. |  |
**voice_fallback_url** | Option<**String**> | The URL that we should call when an error occurs retrieving or executing the TwiML requested by `url`. |  |
**voice_method** | Option<**String**> | The HTTP method that we should use to call `voice_url`. Can be: `GET` or `POST` and defaults to `POST`. |  |
**voice_url** | Option<**String**> | The URL that we should call to answer a call to the new phone number. The `voice_url` will not be called if a `voice_application_sid` or a `trunk_sid` is set. |  |
**identity_sid** | Option<**String**> | The SID of the Identity resource that we should associate with the new phone number. Some regions require an identity to meet local regulations. |  |
**address_sid** | Option<**String**> | The SID of the Address resource we should associate with the new phone number. Some regions require addresses to meet local regulations. |  |
**emergency_status** | Option<**crate::models::IncomingPhoneNumberMobileEnumEmergencyStatus**> |  |  |
**emergency_address_sid** | Option<**String**> | The SID of the emergency address configuration to use for emergency calling from the new phone number. |  |
**trunk_sid** | Option<**String**> | The SID of the Trunk we should use to handle calls to the new phone number. If a `trunk_sid` is present, we ignore all of the voice urls and voice applications and use only those set on the Trunk. Setting a `trunk_sid` will automatically delete your `voice_application_sid` and vice versa. |  |
**voice_receive_mode** | Option<**crate::models::IncomingPhoneNumberMobileEnumVoiceReceiveMode**> |  |  |
**bundle_sid** | Option<**String**> | The SID of the Bundle resource that you associate with the phone number. Some regions require a Bundle to meet local Regulations. |  |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodIncomingPhoneNumberPeriodIncomingPhoneNumberMobile**](api.v2010.account.incoming_phone_number.incoming_phone_number_mobile.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_incoming_phone_number_toll_free

> crate::models::ApiPeriodV2010PeriodAccountPeriodIncomingPhoneNumberPeriodIncomingPhoneNumberTollFree create_incoming_phone_number_toll_free(account_sid, phone_number, api_version, friendly_name, sms_application_sid, sms_fallback_method, sms_fallback_url, sms_method, sms_url, status_callback, status_callback_method, voice_application_sid, voice_caller_id_lookup, voice_fallback_method, voice_fallback_url, voice_method, voice_url, identity_sid, address_sid, emergency_status, emergency_address_sid, trunk_sid, voice_receive_mode, bundle_sid)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that will create the resource. | [required] |
**phone_number** | **String** | The phone number to purchase specified in [E.164](https://www.twilio.com/docs/glossary/what-e164) format.  E.164 phone numbers consist of a + followed by the country code and subscriber number without punctuation characters. For example, +14155551234. | [required] |
**api_version** | Option<**String**> | The API version to use for incoming calls made to the new phone number. The default is `2010-04-01`. |  |
**friendly_name** | Option<**String**> | A descriptive string that you created to describe the new phone number. It can be up to 64 characters long. By default, this is a formatted version of the phone number. |  |
**sms_application_sid** | Option<**String**> | The SID of the application that should handle SMS messages sent to the new phone number. If an `sms_application_sid` is present, we ignore all `sms_*_url` values and use those of the application. |  |
**sms_fallback_method** | Option<**String**> | The HTTP method that we should use to call `sms_fallback_url`. Can be: `GET` or `POST` and defaults to `POST`. |  |
**sms_fallback_url** | Option<**String**> | The URL that we should call when an error occurs while requesting or executing the TwiML defined by `sms_url`. |  |
**sms_method** | Option<**String**> | The HTTP method that we should use to call `sms_url`. Can be: `GET` or `POST` and defaults to `POST`. |  |
**sms_url** | Option<**String**> | The URL we should call when the new phone number receives an incoming SMS message. |  |
**status_callback** | Option<**String**> | The URL we should call using the `status_callback_method` to send status information to your application. |  |
**status_callback_method** | Option<**String**> | The HTTP method we should use to call `status_callback`. Can be: `GET` or `POST` and defaults to `POST`. |  |
**voice_application_sid** | Option<**String**> | The SID of the application we should use to handle calls to the new phone number. If a `voice_application_sid` is present, we ignore all of the voice urls and use those set on the application. Setting a `voice_application_sid` will automatically delete your `trunk_sid` and vice versa. |  |
**voice_caller_id_lookup** | Option<**bool**> | Whether to lookup the caller's name from the CNAM database and post it to your app. Can be: `true` or `false` and defaults to `false`. |  |
**voice_fallback_method** | Option<**String**> | The HTTP method that we should use to call `voice_fallback_url`. Can be: `GET` or `POST` and defaults to `POST`. |  |
**voice_fallback_url** | Option<**String**> | The URL that we should call when an error occurs retrieving or executing the TwiML requested by `url`. |  |
**voice_method** | Option<**String**> | The HTTP method that we should use to call `voice_url`. Can be: `GET` or `POST` and defaults to `POST`. |  |
**voice_url** | Option<**String**> | The URL that we should call to answer a call to the new phone number. The `voice_url` will not be called if a `voice_application_sid` or a `trunk_sid` is set. |  |
**identity_sid** | Option<**String**> | The SID of the Identity resource that we should associate with the new phone number. Some regions require an Identity to meet local regulations. |  |
**address_sid** | Option<**String**> | The SID of the Address resource we should associate with the new phone number. Some regions require addresses to meet local regulations. |  |
**emergency_status** | Option<**crate::models::IncomingPhoneNumberTollFreeEnumEmergencyStatus**> |  |  |
**emergency_address_sid** | Option<**String**> | The SID of the emergency address configuration to use for emergency calling from the new phone number. |  |
**trunk_sid** | Option<**String**> | The SID of the Trunk we should use to handle calls to the new phone number. If a `trunk_sid` is present, we ignore all of the voice urls and voice applications and use only those set on the Trunk. Setting a `trunk_sid` will automatically delete your `voice_application_sid` and vice versa. |  |
**voice_receive_mode** | Option<**crate::models::IncomingPhoneNumberTollFreeEnumVoiceReceiveMode**> |  |  |
**bundle_sid** | Option<**String**> | The SID of the Bundle resource that you associate with the phone number. Some regions require a Bundle to meet local Regulations. |  |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodIncomingPhoneNumberPeriodIncomingPhoneNumberTollFree**](api.v2010.account.incoming_phone_number.incoming_phone_number_toll_free.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_message

> crate::models::ApiPeriodV2010PeriodAccountPeriodMessage create_message(account_sid, to, status_callback, application_sid, max_price, provide_feedback, attempt, validity_period, force_delivery, content_retention, address_retention, smart_encoded, persistent_action, shorten_urls, schedule_type, send_at, send_as_mms, content_sid, content_variables, from, messaging_service_sid, body, media_url)


Send a message from the account used to make the request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that will create the resource. | [required] |
**to** | **String** | The destination phone number in [E.164](https://www.twilio.com/docs/glossary/what-e164) format for SMS/MMS or [Channel user address](https://www.twilio.com/docs/sms/channels#channel-addresses) for other 3rd-party channels. | [required] |
**status_callback** | Option<**String**> | The URL we should call using the `status_callback_method` to send status information to your application. If specified, we POST these message status changes to the URL: `queued`, `failed`, `sent`, `delivered`, or `undelivered`. Twilio will POST its [standard request parameters](https://www.twilio.com/docs/sms/twiml#request-parameters) as well as some additional parameters including `MessageSid`, `MessageStatus`, and `ErrorCode`. If you include this parameter with the `messaging_service_sid`, we use this URL instead of the Status Callback URL of the [Messaging Service](https://www.twilio.com/docs/sms/services/api). URLs must contain a valid hostname and underscores are not allowed. |  |
**application_sid** | Option<**String**> | The SID of the application that should receive message status. We POST a `message_sid` parameter and a `message_status` parameter with a value of `sent` or `failed` to the [application](https://www.twilio.com/docs/usage/api/applications)'s `message_status_callback`. If a `status_callback` parameter is also passed, it will be ignored and the application's `message_status_callback` parameter will be used. |  |
**max_price** | Option<**f32**> | The maximum total price in US dollars that you will pay for the message to be delivered. Can be a decimal value that has up to 4 decimal places. All messages are queued for delivery and the message cost is checked before the message is sent. If the cost exceeds `max_price`, the message will fail and a status of `Failed` is sent to the status callback. If `MaxPrice` is not set, the message cost is not checked. |  |
**provide_feedback** | Option<**bool**> | Whether to confirm delivery of the message. Set this value to `true` if you are sending messages that have a trackable user action and you intend to confirm delivery of the message using the [Message Feedback API](https://www.twilio.com/docs/sms/api/message-feedback-resource). This parameter is `false` by default. |  |
**attempt** | Option<**i32**> | Total number of attempts made ( including this ) to send out the message regardless of the provider used |  |
**validity_period** | Option<**i32**> | How long in seconds the message can remain in our outgoing message queue. After this period elapses, the message fails and we call your status callback. Can be between 1 and the default value of 14,400 seconds. After a message has been accepted by a carrier, however, we cannot guarantee that the message will not be queued after this period. We recommend that this value be at least 5 seconds. |  |
**force_delivery** | Option<**bool**> | Reserved |  |
**content_retention** | Option<**crate::models::MessageEnumContentRetention**> |  |  |
**address_retention** | Option<**crate::models::MessageEnumAddressRetention**> |  |  |
**smart_encoded** | Option<**bool**> | Whether to detect Unicode characters that have a similar GSM-7 character and replace them. Can be: `true` or `false`. |  |
**persistent_action** | Option<[**Vec<String>**](String.md)> | Rich actions for Channels Messages. |  |
**shorten_urls** | Option<**bool**> | Determines the usage of Click Tracking. Setting it to `true` will instruct Twilio to replace all links in the Message with a shortened version based on the associated Domain Sid and track clicks on them. If this parameter is not set on an API call, we will use the value set on the Messaging Service. If this parameter is not set and the value is not configured on the Messaging Service used this will default to `false`. |  |
**schedule_type** | Option<**crate::models::MessageEnumScheduleType**> |  |  |
**send_at** | Option<**String**> | The time that Twilio will send the message. Must be in ISO 8601 format. |  |
**send_as_mms** | Option<**bool**> | If set to True, Twilio will deliver the message as a single MMS message, regardless of the presence of media. |  |
**content_sid** | Option<**String**> | The SID of the Content object returned at Content API content create time (https://www.twilio.com/docs/content-api/create-and-send-your-first-content-api-template#create-a-template). If this parameter is not specified, then the Content API will not be utilized. |  |
**content_variables** | Option<**String**> | Key-value pairs of variable names to substitution values, used alongside a content_sid. If not specified, Content API will default to the default variables defined at create time. |  |
**from** | Option<**String**> | A Twilio phone number in [E.164](https://www.twilio.com/docs/glossary/what-e164) format, an [alphanumeric sender ID](https://www.twilio.com/docs/sms/send-messages#use-an-alphanumeric-sender-id), or a [Channel Endpoint address](https://www.twilio.com/docs/sms/channels#channel-addresses) that is enabled for the type of message you want to send. Phone numbers or [short codes](https://www.twilio.com/docs/sms/api/short-code) purchased from Twilio also work here. You cannot, for example, spoof messages from a private cell phone number. If you are using `messaging_service_sid`, this parameter must be empty. |  |
**messaging_service_sid** | Option<**String**> | The SID of the [Messaging Service](https://www.twilio.com/docs/sms/services#send-a-message-with-copilot) you want to associate with the Message. Set this parameter to use the [Messaging Service Settings and Copilot Features](https://www.twilio.com/console/sms/services) you have configured and leave the `from` parameter empty. When only this parameter is set, Twilio will use your enabled Copilot Features to select the `from` phone number for delivery. |  |
**body** | Option<**String**> | The text of the message you want to send. Can be up to 1,600 characters in length. |  |
**media_url** | Option<[**Vec<String>**](String.md)> | The URL of the media to send with the message. The media can be of type `gif`, `png`, and `jpeg` and will be formatted correctly on the recipient's device. The media size limit is 5MB for supported file types (JPEG, PNG, GIF) and 500KB for [other types](https://www.twilio.com/docs/sms/accepted-mime-types) of accepted media. To send more than one image in the message body, provide multiple `media_url` parameters in the POST request. You can include up to 10 `media_url` parameters per message. You can send images in an SMS message in only the US and Canada. |  |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodMessage**](api.v2010.account.message.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_message_feedback

> crate::models::ApiPeriodV2010PeriodAccountPeriodMessagePeriodMessageFeedback create_message_feedback(account_sid, message_sid, outcome)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that will create the resource. | [required] |
**message_sid** | **String** | The SID of the Message resource for which the feedback was provided. | [required] |
**outcome** | Option<**crate::models::MessageFeedbackEnumOutcome**> |  |  |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodMessagePeriodMessageFeedback**](api.v2010.account.message.message_feedback.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_new_key

> crate::models::ApiPeriodV2010PeriodAccountPeriodNewKey create_new_key(account_sid, friendly_name)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that will be responsible for the new Key resource. | [required] |
**friendly_name** | Option<**String**> | A descriptive string that you create to describe the resource. It can be up to 64 characters long. |  |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodNewKey**](api.v2010.account.new_key.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_new_signing_key

> crate::models::ApiPeriodV2010PeriodAccountPeriodNewSigningKey create_new_signing_key(account_sid, friendly_name)


Create a new Signing Key for the account making the request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that will be responsible for the new Key resource. | [required] |
**friendly_name** | Option<**String**> | A descriptive string that you create to describe the resource. It can be up to 64 characters long. |  |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodNewSigningKey**](api.v2010.account.new_signing_key.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_participant

> crate::models::ApiPeriodV2010PeriodAccountPeriodConferencePeriodParticipant create_participant(account_sid, conference_sid, from, to, status_callback, status_callback_method, status_callback_event, label, timeout, record, muted, beep, start_conference_on_enter, end_conference_on_exit, wait_url, wait_method, early_media, max_participants, conference_record, conference_trim, conference_status_callback, conference_status_callback_method, conference_status_callback_event, recording_channels, recording_status_callback, recording_status_callback_method, sip_auth_username, sip_auth_password, region, conference_recording_status_callback, conference_recording_status_callback_method, recording_status_callback_event, conference_recording_status_callback_event, coaching, call_sid_to_coach, jitter_buffer_size, byoc, caller_id, call_reason, recording_track, time_limit, machine_detection, machine_detection_timeout, machine_detection_speech_threshold, machine_detection_speech_end_threshold, machine_detection_silence_timeout, amd_status_callback, amd_status_callback_method)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that will create the resource. | [required] |
**conference_sid** | **String** | The SID of the participant's conference. | [required] |
**from** | **String** | The phone number, Client identifier, or username portion of SIP address that made this call. Phone numbers are in [E.164](https://www.twilio.com/docs/glossary/what-e164) format (e.g., +16175551212). Client identifiers are formatted `client:name`. If using a phone number, it must be a Twilio number or a Verified [outgoing caller id](https://www.twilio.com/docs/voice/api/outgoing-caller-ids) for your account. If the `to` parameter is a phone number, `from` must also be a phone number. If `to` is sip address, this value of `from` should be a username portion to be used to populate the P-Asserted-Identity header that is passed to the SIP endpoint. | [required] |
**to** | **String** | The phone number, SIP address, or Client identifier that received this call. Phone numbers are in [E.164](https://www.twilio.com/docs/glossary/what-e164) format (e.g., +16175551212). SIP addresses are formatted as `sip:name@company.com`. Client identifiers are formatted `client:name`. [Custom parameters](https://www.twilio.com/docs/voice/api/conference-participant-resource#custom-parameters) may also be specified. | [required] |
**status_callback** | Option<**String**> | The URL we should call using the `status_callback_method` to send status information to your application. |  |
**status_callback_method** | Option<**String**> | The HTTP method we should use to call `status_callback`. Can be: `GET` and `POST` and defaults to `POST`. |  |
**status_callback_event** | Option<[**Vec<String>**](String.md)> | The conference state changes that should generate a call to `status_callback`. Can be: `initiated`, `ringing`, `answered`, and `completed`. Separate multiple values with a space. The default value is `completed`. |  |
**label** | Option<**String**> | A label for this participant. If one is supplied, it may subsequently be used to fetch, update or delete the participant. |  |
**timeout** | Option<**i32**> | The number of seconds that we should allow the phone to ring before assuming there is no answer. Can be an integer between `5` and `600`, inclusive. The default value is `60`. We always add a 5-second timeout buffer to outgoing calls, so  value of 10 would result in an actual timeout that was closer to 15 seconds. |  |
**record** | Option<**bool**> | Whether to record the participant and their conferences, including the time between conferences. Can be `true` or `false` and the default is `false`. |  |
**muted** | Option<**bool**> | Whether the agent is muted in the conference. Can be `true` or `false` and the default is `false`. |  |
**beep** | Option<**String**> | Whether to play a notification beep to the conference when the participant joins. Can be: `true`, `false`, `onEnter`, or `onExit`. The default value is `true`. |  |
**start_conference_on_enter** | Option<**bool**> | Whether to start the conference when the participant joins, if it has not already started. Can be: `true` or `false` and the default is `true`. If `false` and the conference has not started, the participant is muted and hears background music until another participant starts the conference. |  |
**end_conference_on_exit** | Option<**bool**> | Whether to end the conference when the participant leaves. Can be: `true` or `false` and defaults to `false`. |  |
**wait_url** | Option<**String**> | The URL we should call using the `wait_method` for the music to play while participants are waiting for the conference to start. The default value is the URL of our standard hold music. [Learn more about hold music](https://www.twilio.com/labs/twimlets/holdmusic). |  |
**wait_method** | Option<**String**> | The HTTP method we should use to call `wait_url`. Can be `GET` or `POST` and the default is `POST`. When using a static audio file, this should be `GET` so that we can cache the file. |  |
**early_media** | Option<**bool**> | Whether to allow an agent to hear the state of the outbound call, including ringing or disconnect messages. Can be: `true` or `false` and defaults to `true`. |  |
**max_participants** | Option<**i32**> | The maximum number of participants in the conference. Can be a positive integer from `2` to `250`. The default value is `250`. |  |
**conference_record** | Option<**String**> | Whether to record the conference the participant is joining. Can be: `true`, `false`, `record-from-start`, and `do-not-record`. The default value is `false`. |  |
**conference_trim** | Option<**String**> | Whether to trim leading and trailing silence from your recorded conference audio files. Can be: `trim-silence` or `do-not-trim` and defaults to `trim-silence`. |  |
**conference_status_callback** | Option<**String**> | The URL we should call using the `conference_status_callback_method` when the conference events in `conference_status_callback_event` occur. Only the value set by the first participant to join the conference is used. Subsequent `conference_status_callback` values are ignored. |  |
**conference_status_callback_method** | Option<**String**> | The HTTP method we should use to call `conference_status_callback`. Can be: `GET` or `POST` and defaults to `POST`. |  |
**conference_status_callback_event** | Option<[**Vec<String>**](String.md)> | The conference state changes that should generate a call to `conference_status_callback`. Can be: `start`, `end`, `join`, `leave`, `mute`, `hold`, `modify`, `speaker`, and `announcement`. Separate multiple values with a space. Defaults to `start end`. |  |
**recording_channels** | Option<**String**> | The recording channels for the final recording. Can be: `mono` or `dual` and the default is `mono`. |  |
**recording_status_callback** | Option<**String**> | The URL that we should call using the `recording_status_callback_method` when the recording status changes. |  |
**recording_status_callback_method** | Option<**String**> | The HTTP method we should use when we call `recording_status_callback`. Can be: `GET` or `POST` and defaults to `POST`. |  |
**sip_auth_username** | Option<**String**> | The SIP username used for authentication. |  |
**sip_auth_password** | Option<**String**> | The SIP password for authentication. |  |
**region** | Option<**String**> | The [region](https://support.twilio.com/hc/en-us/articles/223132167-How-global-low-latency-routing-and-region-selection-work-for-conferences-and-Client-calls) where we should mix the recorded audio. Can be:`us1`, `ie1`, `de1`, `sg1`, `br1`, `au1`, or `jp1`. |  |
**conference_recording_status_callback** | Option<**String**> | The URL we should call using the `conference_recording_status_callback_method` when the conference recording is available. |  |
**conference_recording_status_callback_method** | Option<**String**> | The HTTP method we should use to call `conference_recording_status_callback`. Can be: `GET` or `POST` and defaults to `POST`. |  |
**recording_status_callback_event** | Option<[**Vec<String>**](String.md)> | The recording state changes that should generate a call to `recording_status_callback`. Can be: `started`, `in-progress`, `paused`, `resumed`, `stopped`, `completed`, `failed`, and `absent`. Separate multiple values with a space, ex: `'in-progress completed failed'`. |  |
**conference_recording_status_callback_event** | Option<[**Vec<String>**](String.md)> | The conference recording state changes that generate a call to `conference_recording_status_callback`. Can be: `in-progress`, `completed`, `failed`, and `absent`. Separate multiple values with a space, ex: `'in-progress completed failed'` |  |
**coaching** | Option<**bool**> | Whether the participant is coaching another call. Can be: `true` or `false`. If not present, defaults to `false` unless `call_sid_to_coach` is defined. If `true`, `call_sid_to_coach` must be defined. |  |
**call_sid_to_coach** | Option<**String**> | The SID of the participant who is being `coached`. The participant being coached is the only participant who can hear the participant who is `coaching`. |  |
**jitter_buffer_size** | Option<**String**> | Jitter buffer size for the connecting participant. Twilio will use this setting to apply Jitter Buffer before participant's audio is mixed into the conference. Can be: `off`, `small`, `medium`, and `large`. Default to `large`. |  |
**byoc** | Option<**String**> | The SID of a BYOC (Bring Your Own Carrier) trunk to route this call with. Note that `byoc` is only meaningful when `to` is a phone number; it will otherwise be ignored. (Beta) |  |
**caller_id** | Option<**String**> | The phone number, Client identifier, or username portion of SIP address that made this call. Phone numbers are in [E.164](https://www.twilio.com/docs/glossary/what-e164) format (e.g., +16175551212). Client identifiers are formatted `client:name`. If using a phone number, it must be a Twilio number or a Verified [outgoing caller id](https://www.twilio.com/docs/voice/api/outgoing-caller-ids) for your account. If the `to` parameter is a phone number, `callerId` must also be a phone number. If `to` is sip address, this value of `callerId` should be a username portion to be used to populate the From header that is passed to the SIP endpoint. |  |
**call_reason** | Option<**String**> | The Reason for the outgoing call. Use it to specify the purpose of the call that is presented on the called party's phone. (Branded Calls Beta) |  |
**recording_track** | Option<**String**> | The audio track to record for the call. Can be: `inbound`, `outbound` or `both`. The default is `both`. `inbound` records the audio that is received by Twilio. `outbound` records the audio that is sent from Twilio. `both` records the audio that is received and sent by Twilio. |  |
**time_limit** | Option<**i32**> | The maximum duration of the call in seconds. Constraints depend on account and configuration. |  |
**machine_detection** | Option<**String**> | Whether to detect if a human, answering machine, or fax has picked up the call. Can be: `Enable` or `DetectMessageEnd`. Use `Enable` if you would like us to return `AnsweredBy` as soon as the called party is identified. Use `DetectMessageEnd`, if you would like to leave a message on an answering machine. If `send_digits` is provided, this parameter is ignored. For more information, see [Answering Machine Detection](https://www.twilio.com/docs/voice/answering-machine-detection). |  |
**machine_detection_timeout** | Option<**i32**> | The number of seconds that we should attempt to detect an answering machine before timing out and sending a voice request with `AnsweredBy` of `unknown`. The default timeout is 30 seconds. |  |
**machine_detection_speech_threshold** | Option<**i32**> | The number of milliseconds that is used as the measuring stick for the length of the speech activity, where durations lower than this value will be interpreted as a human and longer than this value as a machine. Possible Values: 1000-6000. Default: 2400. |  |
**machine_detection_speech_end_threshold** | Option<**i32**> | The number of milliseconds of silence after speech activity at which point the speech activity is considered complete. Possible Values: 500-5000. Default: 1200. |  |
**machine_detection_silence_timeout** | Option<**i32**> | The number of milliseconds of initial silence after which an `unknown` AnsweredBy result will be returned. Possible Values: 2000-10000. Default: 5000. |  |
**amd_status_callback** | Option<**String**> | The URL that we should call using the `amd_status_callback_method` to notify customer application whether the call was answered by human, machine or fax. |  |
**amd_status_callback_method** | Option<**String**> | The HTTP method we should use when calling the `amd_status_callback` URL. Can be: `GET` or `POST` and the default is `POST`. |  |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodConferencePeriodParticipant**](api.v2010.account.conference.participant.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_payments

> crate::models::ApiPeriodV2010PeriodAccountPeriodCallPeriodPayments create_payments(account_sid, call_sid, idempotency_key, status_callback, bank_account_type, charge_amount, currency, description, input, min_postal_code_length, parameter, payment_connector, payment_method, postal_code, security_code, timeout, token_type, valid_card_types)


create an instance of payments. This will start a new payments session

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that will create the resource. | [required] |
**call_sid** | **String** | The SID of the call that will create the resource. Call leg associated with this sid is expected to provide payment information thru DTMF. | [required] |
**idempotency_key** | **String** | A unique token that will be used to ensure that multiple API calls with the same information do not result in multiple transactions. This should be a unique string value per API call and can be a randomly generated. | [required] |
**status_callback** | **String** | Provide an absolute or relative URL to receive status updates regarding your Pay session. Read more about the [expected StatusCallback values](https://www.twilio.com/docs/voice/api/payment-resource#statuscallback) | [required] |
**bank_account_type** | Option<**crate::models::PaymentsEnumBankAccountType**> |  |  |
**charge_amount** | Option<**f32**> | A positive decimal value less than 1,000,000 to charge against the credit card or bank account. Default currency can be overwritten with `currency` field. Leave blank or set to 0 to tokenize. |  |
**currency** | Option<**String**> | The currency of the `charge_amount`, formatted as [ISO 4127](http://www.iso.org/iso/home/standards/currency_codes.htm) format. The default value is `USD` and all values allowed from the Pay Connector are accepted. |  |
**description** | Option<**String**> | The description can be used to provide more details regarding the transaction. This information is submitted along with the payment details to the Payment Connector which are then posted on the transactions. |  |
**input** | Option<**String**> | A list of inputs that should be accepted. Currently only `dtmf` is supported. All digits captured during a pay session are redacted from the logs. |  |
**min_postal_code_length** | Option<**i32**> | A positive integer that is used to validate the length of the `PostalCode` inputted by the user. User must enter this many digits. |  |
**parameter** | Option<[**serde_json::Value**](serde_json::Value.md)> | A single-level JSON object used to pass custom parameters to payment processors. (Required for ACH payments). The information that has to be included here depends on the <Pay> Connector. [Read more](https://www.twilio.com/console/voice/pay-connectors). |  |
**payment_connector** | Option<**String**> | This is the unique name corresponding to the Pay Connector installed in the Twilio Add-ons. Learn more about [<Pay> Connectors](https://www.twilio.com/console/voice/pay-connectors). The default value is `Default`. |  |
**payment_method** | Option<**crate::models::PaymentsEnumPaymentMethod**> |  |  |
**postal_code** | Option<**bool**> | Indicates whether the credit card postal code (zip code) is a required piece of payment information that must be provided by the caller. The default is `true`. |  |
**security_code** | Option<**bool**> | Indicates whether the credit card security code is a required piece of payment information that must be provided by the caller. The default is `true`. |  |
**timeout** | Option<**i32**> | The number of seconds that <Pay> should wait for the caller to press a digit between each subsequent digit, after the first one, before moving on to validate the digits captured. The default is `5`, maximum is `600`. |  |
**token_type** | Option<**crate::models::PaymentsEnumTokenType**> |  |  |
**valid_card_types** | Option<**String**> | Credit card types separated by space that Pay should accept. The default value is `visa mastercard amex` |  |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodCallPeriodPayments**](api.v2010.account.call.payments.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_queue

> crate::models::ApiPeriodV2010PeriodAccountPeriodQueue create_queue(account_sid, friendly_name, max_size)


Create a queue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that will create the resource. | [required] |
**friendly_name** | **String** | A descriptive string that you created to describe this resource. It can be up to 64 characters long. | [required] |
**max_size** | Option<**i32**> | The maximum number of calls allowed to be in the queue. The default is 100. The maximum is 5000. |  |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodQueue**](api.v2010.account.queue.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_sip_auth_calls_credential_list_mapping

> crate::models::ApiPeriodV2010PeriodAccountPeriodSipPeriodSipDomainPeriodSipAuthPeriodSipAuthCallsPeriodSipAuthCallsCredentialListMapping create_sip_auth_calls_credential_list_mapping(account_sid, domain_sid, credential_list_sid)


Create a new credential list mapping resource

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that will create the resource. | [required] |
**domain_sid** | **String** | The SID of the SIP domain that will contain the new resource. | [required] |
**credential_list_sid** | **String** | The SID of the CredentialList resource to map to the SIP domain. | [required] |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodSipPeriodSipDomainPeriodSipAuthPeriodSipAuthCallsPeriodSipAuthCallsCredentialListMapping**](api.v2010.account.sip.sip_domain.sip_auth.sip_auth_calls.sip_auth_calls_credential_list_mapping.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_sip_auth_calls_ip_access_control_list_mapping

> crate::models::ApiPeriodV2010PeriodAccountPeriodSipPeriodSipDomainPeriodSipAuthPeriodSipAuthCallsPeriodSipAuthCallsIpAccessControlListMapping create_sip_auth_calls_ip_access_control_list_mapping(account_sid, domain_sid, ip_access_control_list_sid)


Create a new IP Access Control List mapping

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that will create the resource. | [required] |
**domain_sid** | **String** | The SID of the SIP domain that will contain the new resource. | [required] |
**ip_access_control_list_sid** | **String** | The SID of the IpAccessControlList resource to map to the SIP domain. | [required] |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodSipPeriodSipDomainPeriodSipAuthPeriodSipAuthCallsPeriodSipAuthCallsIpAccessControlListMapping**](api.v2010.account.sip.sip_domain.sip_auth.sip_auth_calls.sip_auth_calls_ip_access_control_list_mapping.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_sip_auth_registrations_credential_list_mapping

> crate::models::ApiPeriodV2010PeriodAccountPeriodSipPeriodSipDomainPeriodSipAuthPeriodSipAuthRegistrationsPeriodSipAuthRegistrationsCredentialListMapping create_sip_auth_registrations_credential_list_mapping(account_sid, domain_sid, credential_list_sid)


Create a new credential list mapping resource

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that will create the resource. | [required] |
**domain_sid** | **String** | The SID of the SIP domain that will contain the new resource. | [required] |
**credential_list_sid** | **String** | The SID of the CredentialList resource to map to the SIP domain. | [required] |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodSipPeriodSipDomainPeriodSipAuthPeriodSipAuthRegistrationsPeriodSipAuthRegistrationsCredentialListMapping**](api.v2010.account.sip.sip_domain.sip_auth.sip_auth_registrations.sip_auth_registrations_credential_list_mapping.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_sip_credential

> crate::models::ApiPeriodV2010PeriodAccountPeriodSipPeriodSipCredentialListPeriodSipCredential create_sip_credential(account_sid, credential_list_sid, username, password)


Create a new credential resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The unique id of the Account that is responsible for this resource. | [required] |
**credential_list_sid** | **String** | The unique id that identifies the credential list to include the created credential. | [required] |
**username** | **String** | The username that will be passed when authenticating SIP requests. The username should be sent in response to Twilio's challenge of the initial INVITE. It can be up to 32 characters long. | [required] |
**password** | **String** | The password that the username will use when authenticating SIP requests. The password must be a minimum of 12 characters, contain at least 1 digit, and have mixed case. (eg `IWasAtSignal2018`) | [required] |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodSipPeriodSipCredentialListPeriodSipCredential**](api.v2010.account.sip.sip_credential_list.sip_credential.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_sip_credential_list

> crate::models::ApiPeriodV2010PeriodAccountPeriodSipPeriodSipCredentialList create_sip_credential_list(account_sid, friendly_name)


Create a Credential List

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The unique id of the Account that is responsible for this resource. | [required] |
**friendly_name** | **String** | A human readable descriptive text that describes the CredentialList, up to 64 characters long. | [required] |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodSipPeriodSipCredentialList**](api.v2010.account.sip.sip_credential_list.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_sip_credential_list_mapping

> crate::models::ApiPeriodV2010PeriodAccountPeriodSipPeriodSipDomainPeriodSipCredentialListMapping create_sip_credential_list_mapping(account_sid, domain_sid, credential_list_sid)


Create a CredentialListMapping resource for an account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The unique id of the [Account](https://www.twilio.com/docs/iam/api/account) responsible for this resource. | [required] |
**domain_sid** | **String** | A 34 character string that uniquely identifies the SIP Domain for which the CredentialList resource will be mapped. | [required] |
**credential_list_sid** | **String** | A 34 character string that uniquely identifies the CredentialList resource to map to the SIP domain. | [required] |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodSipPeriodSipDomainPeriodSipCredentialListMapping**](api.v2010.account.sip.sip_domain.sip_credential_list_mapping.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_sip_domain

> crate::models::ApiPeriodV2010PeriodAccountPeriodSipPeriodSipDomain create_sip_domain(account_sid, domain_name, friendly_name, voice_url, voice_method, voice_fallback_url, voice_fallback_method, voice_status_callback_url, voice_status_callback_method, sip_registration, emergency_calling_enabled, secure, byoc_trunk_sid, emergency_caller_sid)


Create a new Domain

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that will create the resource. | [required] |
**domain_name** | **String** | The unique address you reserve on Twilio to which you route your SIP traffic. Domain names can contain letters, digits, and \\\"-\\\" and must end with `sip.twilio.com`. | [required] |
**friendly_name** | Option<**String**> | A descriptive string that you created to describe the resource. It can be up to 64 characters long. |  |
**voice_url** | Option<**String**> | The URL we should when the domain receives a call. |  |
**voice_method** | Option<**String**> | The HTTP method we should use to call `voice_url`. Can be: `GET` or `POST`. |  |
**voice_fallback_url** | Option<**String**> | The URL that we should call when an error occurs while retrieving or executing the TwiML from `voice_url`. |  |
**voice_fallback_method** | Option<**String**> | The HTTP method we should use to call `voice_fallback_url`. Can be: `GET` or `POST`. |  |
**voice_status_callback_url** | Option<**String**> | The URL that we should call to pass status parameters (such as call ended) to your application. |  |
**voice_status_callback_method** | Option<**String**> | The HTTP method we should use to call `voice_status_callback_url`. Can be: `GET` or `POST`. |  |
**sip_registration** | Option<**bool**> | Whether to allow SIP Endpoints to register with the domain to receive calls. Can be `true` or `false`. `true` allows SIP Endpoints to register with the domain to receive calls, `false` does not. |  |
**emergency_calling_enabled** | Option<**bool**> | Whether emergency calling is enabled for the domain. If enabled, allows emergency calls on the domain from phone numbers with validated addresses. |  |
**secure** | Option<**bool**> | Whether secure SIP is enabled for the domain. If enabled, TLS will be enforced and SRTP will be negotiated on all incoming calls to this sip domain. |  |
**byoc_trunk_sid** | Option<**String**> | The SID of the BYOC Trunk(Bring Your Own Carrier) resource that the Sip Domain will be associated with. |  |
**emergency_caller_sid** | Option<**String**> | Whether an emergency caller sid is configured for the domain. If present, this phone number will be used as the callback for the emergency call. |  |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodSipPeriodSipDomain**](api.v2010.account.sip.sip_domain.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_sip_ip_access_control_list

> crate::models::ApiPeriodV2010PeriodAccountPeriodSipPeriodSipIpAccessControlList create_sip_ip_access_control_list(account_sid, friendly_name)


Create a new IpAccessControlList resource

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The unique id of the [Account](https://www.twilio.com/docs/iam/api/account) responsible for this resource. | [required] |
**friendly_name** | **String** | A human readable descriptive text that describes the IpAccessControlList, up to 255 characters long. | [required] |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodSipPeriodSipIpAccessControlList**](api.v2010.account.sip.sip_ip_access_control_list.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_sip_ip_access_control_list_mapping

> crate::models::ApiPeriodV2010PeriodAccountPeriodSipPeriodSipDomainPeriodSipIpAccessControlListMapping create_sip_ip_access_control_list_mapping(account_sid, domain_sid, ip_access_control_list_sid)


Create a new IpAccessControlListMapping resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The unique id of the Account that is responsible for this resource. | [required] |
**domain_sid** | **String** | A 34 character string that uniquely identifies the SIP domain. | [required] |
**ip_access_control_list_sid** | **String** | The unique id of the IP access control list to map to the SIP domain. | [required] |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodSipPeriodSipDomainPeriodSipIpAccessControlListMapping**](api.v2010.account.sip.sip_domain.sip_ip_access_control_list_mapping.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_sip_ip_address

> crate::models::ApiPeriodV2010PeriodAccountPeriodSipPeriodSipIpAccessControlListPeriodSipIpAddress create_sip_ip_address(account_sid, ip_access_control_list_sid, friendly_name, ip_address, cidr_prefix_length)


Create a new IpAddress resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The unique id of the [Account](https://www.twilio.com/docs/iam/api/account) responsible for this resource. | [required] |
**ip_access_control_list_sid** | **String** | The IpAccessControlList Sid with which to associate the created IpAddress resource. | [required] |
**friendly_name** | **String** | A human readable descriptive text for this resource, up to 255 characters long. | [required] |
**ip_address** | **String** | An IP address in dotted decimal notation from which you want to accept traffic. Any SIP requests from this IP address will be allowed by Twilio. IPv4 only supported today. | [required] |
**cidr_prefix_length** | Option<**i32**> | An integer representing the length of the CIDR prefix to use with this IP address when accepting traffic. By default the entire IP address is used. |  |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodSipPeriodSipIpAccessControlListPeriodSipIpAddress**](api.v2010.account.sip.sip_ip_access_control_list.sip_ip_address.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_siprec

> crate::models::ApiPeriodV2010PeriodAccountPeriodCallPeriodSiprec create_siprec(account_sid, call_sid, name, connector_name, track, status_callback, status_callback_method, parameter1_period_name, parameter1_period_value, parameter2_period_name, parameter2_period_value, parameter3_period_name, parameter3_period_value, parameter4_period_name, parameter4_period_value, parameter5_period_name, parameter5_period_value, parameter6_period_name, parameter6_period_value, parameter7_period_name, parameter7_period_value, parameter8_period_name, parameter8_period_value, parameter9_period_name, parameter9_period_value, parameter10_period_name, parameter10_period_value, parameter11_period_name, parameter11_period_value, parameter12_period_name, parameter12_period_value, parameter13_period_name, parameter13_period_value, parameter14_period_name, parameter14_period_value, parameter15_period_name, parameter15_period_value, parameter16_period_name, parameter16_period_value, parameter17_period_name, parameter17_period_value, parameter18_period_name, parameter18_period_value, parameter19_period_name, parameter19_period_value, parameter20_period_name, parameter20_period_value, parameter21_period_name, parameter21_period_value, parameter22_period_name, parameter22_period_value, parameter23_period_name, parameter23_period_value, parameter24_period_name, parameter24_period_value, parameter25_period_name, parameter25_period_value, parameter26_period_name, parameter26_period_value, parameter27_period_name, parameter27_period_value, parameter28_period_name, parameter28_period_value, parameter29_period_name, parameter29_period_value, parameter30_period_name, parameter30_period_value, parameter31_period_name, parameter31_period_value, parameter32_period_name, parameter32_period_value, parameter33_period_name, parameter33_period_value, parameter34_period_name, parameter34_period_value, parameter35_period_name, parameter35_period_value, parameter36_period_name, parameter36_period_value, parameter37_period_name, parameter37_period_value, parameter38_period_name, parameter38_period_value, parameter39_period_name, parameter39_period_value, parameter40_period_name, parameter40_period_value, parameter41_period_name, parameter41_period_value, parameter42_period_name, parameter42_period_value, parameter43_period_name, parameter43_period_value, parameter44_period_name, parameter44_period_value, parameter45_period_name, parameter45_period_value, parameter46_period_name, parameter46_period_value, parameter47_period_name, parameter47_period_value, parameter48_period_name, parameter48_period_value, parameter49_period_name, parameter49_period_value, parameter50_period_name, parameter50_period_value, parameter51_period_name, parameter51_period_value, parameter52_period_name, parameter52_period_value, parameter53_period_name, parameter53_period_value, parameter54_period_name, parameter54_period_value, parameter55_period_name, parameter55_period_value, parameter56_period_name, parameter56_period_value, parameter57_period_name, parameter57_period_value, parameter58_period_name, parameter58_period_value, parameter59_period_name, parameter59_period_value, parameter60_period_name, parameter60_period_value, parameter61_period_name, parameter61_period_value, parameter62_period_name, parameter62_period_value, parameter63_period_name, parameter63_period_value, parameter64_period_name, parameter64_period_value, parameter65_period_name, parameter65_period_value, parameter66_period_name, parameter66_period_value, parameter67_period_name, parameter67_period_value, parameter68_period_name, parameter68_period_value, parameter69_period_name, parameter69_period_value, parameter70_period_name, parameter70_period_value, parameter71_period_name, parameter71_period_value, parameter72_period_name, parameter72_period_value, parameter73_period_name, parameter73_period_value, parameter74_period_name, parameter74_period_value, parameter75_period_name, parameter75_period_value, parameter76_period_name, parameter76_period_value, parameter77_period_name, parameter77_period_value, parameter78_period_name, parameter78_period_value, parameter79_period_name, parameter79_period_value, parameter80_period_name, parameter80_period_value, parameter81_period_name, parameter81_period_value, parameter82_period_name, parameter82_period_value, parameter83_period_name, parameter83_period_value, parameter84_period_name, parameter84_period_value, parameter85_period_name, parameter85_period_value, parameter86_period_name, parameter86_period_value, parameter87_period_name, parameter87_period_value, parameter88_period_name, parameter88_period_value, parameter89_period_name, parameter89_period_value, parameter90_period_name, parameter90_period_value, parameter91_period_name, parameter91_period_value, parameter92_period_name, parameter92_period_value, parameter93_period_name, parameter93_period_value, parameter94_period_name, parameter94_period_value, parameter95_period_name, parameter95_period_value, parameter96_period_name, parameter96_period_value, parameter97_period_name, parameter97_period_value, parameter98_period_name, parameter98_period_value, parameter99_period_name, parameter99_period_value)


Create a Siprec

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created this Siprec resource. | [required] |
**call_sid** | **String** | The SID of the [Call](https://www.twilio.com/docs/voice/api/call-resource) the Siprec resource is associated with. | [required] |
**name** | Option<**String**> | The user-specified name of this Siprec, if one was given when the Siprec was created. This may be used to stop the Siprec. |  |
**connector_name** | Option<**String**> | Unique name used when configuring the connector via Marketplace Add-on. |  |
**track** | Option<**crate::models::SiprecEnumTrack**> |  |  |
**status_callback** | Option<**String**> | Absolute URL of the status callback. |  |
**status_callback_method** | Option<**String**> | The http method for the status_callback (one of GET, POST). |  |
**parameter1_period_name** | Option<**String**> | Parameter name |  |
**parameter1_period_value** | Option<**String**> | Parameter value |  |
**parameter2_period_name** | Option<**String**> | Parameter name |  |
**parameter2_period_value** | Option<**String**> | Parameter value |  |
**parameter3_period_name** | Option<**String**> | Parameter name |  |
**parameter3_period_value** | Option<**String**> | Parameter value |  |
**parameter4_period_name** | Option<**String**> | Parameter name |  |
**parameter4_period_value** | Option<**String**> | Parameter value |  |
**parameter5_period_name** | Option<**String**> | Parameter name |  |
**parameter5_period_value** | Option<**String**> | Parameter value |  |
**parameter6_period_name** | Option<**String**> | Parameter name |  |
**parameter6_period_value** | Option<**String**> | Parameter value |  |
**parameter7_period_name** | Option<**String**> | Parameter name |  |
**parameter7_period_value** | Option<**String**> | Parameter value |  |
**parameter8_period_name** | Option<**String**> | Parameter name |  |
**parameter8_period_value** | Option<**String**> | Parameter value |  |
**parameter9_period_name** | Option<**String**> | Parameter name |  |
**parameter9_period_value** | Option<**String**> | Parameter value |  |
**parameter10_period_name** | Option<**String**> | Parameter name |  |
**parameter10_period_value** | Option<**String**> | Parameter value |  |
**parameter11_period_name** | Option<**String**> | Parameter name |  |
**parameter11_period_value** | Option<**String**> | Parameter value |  |
**parameter12_period_name** | Option<**String**> | Parameter name |  |
**parameter12_period_value** | Option<**String**> | Parameter value |  |
**parameter13_period_name** | Option<**String**> | Parameter name |  |
**parameter13_period_value** | Option<**String**> | Parameter value |  |
**parameter14_period_name** | Option<**String**> | Parameter name |  |
**parameter14_period_value** | Option<**String**> | Parameter value |  |
**parameter15_period_name** | Option<**String**> | Parameter name |  |
**parameter15_period_value** | Option<**String**> | Parameter value |  |
**parameter16_period_name** | Option<**String**> | Parameter name |  |
**parameter16_period_value** | Option<**String**> | Parameter value |  |
**parameter17_period_name** | Option<**String**> | Parameter name |  |
**parameter17_period_value** | Option<**String**> | Parameter value |  |
**parameter18_period_name** | Option<**String**> | Parameter name |  |
**parameter18_period_value** | Option<**String**> | Parameter value |  |
**parameter19_period_name** | Option<**String**> | Parameter name |  |
**parameter19_period_value** | Option<**String**> | Parameter value |  |
**parameter20_period_name** | Option<**String**> | Parameter name |  |
**parameter20_period_value** | Option<**String**> | Parameter value |  |
**parameter21_period_name** | Option<**String**> | Parameter name |  |
**parameter21_period_value** | Option<**String**> | Parameter value |  |
**parameter22_period_name** | Option<**String**> | Parameter name |  |
**parameter22_period_value** | Option<**String**> | Parameter value |  |
**parameter23_period_name** | Option<**String**> | Parameter name |  |
**parameter23_period_value** | Option<**String**> | Parameter value |  |
**parameter24_period_name** | Option<**String**> | Parameter name |  |
**parameter24_period_value** | Option<**String**> | Parameter value |  |
**parameter25_period_name** | Option<**String**> | Parameter name |  |
**parameter25_period_value** | Option<**String**> | Parameter value |  |
**parameter26_period_name** | Option<**String**> | Parameter name |  |
**parameter26_period_value** | Option<**String**> | Parameter value |  |
**parameter27_period_name** | Option<**String**> | Parameter name |  |
**parameter27_period_value** | Option<**String**> | Parameter value |  |
**parameter28_period_name** | Option<**String**> | Parameter name |  |
**parameter28_period_value** | Option<**String**> | Parameter value |  |
**parameter29_period_name** | Option<**String**> | Parameter name |  |
**parameter29_period_value** | Option<**String**> | Parameter value |  |
**parameter30_period_name** | Option<**String**> | Parameter name |  |
**parameter30_period_value** | Option<**String**> | Parameter value |  |
**parameter31_period_name** | Option<**String**> | Parameter name |  |
**parameter31_period_value** | Option<**String**> | Parameter value |  |
**parameter32_period_name** | Option<**String**> | Parameter name |  |
**parameter32_period_value** | Option<**String**> | Parameter value |  |
**parameter33_period_name** | Option<**String**> | Parameter name |  |
**parameter33_period_value** | Option<**String**> | Parameter value |  |
**parameter34_period_name** | Option<**String**> | Parameter name |  |
**parameter34_period_value** | Option<**String**> | Parameter value |  |
**parameter35_period_name** | Option<**String**> | Parameter name |  |
**parameter35_period_value** | Option<**String**> | Parameter value |  |
**parameter36_period_name** | Option<**String**> | Parameter name |  |
**parameter36_period_value** | Option<**String**> | Parameter value |  |
**parameter37_period_name** | Option<**String**> | Parameter name |  |
**parameter37_period_value** | Option<**String**> | Parameter value |  |
**parameter38_period_name** | Option<**String**> | Parameter name |  |
**parameter38_period_value** | Option<**String**> | Parameter value |  |
**parameter39_period_name** | Option<**String**> | Parameter name |  |
**parameter39_period_value** | Option<**String**> | Parameter value |  |
**parameter40_period_name** | Option<**String**> | Parameter name |  |
**parameter40_period_value** | Option<**String**> | Parameter value |  |
**parameter41_period_name** | Option<**String**> | Parameter name |  |
**parameter41_period_value** | Option<**String**> | Parameter value |  |
**parameter42_period_name** | Option<**String**> | Parameter name |  |
**parameter42_period_value** | Option<**String**> | Parameter value |  |
**parameter43_period_name** | Option<**String**> | Parameter name |  |
**parameter43_period_value** | Option<**String**> | Parameter value |  |
**parameter44_period_name** | Option<**String**> | Parameter name |  |
**parameter44_period_value** | Option<**String**> | Parameter value |  |
**parameter45_period_name** | Option<**String**> | Parameter name |  |
**parameter45_period_value** | Option<**String**> | Parameter value |  |
**parameter46_period_name** | Option<**String**> | Parameter name |  |
**parameter46_period_value** | Option<**String**> | Parameter value |  |
**parameter47_period_name** | Option<**String**> | Parameter name |  |
**parameter47_period_value** | Option<**String**> | Parameter value |  |
**parameter48_period_name** | Option<**String**> | Parameter name |  |
**parameter48_period_value** | Option<**String**> | Parameter value |  |
**parameter49_period_name** | Option<**String**> | Parameter name |  |
**parameter49_period_value** | Option<**String**> | Parameter value |  |
**parameter50_period_name** | Option<**String**> | Parameter name |  |
**parameter50_period_value** | Option<**String**> | Parameter value |  |
**parameter51_period_name** | Option<**String**> | Parameter name |  |
**parameter51_period_value** | Option<**String**> | Parameter value |  |
**parameter52_period_name** | Option<**String**> | Parameter name |  |
**parameter52_period_value** | Option<**String**> | Parameter value |  |
**parameter53_period_name** | Option<**String**> | Parameter name |  |
**parameter53_period_value** | Option<**String**> | Parameter value |  |
**parameter54_period_name** | Option<**String**> | Parameter name |  |
**parameter54_period_value** | Option<**String**> | Parameter value |  |
**parameter55_period_name** | Option<**String**> | Parameter name |  |
**parameter55_period_value** | Option<**String**> | Parameter value |  |
**parameter56_period_name** | Option<**String**> | Parameter name |  |
**parameter56_period_value** | Option<**String**> | Parameter value |  |
**parameter57_period_name** | Option<**String**> | Parameter name |  |
**parameter57_period_value** | Option<**String**> | Parameter value |  |
**parameter58_period_name** | Option<**String**> | Parameter name |  |
**parameter58_period_value** | Option<**String**> | Parameter value |  |
**parameter59_period_name** | Option<**String**> | Parameter name |  |
**parameter59_period_value** | Option<**String**> | Parameter value |  |
**parameter60_period_name** | Option<**String**> | Parameter name |  |
**parameter60_period_value** | Option<**String**> | Parameter value |  |
**parameter61_period_name** | Option<**String**> | Parameter name |  |
**parameter61_period_value** | Option<**String**> | Parameter value |  |
**parameter62_period_name** | Option<**String**> | Parameter name |  |
**parameter62_period_value** | Option<**String**> | Parameter value |  |
**parameter63_period_name** | Option<**String**> | Parameter name |  |
**parameter63_period_value** | Option<**String**> | Parameter value |  |
**parameter64_period_name** | Option<**String**> | Parameter name |  |
**parameter64_period_value** | Option<**String**> | Parameter value |  |
**parameter65_period_name** | Option<**String**> | Parameter name |  |
**parameter65_period_value** | Option<**String**> | Parameter value |  |
**parameter66_period_name** | Option<**String**> | Parameter name |  |
**parameter66_period_value** | Option<**String**> | Parameter value |  |
**parameter67_period_name** | Option<**String**> | Parameter name |  |
**parameter67_period_value** | Option<**String**> | Parameter value |  |
**parameter68_period_name** | Option<**String**> | Parameter name |  |
**parameter68_period_value** | Option<**String**> | Parameter value |  |
**parameter69_period_name** | Option<**String**> | Parameter name |  |
**parameter69_period_value** | Option<**String**> | Parameter value |  |
**parameter70_period_name** | Option<**String**> | Parameter name |  |
**parameter70_period_value** | Option<**String**> | Parameter value |  |
**parameter71_period_name** | Option<**String**> | Parameter name |  |
**parameter71_period_value** | Option<**String**> | Parameter value |  |
**parameter72_period_name** | Option<**String**> | Parameter name |  |
**parameter72_period_value** | Option<**String**> | Parameter value |  |
**parameter73_period_name** | Option<**String**> | Parameter name |  |
**parameter73_period_value** | Option<**String**> | Parameter value |  |
**parameter74_period_name** | Option<**String**> | Parameter name |  |
**parameter74_period_value** | Option<**String**> | Parameter value |  |
**parameter75_period_name** | Option<**String**> | Parameter name |  |
**parameter75_period_value** | Option<**String**> | Parameter value |  |
**parameter76_period_name** | Option<**String**> | Parameter name |  |
**parameter76_period_value** | Option<**String**> | Parameter value |  |
**parameter77_period_name** | Option<**String**> | Parameter name |  |
**parameter77_period_value** | Option<**String**> | Parameter value |  |
**parameter78_period_name** | Option<**String**> | Parameter name |  |
**parameter78_period_value** | Option<**String**> | Parameter value |  |
**parameter79_period_name** | Option<**String**> | Parameter name |  |
**parameter79_period_value** | Option<**String**> | Parameter value |  |
**parameter80_period_name** | Option<**String**> | Parameter name |  |
**parameter80_period_value** | Option<**String**> | Parameter value |  |
**parameter81_period_name** | Option<**String**> | Parameter name |  |
**parameter81_period_value** | Option<**String**> | Parameter value |  |
**parameter82_period_name** | Option<**String**> | Parameter name |  |
**parameter82_period_value** | Option<**String**> | Parameter value |  |
**parameter83_period_name** | Option<**String**> | Parameter name |  |
**parameter83_period_value** | Option<**String**> | Parameter value |  |
**parameter84_period_name** | Option<**String**> | Parameter name |  |
**parameter84_period_value** | Option<**String**> | Parameter value |  |
**parameter85_period_name** | Option<**String**> | Parameter name |  |
**parameter85_period_value** | Option<**String**> | Parameter value |  |
**parameter86_period_name** | Option<**String**> | Parameter name |  |
**parameter86_period_value** | Option<**String**> | Parameter value |  |
**parameter87_period_name** | Option<**String**> | Parameter name |  |
**parameter87_period_value** | Option<**String**> | Parameter value |  |
**parameter88_period_name** | Option<**String**> | Parameter name |  |
**parameter88_period_value** | Option<**String**> | Parameter value |  |
**parameter89_period_name** | Option<**String**> | Parameter name |  |
**parameter89_period_value** | Option<**String**> | Parameter value |  |
**parameter90_period_name** | Option<**String**> | Parameter name |  |
**parameter90_period_value** | Option<**String**> | Parameter value |  |
**parameter91_period_name** | Option<**String**> | Parameter name |  |
**parameter91_period_value** | Option<**String**> | Parameter value |  |
**parameter92_period_name** | Option<**String**> | Parameter name |  |
**parameter92_period_value** | Option<**String**> | Parameter value |  |
**parameter93_period_name** | Option<**String**> | Parameter name |  |
**parameter93_period_value** | Option<**String**> | Parameter value |  |
**parameter94_period_name** | Option<**String**> | Parameter name |  |
**parameter94_period_value** | Option<**String**> | Parameter value |  |
**parameter95_period_name** | Option<**String**> | Parameter name |  |
**parameter95_period_value** | Option<**String**> | Parameter value |  |
**parameter96_period_name** | Option<**String**> | Parameter name |  |
**parameter96_period_value** | Option<**String**> | Parameter value |  |
**parameter97_period_name** | Option<**String**> | Parameter name |  |
**parameter97_period_value** | Option<**String**> | Parameter value |  |
**parameter98_period_name** | Option<**String**> | Parameter name |  |
**parameter98_period_value** | Option<**String**> | Parameter value |  |
**parameter99_period_name** | Option<**String**> | Parameter name |  |
**parameter99_period_value** | Option<**String**> | Parameter value |  |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodCallPeriodSiprec**](api.v2010.account.call.siprec.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_stream

> crate::models::ApiPeriodV2010PeriodAccountPeriodCallPeriodStream create_stream(account_sid, call_sid, url, name, track, status_callback, status_callback_method, parameter1_period_name, parameter1_period_value, parameter2_period_name, parameter2_period_value, parameter3_period_name, parameter3_period_value, parameter4_period_name, parameter4_period_value, parameter5_period_name, parameter5_period_value, parameter6_period_name, parameter6_period_value, parameter7_period_name, parameter7_period_value, parameter8_period_name, parameter8_period_value, parameter9_period_name, parameter9_period_value, parameter10_period_name, parameter10_period_value, parameter11_period_name, parameter11_period_value, parameter12_period_name, parameter12_period_value, parameter13_period_name, parameter13_period_value, parameter14_period_name, parameter14_period_value, parameter15_period_name, parameter15_period_value, parameter16_period_name, parameter16_period_value, parameter17_period_name, parameter17_period_value, parameter18_period_name, parameter18_period_value, parameter19_period_name, parameter19_period_value, parameter20_period_name, parameter20_period_value, parameter21_period_name, parameter21_period_value, parameter22_period_name, parameter22_period_value, parameter23_period_name, parameter23_period_value, parameter24_period_name, parameter24_period_value, parameter25_period_name, parameter25_period_value, parameter26_period_name, parameter26_period_value, parameter27_period_name, parameter27_period_value, parameter28_period_name, parameter28_period_value, parameter29_period_name, parameter29_period_value, parameter30_period_name, parameter30_period_value, parameter31_period_name, parameter31_period_value, parameter32_period_name, parameter32_period_value, parameter33_period_name, parameter33_period_value, parameter34_period_name, parameter34_period_value, parameter35_period_name, parameter35_period_value, parameter36_period_name, parameter36_period_value, parameter37_period_name, parameter37_period_value, parameter38_period_name, parameter38_period_value, parameter39_period_name, parameter39_period_value, parameter40_period_name, parameter40_period_value, parameter41_period_name, parameter41_period_value, parameter42_period_name, parameter42_period_value, parameter43_period_name, parameter43_period_value, parameter44_period_name, parameter44_period_value, parameter45_period_name, parameter45_period_value, parameter46_period_name, parameter46_period_value, parameter47_period_name, parameter47_period_value, parameter48_period_name, parameter48_period_value, parameter49_period_name, parameter49_period_value, parameter50_period_name, parameter50_period_value, parameter51_period_name, parameter51_period_value, parameter52_period_name, parameter52_period_value, parameter53_period_name, parameter53_period_value, parameter54_period_name, parameter54_period_value, parameter55_period_name, parameter55_period_value, parameter56_period_name, parameter56_period_value, parameter57_period_name, parameter57_period_value, parameter58_period_name, parameter58_period_value, parameter59_period_name, parameter59_period_value, parameter60_period_name, parameter60_period_value, parameter61_period_name, parameter61_period_value, parameter62_period_name, parameter62_period_value, parameter63_period_name, parameter63_period_value, parameter64_period_name, parameter64_period_value, parameter65_period_name, parameter65_period_value, parameter66_period_name, parameter66_period_value, parameter67_period_name, parameter67_period_value, parameter68_period_name, parameter68_period_value, parameter69_period_name, parameter69_period_value, parameter70_period_name, parameter70_period_value, parameter71_period_name, parameter71_period_value, parameter72_period_name, parameter72_period_value, parameter73_period_name, parameter73_period_value, parameter74_period_name, parameter74_period_value, parameter75_period_name, parameter75_period_value, parameter76_period_name, parameter76_period_value, parameter77_period_name, parameter77_period_value, parameter78_period_name, parameter78_period_value, parameter79_period_name, parameter79_period_value, parameter80_period_name, parameter80_period_value, parameter81_period_name, parameter81_period_value, parameter82_period_name, parameter82_period_value, parameter83_period_name, parameter83_period_value, parameter84_period_name, parameter84_period_value, parameter85_period_name, parameter85_period_value, parameter86_period_name, parameter86_period_value, parameter87_period_name, parameter87_period_value, parameter88_period_name, parameter88_period_value, parameter89_period_name, parameter89_period_value, parameter90_period_name, parameter90_period_value, parameter91_period_name, parameter91_period_value, parameter92_period_name, parameter92_period_value, parameter93_period_name, parameter93_period_value, parameter94_period_name, parameter94_period_value, parameter95_period_name, parameter95_period_value, parameter96_period_name, parameter96_period_value, parameter97_period_name, parameter97_period_value, parameter98_period_name, parameter98_period_value, parameter99_period_name, parameter99_period_value)


Create a Stream

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created this Stream resource. | [required] |
**call_sid** | **String** | The SID of the [Call](https://www.twilio.com/docs/voice/api/call-resource) the Stream resource is associated with. | [required] |
**url** | **String** | Relative or absolute url where WebSocket connection will be established. | [required] |
**name** | Option<**String**> | The user-specified name of this Stream, if one was given when the Stream was created. This may be used to stop the Stream. |  |
**track** | Option<**crate::models::StreamEnumTrack**> |  |  |
**status_callback** | Option<**String**> | Absolute URL of the status callback. |  |
**status_callback_method** | Option<**String**> | The http method for the status_callback (one of GET, POST). |  |
**parameter1_period_name** | Option<**String**> | Parameter name |  |
**parameter1_period_value** | Option<**String**> | Parameter value |  |
**parameter2_period_name** | Option<**String**> | Parameter name |  |
**parameter2_period_value** | Option<**String**> | Parameter value |  |
**parameter3_period_name** | Option<**String**> | Parameter name |  |
**parameter3_period_value** | Option<**String**> | Parameter value |  |
**parameter4_period_name** | Option<**String**> | Parameter name |  |
**parameter4_period_value** | Option<**String**> | Parameter value |  |
**parameter5_period_name** | Option<**String**> | Parameter name |  |
**parameter5_period_value** | Option<**String**> | Parameter value |  |
**parameter6_period_name** | Option<**String**> | Parameter name |  |
**parameter6_period_value** | Option<**String**> | Parameter value |  |
**parameter7_period_name** | Option<**String**> | Parameter name |  |
**parameter7_period_value** | Option<**String**> | Parameter value |  |
**parameter8_period_name** | Option<**String**> | Parameter name |  |
**parameter8_period_value** | Option<**String**> | Parameter value |  |
**parameter9_period_name** | Option<**String**> | Parameter name |  |
**parameter9_period_value** | Option<**String**> | Parameter value |  |
**parameter10_period_name** | Option<**String**> | Parameter name |  |
**parameter10_period_value** | Option<**String**> | Parameter value |  |
**parameter11_period_name** | Option<**String**> | Parameter name |  |
**parameter11_period_value** | Option<**String**> | Parameter value |  |
**parameter12_period_name** | Option<**String**> | Parameter name |  |
**parameter12_period_value** | Option<**String**> | Parameter value |  |
**parameter13_period_name** | Option<**String**> | Parameter name |  |
**parameter13_period_value** | Option<**String**> | Parameter value |  |
**parameter14_period_name** | Option<**String**> | Parameter name |  |
**parameter14_period_value** | Option<**String**> | Parameter value |  |
**parameter15_period_name** | Option<**String**> | Parameter name |  |
**parameter15_period_value** | Option<**String**> | Parameter value |  |
**parameter16_period_name** | Option<**String**> | Parameter name |  |
**parameter16_period_value** | Option<**String**> | Parameter value |  |
**parameter17_period_name** | Option<**String**> | Parameter name |  |
**parameter17_period_value** | Option<**String**> | Parameter value |  |
**parameter18_period_name** | Option<**String**> | Parameter name |  |
**parameter18_period_value** | Option<**String**> | Parameter value |  |
**parameter19_period_name** | Option<**String**> | Parameter name |  |
**parameter19_period_value** | Option<**String**> | Parameter value |  |
**parameter20_period_name** | Option<**String**> | Parameter name |  |
**parameter20_period_value** | Option<**String**> | Parameter value |  |
**parameter21_period_name** | Option<**String**> | Parameter name |  |
**parameter21_period_value** | Option<**String**> | Parameter value |  |
**parameter22_period_name** | Option<**String**> | Parameter name |  |
**parameter22_period_value** | Option<**String**> | Parameter value |  |
**parameter23_period_name** | Option<**String**> | Parameter name |  |
**parameter23_period_value** | Option<**String**> | Parameter value |  |
**parameter24_period_name** | Option<**String**> | Parameter name |  |
**parameter24_period_value** | Option<**String**> | Parameter value |  |
**parameter25_period_name** | Option<**String**> | Parameter name |  |
**parameter25_period_value** | Option<**String**> | Parameter value |  |
**parameter26_period_name** | Option<**String**> | Parameter name |  |
**parameter26_period_value** | Option<**String**> | Parameter value |  |
**parameter27_period_name** | Option<**String**> | Parameter name |  |
**parameter27_period_value** | Option<**String**> | Parameter value |  |
**parameter28_period_name** | Option<**String**> | Parameter name |  |
**parameter28_period_value** | Option<**String**> | Parameter value |  |
**parameter29_period_name** | Option<**String**> | Parameter name |  |
**parameter29_period_value** | Option<**String**> | Parameter value |  |
**parameter30_period_name** | Option<**String**> | Parameter name |  |
**parameter30_period_value** | Option<**String**> | Parameter value |  |
**parameter31_period_name** | Option<**String**> | Parameter name |  |
**parameter31_period_value** | Option<**String**> | Parameter value |  |
**parameter32_period_name** | Option<**String**> | Parameter name |  |
**parameter32_period_value** | Option<**String**> | Parameter value |  |
**parameter33_period_name** | Option<**String**> | Parameter name |  |
**parameter33_period_value** | Option<**String**> | Parameter value |  |
**parameter34_period_name** | Option<**String**> | Parameter name |  |
**parameter34_period_value** | Option<**String**> | Parameter value |  |
**parameter35_period_name** | Option<**String**> | Parameter name |  |
**parameter35_period_value** | Option<**String**> | Parameter value |  |
**parameter36_period_name** | Option<**String**> | Parameter name |  |
**parameter36_period_value** | Option<**String**> | Parameter value |  |
**parameter37_period_name** | Option<**String**> | Parameter name |  |
**parameter37_period_value** | Option<**String**> | Parameter value |  |
**parameter38_period_name** | Option<**String**> | Parameter name |  |
**parameter38_period_value** | Option<**String**> | Parameter value |  |
**parameter39_period_name** | Option<**String**> | Parameter name |  |
**parameter39_period_value** | Option<**String**> | Parameter value |  |
**parameter40_period_name** | Option<**String**> | Parameter name |  |
**parameter40_period_value** | Option<**String**> | Parameter value |  |
**parameter41_period_name** | Option<**String**> | Parameter name |  |
**parameter41_period_value** | Option<**String**> | Parameter value |  |
**parameter42_period_name** | Option<**String**> | Parameter name |  |
**parameter42_period_value** | Option<**String**> | Parameter value |  |
**parameter43_period_name** | Option<**String**> | Parameter name |  |
**parameter43_period_value** | Option<**String**> | Parameter value |  |
**parameter44_period_name** | Option<**String**> | Parameter name |  |
**parameter44_period_value** | Option<**String**> | Parameter value |  |
**parameter45_period_name** | Option<**String**> | Parameter name |  |
**parameter45_period_value** | Option<**String**> | Parameter value |  |
**parameter46_period_name** | Option<**String**> | Parameter name |  |
**parameter46_period_value** | Option<**String**> | Parameter value |  |
**parameter47_period_name** | Option<**String**> | Parameter name |  |
**parameter47_period_value** | Option<**String**> | Parameter value |  |
**parameter48_period_name** | Option<**String**> | Parameter name |  |
**parameter48_period_value** | Option<**String**> | Parameter value |  |
**parameter49_period_name** | Option<**String**> | Parameter name |  |
**parameter49_period_value** | Option<**String**> | Parameter value |  |
**parameter50_period_name** | Option<**String**> | Parameter name |  |
**parameter50_period_value** | Option<**String**> | Parameter value |  |
**parameter51_period_name** | Option<**String**> | Parameter name |  |
**parameter51_period_value** | Option<**String**> | Parameter value |  |
**parameter52_period_name** | Option<**String**> | Parameter name |  |
**parameter52_period_value** | Option<**String**> | Parameter value |  |
**parameter53_period_name** | Option<**String**> | Parameter name |  |
**parameter53_period_value** | Option<**String**> | Parameter value |  |
**parameter54_period_name** | Option<**String**> | Parameter name |  |
**parameter54_period_value** | Option<**String**> | Parameter value |  |
**parameter55_period_name** | Option<**String**> | Parameter name |  |
**parameter55_period_value** | Option<**String**> | Parameter value |  |
**parameter56_period_name** | Option<**String**> | Parameter name |  |
**parameter56_period_value** | Option<**String**> | Parameter value |  |
**parameter57_period_name** | Option<**String**> | Parameter name |  |
**parameter57_period_value** | Option<**String**> | Parameter value |  |
**parameter58_period_name** | Option<**String**> | Parameter name |  |
**parameter58_period_value** | Option<**String**> | Parameter value |  |
**parameter59_period_name** | Option<**String**> | Parameter name |  |
**parameter59_period_value** | Option<**String**> | Parameter value |  |
**parameter60_period_name** | Option<**String**> | Parameter name |  |
**parameter60_period_value** | Option<**String**> | Parameter value |  |
**parameter61_period_name** | Option<**String**> | Parameter name |  |
**parameter61_period_value** | Option<**String**> | Parameter value |  |
**parameter62_period_name** | Option<**String**> | Parameter name |  |
**parameter62_period_value** | Option<**String**> | Parameter value |  |
**parameter63_period_name** | Option<**String**> | Parameter name |  |
**parameter63_period_value** | Option<**String**> | Parameter value |  |
**parameter64_period_name** | Option<**String**> | Parameter name |  |
**parameter64_period_value** | Option<**String**> | Parameter value |  |
**parameter65_period_name** | Option<**String**> | Parameter name |  |
**parameter65_period_value** | Option<**String**> | Parameter value |  |
**parameter66_period_name** | Option<**String**> | Parameter name |  |
**parameter66_period_value** | Option<**String**> | Parameter value |  |
**parameter67_period_name** | Option<**String**> | Parameter name |  |
**parameter67_period_value** | Option<**String**> | Parameter value |  |
**parameter68_period_name** | Option<**String**> | Parameter name |  |
**parameter68_period_value** | Option<**String**> | Parameter value |  |
**parameter69_period_name** | Option<**String**> | Parameter name |  |
**parameter69_period_value** | Option<**String**> | Parameter value |  |
**parameter70_period_name** | Option<**String**> | Parameter name |  |
**parameter70_period_value** | Option<**String**> | Parameter value |  |
**parameter71_period_name** | Option<**String**> | Parameter name |  |
**parameter71_period_value** | Option<**String**> | Parameter value |  |
**parameter72_period_name** | Option<**String**> | Parameter name |  |
**parameter72_period_value** | Option<**String**> | Parameter value |  |
**parameter73_period_name** | Option<**String**> | Parameter name |  |
**parameter73_period_value** | Option<**String**> | Parameter value |  |
**parameter74_period_name** | Option<**String**> | Parameter name |  |
**parameter74_period_value** | Option<**String**> | Parameter value |  |
**parameter75_period_name** | Option<**String**> | Parameter name |  |
**parameter75_period_value** | Option<**String**> | Parameter value |  |
**parameter76_period_name** | Option<**String**> | Parameter name |  |
**parameter76_period_value** | Option<**String**> | Parameter value |  |
**parameter77_period_name** | Option<**String**> | Parameter name |  |
**parameter77_period_value** | Option<**String**> | Parameter value |  |
**parameter78_period_name** | Option<**String**> | Parameter name |  |
**parameter78_period_value** | Option<**String**> | Parameter value |  |
**parameter79_period_name** | Option<**String**> | Parameter name |  |
**parameter79_period_value** | Option<**String**> | Parameter value |  |
**parameter80_period_name** | Option<**String**> | Parameter name |  |
**parameter80_period_value** | Option<**String**> | Parameter value |  |
**parameter81_period_name** | Option<**String**> | Parameter name |  |
**parameter81_period_value** | Option<**String**> | Parameter value |  |
**parameter82_period_name** | Option<**String**> | Parameter name |  |
**parameter82_period_value** | Option<**String**> | Parameter value |  |
**parameter83_period_name** | Option<**String**> | Parameter name |  |
**parameter83_period_value** | Option<**String**> | Parameter value |  |
**parameter84_period_name** | Option<**String**> | Parameter name |  |
**parameter84_period_value** | Option<**String**> | Parameter value |  |
**parameter85_period_name** | Option<**String**> | Parameter name |  |
**parameter85_period_value** | Option<**String**> | Parameter value |  |
**parameter86_period_name** | Option<**String**> | Parameter name |  |
**parameter86_period_value** | Option<**String**> | Parameter value |  |
**parameter87_period_name** | Option<**String**> | Parameter name |  |
**parameter87_period_value** | Option<**String**> | Parameter value |  |
**parameter88_period_name** | Option<**String**> | Parameter name |  |
**parameter88_period_value** | Option<**String**> | Parameter value |  |
**parameter89_period_name** | Option<**String**> | Parameter name |  |
**parameter89_period_value** | Option<**String**> | Parameter value |  |
**parameter90_period_name** | Option<**String**> | Parameter name |  |
**parameter90_period_value** | Option<**String**> | Parameter value |  |
**parameter91_period_name** | Option<**String**> | Parameter name |  |
**parameter91_period_value** | Option<**String**> | Parameter value |  |
**parameter92_period_name** | Option<**String**> | Parameter name |  |
**parameter92_period_value** | Option<**String**> | Parameter value |  |
**parameter93_period_name** | Option<**String**> | Parameter name |  |
**parameter93_period_value** | Option<**String**> | Parameter value |  |
**parameter94_period_name** | Option<**String**> | Parameter name |  |
**parameter94_period_value** | Option<**String**> | Parameter value |  |
**parameter95_period_name** | Option<**String**> | Parameter name |  |
**parameter95_period_value** | Option<**String**> | Parameter value |  |
**parameter96_period_name** | Option<**String**> | Parameter name |  |
**parameter96_period_value** | Option<**String**> | Parameter value |  |
**parameter97_period_name** | Option<**String**> | Parameter name |  |
**parameter97_period_value** | Option<**String**> | Parameter value |  |
**parameter98_period_name** | Option<**String**> | Parameter name |  |
**parameter98_period_value** | Option<**String**> | Parameter value |  |
**parameter99_period_name** | Option<**String**> | Parameter name |  |
**parameter99_period_value** | Option<**String**> | Parameter value |  |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodCallPeriodStream**](api.v2010.account.call.stream.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_token

> crate::models::ApiPeriodV2010PeriodAccountPeriodToken create_token(account_sid, ttl)


Create a new token for ICE servers

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that will create the resource. | [required] |
**ttl** | Option<**i32**> | The duration in seconds for which the generated credentials are valid. The default value is 86400 (24 hours). |  |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodToken**](api.v2010.account.token.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_usage_trigger

> crate::models::ApiPeriodV2010PeriodAccountPeriodUsagePeriodUsageTrigger create_usage_trigger(account_sid, callback_url, trigger_value, usage_category, callback_method, friendly_name, recurring, trigger_by)


Create a new UsageTrigger

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that will create the resource. | [required] |
**callback_url** | **String** | The URL we should call using `callback_method` when the trigger fires. | [required] |
**trigger_value** | **String** | The usage value at which the trigger should fire.  For convenience, you can use an offset value such as `+30` to specify a trigger_value that is 30 units more than the current usage value. Be sure to urlencode a `+` as `%2B`. | [required] |
**usage_category** | **crate::models::UsageTriggerEnumUsageCategory** |  | [required] |
**callback_method** | Option<**String**> | The HTTP method we should use to call `callback_url`. Can be: `GET` or `POST` and the default is `POST`. |  |
**friendly_name** | Option<**String**> | A descriptive string that you create to describe the resource. It can be up to 64 characters long. |  |
**recurring** | Option<**crate::models::UsageTriggerEnumRecurring**> |  |  |
**trigger_by** | Option<**crate::models::UsageTriggerEnumTriggerField**> |  |  |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodUsagePeriodUsageTrigger**](api.v2010.account.usage.usage_trigger.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user_defined_message

> crate::models::ApiPeriodV2010PeriodAccountPeriodCallPeriodUserDefinedMessage create_user_defined_message(account_sid, call_sid, content, idempotency_key)


Create a new User Defined Message for the given Call SID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created User Defined Message. | [required] |
**call_sid** | **String** | The SID of the [Call](https://www.twilio.com/docs/voice/api/call-resource) the User Defined Message is associated with. | [required] |
**content** | **String** | The User Defined Message in the form of URL-encoded JSON string. | [required] |
**idempotency_key** | Option<**String**> | A unique string value to identify API call. This should be a unique string value per API call and can be a randomly generated. |  |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodCallPeriodUserDefinedMessage**](api.v2010.account.call.user_defined_message.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user_defined_message_subscription

> crate::models::ApiPeriodV2010PeriodAccountPeriodCallPeriodUserDefinedMessageSubscription create_user_defined_message_subscription(account_sid, call_sid, callback, method, idempotency_key)


Subscribe to User Defined Messages for a given Call SID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that subscribed to the User Defined Messages. | [required] |
**call_sid** | **String** | The SID of the [Call](https://www.twilio.com/docs/voice/api/call-resource) the User Defined Messages subscription is associated with. This refers to the Call SID that is producing the user defined messages. | [required] |
**callback** | **String** | The URL we should call using the `method` to send user defined events to your application. URLs must contain a valid hostname (underscores are not permitted). | [required] |
**method** | **String** | The HTTP method Twilio will use when requesting the above `Url`. Either `GET` or `POST`. | [required] |
**idempotency_key** | Option<**String**> | A unique string value to identify API call. This should be a unique string value per API call and can be a randomly generated. |  |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodCallPeriodUserDefinedMessageSubscription**](api.v2010.account.call.user_defined_message_subscription.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_validation_request

> crate::models::ApiPeriodV2010PeriodAccountPeriodValidationRequest create_validation_request(account_sid, phone_number, friendly_name, call_delay, extension, status_callback, status_callback_method)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) responsible for the new caller ID resource. | [required] |
**phone_number** | **String** | The phone number to verify in [E.164](https://www.twilio.com/docs/glossary/what-e164) format, which consists of a + followed by the country code and subscriber number. | [required] |
**friendly_name** | Option<**String**> | A descriptive string that you create to describe the new caller ID resource. It can be up to 64 characters long. The default value is a formatted version of the phone number. |  |
**call_delay** | Option<**i32**> | The number of seconds to delay before initiating the verification call. Can be an integer between `0` and `60`, inclusive. The default is `0`. |  |
**extension** | Option<**String**> | The digits to dial after connecting the verification call. |  |
**status_callback** | Option<**String**> | The URL we should call using the `status_callback_method` to send status information about the verification process to your application. |  |
**status_callback_method** | Option<**String**> | The HTTP method we should use to call `status_callback`. Can be: `GET` or `POST`, and the default is `POST`. |  |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodValidationRequest**](api.v2010.account.validation_request.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_address

> delete_address(account_sid, sid)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that is responsible for the Address resource to delete. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the Address resource to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_application

> delete_application(account_sid, sid)


Delete the application by the specified application sid

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Application resources to delete. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the Application resource to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_call

> delete_call(account_sid, sid)


Delete a Call record from your account. Once the record is deleted, it will no longer appear in the API and Account Portal logs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Call resource(s) to delete. | [required] |
**sid** | **String** | The Twilio-provided Call SID that uniquely identifies the Call resource to delete | [required] |

### Return type

 (empty response body)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_call_feedback_summary

> delete_call_feedback_summary(account_sid, sid)


Delete a FeedbackSummary resource from a call

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The unique id of the [Account](https://www.twilio.com/docs/iam/api/account) responsible for this resource. | [required] |
**sid** | **String** | A 34 character string that uniquely identifies this resource. | [required] |

### Return type

 (empty response body)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_call_recording

> delete_call_recording(account_sid, call_sid, sid)


Delete a recording from your account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Recording resources to delete. | [required] |
**call_sid** | **String** | The [Call](https://www.twilio.com/docs/voice/api/call-resource) SID of the resources to delete. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the Recording resource to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_conference_recording

> delete_conference_recording(account_sid, conference_sid, sid)


Delete a recording from your account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Conference Recording resources to delete. | [required] |
**conference_sid** | **String** | The Conference SID that identifies the conference associated with the recording to delete. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the Conference Recording resource to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_connect_app

> delete_connect_app(account_sid, sid)


Delete an instance of a connect-app

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the ConnectApp resource to fetch. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the ConnectApp resource to fetch. | [required] |

### Return type

 (empty response body)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_incoming_phone_number

> delete_incoming_phone_number(account_sid, sid)


Delete a phone-numbers belonging to the account used to make the request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the IncomingPhoneNumber resources to delete. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the IncomingPhoneNumber resource to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_incoming_phone_number_assigned_add_on

> delete_incoming_phone_number_assigned_add_on(account_sid, resource_sid, sid)


Remove the assignment of an Add-on installation from the Number specified.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the resources to delete. | [required] |
**resource_sid** | **String** | The SID of the Phone Number to which the Add-on is assigned. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the resource to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_key

> delete_key(account_sid, sid)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Key resources to delete. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the Key resource to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_media

> delete_media(account_sid, message_sid, sid)


Delete media from your account. Once delete, you will no longer be billed

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Media resource(s) to delete. | [required] |
**message_sid** | **String** | The SID of the Message resource that this Media resource belongs to. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the Media resource to delete | [required] |

### Return type

 (empty response body)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_message

> delete_message(account_sid, sid)


Deletes a message record from your account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Message resources to delete. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the Message resource to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_outgoing_caller_id

> delete_outgoing_caller_id(account_sid, sid)


Delete the caller-id specified from the account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the OutgoingCallerId resources to delete. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the OutgoingCallerId resource to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_participant

> delete_participant(account_sid, conference_sid, call_sid)


Kick a participant from a given conference

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Participant resources to delete. | [required] |
**conference_sid** | **String** | The SID of the conference with the participants to delete. | [required] |
**call_sid** | **String** | The [Call](https://www.twilio.com/docs/voice/api/call-resource) SID or label of the participant to delete. Non URL safe characters in a label must be percent encoded, for example, a space character is represented as %20. | [required] |

### Return type

 (empty response body)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_queue

> delete_queue(account_sid, sid)


Remove an empty queue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Queue resource to delete. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the Queue resource to delete | [required] |

### Return type

 (empty response body)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_recording

> delete_recording(account_sid, sid)


Delete a recording from your account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Recording resources to delete. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the Recording resource to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_recording_add_on_result

> delete_recording_add_on_result(account_sid, reference_sid, sid)


Delete a result and purge all associated Payloads

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Recording AddOnResult resources to delete. | [required] |
**reference_sid** | **String** | The SID of the recording to which the result to delete belongs. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the Recording AddOnResult resource to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_recording_add_on_result_payload

> delete_recording_add_on_result_payload(account_sid, reference_sid, add_on_result_sid, sid)


Delete a payload from the result along with all associated Data

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Recording AddOnResult Payload resources to delete. | [required] |
**reference_sid** | **String** | The SID of the recording to which the AddOnResult resource that contains the payloads to delete belongs. | [required] |
**add_on_result_sid** | **String** | The SID of the AddOnResult to which the payloads to delete belongs. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the Recording AddOnResult Payload resource to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_recording_transcription

> delete_recording_transcription(account_sid, recording_sid, sid)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Transcription resources to delete. | [required] |
**recording_sid** | **String** | The SID of the [Recording](https://www.twilio.com/docs/voice/api/recording) that created the transcription to delete. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the Transcription resource to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_signing_key

> delete_signing_key(account_sid, sid)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** |  | [required] |
**sid** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_sip_auth_calls_credential_list_mapping

> delete_sip_auth_calls_credential_list_mapping(account_sid, domain_sid, sid)


Delete a credential list mapping from the requested domain

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the CredentialListMapping resources to delete. | [required] |
**domain_sid** | **String** | The SID of the SIP domain that contains the resource to delete. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the CredentialListMapping resource to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_sip_auth_calls_ip_access_control_list_mapping

> delete_sip_auth_calls_ip_access_control_list_mapping(account_sid, domain_sid, sid)


Delete an IP Access Control List mapping from the requested domain

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the IpAccessControlListMapping resources to delete. | [required] |
**domain_sid** | **String** | The SID of the SIP domain that contains the resources to delete. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the IpAccessControlListMapping resource to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_sip_auth_registrations_credential_list_mapping

> delete_sip_auth_registrations_credential_list_mapping(account_sid, domain_sid, sid)


Delete a credential list mapping from the requested domain

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the CredentialListMapping resources to delete. | [required] |
**domain_sid** | **String** | The SID of the SIP domain that contains the resources to delete. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the CredentialListMapping resource to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_sip_credential

> delete_sip_credential(account_sid, credential_list_sid, sid)


Delete a credential resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The unique id of the Account that is responsible for this resource. | [required] |
**credential_list_sid** | **String** | The unique id that identifies the credential list that contains the desired credentials. | [required] |
**sid** | **String** | The unique id that identifies the resource to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_sip_credential_list

> delete_sip_credential_list(account_sid, sid)


Delete a Credential List

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The unique id of the Account that is responsible for this resource. | [required] |
**sid** | **String** | The credential list Sid that uniquely identifies this resource | [required] |

### Return type

 (empty response body)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_sip_credential_list_mapping

> delete_sip_credential_list_mapping(account_sid, domain_sid, sid)


Delete a CredentialListMapping resource from an account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The unique id of the [Account](https://www.twilio.com/docs/iam/api/account) responsible for this resource. | [required] |
**domain_sid** | **String** | A 34 character string that uniquely identifies the SIP Domain that includes the resource to delete. | [required] |
**sid** | **String** | A 34 character string that uniquely identifies the resource to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_sip_domain

> delete_sip_domain(account_sid, sid)


Delete an instance of a Domain

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the SipDomain resources to delete. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the SipDomain resource to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_sip_ip_access_control_list

> delete_sip_ip_access_control_list(account_sid, sid)


Delete an IpAccessControlList from the requested account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The unique id of the [Account](https://www.twilio.com/docs/iam/api/account) responsible for this resource. | [required] |
**sid** | **String** | A 34 character string that uniquely identifies the resource to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_sip_ip_access_control_list_mapping

> delete_sip_ip_access_control_list_mapping(account_sid, domain_sid, sid)


Delete an IpAccessControlListMapping resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The unique id of the Account that is responsible for this resource. | [required] |
**domain_sid** | **String** | A 34 character string that uniquely identifies the SIP domain. | [required] |
**sid** | **String** | A 34 character string that uniquely identifies the resource to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_sip_ip_address

> delete_sip_ip_address(account_sid, ip_access_control_list_sid, sid)


Delete an IpAddress resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The unique id of the [Account](https://www.twilio.com/docs/iam/api/account) responsible for this resource. | [required] |
**ip_access_control_list_sid** | **String** | The IpAccessControlList Sid that identifies the IpAddress resources to delete. | [required] |
**sid** | **String** | A 34 character string that uniquely identifies the resource to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_transcription

> delete_transcription(account_sid, sid)


Delete a transcription from the account used to make the request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Transcription resources to delete. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the Transcription resource to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_usage_trigger

> delete_usage_trigger(account_sid, sid)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the UsageTrigger resources to delete. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the UsageTrigger resource to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_defined_message_subscription

> delete_user_defined_message_subscription(account_sid, call_sid, sid)


Delete a specific User Defined Message Subscription.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that subscribed to the User Defined Messages. | [required] |
**call_sid** | **String** | The SID of the [Call](https://www.twilio.com/docs/voice/api/call-resource) the User Defined Message Subscription is associated with. This refers to the Call SID that is producing the User Defined Messages. | [required] |
**sid** | **String** | The SID that uniquely identifies this User Defined Message Subscription. | [required] |

### Return type

 (empty response body)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_account

> crate::models::ApiPeriodV2010PeriodAccount fetch_account(sid)


Fetch the account specified by the provided Account Sid

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sid** | **String** | The Account Sid that uniquely identifies the account to fetch | [required] |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccount**](api.v2010.account.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_address

> crate::models::ApiPeriodV2010PeriodAccountPeriodAddress fetch_address(account_sid, sid)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that is responsible for the Address resource to fetch. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the Address resource to fetch. | [required] |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodAddress**](api.v2010.account.address.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_application

> crate::models::ApiPeriodV2010PeriodAccountPeriodApplication fetch_application(account_sid, sid)


Fetch the application specified by the provided sid

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Application resource to fetch. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the Application resource to fetch. | [required] |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodApplication**](api.v2010.account.application.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_authorized_connect_app

> crate::models::ApiPeriodV2010PeriodAccountPeriodAuthorizedConnectApp fetch_authorized_connect_app(account_sid, connect_app_sid)


Fetch an instance of an authorized-connect-app

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the AuthorizedConnectApp resource to fetch. | [required] |
**connect_app_sid** | **String** | The SID of the Connect App to fetch. | [required] |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodAuthorizedConnectApp**](api.v2010.account.authorized_connect_app.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_available_phone_number_country

> crate::models::ApiPeriodV2010PeriodAccountPeriodAvailablePhoneNumberCountry fetch_available_phone_number_country(account_sid, country_code)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) requesting the available phone number Country resource. | [required] |
**country_code** | **String** | The [ISO-3166-1](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) country code of the country to fetch available phone number information about. | [required] |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodAvailablePhoneNumberCountry**](api.v2010.account.available_phone_number_country.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_balance

> crate::models::ApiPeriodV2010PeriodAccountPeriodBalance fetch_balance(account_sid)


Fetch the balance for an Account based on Account Sid. Balance changes may not be reflected immediately. Child accounts do not contain balance information

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The unique SID identifier of the Account. | [required] |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodBalance**](api.v2010.account.balance.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_call

> crate::models::ApiPeriodV2010PeriodAccountPeriodCall fetch_call(account_sid, sid)


Fetch the call specified by the provided Call SID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Call resource(s) to fetch. | [required] |
**sid** | **String** | The SID of the Call resource to fetch. | [required] |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodCall**](api.v2010.account.call.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_call_feedback

> crate::models::ApiPeriodV2010PeriodAccountPeriodCallPeriodCallFeedback fetch_call_feedback(account_sid, call_sid)


Fetch a Feedback resource from a call

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The unique id of the [Account](https://www.twilio.com/docs/iam/api/account) responsible for this resource. | [required] |
**call_sid** | **String** | The call sid that uniquely identifies the call | [required] |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodCallPeriodCallFeedback**](api.v2010.account.call.call_feedback.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_call_feedback_summary

> crate::models::ApiPeriodV2010PeriodAccountPeriodCallPeriodCallFeedbackSummary fetch_call_feedback_summary(account_sid, sid)


Fetch a FeedbackSummary resource from a call

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The unique id of the [Account](https://www.twilio.com/docs/iam/api/account) responsible for this resource. | [required] |
**sid** | **String** | A 34 character string that uniquely identifies this resource. | [required] |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodCallPeriodCallFeedbackSummary**](api.v2010.account.call.call_feedback_summary.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_call_notification

> crate::models::ApiPeriodV2010PeriodAccountPeriodCallPeriodCallNotificationInstance fetch_call_notification(account_sid, call_sid, sid)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Call Notification resource to fetch. | [required] |
**call_sid** | **String** | The [Call](https://www.twilio.com/docs/voice/api/call-resource) SID of the Call Notification resource to fetch. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the Call Notification resource to fetch. | [required] |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodCallPeriodCallNotificationInstance**](api.v2010.account.call.call_notification-instance.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_call_recording

> crate::models::ApiPeriodV2010PeriodAccountPeriodCallPeriodCallRecording fetch_call_recording(account_sid, call_sid, sid)


Fetch an instance of a recording for a call

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Recording resource to fetch. | [required] |
**call_sid** | **String** | The [Call](https://www.twilio.com/docs/voice/api/call-resource) SID of the resource to fetch. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the Recording resource to fetch. | [required] |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodCallPeriodCallRecording**](api.v2010.account.call.call_recording.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_conference

> crate::models::ApiPeriodV2010PeriodAccountPeriodConference fetch_conference(account_sid, sid)


Fetch an instance of a conference

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Conference resource(s) to fetch. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the Conference resource to fetch | [required] |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodConference**](api.v2010.account.conference.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_conference_recording

> crate::models::ApiPeriodV2010PeriodAccountPeriodConferencePeriodConferenceRecording fetch_conference_recording(account_sid, conference_sid, sid)


Fetch an instance of a recording for a call

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Conference Recording resource to fetch. | [required] |
**conference_sid** | **String** | The Conference SID that identifies the conference associated with the recording to fetch. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the Conference Recording resource to fetch. | [required] |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodConferencePeriodConferenceRecording**](api.v2010.account.conference.conference_recording.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_connect_app

> crate::models::ApiPeriodV2010PeriodAccountPeriodConnectApp fetch_connect_app(account_sid, sid)


Fetch an instance of a connect-app

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the ConnectApp resource to fetch. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the ConnectApp resource to fetch. | [required] |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodConnectApp**](api.v2010.account.connect_app.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_incoming_phone_number

> crate::models::ApiPeriodV2010PeriodAccountPeriodIncomingPhoneNumber fetch_incoming_phone_number(account_sid, sid)


Fetch an incoming-phone-number belonging to the account used to make the request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the IncomingPhoneNumber resource to fetch. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the IncomingPhoneNumber resource to fetch. | [required] |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodIncomingPhoneNumber**](api.v2010.account.incoming_phone_number.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_incoming_phone_number_assigned_add_on

> crate::models::ApiPeriodV2010PeriodAccountPeriodIncomingPhoneNumberPeriodIncomingPhoneNumberAssignedAddOn fetch_incoming_phone_number_assigned_add_on(account_sid, resource_sid, sid)


Fetch an instance of an Add-on installation currently assigned to this Number.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the resource to fetch. | [required] |
**resource_sid** | **String** | The SID of the Phone Number to which the Add-on is assigned. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the resource to fetch. | [required] |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodIncomingPhoneNumberPeriodIncomingPhoneNumberAssignedAddOn**](api.v2010.account.incoming_phone_number.incoming_phone_number_assigned_add_on.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_incoming_phone_number_assigned_add_on_extension

> crate::models::ApiPeriodV2010PeriodAccountPeriodIncomingPhoneNumberPeriodIncomingPhoneNumberAssignedAddOnPeriodIncomingPhoneNumberAssignedAddOnExtension fetch_incoming_phone_number_assigned_add_on_extension(account_sid, resource_sid, assigned_add_on_sid, sid)


Fetch an instance of an Extension for the Assigned Add-on.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the resource to fetch. | [required] |
**resource_sid** | **String** | The SID of the Phone Number to which the Add-on is assigned. | [required] |
**assigned_add_on_sid** | **String** | The SID that uniquely identifies the assigned Add-on installation. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the resource to fetch. | [required] |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodIncomingPhoneNumberPeriodIncomingPhoneNumberAssignedAddOnPeriodIncomingPhoneNumberAssignedAddOnExtension**](api.v2010.account.incoming_phone_number.incoming_phone_number_assigned_add_on.incoming_phone_number_assigned_add_on_extension.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_key

> crate::models::ApiPeriodV2010PeriodAccountPeriodKey fetch_key(account_sid, sid)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Key resource to fetch. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the Key resource to fetch. | [required] |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodKey**](api.v2010.account.key.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_media

> crate::models::ApiPeriodV2010PeriodAccountPeriodMessagePeriodMedia fetch_media(account_sid, message_sid, sid)


Fetch a single media instance belonging to the account used to make the request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Media resource(s) to fetch. | [required] |
**message_sid** | **String** | The SID of the Message resource that this Media resource belongs to. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the Media resource to fetch | [required] |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodMessagePeriodMedia**](api.v2010.account.message.media.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_member

> crate::models::ApiPeriodV2010PeriodAccountPeriodQueuePeriodMember fetch_member(account_sid, queue_sid, call_sid)


Fetch a specific member from the queue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Member resource(s) to fetch. | [required] |
**queue_sid** | **String** | The SID of the Queue in which to find the members to fetch. | [required] |
**call_sid** | **String** | The [Call](https://www.twilio.com/docs/voice/api/call-resource) SID of the resource(s) to fetch. | [required] |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodQueuePeriodMember**](api.v2010.account.queue.member.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_message

> crate::models::ApiPeriodV2010PeriodAccountPeriodMessage fetch_message(account_sid, sid)


Fetch a message belonging to the account used to make the request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Message resource to fetch. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the Message resource to fetch. | [required] |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodMessage**](api.v2010.account.message.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_notification

> crate::models::ApiPeriodV2010PeriodAccountPeriodNotificationInstance fetch_notification(account_sid, sid)


Fetch a notification belonging to the account used to make the request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Notification resource to fetch. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the Notification resource to fetch. | [required] |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodNotificationInstance**](api.v2010.account.notification-instance.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_outgoing_caller_id

> crate::models::ApiPeriodV2010PeriodAccountPeriodOutgoingCallerId fetch_outgoing_caller_id(account_sid, sid)


Fetch an outgoing-caller-id belonging to the account used to make the request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the OutgoingCallerId resource to fetch. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the OutgoingCallerId resource to fetch. | [required] |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodOutgoingCallerId**](api.v2010.account.outgoing_caller_id.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_participant

> crate::models::ApiPeriodV2010PeriodAccountPeriodConferencePeriodParticipant fetch_participant(account_sid, conference_sid, call_sid)


Fetch an instance of a participant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Participant resource to fetch. | [required] |
**conference_sid** | **String** | The SID of the conference with the participant to fetch. | [required] |
**call_sid** | **String** | The [Call](https://www.twilio.com/docs/voice/api/call-resource) SID or label of the participant to fetch. Non URL safe characters in a label must be percent encoded, for example, a space character is represented as %20. | [required] |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodConferencePeriodParticipant**](api.v2010.account.conference.participant.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_queue

> crate::models::ApiPeriodV2010PeriodAccountPeriodQueue fetch_queue(account_sid, sid)


Fetch an instance of a queue identified by the QueueSid

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Queue resource to fetch. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the Queue resource to fetch | [required] |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodQueue**](api.v2010.account.queue.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_recording

> crate::models::ApiPeriodV2010PeriodAccountPeriodRecording fetch_recording(account_sid, sid, include_soft_deleted)


Fetch an instance of a recording

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Recording resource to fetch. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the Recording resource to fetch. | [required] |
**include_soft_deleted** | Option<**bool**> | A boolean parameter indicating whether to retrieve soft deleted recordings or not. Recordings metadata are kept after deletion for a retention period of 40 days. |  |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodRecording**](api.v2010.account.recording.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_recording_add_on_result

> crate::models::ApiPeriodV2010PeriodAccountPeriodRecordingPeriodRecordingAddOnResult fetch_recording_add_on_result(account_sid, reference_sid, sid)


Fetch an instance of an AddOnResult

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Recording AddOnResult resource to fetch. | [required] |
**reference_sid** | **String** | The SID of the recording to which the result to fetch belongs. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the Recording AddOnResult resource to fetch. | [required] |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodRecordingPeriodRecordingAddOnResult**](api.v2010.account.recording.recording_add_on_result.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_recording_add_on_result_payload

> crate::models::ApiPeriodV2010PeriodAccountPeriodRecordingPeriodRecordingAddOnResultPeriodRecordingAddOnResultPayload fetch_recording_add_on_result_payload(account_sid, reference_sid, add_on_result_sid, sid)


Fetch an instance of a result payload

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Recording AddOnResult Payload resource to fetch. | [required] |
**reference_sid** | **String** | The SID of the recording to which the AddOnResult resource that contains the payload to fetch belongs. | [required] |
**add_on_result_sid** | **String** | The SID of the AddOnResult to which the payload to fetch belongs. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the Recording AddOnResult Payload resource to fetch. | [required] |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodRecordingPeriodRecordingAddOnResultPeriodRecordingAddOnResultPayload**](api.v2010.account.recording.recording_add_on_result.recording_add_on_result_payload.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_recording_transcription

> crate::models::ApiPeriodV2010PeriodAccountPeriodRecordingPeriodRecordingTranscription fetch_recording_transcription(account_sid, recording_sid, sid)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Transcription resource to fetch. | [required] |
**recording_sid** | **String** | The SID of the [Recording](https://www.twilio.com/docs/voice/api/recording) that created the transcription to fetch. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the Transcription resource to fetch. | [required] |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodRecordingPeriodRecordingTranscription**](api.v2010.account.recording.recording_transcription.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_short_code

> crate::models::ApiPeriodV2010PeriodAccountPeriodShortCode fetch_short_code(account_sid, sid)


Fetch an instance of a short code

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the ShortCode resource(s) to fetch. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the ShortCode resource to fetch | [required] |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodShortCode**](api.v2010.account.short_code.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_signing_key

> crate::models::ApiPeriodV2010PeriodAccountPeriodSigningKey fetch_signing_key(account_sid, sid)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** |  | [required] |
**sid** | **String** |  | [required] |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodSigningKey**](api.v2010.account.signing_key.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_sip_auth_calls_credential_list_mapping

> crate::models::ApiPeriodV2010PeriodAccountPeriodSipPeriodSipDomainPeriodSipAuthPeriodSipAuthCallsPeriodSipAuthCallsCredentialListMapping fetch_sip_auth_calls_credential_list_mapping(account_sid, domain_sid, sid)


Fetch a specific instance of a credential list mapping

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the CredentialListMapping resource to fetch. | [required] |
**domain_sid** | **String** | The SID of the SIP domain that contains the resource to fetch. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the CredentialListMapping resource to fetch. | [required] |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodSipPeriodSipDomainPeriodSipAuthPeriodSipAuthCallsPeriodSipAuthCallsCredentialListMapping**](api.v2010.account.sip.sip_domain.sip_auth.sip_auth_calls.sip_auth_calls_credential_list_mapping.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_sip_auth_calls_ip_access_control_list_mapping

> crate::models::ApiPeriodV2010PeriodAccountPeriodSipPeriodSipDomainPeriodSipAuthPeriodSipAuthCallsPeriodSipAuthCallsIpAccessControlListMapping fetch_sip_auth_calls_ip_access_control_list_mapping(account_sid, domain_sid, sid)


Fetch a specific instance of an IP Access Control List mapping

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the IpAccessControlListMapping resource to fetch. | [required] |
**domain_sid** | **String** | The SID of the SIP domain that contains the resource to fetch. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the IpAccessControlListMapping resource to fetch. | [required] |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodSipPeriodSipDomainPeriodSipAuthPeriodSipAuthCallsPeriodSipAuthCallsIpAccessControlListMapping**](api.v2010.account.sip.sip_domain.sip_auth.sip_auth_calls.sip_auth_calls_ip_access_control_list_mapping.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_sip_auth_registrations_credential_list_mapping

> crate::models::ApiPeriodV2010PeriodAccountPeriodSipPeriodSipDomainPeriodSipAuthPeriodSipAuthRegistrationsPeriodSipAuthRegistrationsCredentialListMapping fetch_sip_auth_registrations_credential_list_mapping(account_sid, domain_sid, sid)


Fetch a specific instance of a credential list mapping

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the CredentialListMapping resource to fetch. | [required] |
**domain_sid** | **String** | The SID of the SIP domain that contains the resource to fetch. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the CredentialListMapping resource to fetch. | [required] |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodSipPeriodSipDomainPeriodSipAuthPeriodSipAuthRegistrationsPeriodSipAuthRegistrationsCredentialListMapping**](api.v2010.account.sip.sip_domain.sip_auth.sip_auth_registrations.sip_auth_registrations_credential_list_mapping.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_sip_credential

> crate::models::ApiPeriodV2010PeriodAccountPeriodSipPeriodSipCredentialListPeriodSipCredential fetch_sip_credential(account_sid, credential_list_sid, sid)


Fetch a single credential.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The unique id of the Account that is responsible for this resource. | [required] |
**credential_list_sid** | **String** | The unique id that identifies the credential list that contains the desired credential. | [required] |
**sid** | **String** | The unique id that identifies the resource to fetch. | [required] |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodSipPeriodSipCredentialListPeriodSipCredential**](api.v2010.account.sip.sip_credential_list.sip_credential.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_sip_credential_list

> crate::models::ApiPeriodV2010PeriodAccountPeriodSipPeriodSipCredentialList fetch_sip_credential_list(account_sid, sid)


Get a Credential List

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The unique id of the Account that is responsible for this resource. | [required] |
**sid** | **String** | The credential list Sid that uniquely identifies this resource | [required] |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodSipPeriodSipCredentialList**](api.v2010.account.sip.sip_credential_list.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_sip_credential_list_mapping

> crate::models::ApiPeriodV2010PeriodAccountPeriodSipPeriodSipDomainPeriodSipCredentialListMapping fetch_sip_credential_list_mapping(account_sid, domain_sid, sid)


Fetch a single CredentialListMapping resource from an account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The unique id of the [Account](https://www.twilio.com/docs/iam/api/account) responsible for this resource. | [required] |
**domain_sid** | **String** | A 34 character string that uniquely identifies the SIP Domain that includes the resource to fetch. | [required] |
**sid** | **String** | A 34 character string that uniquely identifies the resource to fetch. | [required] |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodSipPeriodSipDomainPeriodSipCredentialListMapping**](api.v2010.account.sip.sip_domain.sip_credential_list_mapping.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_sip_domain

> crate::models::ApiPeriodV2010PeriodAccountPeriodSipPeriodSipDomain fetch_sip_domain(account_sid, sid)


Fetch an instance of a Domain

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the SipDomain resource to fetch. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the SipDomain resource to fetch. | [required] |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodSipPeriodSipDomain**](api.v2010.account.sip.sip_domain.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_sip_ip_access_control_list

> crate::models::ApiPeriodV2010PeriodAccountPeriodSipPeriodSipIpAccessControlList fetch_sip_ip_access_control_list(account_sid, sid)


Fetch a specific instance of an IpAccessControlList

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The unique id of the [Account](https://www.twilio.com/docs/iam/api/account) responsible for this resource. | [required] |
**sid** | **String** | A 34 character string that uniquely identifies the resource to fetch. | [required] |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodSipPeriodSipIpAccessControlList**](api.v2010.account.sip.sip_ip_access_control_list.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_sip_ip_access_control_list_mapping

> crate::models::ApiPeriodV2010PeriodAccountPeriodSipPeriodSipDomainPeriodSipIpAccessControlListMapping fetch_sip_ip_access_control_list_mapping(account_sid, domain_sid, sid)


Fetch an IpAccessControlListMapping resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The unique id of the Account that is responsible for this resource. | [required] |
**domain_sid** | **String** | A 34 character string that uniquely identifies the SIP domain. | [required] |
**sid** | **String** | A 34 character string that uniquely identifies the resource to fetch. | [required] |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodSipPeriodSipDomainPeriodSipIpAccessControlListMapping**](api.v2010.account.sip.sip_domain.sip_ip_access_control_list_mapping.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_sip_ip_address

> crate::models::ApiPeriodV2010PeriodAccountPeriodSipPeriodSipIpAccessControlListPeriodSipIpAddress fetch_sip_ip_address(account_sid, ip_access_control_list_sid, sid)


Read one IpAddress resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The unique id of the [Account](https://www.twilio.com/docs/iam/api/account) responsible for this resource. | [required] |
**ip_access_control_list_sid** | **String** | The IpAccessControlList Sid that identifies the IpAddress resources to fetch. | [required] |
**sid** | **String** | A 34 character string that uniquely identifies the IpAddress resource to fetch. | [required] |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodSipPeriodSipIpAccessControlListPeriodSipIpAddress**](api.v2010.account.sip.sip_ip_access_control_list.sip_ip_address.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_transcription

> crate::models::ApiPeriodV2010PeriodAccountPeriodTranscription fetch_transcription(account_sid, sid)


Fetch an instance of a Transcription

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Transcription resource to fetch. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the Transcription resource to fetch. | [required] |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodTranscription**](api.v2010.account.transcription.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_usage_trigger

> crate::models::ApiPeriodV2010PeriodAccountPeriodUsagePeriodUsageTrigger fetch_usage_trigger(account_sid, sid)


Fetch and instance of a usage-trigger

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the UsageTrigger resource to fetch. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the UsageTrigger resource to fetch. | [required] |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodUsagePeriodUsageTrigger**](api.v2010.account.usage.usage_trigger.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_account

> crate::models::ListAccountResponse list_account(friendly_name, status, page_size)


Retrieves a collection of Accounts belonging to the account used to make the request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**friendly_name** | Option<**String**> | Only return the Account resources with friendly names that exactly match this name. |  |
**status** | Option<**AccountEnumStatus**> | Only return Account resources with the given status. Can be `closed`, `suspended` or `active`. |  |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListAccountResponse**](ListAccountResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_address

> crate::models::ListAddressResponse list_address(account_sid, customer_name, friendly_name, iso_country, page_size)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that is responsible for the Address resource to read. | [required] |
**customer_name** | Option<**String**> | The `customer_name` of the Address resources to read. |  |
**friendly_name** | Option<**String**> | The string that identifies the Address resources to read. |  |
**iso_country** | Option<**String**> | The ISO country code of the Address resources to read. |  |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListAddressResponse**](ListAddressResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_application

> crate::models::ListApplicationResponse list_application(account_sid, friendly_name, page_size)


Retrieve a list of applications representing an application within the requesting account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Application resources to read. | [required] |
**friendly_name** | Option<**String**> | The string that identifies the Application resources to read. |  |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListApplicationResponse**](ListApplicationResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_authorized_connect_app

> crate::models::ListAuthorizedConnectAppResponse list_authorized_connect_app(account_sid, page_size)


Retrieve a list of authorized-connect-apps belonging to the account used to make the request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the AuthorizedConnectApp resources to read. | [required] |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListAuthorizedConnectAppResponse**](ListAuthorizedConnectAppResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_available_phone_number_country

> crate::models::ListAvailablePhoneNumberCountryResponse list_available_phone_number_country(account_sid, page_size)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) requesting the available phone number Country resources. | [required] |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListAvailablePhoneNumberCountryResponse**](ListAvailablePhoneNumberCountryResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_available_phone_number_local

> crate::models::ListAvailablePhoneNumberLocalResponse list_available_phone_number_local(account_sid, country_code, area_code, contains, sms_enabled, mms_enabled, voice_enabled, exclude_all_address_required, exclude_local_address_required, exclude_foreign_address_required, beta, near_number, near_lat_long, distance, in_postal_code, in_region, in_rate_center, in_lata, in_locality, fax_enabled, page_size)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) requesting the AvailablePhoneNumber resources. | [required] |
**country_code** | **String** | The [ISO-3166-1](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) country code of the country from which to read phone numbers. | [required] |
**area_code** | Option<**i32**> | The area code of the phone numbers to read. Applies to only phone numbers in the US and Canada. |  |
**contains** | Option<**String**> | The pattern on which to match phone numbers. Valid characters are `*`, `0-9`, `a-z`, and `A-Z`. The `*` character matches any single digit. For examples, see [Example 2](https://www.twilio.com/docs/phone-numbers/api/availablephonenumberlocal-resource?code-sample=code-find-phone-numbers-by-number-pattern) and [Example 3](https://www.twilio.com/docs/phone-numbers/api/availablephonenumberlocal-resource?code-sample=code-find-phone-numbers-by-character-pattern). If specified, this value must have at least two characters. |  |
**sms_enabled** | Option<**bool**> | Whether the phone numbers can receive text messages. Can be: `true` or `false`. |  |
**mms_enabled** | Option<**bool**> | Whether the phone numbers can receive MMS messages. Can be: `true` or `false`. |  |
**voice_enabled** | Option<**bool**> | Whether the phone numbers can receive calls. Can be: `true` or `false`. |  |
**exclude_all_address_required** | Option<**bool**> | Whether to exclude phone numbers that require an [Address](https://www.twilio.com/docs/usage/api/address). Can be: `true` or `false` and the default is `false`. |  |
**exclude_local_address_required** | Option<**bool**> | Whether to exclude phone numbers that require a local [Address](https://www.twilio.com/docs/usage/api/address). Can be: `true` or `false` and the default is `false`. |  |
**exclude_foreign_address_required** | Option<**bool**> | Whether to exclude phone numbers that require a foreign [Address](https://www.twilio.com/docs/usage/api/address). Can be: `true` or `false` and the default is `false`. |  |
**beta** | Option<**bool**> | Whether to read phone numbers that are new to the Twilio platform. Can be: `true` or `false` and the default is `true`. |  |
**near_number** | Option<**String**> | Given a phone number, find a geographically close number within `distance` miles. Distance defaults to 25 miles. Applies to only phone numbers in the US and Canada. |  |
**near_lat_long** | Option<**String**> | Given a latitude/longitude pair `lat,long` find geographically close numbers within `distance` miles. Applies to only phone numbers in the US and Canada. |  |
**distance** | Option<**i32**> | The search radius, in miles, for a `near_` query.  Can be up to `500` and the default is `25`. Applies to only phone numbers in the US and Canada. |  |
**in_postal_code** | Option<**String**> | Limit results to a particular postal code. Given a phone number, search within the same postal code as that number. Applies to only phone numbers in the US and Canada. |  |
**in_region** | Option<**String**> | Limit results to a particular region, state, or province. Given a phone number, search within the same region as that number. Applies to only phone numbers in the US and Canada. |  |
**in_rate_center** | Option<**String**> | Limit results to a specific rate center, or given a phone number search within the same rate center as that number. Requires `in_lata` to be set as well. Applies to only phone numbers in the US and Canada. |  |
**in_lata** | Option<**String**> | Limit results to a specific local access and transport area ([LATA](https://en.wikipedia.org/wiki/Local_access_and_transport_area)). Given a phone number, search within the same [LATA](https://en.wikipedia.org/wiki/Local_access_and_transport_area) as that number. Applies to only phone numbers in the US and Canada. |  |
**in_locality** | Option<**String**> | Limit results to a particular locality or city. Given a phone number, search within the same Locality as that number. |  |
**fax_enabled** | Option<**bool**> | Whether the phone numbers can receive faxes. Can be: `true` or `false`. |  |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListAvailablePhoneNumberLocalResponse**](ListAvailablePhoneNumberLocalResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_available_phone_number_machine_to_machine

> crate::models::ListAvailablePhoneNumberMachineToMachineResponse list_available_phone_number_machine_to_machine(account_sid, country_code, area_code, contains, sms_enabled, mms_enabled, voice_enabled, exclude_all_address_required, exclude_local_address_required, exclude_foreign_address_required, beta, near_number, near_lat_long, distance, in_postal_code, in_region, in_rate_center, in_lata, in_locality, fax_enabled, page_size)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) requesting the AvailablePhoneNumber resources. | [required] |
**country_code** | **String** | The [ISO-3166-1](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) country code of the country from which to read phone numbers. | [required] |
**area_code** | Option<**i32**> | The area code of the phone numbers to read. Applies to only phone numbers in the US and Canada. |  |
**contains** | Option<**String**> | The pattern on which to match phone numbers. Valid characters are `*`, `0-9`, `a-z`, and `A-Z`. The `*` character matches any single digit. For examples, see [Example 2](https://www.twilio.com/docs/phone-numbers/api/availablephonenumber-resource#local-get-basic-example-2) and [Example 3](https://www.twilio.com/docs/phone-numbers/api/availablephonenumber-resource#local-get-basic-example-3). If specified, this value must have at least two characters. |  |
**sms_enabled** | Option<**bool**> | Whether the phone numbers can receive text messages. Can be: `true` or `false`. |  |
**mms_enabled** | Option<**bool**> | Whether the phone numbers can receive MMS messages. Can be: `true` or `false`. |  |
**voice_enabled** | Option<**bool**> | Whether the phone numbers can receive calls. Can be: `true` or `false`. |  |
**exclude_all_address_required** | Option<**bool**> | Whether to exclude phone numbers that require an [Address](https://www.twilio.com/docs/usage/api/address). Can be: `true` or `false` and the default is `false`. |  |
**exclude_local_address_required** | Option<**bool**> | Whether to exclude phone numbers that require a local [Address](https://www.twilio.com/docs/usage/api/address). Can be: `true` or `false` and the default is `false`. |  |
**exclude_foreign_address_required** | Option<**bool**> | Whether to exclude phone numbers that require a foreign [Address](https://www.twilio.com/docs/usage/api/address). Can be: `true` or `false` and the default is `false`. |  |
**beta** | Option<**bool**> | Whether to read phone numbers that are new to the Twilio platform. Can be: `true` or `false` and the default is `true`. |  |
**near_number** | Option<**String**> | Given a phone number, find a geographically close number within `distance` miles. Distance defaults to 25 miles. Applies to only phone numbers in the US and Canada. |  |
**near_lat_long** | Option<**String**> | Given a latitude/longitude pair `lat,long` find geographically close numbers within `distance` miles. Applies to only phone numbers in the US and Canada. |  |
**distance** | Option<**i32**> | The search radius, in miles, for a `near_` query.  Can be up to `500` and the default is `25`. Applies to only phone numbers in the US and Canada. |  |
**in_postal_code** | Option<**String**> | Limit results to a particular postal code. Given a phone number, search within the same postal code as that number. Applies to only phone numbers in the US and Canada. |  |
**in_region** | Option<**String**> | Limit results to a particular region, state, or province. Given a phone number, search within the same region as that number. Applies to only phone numbers in the US and Canada. |  |
**in_rate_center** | Option<**String**> | Limit results to a specific rate center, or given a phone number search within the same rate center as that number. Requires `in_lata` to be set as well. Applies to only phone numbers in the US and Canada. |  |
**in_lata** | Option<**String**> | Limit results to a specific local access and transport area ([LATA](https://en.wikipedia.org/wiki/Local_access_and_transport_area)). Given a phone number, search within the same [LATA](https://en.wikipedia.org/wiki/Local_access_and_transport_area) as that number. Applies to only phone numbers in the US and Canada. |  |
**in_locality** | Option<**String**> | Limit results to a particular locality or city. Given a phone number, search within the same Locality as that number. |  |
**fax_enabled** | Option<**bool**> | Whether the phone numbers can receive faxes. Can be: `true` or `false`. |  |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListAvailablePhoneNumberMachineToMachineResponse**](ListAvailablePhoneNumberMachineToMachineResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_available_phone_number_mobile

> crate::models::ListAvailablePhoneNumberMobileResponse list_available_phone_number_mobile(account_sid, country_code, area_code, contains, sms_enabled, mms_enabled, voice_enabled, exclude_all_address_required, exclude_local_address_required, exclude_foreign_address_required, beta, near_number, near_lat_long, distance, in_postal_code, in_region, in_rate_center, in_lata, in_locality, fax_enabled, page_size)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) requesting the AvailablePhoneNumber resources. | [required] |
**country_code** | **String** | The [ISO-3166-1](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) country code of the country from which to read phone numbers. | [required] |
**area_code** | Option<**i32**> | The area code of the phone numbers to read. Applies to only phone numbers in the US and Canada. |  |
**contains** | Option<**String**> | The pattern on which to match phone numbers. Valid characters are `*`, `0-9`, `a-z`, and `A-Z`. The `*` character matches any single digit. For examples, see [Example 2](https://www.twilio.com/docs/phone-numbers/api/availablephonenumber-resource#local-get-basic-example-2) and [Example 3](https://www.twilio.com/docs/phone-numbers/api/availablephonenumber-resource#local-get-basic-example-3). If specified, this value must have at least two characters. |  |
**sms_enabled** | Option<**bool**> | Whether the phone numbers can receive text messages. Can be: `true` or `false`. |  |
**mms_enabled** | Option<**bool**> | Whether the phone numbers can receive MMS messages. Can be: `true` or `false`. |  |
**voice_enabled** | Option<**bool**> | Whether the phone numbers can receive calls. Can be: `true` or `false`. |  |
**exclude_all_address_required** | Option<**bool**> | Whether to exclude phone numbers that require an [Address](https://www.twilio.com/docs/usage/api/address). Can be: `true` or `false` and the default is `false`. |  |
**exclude_local_address_required** | Option<**bool**> | Whether to exclude phone numbers that require a local [Address](https://www.twilio.com/docs/usage/api/address). Can be: `true` or `false` and the default is `false`. |  |
**exclude_foreign_address_required** | Option<**bool**> | Whether to exclude phone numbers that require a foreign [Address](https://www.twilio.com/docs/usage/api/address). Can be: `true` or `false` and the default is `false`. |  |
**beta** | Option<**bool**> | Whether to read phone numbers that are new to the Twilio platform. Can be: `true` or `false` and the default is `true`. |  |
**near_number** | Option<**String**> | Given a phone number, find a geographically close number within `distance` miles. Distance defaults to 25 miles. Applies to only phone numbers in the US and Canada. |  |
**near_lat_long** | Option<**String**> | Given a latitude/longitude pair `lat,long` find geographically close numbers within `distance` miles. Applies to only phone numbers in the US and Canada. |  |
**distance** | Option<**i32**> | The search radius, in miles, for a `near_` query.  Can be up to `500` and the default is `25`. Applies to only phone numbers in the US and Canada. |  |
**in_postal_code** | Option<**String**> | Limit results to a particular postal code. Given a phone number, search within the same postal code as that number. Applies to only phone numbers in the US and Canada. |  |
**in_region** | Option<**String**> | Limit results to a particular region, state, or province. Given a phone number, search within the same region as that number. Applies to only phone numbers in the US and Canada. |  |
**in_rate_center** | Option<**String**> | Limit results to a specific rate center, or given a phone number search within the same rate center as that number. Requires `in_lata` to be set as well. Applies to only phone numbers in the US and Canada. |  |
**in_lata** | Option<**String**> | Limit results to a specific local access and transport area ([LATA](https://en.wikipedia.org/wiki/Local_access_and_transport_area)). Given a phone number, search within the same [LATA](https://en.wikipedia.org/wiki/Local_access_and_transport_area) as that number. Applies to only phone numbers in the US and Canada. |  |
**in_locality** | Option<**String**> | Limit results to a particular locality or city. Given a phone number, search within the same Locality as that number. |  |
**fax_enabled** | Option<**bool**> | Whether the phone numbers can receive faxes. Can be: `true` or `false`. |  |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListAvailablePhoneNumberMobileResponse**](ListAvailablePhoneNumberMobileResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_available_phone_number_national

> crate::models::ListAvailablePhoneNumberNationalResponse list_available_phone_number_national(account_sid, country_code, area_code, contains, sms_enabled, mms_enabled, voice_enabled, exclude_all_address_required, exclude_local_address_required, exclude_foreign_address_required, beta, near_number, near_lat_long, distance, in_postal_code, in_region, in_rate_center, in_lata, in_locality, fax_enabled, page_size)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) requesting the AvailablePhoneNumber resources. | [required] |
**country_code** | **String** | The [ISO-3166-1](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) country code of the country from which to read phone numbers. | [required] |
**area_code** | Option<**i32**> | The area code of the phone numbers to read. Applies to only phone numbers in the US and Canada. |  |
**contains** | Option<**String**> | The pattern on which to match phone numbers. Valid characters are `*`, `0-9`, `a-z`, and `A-Z`. The `*` character matches any single digit. For examples, see [Example 2](https://www.twilio.com/docs/phone-numbers/api/availablephonenumber-resource#local-get-basic-example-2) and [Example 3](https://www.twilio.com/docs/phone-numbers/api/availablephonenumber-resource#local-get-basic-example-3). If specified, this value must have at least two characters. |  |
**sms_enabled** | Option<**bool**> | Whether the phone numbers can receive text messages. Can be: `true` or `false`. |  |
**mms_enabled** | Option<**bool**> | Whether the phone numbers can receive MMS messages. Can be: `true` or `false`. |  |
**voice_enabled** | Option<**bool**> | Whether the phone numbers can receive calls. Can be: `true` or `false`. |  |
**exclude_all_address_required** | Option<**bool**> | Whether to exclude phone numbers that require an [Address](https://www.twilio.com/docs/usage/api/address). Can be: `true` or `false` and the default is `false`. |  |
**exclude_local_address_required** | Option<**bool**> | Whether to exclude phone numbers that require a local [Address](https://www.twilio.com/docs/usage/api/address). Can be: `true` or `false` and the default is `false`. |  |
**exclude_foreign_address_required** | Option<**bool**> | Whether to exclude phone numbers that require a foreign [Address](https://www.twilio.com/docs/usage/api/address). Can be: `true` or `false` and the default is `false`. |  |
**beta** | Option<**bool**> | Whether to read phone numbers that are new to the Twilio platform. Can be: `true` or `false` and the default is `true`. |  |
**near_number** | Option<**String**> | Given a phone number, find a geographically close number within `distance` miles. Distance defaults to 25 miles. Applies to only phone numbers in the US and Canada. |  |
**near_lat_long** | Option<**String**> | Given a latitude/longitude pair `lat,long` find geographically close numbers within `distance` miles. Applies to only phone numbers in the US and Canada. |  |
**distance** | Option<**i32**> | The search radius, in miles, for a `near_` query.  Can be up to `500` and the default is `25`. Applies to only phone numbers in the US and Canada. |  |
**in_postal_code** | Option<**String**> | Limit results to a particular postal code. Given a phone number, search within the same postal code as that number. Applies to only phone numbers in the US and Canada. |  |
**in_region** | Option<**String**> | Limit results to a particular region, state, or province. Given a phone number, search within the same region as that number. Applies to only phone numbers in the US and Canada. |  |
**in_rate_center** | Option<**String**> | Limit results to a specific rate center, or given a phone number search within the same rate center as that number. Requires `in_lata` to be set as well. Applies to only phone numbers in the US and Canada. |  |
**in_lata** | Option<**String**> | Limit results to a specific local access and transport area ([LATA](https://en.wikipedia.org/wiki/Local_access_and_transport_area)). Given a phone number, search within the same [LATA](https://en.wikipedia.org/wiki/Local_access_and_transport_area) as that number. Applies to only phone numbers in the US and Canada. |  |
**in_locality** | Option<**String**> | Limit results to a particular locality or city. Given a phone number, search within the same Locality as that number. |  |
**fax_enabled** | Option<**bool**> | Whether the phone numbers can receive faxes. Can be: `true` or `false`. |  |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListAvailablePhoneNumberNationalResponse**](ListAvailablePhoneNumberNationalResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_available_phone_number_shared_cost

> crate::models::ListAvailablePhoneNumberSharedCostResponse list_available_phone_number_shared_cost(account_sid, country_code, area_code, contains, sms_enabled, mms_enabled, voice_enabled, exclude_all_address_required, exclude_local_address_required, exclude_foreign_address_required, beta, near_number, near_lat_long, distance, in_postal_code, in_region, in_rate_center, in_lata, in_locality, fax_enabled, page_size)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) requesting the AvailablePhoneNumber resources. | [required] |
**country_code** | **String** | The [ISO-3166-1](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) country code of the country from which to read phone numbers. | [required] |
**area_code** | Option<**i32**> | The area code of the phone numbers to read. Applies to only phone numbers in the US and Canada. |  |
**contains** | Option<**String**> | The pattern on which to match phone numbers. Valid characters are `*`, `0-9`, `a-z`, and `A-Z`. The `*` character matches any single digit. For examples, see [Example 2](https://www.twilio.com/docs/phone-numbers/api/availablephonenumber-resource#local-get-basic-example-2) and [Example 3](https://www.twilio.com/docs/phone-numbers/api/availablephonenumber-resource#local-get-basic-example-3). If specified, this value must have at least two characters. |  |
**sms_enabled** | Option<**bool**> | Whether the phone numbers can receive text messages. Can be: `true` or `false`. |  |
**mms_enabled** | Option<**bool**> | Whether the phone numbers can receive MMS messages. Can be: `true` or `false`. |  |
**voice_enabled** | Option<**bool**> | Whether the phone numbers can receive calls. Can be: `true` or `false`. |  |
**exclude_all_address_required** | Option<**bool**> | Whether to exclude phone numbers that require an [Address](https://www.twilio.com/docs/usage/api/address). Can be: `true` or `false` and the default is `false`. |  |
**exclude_local_address_required** | Option<**bool**> | Whether to exclude phone numbers that require a local [Address](https://www.twilio.com/docs/usage/api/address). Can be: `true` or `false` and the default is `false`. |  |
**exclude_foreign_address_required** | Option<**bool**> | Whether to exclude phone numbers that require a foreign [Address](https://www.twilio.com/docs/usage/api/address). Can be: `true` or `false` and the default is `false`. |  |
**beta** | Option<**bool**> | Whether to read phone numbers that are new to the Twilio platform. Can be: `true` or `false` and the default is `true`. |  |
**near_number** | Option<**String**> | Given a phone number, find a geographically close number within `distance` miles. Distance defaults to 25 miles. Applies to only phone numbers in the US and Canada. |  |
**near_lat_long** | Option<**String**> | Given a latitude/longitude pair `lat,long` find geographically close numbers within `distance` miles. Applies to only phone numbers in the US and Canada. |  |
**distance** | Option<**i32**> | The search radius, in miles, for a `near_` query.  Can be up to `500` and the default is `25`. Applies to only phone numbers in the US and Canada. |  |
**in_postal_code** | Option<**String**> | Limit results to a particular postal code. Given a phone number, search within the same postal code as that number. Applies to only phone numbers in the US and Canada. |  |
**in_region** | Option<**String**> | Limit results to a particular region, state, or province. Given a phone number, search within the same region as that number. Applies to only phone numbers in the US and Canada. |  |
**in_rate_center** | Option<**String**> | Limit results to a specific rate center, or given a phone number search within the same rate center as that number. Requires `in_lata` to be set as well. Applies to only phone numbers in the US and Canada. |  |
**in_lata** | Option<**String**> | Limit results to a specific local access and transport area ([LATA](https://en.wikipedia.org/wiki/Local_access_and_transport_area)). Given a phone number, search within the same [LATA](https://en.wikipedia.org/wiki/Local_access_and_transport_area) as that number. Applies to only phone numbers in the US and Canada. |  |
**in_locality** | Option<**String**> | Limit results to a particular locality or city. Given a phone number, search within the same Locality as that number. |  |
**fax_enabled** | Option<**bool**> | Whether the phone numbers can receive faxes. Can be: `true` or `false`. |  |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListAvailablePhoneNumberSharedCostResponse**](ListAvailablePhoneNumberSharedCostResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_available_phone_number_toll_free

> crate::models::ListAvailablePhoneNumberTollFreeResponse list_available_phone_number_toll_free(account_sid, country_code, area_code, contains, sms_enabled, mms_enabled, voice_enabled, exclude_all_address_required, exclude_local_address_required, exclude_foreign_address_required, beta, near_number, near_lat_long, distance, in_postal_code, in_region, in_rate_center, in_lata, in_locality, fax_enabled, page_size)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) requesting the AvailablePhoneNumber resources. | [required] |
**country_code** | **String** | The [ISO-3166-1](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) country code of the country from which to read phone numbers. | [required] |
**area_code** | Option<**i32**> | The area code of the phone numbers to read. Applies to only phone numbers in the US and Canada. |  |
**contains** | Option<**String**> | The pattern on which to match phone numbers. Valid characters are `*`, `0-9`, `a-z`, and `A-Z`. The `*` character matches any single digit. For examples, see [Example 2](https://www.twilio.com/docs/phone-numbers/api/availablephonenumber-resource#local-get-basic-example-2) and [Example 3](https://www.twilio.com/docs/phone-numbers/api/availablephonenumber-resource#local-get-basic-example-3). If specified, this value must have at least two characters. |  |
**sms_enabled** | Option<**bool**> | Whether the phone numbers can receive text messages. Can be: `true` or `false`. |  |
**mms_enabled** | Option<**bool**> | Whether the phone numbers can receive MMS messages. Can be: `true` or `false`. |  |
**voice_enabled** | Option<**bool**> | Whether the phone numbers can receive calls. Can be: `true` or `false`. |  |
**exclude_all_address_required** | Option<**bool**> | Whether to exclude phone numbers that require an [Address](https://www.twilio.com/docs/usage/api/address). Can be: `true` or `false` and the default is `false`. |  |
**exclude_local_address_required** | Option<**bool**> | Whether to exclude phone numbers that require a local [Address](https://www.twilio.com/docs/usage/api/address). Can be: `true` or `false` and the default is `false`. |  |
**exclude_foreign_address_required** | Option<**bool**> | Whether to exclude phone numbers that require a foreign [Address](https://www.twilio.com/docs/usage/api/address). Can be: `true` or `false` and the default is `false`. |  |
**beta** | Option<**bool**> | Whether to read phone numbers that are new to the Twilio platform. Can be: `true` or `false` and the default is `true`. |  |
**near_number** | Option<**String**> | Given a phone number, find a geographically close number within `distance` miles. Distance defaults to 25 miles. Applies to only phone numbers in the US and Canada. |  |
**near_lat_long** | Option<**String**> | Given a latitude/longitude pair `lat,long` find geographically close numbers within `distance` miles. Applies to only phone numbers in the US and Canada. |  |
**distance** | Option<**i32**> | The search radius, in miles, for a `near_` query.  Can be up to `500` and the default is `25`. Applies to only phone numbers in the US and Canada. |  |
**in_postal_code** | Option<**String**> | Limit results to a particular postal code. Given a phone number, search within the same postal code as that number. Applies to only phone numbers in the US and Canada. |  |
**in_region** | Option<**String**> | Limit results to a particular region, state, or province. Given a phone number, search within the same region as that number. Applies to only phone numbers in the US and Canada. |  |
**in_rate_center** | Option<**String**> | Limit results to a specific rate center, or given a phone number search within the same rate center as that number. Requires `in_lata` to be set as well. Applies to only phone numbers in the US and Canada. |  |
**in_lata** | Option<**String**> | Limit results to a specific local access and transport area ([LATA](https://en.wikipedia.org/wiki/Local_access_and_transport_area)). Given a phone number, search within the same [LATA](https://en.wikipedia.org/wiki/Local_access_and_transport_area) as that number. Applies to only phone numbers in the US and Canada. |  |
**in_locality** | Option<**String**> | Limit results to a particular locality or city. Given a phone number, search within the same Locality as that number. |  |
**fax_enabled** | Option<**bool**> | Whether the phone numbers can receive faxes. Can be: `true` or `false`. |  |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListAvailablePhoneNumberTollFreeResponse**](ListAvailablePhoneNumberTollFreeResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_available_phone_number_voip

> crate::models::ListAvailablePhoneNumberVoipResponse list_available_phone_number_voip(account_sid, country_code, area_code, contains, sms_enabled, mms_enabled, voice_enabled, exclude_all_address_required, exclude_local_address_required, exclude_foreign_address_required, beta, near_number, near_lat_long, distance, in_postal_code, in_region, in_rate_center, in_lata, in_locality, fax_enabled, page_size)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) requesting the AvailablePhoneNumber resources. | [required] |
**country_code** | **String** | The [ISO-3166-1](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) country code of the country from which to read phone numbers. | [required] |
**area_code** | Option<**i32**> | The area code of the phone numbers to read. Applies to only phone numbers in the US and Canada. |  |
**contains** | Option<**String**> | The pattern on which to match phone numbers. Valid characters are `*`, `0-9`, `a-z`, and `A-Z`. The `*` character matches any single digit. For examples, see [Example 2](https://www.twilio.com/docs/phone-numbers/api/availablephonenumber-resource#local-get-basic-example-2) and [Example 3](https://www.twilio.com/docs/phone-numbers/api/availablephonenumber-resource#local-get-basic-example-3). If specified, this value must have at least two characters. |  |
**sms_enabled** | Option<**bool**> | Whether the phone numbers can receive text messages. Can be: `true` or `false`. |  |
**mms_enabled** | Option<**bool**> | Whether the phone numbers can receive MMS messages. Can be: `true` or `false`. |  |
**voice_enabled** | Option<**bool**> | Whether the phone numbers can receive calls. Can be: `true` or `false`. |  |
**exclude_all_address_required** | Option<**bool**> | Whether to exclude phone numbers that require an [Address](https://www.twilio.com/docs/usage/api/address). Can be: `true` or `false` and the default is `false`. |  |
**exclude_local_address_required** | Option<**bool**> | Whether to exclude phone numbers that require a local [Address](https://www.twilio.com/docs/usage/api/address). Can be: `true` or `false` and the default is `false`. |  |
**exclude_foreign_address_required** | Option<**bool**> | Whether to exclude phone numbers that require a foreign [Address](https://www.twilio.com/docs/usage/api/address). Can be: `true` or `false` and the default is `false`. |  |
**beta** | Option<**bool**> | Whether to read phone numbers that are new to the Twilio platform. Can be: `true` or `false` and the default is `true`. |  |
**near_number** | Option<**String**> | Given a phone number, find a geographically close number within `distance` miles. Distance defaults to 25 miles. Applies to only phone numbers in the US and Canada. |  |
**near_lat_long** | Option<**String**> | Given a latitude/longitude pair `lat,long` find geographically close numbers within `distance` miles. Applies to only phone numbers in the US and Canada. |  |
**distance** | Option<**i32**> | The search radius, in miles, for a `near_` query.  Can be up to `500` and the default is `25`. Applies to only phone numbers in the US and Canada. |  |
**in_postal_code** | Option<**String**> | Limit results to a particular postal code. Given a phone number, search within the same postal code as that number. Applies to only phone numbers in the US and Canada. |  |
**in_region** | Option<**String**> | Limit results to a particular region, state, or province. Given a phone number, search within the same region as that number. Applies to only phone numbers in the US and Canada. |  |
**in_rate_center** | Option<**String**> | Limit results to a specific rate center, or given a phone number search within the same rate center as that number. Requires `in_lata` to be set as well. Applies to only phone numbers in the US and Canada. |  |
**in_lata** | Option<**String**> | Limit results to a specific local access and transport area ([LATA](https://en.wikipedia.org/wiki/Local_access_and_transport_area)). Given a phone number, search within the same [LATA](https://en.wikipedia.org/wiki/Local_access_and_transport_area) as that number. Applies to only phone numbers in the US and Canada. |  |
**in_locality** | Option<**String**> | Limit results to a particular locality or city. Given a phone number, search within the same Locality as that number. |  |
**fax_enabled** | Option<**bool**> | Whether the phone numbers can receive faxes. Can be: `true` or `false`. |  |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListAvailablePhoneNumberVoipResponse**](ListAvailablePhoneNumberVoipResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_call

> crate::models::ListCallResponse list_call(account_sid, to, from, parent_call_sid, status, start_time, start_time_less_than, start_time_greater_than, end_time, end_time_less_than, end_time_greater_than, page_size)


Retrieves a collection of calls made to and from your account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Call resource(s) to read. | [required] |
**to** | Option<**String**> | Only show calls made to this phone number, SIP address, Client identifier or SIM SID. |  |
**from** | Option<**String**> | Only include calls from this phone number, SIP address, Client identifier or SIM SID. |  |
**parent_call_sid** | Option<**String**> | Only include calls spawned by calls with this SID. |  |
**status** | Option<**CallEnumStatus**> | The status of the calls to include. Can be: `queued`, `ringing`, `in-progress`, `canceled`, `completed`, `failed`, `busy`, or `no-answer`. |  |
**start_time** | Option<**String**> | Only include calls that started on this date. Specify a date as `YYYY-MM-DD` in GMT, for example: `2009-07-06`, to read only calls that started on this date. You can also specify an inequality, such as `StartTime<=YYYY-MM-DD`, to read calls that started on or before midnight of this date, and `StartTime>=YYYY-MM-DD` to read calls that started on or after midnight of this date. |  |
**start_time_less_than** | Option<**String**> | Only include calls that started on this date. Specify a date as `YYYY-MM-DD` in GMT, for example: `2009-07-06`, to read only calls that started on this date. You can also specify an inequality, such as `StartTime<=YYYY-MM-DD`, to read calls that started on or before midnight of this date, and `StartTime>=YYYY-MM-DD` to read calls that started on or after midnight of this date. |  |
**start_time_greater_than** | Option<**String**> | Only include calls that started on this date. Specify a date as `YYYY-MM-DD` in GMT, for example: `2009-07-06`, to read only calls that started on this date. You can also specify an inequality, such as `StartTime<=YYYY-MM-DD`, to read calls that started on or before midnight of this date, and `StartTime>=YYYY-MM-DD` to read calls that started on or after midnight of this date. |  |
**end_time** | Option<**String**> | Only include calls that ended on this date. Specify a date as `YYYY-MM-DD` in GMT, for example: `2009-07-06`, to read only calls that ended on this date. You can also specify an inequality, such as `EndTime<=YYYY-MM-DD`, to read calls that ended on or before midnight of this date, and `EndTime>=YYYY-MM-DD` to read calls that ended on or after midnight of this date. |  |
**end_time_less_than** | Option<**String**> | Only include calls that ended on this date. Specify a date as `YYYY-MM-DD` in GMT, for example: `2009-07-06`, to read only calls that ended on this date. You can also specify an inequality, such as `EndTime<=YYYY-MM-DD`, to read calls that ended on or before midnight of this date, and `EndTime>=YYYY-MM-DD` to read calls that ended on or after midnight of this date. |  |
**end_time_greater_than** | Option<**String**> | Only include calls that ended on this date. Specify a date as `YYYY-MM-DD` in GMT, for example: `2009-07-06`, to read only calls that ended on this date. You can also specify an inequality, such as `EndTime<=YYYY-MM-DD`, to read calls that ended on or before midnight of this date, and `EndTime>=YYYY-MM-DD` to read calls that ended on or after midnight of this date. |  |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListCallResponse**](ListCallResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_call_event

> crate::models::ListCallEventResponse list_call_event(account_sid, call_sid, page_size)


Retrieve a list of all events for a call.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The unique SID identifier of the Account. | [required] |
**call_sid** | **String** | The unique SID identifier of the Call. | [required] |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListCallEventResponse**](ListCallEventResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_call_notification

> crate::models::ListCallNotificationResponse list_call_notification(account_sid, call_sid, log, message_date, message_date_less_than, message_date_greater_than, page_size)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Call Notification resources to read. | [required] |
**call_sid** | **String** | The [Call](https://www.twilio.com/docs/voice/api/call-resource) SID of the Call Notification resources to read. | [required] |
**log** | Option<**i32**> | Only read notifications of the specified log level. Can be:  `0` to read only ERROR notifications or `1` to read only WARNING notifications. By default, all notifications are read. |  |
**message_date** | Option<**String**> | Only show notifications for the specified date, formatted as `YYYY-MM-DD`. You can also specify an inequality, such as `<=YYYY-MM-DD` for messages logged at or before midnight on a date, or `>=YYYY-MM-DD` for messages logged at or after midnight on a date. |  |
**message_date_less_than** | Option<**String**> | Only show notifications for the specified date, formatted as `YYYY-MM-DD`. You can also specify an inequality, such as `<=YYYY-MM-DD` for messages logged at or before midnight on a date, or `>=YYYY-MM-DD` for messages logged at or after midnight on a date. |  |
**message_date_greater_than** | Option<**String**> | Only show notifications for the specified date, formatted as `YYYY-MM-DD`. You can also specify an inequality, such as `<=YYYY-MM-DD` for messages logged at or before midnight on a date, or `>=YYYY-MM-DD` for messages logged at or after midnight on a date. |  |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListCallNotificationResponse**](ListCallNotificationResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_call_recording

> crate::models::ListCallRecordingResponse list_call_recording(account_sid, call_sid, date_created, date_created_less_than, date_created_greater_than, page_size)


Retrieve a list of recordings belonging to the call used to make the request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Recording resources to read. | [required] |
**call_sid** | **String** | The [Call](https://www.twilio.com/docs/voice/api/call-resource) SID of the resources to read. | [required] |
**date_created** | Option<**String**> | The `date_created` value, specified as `YYYY-MM-DD`, of the resources to read. You can also specify inequality: `DateCreated<=YYYY-MM-DD` will return recordings generated at or before midnight on a given date, and `DateCreated>=YYYY-MM-DD` returns recordings generated at or after midnight on a date. |  |
**date_created_less_than** | Option<**String**> | The `date_created` value, specified as `YYYY-MM-DD`, of the resources to read. You can also specify inequality: `DateCreated<=YYYY-MM-DD` will return recordings generated at or before midnight on a given date, and `DateCreated>=YYYY-MM-DD` returns recordings generated at or after midnight on a date. |  |
**date_created_greater_than** | Option<**String**> | The `date_created` value, specified as `YYYY-MM-DD`, of the resources to read. You can also specify inequality: `DateCreated<=YYYY-MM-DD` will return recordings generated at or before midnight on a given date, and `DateCreated>=YYYY-MM-DD` returns recordings generated at or after midnight on a date. |  |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListCallRecordingResponse**](ListCallRecordingResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_conference

> crate::models::ListConferenceResponse list_conference(account_sid, date_created, date_created_less_than, date_created_greater_than, date_updated, date_updated_less_than, date_updated_greater_than, friendly_name, status, page_size)


Retrieve a list of conferences belonging to the account used to make the request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Conference resource(s) to read. | [required] |
**date_created** | Option<**String**> | The `date_created` value, specified as `YYYY-MM-DD`, of the resources to read. To read conferences that started on or before midnight on a date, use `<=YYYY-MM-DD`, and to specify  conferences that started on or after midnight on a date, use `>=YYYY-MM-DD`. |  |
**date_created_less_than** | Option<**String**> | The `date_created` value, specified as `YYYY-MM-DD`, of the resources to read. To read conferences that started on or before midnight on a date, use `<=YYYY-MM-DD`, and to specify  conferences that started on or after midnight on a date, use `>=YYYY-MM-DD`. |  |
**date_created_greater_than** | Option<**String**> | The `date_created` value, specified as `YYYY-MM-DD`, of the resources to read. To read conferences that started on or before midnight on a date, use `<=YYYY-MM-DD`, and to specify  conferences that started on or after midnight on a date, use `>=YYYY-MM-DD`. |  |
**date_updated** | Option<**String**> | The `date_updated` value, specified as `YYYY-MM-DD`, of the resources to read. To read conferences that were last updated on or before midnight on a date, use `<=YYYY-MM-DD`, and to specify conferences that were last updated on or after midnight on a given date, use  `>=YYYY-MM-DD`. |  |
**date_updated_less_than** | Option<**String**> | The `date_updated` value, specified as `YYYY-MM-DD`, of the resources to read. To read conferences that were last updated on or before midnight on a date, use `<=YYYY-MM-DD`, and to specify conferences that were last updated on or after midnight on a given date, use  `>=YYYY-MM-DD`. |  |
**date_updated_greater_than** | Option<**String**> | The `date_updated` value, specified as `YYYY-MM-DD`, of the resources to read. To read conferences that were last updated on or before midnight on a date, use `<=YYYY-MM-DD`, and to specify conferences that were last updated on or after midnight on a given date, use  `>=YYYY-MM-DD`. |  |
**friendly_name** | Option<**String**> | The string that identifies the Conference resources to read. |  |
**status** | Option<**ConferenceEnumStatus**> | The status of the resources to read. Can be: `init`, `in-progress`, or `completed`. |  |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListConferenceResponse**](ListConferenceResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_conference_recording

> crate::models::ListConferenceRecordingResponse list_conference_recording(account_sid, conference_sid, date_created, date_created_less_than, date_created_greater_than, page_size)


Retrieve a list of recordings belonging to the call used to make the request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Conference Recording resources to read. | [required] |
**conference_sid** | **String** | The Conference SID that identifies the conference associated with the recording to read. | [required] |
**date_created** | Option<**String**> | The `date_created` value, specified as `YYYY-MM-DD`, of the resources to read. You can also specify inequality: `DateCreated<=YYYY-MM-DD` will return recordings generated at or before midnight on a given date, and `DateCreated>=YYYY-MM-DD` returns recordings generated at or after midnight on a date. |  |
**date_created_less_than** | Option<**String**> | The `date_created` value, specified as `YYYY-MM-DD`, of the resources to read. You can also specify inequality: `DateCreated<=YYYY-MM-DD` will return recordings generated at or before midnight on a given date, and `DateCreated>=YYYY-MM-DD` returns recordings generated at or after midnight on a date. |  |
**date_created_greater_than** | Option<**String**> | The `date_created` value, specified as `YYYY-MM-DD`, of the resources to read. You can also specify inequality: `DateCreated<=YYYY-MM-DD` will return recordings generated at or before midnight on a given date, and `DateCreated>=YYYY-MM-DD` returns recordings generated at or after midnight on a date. |  |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListConferenceRecordingResponse**](ListConferenceRecordingResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_connect_app

> crate::models::ListConnectAppResponse list_connect_app(account_sid, page_size)


Retrieve a list of connect-apps belonging to the account used to make the request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the ConnectApp resources to read. | [required] |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListConnectAppResponse**](ListConnectAppResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_dependent_phone_number

> crate::models::ListDependentPhoneNumberResponse list_dependent_phone_number(account_sid, address_sid, page_size)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the DependentPhoneNumber resources to read. | [required] |
**address_sid** | **String** | The SID of the Address resource associated with the phone number. | [required] |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListDependentPhoneNumberResponse**](ListDependentPhoneNumberResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_incoming_phone_number

> crate::models::ListIncomingPhoneNumberResponse list_incoming_phone_number(account_sid, beta, friendly_name, phone_number, origin, page_size)


Retrieve a list of incoming-phone-numbers belonging to the account used to make the request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the IncomingPhoneNumber resources to read. | [required] |
**beta** | Option<**bool**> | Whether to include phone numbers new to the Twilio platform. Can be: `true` or `false` and the default is `true`. |  |
**friendly_name** | Option<**String**> | A string that identifies the IncomingPhoneNumber resources to read. |  |
**phone_number** | Option<**String**> | The phone numbers of the IncomingPhoneNumber resources to read. You can specify partial numbers and use '*' as a wildcard for any digit. |  |
**origin** | Option<**String**> | Whether to include phone numbers based on their origin. Can be: `twilio` or `hosted`. By default, phone numbers of all origin are included. |  |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListIncomingPhoneNumberResponse**](ListIncomingPhoneNumberResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_incoming_phone_number_assigned_add_on

> crate::models::ListIncomingPhoneNumberAssignedAddOnResponse list_incoming_phone_number_assigned_add_on(account_sid, resource_sid, page_size)


Retrieve a list of Add-on installations currently assigned to this Number.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the resources to read. | [required] |
**resource_sid** | **String** | The SID of the Phone Number to which the Add-on is assigned. | [required] |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListIncomingPhoneNumberAssignedAddOnResponse**](ListIncomingPhoneNumberAssignedAddOnResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_incoming_phone_number_assigned_add_on_extension

> crate::models::ListIncomingPhoneNumberAssignedAddOnExtensionResponse list_incoming_phone_number_assigned_add_on_extension(account_sid, resource_sid, assigned_add_on_sid, page_size)


Retrieve a list of Extensions for the Assigned Add-on.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the resources to read. | [required] |
**resource_sid** | **String** | The SID of the Phone Number to which the Add-on is assigned. | [required] |
**assigned_add_on_sid** | **String** | The SID that uniquely identifies the assigned Add-on installation. | [required] |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListIncomingPhoneNumberAssignedAddOnExtensionResponse**](ListIncomingPhoneNumberAssignedAddOnExtensionResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_incoming_phone_number_local

> crate::models::ListIncomingPhoneNumberLocalResponse list_incoming_phone_number_local(account_sid, beta, friendly_name, phone_number, origin, page_size)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the resources to read. | [required] |
**beta** | Option<**bool**> | Whether to include phone numbers new to the Twilio platform. Can be: `true` or `false` and the default is `true`. |  |
**friendly_name** | Option<**String**> | A string that identifies the resources to read. |  |
**phone_number** | Option<**String**> | The phone numbers of the IncomingPhoneNumber resources to read. You can specify partial numbers and use '*' as a wildcard for any digit. |  |
**origin** | Option<**String**> | Whether to include phone numbers based on their origin. Can be: `twilio` or `hosted`. By default, phone numbers of all origin are included. |  |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListIncomingPhoneNumberLocalResponse**](ListIncomingPhoneNumberLocalResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_incoming_phone_number_mobile

> crate::models::ListIncomingPhoneNumberMobileResponse list_incoming_phone_number_mobile(account_sid, beta, friendly_name, phone_number, origin, page_size)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the resources to read. | [required] |
**beta** | Option<**bool**> | Whether to include phone numbers new to the Twilio platform. Can be: `true` or `false` and the default is `true`. |  |
**friendly_name** | Option<**String**> | A string that identifies the resources to read. |  |
**phone_number** | Option<**String**> | The phone numbers of the IncomingPhoneNumber resources to read. You can specify partial numbers and use '*' as a wildcard for any digit. |  |
**origin** | Option<**String**> | Whether to include phone numbers based on their origin. Can be: `twilio` or `hosted`. By default, phone numbers of all origin are included. |  |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListIncomingPhoneNumberMobileResponse**](ListIncomingPhoneNumberMobileResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_incoming_phone_number_toll_free

> crate::models::ListIncomingPhoneNumberTollFreeResponse list_incoming_phone_number_toll_free(account_sid, beta, friendly_name, phone_number, origin, page_size)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the resources to read. | [required] |
**beta** | Option<**bool**> | Whether to include phone numbers new to the Twilio platform. Can be: `true` or `false` and the default is `true`. |  |
**friendly_name** | Option<**String**> | A string that identifies the resources to read. |  |
**phone_number** | Option<**String**> | The phone numbers of the IncomingPhoneNumber resources to read. You can specify partial numbers and use '*' as a wildcard for any digit. |  |
**origin** | Option<**String**> | Whether to include phone numbers based on their origin. Can be: `twilio` or `hosted`. By default, phone numbers of all origin are included. |  |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListIncomingPhoneNumberTollFreeResponse**](ListIncomingPhoneNumberTollFreeResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_key

> crate::models::ListKeyResponse list_key(account_sid, page_size)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Key resources to read. | [required] |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListKeyResponse**](ListKeyResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_media

> crate::models::ListMediaResponse list_media(account_sid, message_sid, date_created, date_created_less_than, date_created_greater_than, page_size)


Retrieve a list of Media resources belonging to the account used to make the request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Media resource(s) to read. | [required] |
**message_sid** | **String** | The SID of the Message resource that this Media resource belongs to. | [required] |
**date_created** | Option<**String**> | Only include media that was created on this date. Specify a date as `YYYY-MM-DD` in GMT, for example: `2009-07-06`, to read media that was created on this date. You can also specify an inequality, such as `StartTime<=YYYY-MM-DD`, to read media that was created on or before midnight of this date, and `StartTime>=YYYY-MM-DD` to read media that was created on or after midnight of this date. |  |
**date_created_less_than** | Option<**String**> | Only include media that was created on this date. Specify a date as `YYYY-MM-DD` in GMT, for example: `2009-07-06`, to read media that was created on this date. You can also specify an inequality, such as `StartTime<=YYYY-MM-DD`, to read media that was created on or before midnight of this date, and `StartTime>=YYYY-MM-DD` to read media that was created on or after midnight of this date. |  |
**date_created_greater_than** | Option<**String**> | Only include media that was created on this date. Specify a date as `YYYY-MM-DD` in GMT, for example: `2009-07-06`, to read media that was created on this date. You can also specify an inequality, such as `StartTime<=YYYY-MM-DD`, to read media that was created on or before midnight of this date, and `StartTime>=YYYY-MM-DD` to read media that was created on or after midnight of this date. |  |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListMediaResponse**](ListMediaResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_member

> crate::models::ListMemberResponse list_member(account_sid, queue_sid, page_size)


Retrieve the members of the queue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Member resource(s) to read. | [required] |
**queue_sid** | **String** | The SID of the Queue in which to find the members | [required] |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListMemberResponse**](ListMemberResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_message

> crate::models::ListMessageResponse list_message(account_sid, to, from, date_sent, date_sent_less_than, date_sent_greater_than, page_size)


Retrieve a list of messages belonging to the account used to make the request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Message resources to read. | [required] |
**to** | Option<**String**> | Read messages sent to only this phone number. |  |
**from** | Option<**String**> | Read messages sent from only this phone number or alphanumeric sender ID. |  |
**date_sent** | Option<**String**> | The date of the messages to show. Specify a date as `YYYY-MM-DD` in GMT to read only messages sent on this date. For example: `2009-07-06`. You can also specify an inequality, such as `DateSent<=YYYY-MM-DD`, to read messages sent on or before midnight on a date, and `DateSent>=YYYY-MM-DD` to read messages sent on or after midnight on a date. |  |
**date_sent_less_than** | Option<**String**> | The date of the messages to show. Specify a date as `YYYY-MM-DD` in GMT to read only messages sent on this date. For example: `2009-07-06`. You can also specify an inequality, such as `DateSent<=YYYY-MM-DD`, to read messages sent on or before midnight on a date, and `DateSent>=YYYY-MM-DD` to read messages sent on or after midnight on a date. |  |
**date_sent_greater_than** | Option<**String**> | The date of the messages to show. Specify a date as `YYYY-MM-DD` in GMT to read only messages sent on this date. For example: `2009-07-06`. You can also specify an inequality, such as `DateSent<=YYYY-MM-DD`, to read messages sent on or before midnight on a date, and `DateSent>=YYYY-MM-DD` to read messages sent on or after midnight on a date. |  |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListMessageResponse**](ListMessageResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_notification

> crate::models::ListNotificationResponse list_notification(account_sid, log, message_date, message_date_less_than, message_date_greater_than, page_size)


Retrieve a list of notifications belonging to the account used to make the request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Notification resources to read. | [required] |
**log** | Option<**i32**> | Only read notifications of the specified log level. Can be:  `0` to read only ERROR notifications or `1` to read only WARNING notifications. By default, all notifications are read. |  |
**message_date** | Option<**String**> | Only show notifications for the specified date, formatted as `YYYY-MM-DD`. You can also specify an inequality, such as `<=YYYY-MM-DD` for messages logged at or before midnight on a date, or `>=YYYY-MM-DD` for messages logged at or after midnight on a date. |  |
**message_date_less_than** | Option<**String**> | Only show notifications for the specified date, formatted as `YYYY-MM-DD`. You can also specify an inequality, such as `<=YYYY-MM-DD` for messages logged at or before midnight on a date, or `>=YYYY-MM-DD` for messages logged at or after midnight on a date. |  |
**message_date_greater_than** | Option<**String**> | Only show notifications for the specified date, formatted as `YYYY-MM-DD`. You can also specify an inequality, such as `<=YYYY-MM-DD` for messages logged at or before midnight on a date, or `>=YYYY-MM-DD` for messages logged at or after midnight on a date. |  |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListNotificationResponse**](ListNotificationResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_outgoing_caller_id

> crate::models::ListOutgoingCallerIdResponse list_outgoing_caller_id(account_sid, phone_number, friendly_name, page_size)


Retrieve a list of outgoing-caller-ids belonging to the account used to make the request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the OutgoingCallerId resources to read. | [required] |
**phone_number** | Option<**String**> | The phone number of the OutgoingCallerId resources to read. |  |
**friendly_name** | Option<**String**> | The string that identifies the OutgoingCallerId resources to read. |  |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListOutgoingCallerIdResponse**](ListOutgoingCallerIdResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_participant

> crate::models::ListParticipantResponse list_participant(account_sid, conference_sid, muted, hold, coaching, page_size)


Retrieve a list of participants belonging to the account used to make the request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Participant resources to read. | [required] |
**conference_sid** | **String** | The SID of the conference with the participants to read. | [required] |
**muted** | Option<**bool**> | Whether to return only participants that are muted. Can be: `true` or `false`. |  |
**hold** | Option<**bool**> | Whether to return only participants that are on hold. Can be: `true` or `false`. |  |
**coaching** | Option<**bool**> | Whether to return only participants who are coaching another call. Can be: `true` or `false`. |  |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListParticipantResponse**](ListParticipantResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_queue

> crate::models::ListQueueResponse list_queue(account_sid, page_size)


Retrieve a list of queues belonging to the account used to make the request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Queue resources to read. | [required] |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListQueueResponse**](ListQueueResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_recording

> crate::models::ListRecordingResponse list_recording(account_sid, date_created, date_created_less_than, date_created_greater_than, call_sid, conference_sid, include_soft_deleted, page_size)


Retrieve a list of recordings belonging to the account used to make the request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Recording resources to read. | [required] |
**date_created** | Option<**String**> | Only include recordings that were created on this date. Specify a date as `YYYY-MM-DD` in GMT, for example: `2009-07-06`, to read recordings that were created on this date. You can also specify an inequality, such as `DateCreated<=YYYY-MM-DD`, to read recordings that were created on or before midnight of this date, and `DateCreated>=YYYY-MM-DD` to read recordings that were created on or after midnight of this date. |  |
**date_created_less_than** | Option<**String**> | Only include recordings that were created on this date. Specify a date as `YYYY-MM-DD` in GMT, for example: `2009-07-06`, to read recordings that were created on this date. You can also specify an inequality, such as `DateCreated<=YYYY-MM-DD`, to read recordings that were created on or before midnight of this date, and `DateCreated>=YYYY-MM-DD` to read recordings that were created on or after midnight of this date. |  |
**date_created_greater_than** | Option<**String**> | Only include recordings that were created on this date. Specify a date as `YYYY-MM-DD` in GMT, for example: `2009-07-06`, to read recordings that were created on this date. You can also specify an inequality, such as `DateCreated<=YYYY-MM-DD`, to read recordings that were created on or before midnight of this date, and `DateCreated>=YYYY-MM-DD` to read recordings that were created on or after midnight of this date. |  |
**call_sid** | Option<**String**> | The [Call](https://www.twilio.com/docs/voice/api/call-resource) SID of the resources to read. |  |
**conference_sid** | Option<**String**> | The Conference SID that identifies the conference associated with the recording to read. |  |
**include_soft_deleted** | Option<**bool**> | A boolean parameter indicating whether to retrieve soft deleted recordings or not. Recordings metadata are kept after deletion for a retention period of 40 days. |  |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListRecordingResponse**](ListRecordingResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_recording_add_on_result

> crate::models::ListRecordingAddOnResultResponse list_recording_add_on_result(account_sid, reference_sid, page_size)


Retrieve a list of results belonging to the recording

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Recording AddOnResult resources to read. | [required] |
**reference_sid** | **String** | The SID of the recording to which the result to read belongs. | [required] |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListRecordingAddOnResultResponse**](ListRecordingAddOnResultResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_recording_add_on_result_payload

> crate::models::ListRecordingAddOnResultPayloadResponse list_recording_add_on_result_payload(account_sid, reference_sid, add_on_result_sid, page_size)


Retrieve a list of payloads belonging to the AddOnResult

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Recording AddOnResult Payload resources to read. | [required] |
**reference_sid** | **String** | The SID of the recording to which the AddOnResult resource that contains the payloads to read belongs. | [required] |
**add_on_result_sid** | **String** | The SID of the AddOnResult to which the payloads to read belongs. | [required] |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListRecordingAddOnResultPayloadResponse**](ListRecordingAddOnResultPayloadResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_recording_transcription

> crate::models::ListRecordingTranscriptionResponse list_recording_transcription(account_sid, recording_sid, page_size)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Transcription resources to read. | [required] |
**recording_sid** | **String** | The SID of the [Recording](https://www.twilio.com/docs/voice/api/recording) that created the transcriptions to read. | [required] |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListRecordingTranscriptionResponse**](ListRecordingTranscriptionResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_short_code

> crate::models::ListShortCodeResponse list_short_code(account_sid, friendly_name, short_code, page_size)


Retrieve a list of short-codes belonging to the account used to make the request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the ShortCode resource(s) to read. | [required] |
**friendly_name** | Option<**String**> | The string that identifies the ShortCode resources to read. |  |
**short_code** | Option<**String**> | Only show the ShortCode resources that match this pattern. You can specify partial numbers and use '*' as a wildcard for any digit. |  |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListShortCodeResponse**](ListShortCodeResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_signing_key

> crate::models::ListSigningKeyResponse list_signing_key(account_sid, page_size)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** |  | [required] |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListSigningKeyResponse**](ListSigningKeyResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_sip_auth_calls_credential_list_mapping

> crate::models::ListSipAuthCallsCredentialListMappingResponse list_sip_auth_calls_credential_list_mapping(account_sid, domain_sid, page_size)


Retrieve a list of credential list mappings belonging to the domain used in the request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the CredentialListMapping resources to read. | [required] |
**domain_sid** | **String** | The SID of the SIP domain that contains the resources to read. | [required] |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListSipAuthCallsCredentialListMappingResponse**](ListSipAuthCallsCredentialListMappingResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_sip_auth_calls_ip_access_control_list_mapping

> crate::models::ListSipAuthCallsIpAccessControlListMappingResponse list_sip_auth_calls_ip_access_control_list_mapping(account_sid, domain_sid, page_size)


Retrieve a list of IP Access Control List mappings belonging to the domain used in the request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the IpAccessControlListMapping resources to read. | [required] |
**domain_sid** | **String** | The SID of the SIP domain that contains the resources to read. | [required] |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListSipAuthCallsIpAccessControlListMappingResponse**](ListSipAuthCallsIpAccessControlListMappingResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_sip_auth_registrations_credential_list_mapping

> crate::models::ListSipAuthRegistrationsCredentialListMappingResponse list_sip_auth_registrations_credential_list_mapping(account_sid, domain_sid, page_size)


Retrieve a list of credential list mappings belonging to the domain used in the request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the CredentialListMapping resources to read. | [required] |
**domain_sid** | **String** | The SID of the SIP domain that contains the resources to read. | [required] |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListSipAuthRegistrationsCredentialListMappingResponse**](ListSipAuthRegistrationsCredentialListMappingResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_sip_credential

> crate::models::ListSipCredentialResponse list_sip_credential(account_sid, credential_list_sid, page_size)


Retrieve a list of credentials.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The unique id of the Account that is responsible for this resource. | [required] |
**credential_list_sid** | **String** | The unique id that identifies the credential list that contains the desired credentials. | [required] |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListSipCredentialResponse**](ListSipCredentialResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_sip_credential_list

> crate::models::ListSipCredentialListResponse list_sip_credential_list(account_sid, page_size)


Get All Credential Lists

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The unique id of the Account that is responsible for this resource. | [required] |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListSipCredentialListResponse**](ListSipCredentialListResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_sip_credential_list_mapping

> crate::models::ListSipCredentialListMappingResponse list_sip_credential_list_mapping(account_sid, domain_sid, page_size)


Read multiple CredentialListMapping resources from an account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The unique id of the [Account](https://www.twilio.com/docs/iam/api/account) responsible for this resource. | [required] |
**domain_sid** | **String** | A 34 character string that uniquely identifies the SIP Domain that includes the resource to read. | [required] |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListSipCredentialListMappingResponse**](ListSipCredentialListMappingResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_sip_domain

> crate::models::ListSipDomainResponse list_sip_domain(account_sid, page_size)


Retrieve a list of domains belonging to the account used to make the request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the SipDomain resources to read. | [required] |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListSipDomainResponse**](ListSipDomainResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_sip_ip_access_control_list

> crate::models::ListSipIpAccessControlListResponse list_sip_ip_access_control_list(account_sid, page_size)


Retrieve a list of IpAccessControlLists that belong to the account used to make the request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The unique id of the [Account](https://www.twilio.com/docs/iam/api/account) responsible for this resource. | [required] |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListSipIpAccessControlListResponse**](ListSipIpAccessControlListResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_sip_ip_access_control_list_mapping

> crate::models::ListSipIpAccessControlListMappingResponse list_sip_ip_access_control_list_mapping(account_sid, domain_sid, page_size)


Retrieve a list of IpAccessControlListMapping resources.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The unique id of the Account that is responsible for this resource. | [required] |
**domain_sid** | **String** | A 34 character string that uniquely identifies the SIP domain. | [required] |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListSipIpAccessControlListMappingResponse**](ListSipIpAccessControlListMappingResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_sip_ip_address

> crate::models::ListSipIpAddressResponse list_sip_ip_address(account_sid, ip_access_control_list_sid, page_size)


Read multiple IpAddress resources.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The unique id of the [Account](https://www.twilio.com/docs/iam/api/account) responsible for this resource. | [required] |
**ip_access_control_list_sid** | **String** | The IpAccessControlList Sid that identifies the IpAddress resources to read. | [required] |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListSipIpAddressResponse**](ListSipIpAddressResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_transcription

> crate::models::ListTranscriptionResponse list_transcription(account_sid, page_size)


Retrieve a list of transcriptions belonging to the account used to make the request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Transcription resources to read. | [required] |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListTranscriptionResponse**](ListTranscriptionResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_usage_record

> crate::models::ListUsageRecordResponse list_usage_record(account_sid, category, start_date, end_date, include_subaccounts, page_size)


Retrieve a list of usage-records belonging to the account used to make the request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the UsageRecord resources to read. | [required] |
**category** | Option<**UsageRecordEnumCategory**> | The [usage category](https://www.twilio.com/docs/usage/api/usage-record#usage-categories) of the UsageRecord resources to read. Only UsageRecord resources in the specified category are retrieved. |  |
**start_date** | Option<**String**> | Only include usage that has occurred on or after this date. Specify the date in GMT and format as `YYYY-MM-DD`. You can also specify offsets from the current date, such as: `-30days`, which will set the start date to be 30 days before the current date. |  |
**end_date** | Option<**String**> | Only include usage that occurred on or before this date. Specify the date in GMT and format as `YYYY-MM-DD`.  You can also specify offsets from the current date, such as: `+30days`, which will set the end date to 30 days from the current date. |  |
**include_subaccounts** | Option<**bool**> | Whether to include usage from the master account and all its subaccounts. Can be: `true` (the default) to include usage from the master account and all subaccounts or `false` to retrieve usage from only the specified account. |  |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListUsageRecordResponse**](ListUsageRecordResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_usage_record_all_time

> crate::models::ListUsageRecordAllTimeResponse list_usage_record_all_time(account_sid, category, start_date, end_date, include_subaccounts, page_size)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the UsageRecord resources to read. | [required] |
**category** | Option<**UsageRecordAllTimeEnumCategory**> | The [usage category](https://www.twilio.com/docs/usage/api/usage-record#usage-categories) of the UsageRecord resources to read. Only UsageRecord resources in the specified category are retrieved. |  |
**start_date** | Option<**String**> | Only include usage that has occurred on or after this date. Specify the date in GMT and format as `YYYY-MM-DD`. You can also specify offsets from the current date, such as: `-30days`, which will set the start date to be 30 days before the current date. |  |
**end_date** | Option<**String**> | Only include usage that occurred on or before this date. Specify the date in GMT and format as `YYYY-MM-DD`.  You can also specify offsets from the current date, such as: `+30days`, which will set the end date to 30 days from the current date. |  |
**include_subaccounts** | Option<**bool**> | Whether to include usage from the master account and all its subaccounts. Can be: `true` (the default) to include usage from the master account and all subaccounts or `false` to retrieve usage from only the specified account. |  |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListUsageRecordAllTimeResponse**](ListUsageRecordAllTimeResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_usage_record_daily

> crate::models::ListUsageRecordDailyResponse list_usage_record_daily(account_sid, category, start_date, end_date, include_subaccounts, page_size)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the UsageRecord resources to read. | [required] |
**category** | Option<**UsageRecordDailyEnumCategory**> | The [usage category](https://www.twilio.com/docs/usage/api/usage-record#usage-categories) of the UsageRecord resources to read. Only UsageRecord resources in the specified category are retrieved. |  |
**start_date** | Option<**String**> | Only include usage that has occurred on or after this date. Specify the date in GMT and format as `YYYY-MM-DD`. You can also specify offsets from the current date, such as: `-30days`, which will set the start date to be 30 days before the current date. |  |
**end_date** | Option<**String**> | Only include usage that occurred on or before this date. Specify the date in GMT and format as `YYYY-MM-DD`.  You can also specify offsets from the current date, such as: `+30days`, which will set the end date to 30 days from the current date. |  |
**include_subaccounts** | Option<**bool**> | Whether to include usage from the master account and all its subaccounts. Can be: `true` (the default) to include usage from the master account and all subaccounts or `false` to retrieve usage from only the specified account. |  |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListUsageRecordDailyResponse**](ListUsageRecordDailyResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_usage_record_last_month

> crate::models::ListUsageRecordLastMonthResponse list_usage_record_last_month(account_sid, category, start_date, end_date, include_subaccounts, page_size)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the UsageRecord resources to read. | [required] |
**category** | Option<**UsageRecordLastMonthEnumCategory**> | The [usage category](https://www.twilio.com/docs/usage/api/usage-record#usage-categories) of the UsageRecord resources to read. Only UsageRecord resources in the specified category are retrieved. |  |
**start_date** | Option<**String**> | Only include usage that has occurred on or after this date. Specify the date in GMT and format as `YYYY-MM-DD`. You can also specify offsets from the current date, such as: `-30days`, which will set the start date to be 30 days before the current date. |  |
**end_date** | Option<**String**> | Only include usage that occurred on or before this date. Specify the date in GMT and format as `YYYY-MM-DD`.  You can also specify offsets from the current date, such as: `+30days`, which will set the end date to 30 days from the current date. |  |
**include_subaccounts** | Option<**bool**> | Whether to include usage from the master account and all its subaccounts. Can be: `true` (the default) to include usage from the master account and all subaccounts or `false` to retrieve usage from only the specified account. |  |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListUsageRecordLastMonthResponse**](ListUsageRecordLastMonthResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_usage_record_monthly

> crate::models::ListUsageRecordMonthlyResponse list_usage_record_monthly(account_sid, category, start_date, end_date, include_subaccounts, page_size)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the UsageRecord resources to read. | [required] |
**category** | Option<**UsageRecordMonthlyEnumCategory**> | The [usage category](https://www.twilio.com/docs/usage/api/usage-record#usage-categories) of the UsageRecord resources to read. Only UsageRecord resources in the specified category are retrieved. |  |
**start_date** | Option<**String**> | Only include usage that has occurred on or after this date. Specify the date in GMT and format as `YYYY-MM-DD`. You can also specify offsets from the current date, such as: `-30days`, which will set the start date to be 30 days before the current date. |  |
**end_date** | Option<**String**> | Only include usage that occurred on or before this date. Specify the date in GMT and format as `YYYY-MM-DD`.  You can also specify offsets from the current date, such as: `+30days`, which will set the end date to 30 days from the current date. |  |
**include_subaccounts** | Option<**bool**> | Whether to include usage from the master account and all its subaccounts. Can be: `true` (the default) to include usage from the master account and all subaccounts or `false` to retrieve usage from only the specified account. |  |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListUsageRecordMonthlyResponse**](ListUsageRecordMonthlyResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_usage_record_this_month

> crate::models::ListUsageRecordThisMonthResponse list_usage_record_this_month(account_sid, category, start_date, end_date, include_subaccounts, page_size)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the UsageRecord resources to read. | [required] |
**category** | Option<**UsageRecordThisMonthEnumCategory**> | The [usage category](https://www.twilio.com/docs/usage/api/usage-record#usage-categories) of the UsageRecord resources to read. Only UsageRecord resources in the specified category are retrieved. |  |
**start_date** | Option<**String**> | Only include usage that has occurred on or after this date. Specify the date in GMT and format as `YYYY-MM-DD`. You can also specify offsets from the current date, such as: `-30days`, which will set the start date to be 30 days before the current date. |  |
**end_date** | Option<**String**> | Only include usage that occurred on or before this date. Specify the date in GMT and format as `YYYY-MM-DD`.  You can also specify offsets from the current date, such as: `+30days`, which will set the end date to 30 days from the current date. |  |
**include_subaccounts** | Option<**bool**> | Whether to include usage from the master account and all its subaccounts. Can be: `true` (the default) to include usage from the master account and all subaccounts or `false` to retrieve usage from only the specified account. |  |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListUsageRecordThisMonthResponse**](ListUsageRecordThisMonthResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_usage_record_today

> crate::models::ListUsageRecordTodayResponse list_usage_record_today(account_sid, category, start_date, end_date, include_subaccounts, page_size)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the UsageRecord resources to read. | [required] |
**category** | Option<**UsageRecordTodayEnumCategory**> | The [usage category](https://www.twilio.com/docs/usage/api/usage-record#usage-categories) of the UsageRecord resources to read. Only UsageRecord resources in the specified category are retrieved. |  |
**start_date** | Option<**String**> | Only include usage that has occurred on or after this date. Specify the date in GMT and format as `YYYY-MM-DD`. You can also specify offsets from the current date, such as: `-30days`, which will set the start date to be 30 days before the current date. |  |
**end_date** | Option<**String**> | Only include usage that occurred on or before this date. Specify the date in GMT and format as `YYYY-MM-DD`.  You can also specify offsets from the current date, such as: `+30days`, which will set the end date to 30 days from the current date. |  |
**include_subaccounts** | Option<**bool**> | Whether to include usage from the master account and all its subaccounts. Can be: `true` (the default) to include usage from the master account and all subaccounts or `false` to retrieve usage from only the specified account. |  |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListUsageRecordTodayResponse**](ListUsageRecordTodayResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_usage_record_yearly

> crate::models::ListUsageRecordYearlyResponse list_usage_record_yearly(account_sid, category, start_date, end_date, include_subaccounts, page_size)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the UsageRecord resources to read. | [required] |
**category** | Option<**UsageRecordYearlyEnumCategory**> | The [usage category](https://www.twilio.com/docs/usage/api/usage-record#usage-categories) of the UsageRecord resources to read. Only UsageRecord resources in the specified category are retrieved. |  |
**start_date** | Option<**String**> | Only include usage that has occurred on or after this date. Specify the date in GMT and format as `YYYY-MM-DD`. You can also specify offsets from the current date, such as: `-30days`, which will set the start date to be 30 days before the current date. |  |
**end_date** | Option<**String**> | Only include usage that occurred on or before this date. Specify the date in GMT and format as `YYYY-MM-DD`.  You can also specify offsets from the current date, such as: `+30days`, which will set the end date to 30 days from the current date. |  |
**include_subaccounts** | Option<**bool**> | Whether to include usage from the master account and all its subaccounts. Can be: `true` (the default) to include usage from the master account and all subaccounts or `false` to retrieve usage from only the specified account. |  |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListUsageRecordYearlyResponse**](ListUsageRecordYearlyResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_usage_record_yesterday

> crate::models::ListUsageRecordYesterdayResponse list_usage_record_yesterday(account_sid, category, start_date, end_date, include_subaccounts, page_size)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the UsageRecord resources to read. | [required] |
**category** | Option<**UsageRecordYesterdayEnumCategory**> | The [usage category](https://www.twilio.com/docs/usage/api/usage-record#usage-categories) of the UsageRecord resources to read. Only UsageRecord resources in the specified category are retrieved. |  |
**start_date** | Option<**String**> | Only include usage that has occurred on or after this date. Specify the date in GMT and format as `YYYY-MM-DD`. You can also specify offsets from the current date, such as: `-30days`, which will set the start date to be 30 days before the current date. |  |
**end_date** | Option<**String**> | Only include usage that occurred on or before this date. Specify the date in GMT and format as `YYYY-MM-DD`.  You can also specify offsets from the current date, such as: `+30days`, which will set the end date to 30 days from the current date. |  |
**include_subaccounts** | Option<**bool**> | Whether to include usage from the master account and all its subaccounts. Can be: `true` (the default) to include usage from the master account and all subaccounts or `false` to retrieve usage from only the specified account. |  |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListUsageRecordYesterdayResponse**](ListUsageRecordYesterdayResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_usage_trigger

> crate::models::ListUsageTriggerResponse list_usage_trigger(account_sid, recurring, trigger_by, usage_category, page_size)


Retrieve a list of usage-triggers belonging to the account used to make the request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the UsageTrigger resources to read. | [required] |
**recurring** | Option<**UsageTriggerEnumRecurring**> | The frequency of recurring UsageTriggers to read. Can be: `daily`, `monthly`, or `yearly` to read recurring UsageTriggers. An empty value or a value of `alltime` reads non-recurring UsageTriggers. |  |
**trigger_by** | Option<**UsageTriggerEnumTriggerField**> | The trigger field of the UsageTriggers to read.  Can be: `count`, `usage`, or `price` as described in the [UsageRecords documentation](https://www.twilio.com/docs/usage/api/usage-record#usage-count-price). |  |
**usage_category** | Option<**UsageTriggerEnumUsageCategory**> | The usage category of the UsageTriggers to read. Must be a supported [usage categories](https://www.twilio.com/docs/usage/api/usage-record#usage-categories). |  |
**page_size** | Option<**i32**> | How many resources to return in each list page. The default is 50, and the maximum is 1000. |  |

### Return type

[**crate::models::ListUsageTriggerResponse**](ListUsageTriggerResponse.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_account

> crate::models::ApiPeriodV2010PeriodAccount update_account(sid, friendly_name, status)


Modify the properties of a given Account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sid** | **String** | The Account Sid that uniquely identifies the account to update | [required] |
**friendly_name** | Option<**String**> | Update the human-readable description of this Account |  |
**status** | Option<**crate::models::AccountEnumStatus**> |  |  |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccount**](api.v2010.account.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_address

> crate::models::ApiPeriodV2010PeriodAccountPeriodAddress update_address(account_sid, sid, friendly_name, customer_name, street, city, region, postal_code, emergency_enabled, auto_correct_address)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that is responsible for the Address resource to update. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the Address resource to update. | [required] |
**friendly_name** | Option<**String**> | A descriptive string that you create to describe the address. It can be up to 64 characters long. |  |
**customer_name** | Option<**String**> | The name to associate with the address. |  |
**street** | Option<**String**> | The number and street address of the address. |  |
**city** | Option<**String**> | The city of the address. |  |
**region** | Option<**String**> | The state or region of the address. |  |
**postal_code** | Option<**String**> | The postal code of the address. |  |
**emergency_enabled** | Option<**bool**> | Whether to enable emergency calling on the address. Can be: `true` or `false`. |  |
**auto_correct_address** | Option<**bool**> | Whether we should automatically correct the address. Can be: `true` or `false` and the default is `true`. If empty or `true`, we will correct the address you provide if necessary. If `false`, we won't alter the address you provide. |  |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodAddress**](api.v2010.account.address.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_application

> crate::models::ApiPeriodV2010PeriodAccountPeriodApplication update_application(account_sid, sid, friendly_name, api_version, voice_url, voice_method, voice_fallback_url, voice_fallback_method, status_callback, status_callback_method, voice_caller_id_lookup, sms_url, sms_method, sms_fallback_url, sms_fallback_method, sms_status_callback, message_status_callback)


Updates the application's properties

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Application resources to update. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the Application resource to update. | [required] |
**friendly_name** | Option<**String**> | A descriptive string that you create to describe the resource. It can be up to 64 characters long. |  |
**api_version** | Option<**String**> | The API version to use to start a new TwiML session. Can be: `2010-04-01` or `2008-08-01`. The default value is your account's default API version. |  |
**voice_url** | Option<**String**> | The URL we should call when the phone number assigned to this application receives a call. |  |
**voice_method** | Option<**String**> | The HTTP method we should use to call `voice_url`. Can be: `GET` or `POST`. |  |
**voice_fallback_url** | Option<**String**> | The URL that we should call when an error occurs retrieving or executing the TwiML requested by `url`. |  |
**voice_fallback_method** | Option<**String**> | The HTTP method we should use to call `voice_fallback_url`. Can be: `GET` or `POST`. |  |
**status_callback** | Option<**String**> | The URL we should call using the `status_callback_method` to send status information to your application. |  |
**status_callback_method** | Option<**String**> | The HTTP method we should use to call `status_callback`. Can be: `GET` or `POST`. |  |
**voice_caller_id_lookup** | Option<**bool**> | Whether we should look up the caller's caller-ID name from the CNAM database (additional charges apply). Can be: `true` or `false`. |  |
**sms_url** | Option<**String**> | The URL we should call when the phone number receives an incoming SMS message. |  |
**sms_method** | Option<**String**> | The HTTP method we should use to call `sms_url`. Can be: `GET` or `POST`. |  |
**sms_fallback_url** | Option<**String**> | The URL that we should call when an error occurs while retrieving or executing the TwiML from `sms_url`. |  |
**sms_fallback_method** | Option<**String**> | The HTTP method we should use to call `sms_fallback_url`. Can be: `GET` or `POST`. |  |
**sms_status_callback** | Option<**String**> | Same as message_status_callback: The URL we should call using a POST method to send status information about SMS messages sent by the application. Deprecated, included for backwards compatibility. |  |
**message_status_callback** | Option<**String**> | The URL we should call using a POST method to send message status information to your application. |  |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodApplication**](api.v2010.account.application.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_call

> crate::models::ApiPeriodV2010PeriodAccountPeriodCall update_call(account_sid, sid, url, method, status, fallback_url, fallback_method, status_callback, status_callback_method, twiml, time_limit)


Initiates a call redirect or terminates a call

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Call resource(s) to update. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the Call resource to update | [required] |
**url** | Option<**String**> | The absolute URL that returns the TwiML instructions for the call. We will call this URL using the `method` when the call connects. For more information, see the [Url Parameter](https://www.twilio.com/docs/voice/make-calls#specify-a-url-parameter) section in [Making Calls](https://www.twilio.com/docs/voice/make-calls). |  |
**method** | Option<**String**> | The HTTP method we should use when calling the `url`. Can be: `GET` or `POST` and the default is `POST`. If an `application_sid` parameter is present, this parameter is ignored. |  |
**status** | Option<**crate::models::CallEnumUpdateStatus**> |  |  |
**fallback_url** | Option<**String**> | The URL that we call using the `fallback_method` if an error occurs when requesting or executing the TwiML at `url`. If an `application_sid` parameter is present, this parameter is ignored. |  |
**fallback_method** | Option<**String**> | The HTTP method that we should use to request the `fallback_url`. Can be: `GET` or `POST` and the default is `POST`. If an `application_sid` parameter is present, this parameter is ignored. |  |
**status_callback** | Option<**String**> | The URL we should call using the `status_callback_method` to send status information to your application. If no `status_callback_event` is specified, we will send the `completed` status. If an `application_sid` parameter is present, this parameter is ignored. URLs must contain a valid hostname (underscores are not permitted). |  |
**status_callback_method** | Option<**String**> | The HTTP method we should use when requesting the `status_callback` URL. Can be: `GET` or `POST` and the default is `POST`. If an `application_sid` parameter is present, this parameter is ignored. |  |
**twiml** | Option<**String**> | TwiML instructions for the call Twilio will use without fetching Twiml from url. Twiml and url parameters are mutually exclusive |  |
**time_limit** | Option<**i32**> | The maximum duration of the call in seconds. Constraints depend on account and configuration. |  |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodCall**](api.v2010.account.call.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_call_feedback

> crate::models::ApiPeriodV2010PeriodAccountPeriodCallPeriodCallFeedback update_call_feedback(account_sid, call_sid, quality_score, issue)


Update a Feedback resource for a call

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The unique id of the [Account](https://www.twilio.com/docs/iam/api/account) responsible for this resource. | [required] |
**call_sid** | **String** | The call sid that uniquely identifies the call | [required] |
**quality_score** | Option<**i32**> | The call quality expressed as an integer from `1` to `5` where `1` represents very poor call quality and `5` represents a perfect call. |  |
**issue** | Option<[**Vec<crate::models::CallFeedbackEnumIssues>**](crate::models::CallFeedbackEnumIssues.md)> | One or more issues experienced during the call. The issues can be: `imperfect-audio`, `dropped-call`, `incorrect-caller-id`, `post-dial-delay`, `digits-not-captured`, `audio-latency`, `unsolicited-call`, or `one-way-audio`. |  |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodCallPeriodCallFeedback**](api.v2010.account.call.call_feedback.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_call_recording

> crate::models::ApiPeriodV2010PeriodAccountPeriodCallPeriodCallRecording update_call_recording(account_sid, call_sid, sid, status, pause_behavior)


Changes the status of the recording to paused, stopped, or in-progress. Note: Pass `Twilio.CURRENT` instead of recording sid to reference current active recording.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Recording resource to update. | [required] |
**call_sid** | **String** | The [Call](https://www.twilio.com/docs/voice/api/call-resource) SID of the resource to update. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the Recording resource to update. | [required] |
**status** | **crate::models::CallRecordingEnumStatus** |  | [required] |
**pause_behavior** | Option<**String**> | Whether to record during a pause. Can be: `skip` or `silence` and the default is `silence`. `skip` does not record during the pause period, while `silence` will replace the actual audio of the call with silence during the pause period. This parameter only applies when setting `status` is set to `paused`. |  |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodCallPeriodCallRecording**](api.v2010.account.call.call_recording.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_conference

> crate::models::ApiPeriodV2010PeriodAccountPeriodConference update_conference(account_sid, sid, status, announce_url, announce_method)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Conference resource(s) to update. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the Conference resource to update | [required] |
**status** | Option<**crate::models::ConferenceEnumUpdateStatus**> |  |  |
**announce_url** | Option<**String**> | The URL we should call to announce something into the conference. The URL may return an MP3 file, a WAV file, or a TwiML document that contains `<Play>`, `<Say>`, `<Pause>`, or `<Redirect>` verbs. |  |
**announce_method** | Option<**String**> | The HTTP method used to call `announce_url`. Can be: `GET` or `POST` and the default is `POST` |  |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodConference**](api.v2010.account.conference.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_conference_recording

> crate::models::ApiPeriodV2010PeriodAccountPeriodConferencePeriodConferenceRecording update_conference_recording(account_sid, conference_sid, sid, status, pause_behavior)


Changes the status of the recording to paused, stopped, or in-progress. Note: To use `Twilio.CURRENT`, pass it as recording sid.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Conference Recording resource to update. | [required] |
**conference_sid** | **String** | The Conference SID that identifies the conference associated with the recording to update. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the Conference Recording resource to update. Use `Twilio.CURRENT` to reference the current active recording. | [required] |
**status** | **crate::models::ConferenceRecordingEnumStatus** |  | [required] |
**pause_behavior** | Option<**String**> | Whether to record during a pause. Can be: `skip` or `silence` and the default is `silence`. `skip` does not record during the pause period, while `silence` will replace the actual audio of the call with silence during the pause period. This parameter only applies when setting `status` is set to `paused`. |  |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodConferencePeriodConferenceRecording**](api.v2010.account.conference.conference_recording.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_connect_app

> crate::models::ApiPeriodV2010PeriodAccountPeriodConnectApp update_connect_app(account_sid, sid, authorize_redirect_url, company_name, deauthorize_callback_method, deauthorize_callback_url, description, friendly_name, homepage_url, permissions)


Update a connect-app with the specified parameters

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the ConnectApp resources to update. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the ConnectApp resource to update. | [required] |
**authorize_redirect_url** | Option<**String**> | The URL to redirect the user to after we authenticate the user and obtain authorization to access the Connect App. |  |
**company_name** | Option<**String**> | The company name to set for the Connect App. |  |
**deauthorize_callback_method** | Option<**String**> | The HTTP method to use when calling `deauthorize_callback_url`. |  |
**deauthorize_callback_url** | Option<**String**> | The URL to call using the `deauthorize_callback_method` to de-authorize the Connect App. |  |
**description** | Option<**String**> | A description of the Connect App. |  |
**friendly_name** | Option<**String**> | A descriptive string that you create to describe the resource. It can be up to 64 characters long. |  |
**homepage_url** | Option<**String**> | A public URL where users can obtain more information about this Connect App. |  |
**permissions** | Option<[**Vec<crate::models::ConnectAppEnumPermission>**](crate::models::ConnectAppEnumPermission.md)> | A comma-separated list of the permissions you will request from the users of this ConnectApp.  Can include: `get-all` and `post-all`. |  |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodConnectApp**](api.v2010.account.connect_app.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_incoming_phone_number

> crate::models::ApiPeriodV2010PeriodAccountPeriodIncomingPhoneNumber update_incoming_phone_number(account_sid, sid, account_sid2, api_version, friendly_name, sms_application_sid, sms_fallback_method, sms_fallback_url, sms_method, sms_url, status_callback, status_callback_method, voice_application_sid, voice_caller_id_lookup, voice_fallback_method, voice_fallback_url, voice_method, voice_url, emergency_status, emergency_address_sid, trunk_sid, voice_receive_mode, identity_sid, address_sid, bundle_sid)


Update an incoming-phone-number instance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the IncomingPhoneNumber resource to update.  For more information, see [Exchanging Numbers Between Subaccounts](https://www.twilio.com/docs/iam/api/subaccounts#exchanging-numbers). | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the IncomingPhoneNumber resource to update. | [required] |
**account_sid2** | Option<**String**> | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the IncomingPhoneNumber resource to update.  For more information, see [Exchanging Numbers Between Subaccounts](https://www.twilio.com/docs/iam/api/subaccounts#exchanging-numbers). |  |
**api_version** | Option<**String**> | The API version to use for incoming calls made to the phone number. The default is `2010-04-01`. |  |
**friendly_name** | Option<**String**> | A descriptive string that you created to describe this phone number. It can be up to 64 characters long. By default, this is a formatted version of the phone number. |  |
**sms_application_sid** | Option<**String**> | The SID of the application that should handle SMS messages sent to the number. If an `sms_application_sid` is present, we ignore all of the `sms_*_url` urls and use those set on the application. |  |
**sms_fallback_method** | Option<**String**> | The HTTP method that we should use to call `sms_fallback_url`. Can be: `GET` or `POST` and defaults to `POST`. |  |
**sms_fallback_url** | Option<**String**> | The URL that we should call when an error occurs while requesting or executing the TwiML defined by `sms_url`. |  |
**sms_method** | Option<**String**> | The HTTP method that we should use to call `sms_url`. Can be: `GET` or `POST` and defaults to `POST`. |  |
**sms_url** | Option<**String**> | The URL we should call when the phone number receives an incoming SMS message. |  |
**status_callback** | Option<**String**> | The URL we should call using the `status_callback_method` to send status information to your application. |  |
**status_callback_method** | Option<**String**> | The HTTP method we should use to call `status_callback`. Can be: `GET` or `POST` and defaults to `POST`. |  |
**voice_application_sid** | Option<**String**> | The SID of the application we should use to handle phone calls to the phone number. If a `voice_application_sid` is present, we ignore all of the voice urls and use only those set on the application. Setting a `voice_application_sid` will automatically delete your `trunk_sid` and vice versa. |  |
**voice_caller_id_lookup** | Option<**bool**> | Whether to lookup the caller's name from the CNAM database and post it to your app. Can be: `true` or `false` and defaults to `false`. |  |
**voice_fallback_method** | Option<**String**> | The HTTP method that we should use to call `voice_fallback_url`. Can be: `GET` or `POST` and defaults to `POST`. |  |
**voice_fallback_url** | Option<**String**> | The URL that we should call when an error occurs retrieving or executing the TwiML requested by `url`. |  |
**voice_method** | Option<**String**> | The HTTP method that we should use to call `voice_url`. Can be: `GET` or `POST` and defaults to `POST`. |  |
**voice_url** | Option<**String**> | The URL that we should call to answer a call to the phone number. The `voice_url` will not be called if a `voice_application_sid` or a `trunk_sid` is set. |  |
**emergency_status** | Option<**crate::models::IncomingPhoneNumberEnumEmergencyStatus**> |  |  |
**emergency_address_sid** | Option<**String**> | The SID of the emergency address configuration to use for emergency calling from this phone number. |  |
**trunk_sid** | Option<**String**> | The SID of the Trunk we should use to handle phone calls to the phone number. If a `trunk_sid` is present, we ignore all of the voice urls and voice applications and use only those set on the Trunk. Setting a `trunk_sid` will automatically delete your `voice_application_sid` and vice versa. |  |
**voice_receive_mode** | Option<**crate::models::IncomingPhoneNumberEnumVoiceReceiveMode**> |  |  |
**identity_sid** | Option<**String**> | The SID of the Identity resource that we should associate with the phone number. Some regions require an identity to meet local regulations. |  |
**address_sid** | Option<**String**> | The SID of the Address resource we should associate with the phone number. Some regions require addresses to meet local regulations. |  |
**bundle_sid** | Option<**String**> | The SID of the Bundle resource that you associate with the phone number. Some regions require a Bundle to meet local Regulations. |  |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodIncomingPhoneNumber**](api.v2010.account.incoming_phone_number.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_key

> crate::models::ApiPeriodV2010PeriodAccountPeriodKey update_key(account_sid, sid, friendly_name)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Key resources to update. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the Key resource to update. | [required] |
**friendly_name** | Option<**String**> | A descriptive string that you create to describe the resource. It can be up to 64 characters long. |  |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodKey**](api.v2010.account.key.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_member

> crate::models::ApiPeriodV2010PeriodAccountPeriodQueuePeriodMember update_member(account_sid, queue_sid, call_sid, url, method)


Dequeue a member from a queue and have the member's call begin executing the TwiML document at that URL

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Member resource(s) to update. | [required] |
**queue_sid** | **String** | The SID of the Queue in which to find the members to update. | [required] |
**call_sid** | **String** | The [Call](https://www.twilio.com/docs/voice/api/call-resource) SID of the resource(s) to update. | [required] |
**url** | **String** | The absolute URL of the Queue resource. | [required] |
**method** | Option<**String**> | How to pass the update request data. Can be `GET` or `POST` and the default is `POST`. `POST` sends the data as encoded form data and `GET` sends the data as query parameters. |  |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodQueuePeriodMember**](api.v2010.account.queue.member.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_message

> crate::models::ApiPeriodV2010PeriodAccountPeriodMessage update_message(account_sid, sid, body, status)


To redact a message-body from a post-flight message record, post to the message instance resource with an empty body

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Message resources to update. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the Message resource to update. | [required] |
**body** | Option<**String**> | The text of the message you want to send. Can be up to 1,600 characters long. |  |
**status** | Option<**crate::models::MessageEnumUpdateStatus**> |  |  |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodMessage**](api.v2010.account.message.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_outgoing_caller_id

> crate::models::ApiPeriodV2010PeriodAccountPeriodOutgoingCallerId update_outgoing_caller_id(account_sid, sid, friendly_name)


Updates the caller-id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the OutgoingCallerId resources to update. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the OutgoingCallerId resource to update. | [required] |
**friendly_name** | Option<**String**> | A descriptive string that you create to describe the resource. It can be up to 64 characters long. |  |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodOutgoingCallerId**](api.v2010.account.outgoing_caller_id.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_participant

> crate::models::ApiPeriodV2010PeriodAccountPeriodConferencePeriodParticipant update_participant(account_sid, conference_sid, call_sid, muted, hold, hold_url, hold_method, announce_url, announce_method, wait_url, wait_method, beep_on_exit, end_conference_on_exit, coaching, call_sid_to_coach)


Update the properties of the participant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Participant resources to update. | [required] |
**conference_sid** | **String** | The SID of the conference with the participant to update. | [required] |
**call_sid** | **String** | The [Call](https://www.twilio.com/docs/voice/api/call-resource) SID or label of the participant to update. Non URL safe characters in a label must be percent encoded, for example, a space character is represented as %20. | [required] |
**muted** | Option<**bool**> | Whether the participant should be muted. Can be `true` or `false`. `true` will mute the participant, and `false` will un-mute them. Anything value other than `true` or `false` is interpreted as `false`. |  |
**hold** | Option<**bool**> | Whether the participant should be on hold. Can be: `true` or `false`. `true` puts the participant on hold, and `false` lets them rejoin the conference. |  |
**hold_url** | Option<**String**> | The URL we call using the `hold_method` for music that plays when the participant is on hold. The URL may return an MP3 file, a WAV file, or a TwiML document that contains `<Play>`, `<Say>`, `<Pause>`, or `<Redirect>` verbs. |  |
**hold_method** | Option<**String**> | The HTTP method we should use to call `hold_url`. Can be: `GET` or `POST` and the default is `GET`. |  |
**announce_url** | Option<**String**> | The URL we call using the `announce_method` for an announcement to the participant. The URL may return an MP3 file, a WAV file, or a TwiML document that contains `<Play>`, `<Say>`, `<Pause>`, or `<Redirect>` verbs. |  |
**announce_method** | Option<**String**> | The HTTP method we should use to call `announce_url`. Can be: `GET` or `POST` and defaults to `POST`. |  |
**wait_url** | Option<**String**> | The URL we call using the `wait_method` for the music to play while participants are waiting for the conference to start. The URL may return an MP3 file, a WAV file, or a TwiML document that contains `<Play>`, `<Say>`, `<Pause>`, or `<Redirect>` verbs. The default value is the URL of our standard hold music. [Learn more about hold music](https://www.twilio.com/labs/twimlets/holdmusic). |  |
**wait_method** | Option<**String**> | The HTTP method we should use to call `wait_url`. Can be `GET` or `POST` and the default is `POST`. When using a static audio file, this should be `GET` so that we can cache the file. |  |
**beep_on_exit** | Option<**bool**> | Whether to play a notification beep to the conference when the participant exits. Can be: `true` or `false`. |  |
**end_conference_on_exit** | Option<**bool**> | Whether to end the conference when the participant leaves. Can be: `true` or `false` and defaults to `false`. |  |
**coaching** | Option<**bool**> | Whether the participant is coaching another call. Can be: `true` or `false`. If not present, defaults to `false` unless `call_sid_to_coach` is defined. If `true`, `call_sid_to_coach` must be defined. |  |
**call_sid_to_coach** | Option<**String**> | The SID of the participant who is being `coached`. The participant being coached is the only participant who can hear the participant who is `coaching`. |  |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodConferencePeriodParticipant**](api.v2010.account.conference.participant.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_payments

> crate::models::ApiPeriodV2010PeriodAccountPeriodCallPeriodPayments update_payments(account_sid, call_sid, sid, idempotency_key, status_callback, capture, status)


update an instance of payments with different phases of payment flows.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that will update the resource. | [required] |
**call_sid** | **String** | The SID of the call that will update the resource. This should be the same call sid that was used to create payments resource. | [required] |
**sid** | **String** | The SID of Payments session that needs to be updated. | [required] |
**idempotency_key** | **String** | A unique token that will be used to ensure that multiple API calls with the same information do not result in multiple transactions. This should be a unique string value per API call and can be a randomly generated. | [required] |
**status_callback** | **String** | Provide an absolute or relative URL to receive status updates regarding your Pay session. Read more about the [Update](https://www.twilio.com/docs/voice/api/payment-resource#statuscallback-update) and [Complete/Cancel](https://www.twilio.com/docs/voice/api/payment-resource#statuscallback-cancelcomplete) POST requests. | [required] |
**capture** | Option<**crate::models::PaymentsEnumCapture**> |  |  |
**status** | Option<**crate::models::PaymentsEnumStatus**> |  |  |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodCallPeriodPayments**](api.v2010.account.call.payments.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_queue

> crate::models::ApiPeriodV2010PeriodAccountPeriodQueue update_queue(account_sid, sid, friendly_name, max_size)


Update the queue with the new parameters

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Queue resource to update. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the Queue resource to update | [required] |
**friendly_name** | Option<**String**> | A descriptive string that you created to describe this resource. It can be up to 64 characters long. |  |
**max_size** | Option<**i32**> | The maximum number of calls allowed to be in the queue. The default is 100. The maximum is 5000. |  |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodQueue**](api.v2010.account.queue.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_short_code

> crate::models::ApiPeriodV2010PeriodAccountPeriodShortCode update_short_code(account_sid, sid, friendly_name, api_version, sms_url, sms_method, sms_fallback_url, sms_fallback_method)


Update a short code with the following parameters

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the ShortCode resource(s) to update. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the ShortCode resource to update | [required] |
**friendly_name** | Option<**String**> | A descriptive string that you created to describe this resource. It can be up to 64 characters long. By default, the `FriendlyName` is the short code. |  |
**api_version** | Option<**String**> | The API version to use to start a new TwiML session. Can be: `2010-04-01` or `2008-08-01`. |  |
**sms_url** | Option<**String**> | The URL we should call when receiving an incoming SMS message to this short code. |  |
**sms_method** | Option<**String**> | The HTTP method we should use when calling the `sms_url`. Can be: `GET` or `POST`. |  |
**sms_fallback_url** | Option<**String**> | The URL that we should call if an error occurs while retrieving or executing the TwiML from `sms_url`. |  |
**sms_fallback_method** | Option<**String**> | The HTTP method that we should use to call the `sms_fallback_url`. Can be: `GET` or `POST`. |  |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodShortCode**](api.v2010.account.short_code.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_signing_key

> crate::models::ApiPeriodV2010PeriodAccountPeriodSigningKey update_signing_key(account_sid, sid, friendly_name)




### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** |  | [required] |
**sid** | **String** |  | [required] |
**friendly_name** | Option<**String**> |  |  |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodSigningKey**](api.v2010.account.signing_key.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_sip_credential

> crate::models::ApiPeriodV2010PeriodAccountPeriodSipPeriodSipCredentialListPeriodSipCredential update_sip_credential(account_sid, credential_list_sid, sid, password)


Update a credential resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The unique id of the Account that is responsible for this resource. | [required] |
**credential_list_sid** | **String** | The unique id that identifies the credential list that includes this credential. | [required] |
**sid** | **String** | The unique id that identifies the resource to update. | [required] |
**password** | Option<**String**> | The password that the username will use when authenticating SIP requests. The password must be a minimum of 12 characters, contain at least 1 digit, and have mixed case. (eg `IWasAtSignal2018`) |  |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodSipPeriodSipCredentialListPeriodSipCredential**](api.v2010.account.sip.sip_credential_list.sip_credential.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_sip_credential_list

> crate::models::ApiPeriodV2010PeriodAccountPeriodSipPeriodSipCredentialList update_sip_credential_list(account_sid, sid, friendly_name)


Update a Credential List

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The unique id of the Account that is responsible for this resource. | [required] |
**sid** | **String** | The credential list Sid that uniquely identifies this resource | [required] |
**friendly_name** | **String** | A human readable descriptive text for a CredentialList, up to 64 characters long. | [required] |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodSipPeriodSipCredentialList**](api.v2010.account.sip.sip_credential_list.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_sip_domain

> crate::models::ApiPeriodV2010PeriodAccountPeriodSipPeriodSipDomain update_sip_domain(account_sid, sid, friendly_name, voice_fallback_method, voice_fallback_url, voice_method, voice_status_callback_method, voice_status_callback_url, voice_url, sip_registration, domain_name, emergency_calling_enabled, secure, byoc_trunk_sid, emergency_caller_sid)


Update the attributes of a domain

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the SipDomain resource to update. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the SipDomain resource to update. | [required] |
**friendly_name** | Option<**String**> | A descriptive string that you created to describe the resource. It can be up to 64 characters long. |  |
**voice_fallback_method** | Option<**String**> | The HTTP method we should use to call `voice_fallback_url`. Can be: `GET` or `POST`. |  |
**voice_fallback_url** | Option<**String**> | The URL that we should call when an error occurs while retrieving or executing the TwiML requested by `voice_url`. |  |
**voice_method** | Option<**String**> | The HTTP method we should use to call `voice_url` |  |
**voice_status_callback_method** | Option<**String**> | The HTTP method we should use to call `voice_status_callback_url`. Can be: `GET` or `POST`. |  |
**voice_status_callback_url** | Option<**String**> | The URL that we should call to pass status parameters (such as call ended) to your application. |  |
**voice_url** | Option<**String**> | The URL we should call when the domain receives a call. |  |
**sip_registration** | Option<**bool**> | Whether to allow SIP Endpoints to register with the domain to receive calls. Can be `true` or `false`. `true` allows SIP Endpoints to register with the domain to receive calls, `false` does not. |  |
**domain_name** | Option<**String**> | The unique address you reserve on Twilio to which you route your SIP traffic. Domain names can contain letters, digits, and \\\"-\\\" and must end with `sip.twilio.com`. |  |
**emergency_calling_enabled** | Option<**bool**> | Whether emergency calling is enabled for the domain. If enabled, allows emergency calls on the domain from phone numbers with validated addresses. |  |
**secure** | Option<**bool**> | Whether secure SIP is enabled for the domain. If enabled, TLS will be enforced and SRTP will be negotiated on all incoming calls to this sip domain. |  |
**byoc_trunk_sid** | Option<**String**> | The SID of the BYOC Trunk(Bring Your Own Carrier) resource that the Sip Domain will be associated with. |  |
**emergency_caller_sid** | Option<**String**> | Whether an emergency caller sid is configured for the domain. If present, this phone number will be used as the callback for the emergency call. |  |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodSipPeriodSipDomain**](api.v2010.account.sip.sip_domain.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_sip_ip_access_control_list

> crate::models::ApiPeriodV2010PeriodAccountPeriodSipPeriodSipIpAccessControlList update_sip_ip_access_control_list(account_sid, sid, friendly_name)


Rename an IpAccessControlList

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The unique id of the [Account](https://www.twilio.com/docs/iam/api/account) responsible for this resource. | [required] |
**sid** | **String** | A 34 character string that uniquely identifies the resource to udpate. | [required] |
**friendly_name** | **String** | A human readable descriptive text, up to 255 characters long. | [required] |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodSipPeriodSipIpAccessControlList**](api.v2010.account.sip.sip_ip_access_control_list.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_sip_ip_address

> crate::models::ApiPeriodV2010PeriodAccountPeriodSipPeriodSipIpAccessControlListPeriodSipIpAddress update_sip_ip_address(account_sid, ip_access_control_list_sid, sid, ip_address, friendly_name, cidr_prefix_length)


Update an IpAddress resource.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The unique id of the [Account](https://www.twilio.com/docs/iam/api/account) responsible for this resource. | [required] |
**ip_access_control_list_sid** | **String** | The IpAccessControlList Sid that identifies the IpAddress resources to update. | [required] |
**sid** | **String** | A 34 character string that identifies the IpAddress resource to update. | [required] |
**ip_address** | Option<**String**> | An IP address in dotted decimal notation from which you want to accept traffic. Any SIP requests from this IP address will be allowed by Twilio. IPv4 only supported today. |  |
**friendly_name** | Option<**String**> | A human readable descriptive text for this resource, up to 255 characters long. |  |
**cidr_prefix_length** | Option<**i32**> | An integer representing the length of the CIDR prefix to use with this IP address when accepting traffic. By default the entire IP address is used. |  |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodSipPeriodSipIpAccessControlListPeriodSipIpAddress**](api.v2010.account.sip.sip_ip_access_control_list.sip_ip_address.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_siprec

> crate::models::ApiPeriodV2010PeriodAccountPeriodCallPeriodSiprec update_siprec(account_sid, call_sid, sid, status)


Stop a Siprec using either the SID of the Siprec resource or the `name` used when creating the resource

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created this Siprec resource. | [required] |
**call_sid** | **String** | The SID of the [Call](https://www.twilio.com/docs/voice/api/call-resource) the Siprec resource is associated with. | [required] |
**sid** | **String** | The SID of the Siprec resource, or the `name` used when creating the resource | [required] |
**status** | **crate::models::SiprecEnumUpdateStatus** |  | [required] |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodCallPeriodSiprec**](api.v2010.account.call.siprec.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_stream

> crate::models::ApiPeriodV2010PeriodAccountPeriodCallPeriodStream update_stream(account_sid, call_sid, sid, status)


Stop a Stream using either the SID of the Stream resource or the `name` used when creating the resource

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created this Stream resource. | [required] |
**call_sid** | **String** | The SID of the [Call](https://www.twilio.com/docs/voice/api/call-resource) the Stream resource is associated with. | [required] |
**sid** | **String** | The SID of the Stream resource, or the `name` used when creating the resource | [required] |
**status** | **crate::models::StreamEnumUpdateStatus** |  | [required] |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodCallPeriodStream**](api.v2010.account.call.stream.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_usage_trigger

> crate::models::ApiPeriodV2010PeriodAccountPeriodUsagePeriodUsageTrigger update_usage_trigger(account_sid, sid, callback_method, callback_url, friendly_name)


Update an instance of a usage trigger

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_sid** | **String** | The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the UsageTrigger resources to update. | [required] |
**sid** | **String** | The Twilio-provided string that uniquely identifies the UsageTrigger resource to update. | [required] |
**callback_method** | Option<**String**> | The HTTP method we should use to call `callback_url`. Can be: `GET` or `POST` and the default is `POST`. |  |
**callback_url** | Option<**String**> | The URL we should call using `callback_method` when the trigger fires. |  |
**friendly_name** | Option<**String**> | A descriptive string that you create to describe the resource. It can be up to 64 characters long. |  |

### Return type

[**crate::models::ApiPeriodV2010PeriodAccountPeriodUsagePeriodUsageTrigger**](api.v2010.account.usage.usage_trigger.md)

### Authorization

[accountSid_authToken](../README.md#accountSid_authToken)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

