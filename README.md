# twilio-rust - Rust API client for openapi

Auto-generated OpenAPI rust-based Twilio client mixed with some tasty additions like Twiml struct.

This is the public Twilio REST API.

For more information, please visit [https://support.twilio.com](https://support.twilio.com)

## Documentation for API Endpoints

All URIs are relative to *https://api.twilio.com*

| Class        | Method                                                                                                                                | HTTP request                                                                                                                         | Description |
| ------------ | ------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------ | ----------- |
| _DefaultApi_ | [**create_account**](docs/DefaultApi.md#create_account)                                                                               | **POST** /2010-04-01/Accounts.json                                                                                                   |
| _DefaultApi_ | [**create_address**](docs/DefaultApi.md#create_address)                                                                               | **POST** /2010-04-01/Accounts/{AccountSid}/Addresses.json                                                                            |
| _DefaultApi_ | [**create_application**](docs/DefaultApi.md#create_application)                                                                       | **POST** /2010-04-01/Accounts/{AccountSid}/Applications.json                                                                         |
| _DefaultApi_ | [**create_call**](docs/DefaultApi.md#create_call)                                                                                     | **POST** /2010-04-01/Accounts/{AccountSid}/Calls.json                                                                                |
| _DefaultApi_ | [**create_call_feedback_summary**](docs/DefaultApi.md#create_call_feedback_summary)                                                   | **POST** /2010-04-01/Accounts/{AccountSid}/Calls/FeedbackSummary.json                                                                |
| _DefaultApi_ | [**create_call_recording**](docs/DefaultApi.md#create_call_recording)                                                                 | **POST** /2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Recordings.json                                                           |
| _DefaultApi_ | [**create_incoming_phone_number**](docs/DefaultApi.md#create_incoming_phone_number)                                                   | **POST** /2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers.json                                                                 |
| _DefaultApi_ | [**create_incoming_phone_number_assigned_add_on**](docs/DefaultApi.md#create_incoming_phone_number_assigned_add_on)                   | **POST** /2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/{ResourceSid}/AssignedAddOns.json                                    |
| _DefaultApi_ | [**create_incoming_phone_number_local**](docs/DefaultApi.md#create_incoming_phone_number_local)                                       | **POST** /2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/Local.json                                                           |
| _DefaultApi_ | [**create_incoming_phone_number_mobile**](docs/DefaultApi.md#create_incoming_phone_number_mobile)                                     | **POST** /2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/Mobile.json                                                          |
| _DefaultApi_ | [**create_incoming_phone_number_toll_free**](docs/DefaultApi.md#create_incoming_phone_number_toll_free)                               | **POST** /2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/TollFree.json                                                        |
| _DefaultApi_ | [**create_message**](docs/DefaultApi.md#create_message)                                                                               | **POST** /2010-04-01/Accounts/{AccountSid}/Messages.json                                                                             |
| _DefaultApi_ | [**create_message_feedback**](docs/DefaultApi.md#create_message_feedback)                                                             | **POST** /2010-04-01/Accounts/{AccountSid}/Messages/{MessageSid}/Feedback.json                                                       |
| _DefaultApi_ | [**create_new_key**](docs/DefaultApi.md#create_new_key)                                                                               | **POST** /2010-04-01/Accounts/{AccountSid}/Keys.json                                                                                 |
| _DefaultApi_ | [**create_new_signing_key**](docs/DefaultApi.md#create_new_signing_key)                                                               | **POST** /2010-04-01/Accounts/{AccountSid}/SigningKeys.json                                                                          |
| _DefaultApi_ | [**create_participant**](docs/DefaultApi.md#create_participant)                                                                       | **POST** /2010-04-01/Accounts/{AccountSid}/Conferences/{ConferenceSid}/Participants.json                                             |
| _DefaultApi_ | [**create_payments**](docs/DefaultApi.md#create_payments)                                                                             | **POST** /2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Payments.json                                                             |
| _DefaultApi_ | [**create_queue**](docs/DefaultApi.md#create_queue)                                                                                   | **POST** /2010-04-01/Accounts/{AccountSid}/Queues.json                                                                               |
| _DefaultApi_ | [**create_sip_auth_calls_credential_list_mapping**](docs/DefaultApi.md#create_sip_auth_calls_credential_list_mapping)                 | **POST** /2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/Auth/Calls/CredentialListMappings.json                            |
| _DefaultApi_ | [**create_sip_auth_calls_ip_access_control_list_mapping**](docs/DefaultApi.md#create_sip_auth_calls_ip_access_control_list_mapping)   | **POST** /2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/Auth/Calls/IpAccessControlListMappings.json                       |
| _DefaultApi_ | [**create_sip_auth_registrations_credential_list_mapping**](docs/DefaultApi.md#create_sip_auth_registrations_credential_list_mapping) | **POST** /2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/Auth/Registrations/CredentialListMappings.json                    |
| _DefaultApi_ | [**create_sip_credential**](docs/DefaultApi.md#create_sip_credential)                                                                 | **POST** /2010-04-01/Accounts/{AccountSid}/SIP/CredentialLists/{CredentialListSid}/Credentials.json                                  |
| _DefaultApi_ | [**create_sip_credential_list**](docs/DefaultApi.md#create_sip_credential_list)                                                       | **POST** /2010-04-01/Accounts/{AccountSid}/SIP/CredentialLists.json                                                                  |
| _DefaultApi_ | [**create_sip_credential_list_mapping**](docs/DefaultApi.md#create_sip_credential_list_mapping)                                       | **POST** /2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/CredentialListMappings.json                                       |
| _DefaultApi_ | [**create_sip_domain**](docs/DefaultApi.md#create_sip_domain)                                                                         | **POST** /2010-04-01/Accounts/{AccountSid}/SIP/Domains.json                                                                          |
| _DefaultApi_ | [**create_sip_ip_access_control_list**](docs/DefaultApi.md#create_sip_ip_access_control_list)                                         | **POST** /2010-04-01/Accounts/{AccountSid}/SIP/IpAccessControlLists.json                                                             |
| _DefaultApi_ | [**create_sip_ip_access_control_list_mapping**](docs/DefaultApi.md#create_sip_ip_access_control_list_mapping)                         | **POST** /2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/IpAccessControlListMappings.json                                  |
| _DefaultApi_ | [**create_sip_ip_address**](docs/DefaultApi.md#create_sip_ip_address)                                                                 | **POST** /2010-04-01/Accounts/{AccountSid}/SIP/IpAccessControlLists/{IpAccessControlListSid}/IpAddresses.json                        |
| _DefaultApi_ | [**create_siprec**](docs/DefaultApi.md#create_siprec)                                                                                 | **POST** /2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Siprec.json                                                               |
| _DefaultApi_ | [**create_stream**](docs/DefaultApi.md#create_stream)                                                                                 | **POST** /2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Streams.json                                                              |
| _DefaultApi_ | [**create_token**](docs/DefaultApi.md#create_token)                                                                                   | **POST** /2010-04-01/Accounts/{AccountSid}/Tokens.json                                                                               |
| _DefaultApi_ | [**create_usage_trigger**](docs/DefaultApi.md#create_usage_trigger)                                                                   | **POST** /2010-04-01/Accounts/{AccountSid}/Usage/Triggers.json                                                                       |
| _DefaultApi_ | [**create_user_defined_message**](docs/DefaultApi.md#create_user_defined_message)                                                     | **POST** /2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/UserDefinedMessages.json                                                  |
| _DefaultApi_ | [**create_user_defined_message_subscription**](docs/DefaultApi.md#create_user_defined_message_subscription)                           | **POST** /2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/UserDefinedMessageSubscriptions.json                                      |
| _DefaultApi_ | [**create_validation_request**](docs/DefaultApi.md#create_validation_request)                                                         | **POST** /2010-04-01/Accounts/{AccountSid}/OutgoingCallerIds.json                                                                    |
| _DefaultApi_ | [**delete_address**](docs/DefaultApi.md#delete_address)                                                                               | **DELETE** /2010-04-01/Accounts/{AccountSid}/Addresses/{Sid}.json                                                                    |
| _DefaultApi_ | [**delete_application**](docs/DefaultApi.md#delete_application)                                                                       | **DELETE** /2010-04-01/Accounts/{AccountSid}/Applications/{Sid}.json                                                                 |
| _DefaultApi_ | [**delete_call**](docs/DefaultApi.md#delete_call)                                                                                     | **DELETE** /2010-04-01/Accounts/{AccountSid}/Calls/{Sid}.json                                                                        |
| _DefaultApi_ | [**delete_call_feedback_summary**](docs/DefaultApi.md#delete_call_feedback_summary)                                                   | **DELETE** /2010-04-01/Accounts/{AccountSid}/Calls/FeedbackSummary/{Sid}.json                                                        |
| _DefaultApi_ | [**delete_call_recording**](docs/DefaultApi.md#delete_call_recording)                                                                 | **DELETE** /2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Recordings/{Sid}.json                                                   |
| _DefaultApi_ | [**delete_conference_recording**](docs/DefaultApi.md#delete_conference_recording)                                                     | **DELETE** /2010-04-01/Accounts/{AccountSid}/Conferences/{ConferenceSid}/Recordings/{Sid}.json                                       |
| _DefaultApi_ | [**delete_connect_app**](docs/DefaultApi.md#delete_connect_app)                                                                       | **DELETE** /2010-04-01/Accounts/{AccountSid}/ConnectApps/{Sid}.json                                                                  |
| _DefaultApi_ | [**delete_incoming_phone_number**](docs/DefaultApi.md#delete_incoming_phone_number)                                                   | **DELETE** /2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/{Sid}.json                                                         |
| _DefaultApi_ | [**delete_incoming_phone_number_assigned_add_on**](docs/DefaultApi.md#delete_incoming_phone_number_assigned_add_on)                   | **DELETE** /2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/{ResourceSid}/AssignedAddOns/{Sid}.json                            |
| _DefaultApi_ | [**delete_key**](docs/DefaultApi.md#delete_key)                                                                                       | **DELETE** /2010-04-01/Accounts/{AccountSid}/Keys/{Sid}.json                                                                         |
| _DefaultApi_ | [**delete_media**](docs/DefaultApi.md#delete_media)                                                                                   | **DELETE** /2010-04-01/Accounts/{AccountSid}/Messages/{MessageSid}/Media/{Sid}.json                                                  |
| _DefaultApi_ | [**delete_message**](docs/DefaultApi.md#delete_message)                                                                               | **DELETE** /2010-04-01/Accounts/{AccountSid}/Messages/{Sid}.json                                                                     |
| _DefaultApi_ | [**delete_outgoing_caller_id**](docs/DefaultApi.md#delete_outgoing_caller_id)                                                         | **DELETE** /2010-04-01/Accounts/{AccountSid}/OutgoingCallerIds/{Sid}.json                                                            |
| _DefaultApi_ | [**delete_participant**](docs/DefaultApi.md#delete_participant)                                                                       | **DELETE** /2010-04-01/Accounts/{AccountSid}/Conferences/{ConferenceSid}/Participants/{CallSid}.json                                 |
| _DefaultApi_ | [**delete_queue**](docs/DefaultApi.md#delete_queue)                                                                                   | **DELETE** /2010-04-01/Accounts/{AccountSid}/Queues/{Sid}.json                                                                       |
| _DefaultApi_ | [**delete_recording**](docs/DefaultApi.md#delete_recording)                                                                           | **DELETE** /2010-04-01/Accounts/{AccountSid}/Recordings/{Sid}.json                                                                   |
| _DefaultApi_ | [**delete_recording_add_on_result**](docs/DefaultApi.md#delete_recording_add_on_result)                                               | **DELETE** /2010-04-01/Accounts/{AccountSid}/Recordings/{ReferenceSid}/AddOnResults/{Sid}.json                                       |
| _DefaultApi_ | [**delete_recording_add_on_result_payload**](docs/DefaultApi.md#delete_recording_add_on_result_payload)                               | **DELETE** /2010-04-01/Accounts/{AccountSid}/Recordings/{ReferenceSid}/AddOnResults/{AddOnResultSid}/Payloads/{Sid}.json             |
| _DefaultApi_ | [**delete_recording_transcription**](docs/DefaultApi.md#delete_recording_transcription)                                               | **DELETE** /2010-04-01/Accounts/{AccountSid}/Recordings/{RecordingSid}/Transcriptions/{Sid}.json                                     |
| _DefaultApi_ | [**delete_signing_key**](docs/DefaultApi.md#delete_signing_key)                                                                       | **DELETE** /2010-04-01/Accounts/{AccountSid}/SigningKeys/{Sid}.json                                                                  |
| _DefaultApi_ | [**delete_sip_auth_calls_credential_list_mapping**](docs/DefaultApi.md#delete_sip_auth_calls_credential_list_mapping)                 | **DELETE** /2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/Auth/Calls/CredentialListMappings/{Sid}.json                    |
| _DefaultApi_ | [**delete_sip_auth_calls_ip_access_control_list_mapping**](docs/DefaultApi.md#delete_sip_auth_calls_ip_access_control_list_mapping)   | **DELETE** /2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/Auth/Calls/IpAccessControlListMappings/{Sid}.json               |
| _DefaultApi_ | [**delete_sip_auth_registrations_credential_list_mapping**](docs/DefaultApi.md#delete_sip_auth_registrations_credential_list_mapping) | **DELETE** /2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/Auth/Registrations/CredentialListMappings/{Sid}.json            |
| _DefaultApi_ | [**delete_sip_credential**](docs/DefaultApi.md#delete_sip_credential)                                                                 | **DELETE** /2010-04-01/Accounts/{AccountSid}/SIP/CredentialLists/{CredentialListSid}/Credentials/{Sid}.json                          |
| _DefaultApi_ | [**delete_sip_credential_list**](docs/DefaultApi.md#delete_sip_credential_list)                                                       | **DELETE** /2010-04-01/Accounts/{AccountSid}/SIP/CredentialLists/{Sid}.json                                                          |
| _DefaultApi_ | [**delete_sip_credential_list_mapping**](docs/DefaultApi.md#delete_sip_credential_list_mapping)                                       | **DELETE** /2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/CredentialListMappings/{Sid}.json                               |
| _DefaultApi_ | [**delete_sip_domain**](docs/DefaultApi.md#delete_sip_domain)                                                                         | **DELETE** /2010-04-01/Accounts/{AccountSid}/SIP/Domains/{Sid}.json                                                                  |
| _DefaultApi_ | [**delete_sip_ip_access_control_list**](docs/DefaultApi.md#delete_sip_ip_access_control_list)                                         | **DELETE** /2010-04-01/Accounts/{AccountSid}/SIP/IpAccessControlLists/{Sid}.json                                                     |
| _DefaultApi_ | [**delete_sip_ip_access_control_list_mapping**](docs/DefaultApi.md#delete_sip_ip_access_control_list_mapping)                         | **DELETE** /2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/IpAccessControlListMappings/{Sid}.json                          |
| _DefaultApi_ | [**delete_sip_ip_address**](docs/DefaultApi.md#delete_sip_ip_address)                                                                 | **DELETE** /2010-04-01/Accounts/{AccountSid}/SIP/IpAccessControlLists/{IpAccessControlListSid}/IpAddresses/{Sid}.json                |
| _DefaultApi_ | [**delete_transcription**](docs/DefaultApi.md#delete_transcription)                                                                   | **DELETE** /2010-04-01/Accounts/{AccountSid}/Transcriptions/{Sid}.json                                                               |
| _DefaultApi_ | [**delete_usage_trigger**](docs/DefaultApi.md#delete_usage_trigger)                                                                   | **DELETE** /2010-04-01/Accounts/{AccountSid}/Usage/Triggers/{Sid}.json                                                               |
| _DefaultApi_ | [**delete_user_defined_message_subscription**](docs/DefaultApi.md#delete_user_defined_message_subscription)                           | **DELETE** /2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/UserDefinedMessageSubscriptions/{Sid}.json                              |
| _DefaultApi_ | [**fetch_account**](docs/DefaultApi.md#fetch_account)                                                                                 | **GET** /2010-04-01/Accounts/{Sid}.json                                                                                              |
| _DefaultApi_ | [**fetch_address**](docs/DefaultApi.md#fetch_address)                                                                                 | **GET** /2010-04-01/Accounts/{AccountSid}/Addresses/{Sid}.json                                                                       |
| _DefaultApi_ | [**fetch_application**](docs/DefaultApi.md#fetch_application)                                                                         | **GET** /2010-04-01/Accounts/{AccountSid}/Applications/{Sid}.json                                                                    |
| _DefaultApi_ | [**fetch_authorized_connect_app**](docs/DefaultApi.md#fetch_authorized_connect_app)                                                   | **GET** /2010-04-01/Accounts/{AccountSid}/AuthorizedConnectApps/{ConnectAppSid}.json                                                 |
| _DefaultApi_ | [**fetch_available_phone_number_country**](docs/DefaultApi.md#fetch_available_phone_number_country)                                   | **GET** /2010-04-01/Accounts/{AccountSid}/AvailablePhoneNumbers/{CountryCode}.json                                                   |
| _DefaultApi_ | [**fetch_balance**](docs/DefaultApi.md#fetch_balance)                                                                                 | **GET** /2010-04-01/Accounts/{AccountSid}/Balance.json                                                                               |
| _DefaultApi_ | [**fetch_call**](docs/DefaultApi.md#fetch_call)                                                                                       | **GET** /2010-04-01/Accounts/{AccountSid}/Calls/{Sid}.json                                                                           |
| _DefaultApi_ | [**fetch_call_feedback**](docs/DefaultApi.md#fetch_call_feedback)                                                                     | **GET** /2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Feedback.json                                                              |
| _DefaultApi_ | [**fetch_call_feedback_summary**](docs/DefaultApi.md#fetch_call_feedback_summary)                                                     | **GET** /2010-04-01/Accounts/{AccountSid}/Calls/FeedbackSummary/{Sid}.json                                                           |
| _DefaultApi_ | [**fetch_call_notification**](docs/DefaultApi.md#fetch_call_notification)                                                             | **GET** /2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Notifications/{Sid}.json                                                   |
| _DefaultApi_ | [**fetch_call_recording**](docs/DefaultApi.md#fetch_call_recording)                                                                   | **GET** /2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Recordings/{Sid}.json                                                      |
| _DefaultApi_ | [**fetch_conference**](docs/DefaultApi.md#fetch_conference)                                                                           | **GET** /2010-04-01/Accounts/{AccountSid}/Conferences/{Sid}.json                                                                     |
| _DefaultApi_ | [**fetch_conference_recording**](docs/DefaultApi.md#fetch_conference_recording)                                                       | **GET** /2010-04-01/Accounts/{AccountSid}/Conferences/{ConferenceSid}/Recordings/{Sid}.json                                          |
| _DefaultApi_ | [**fetch_connect_app**](docs/DefaultApi.md#fetch_connect_app)                                                                         | **GET** /2010-04-01/Accounts/{AccountSid}/ConnectApps/{Sid}.json                                                                     |
| _DefaultApi_ | [**fetch_incoming_phone_number**](docs/DefaultApi.md#fetch_incoming_phone_number)                                                     | **GET** /2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/{Sid}.json                                                            |
| _DefaultApi_ | [**fetch_incoming_phone_number_assigned_add_on**](docs/DefaultApi.md#fetch_incoming_phone_number_assigned_add_on)                     | **GET** /2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/{ResourceSid}/AssignedAddOns/{Sid}.json                               |
| _DefaultApi_ | [**fetch_incoming_phone_number_assigned_add_on_extension**](docs/DefaultApi.md#fetch_incoming_phone_number_assigned_add_on_extension) | **GET** /2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/{ResourceSid}/AssignedAddOns/{AssignedAddOnSid}/Extensions/{Sid}.json |
| _DefaultApi_ | [**fetch_key**](docs/DefaultApi.md#fetch_key)                                                                                         | **GET** /2010-04-01/Accounts/{AccountSid}/Keys/{Sid}.json                                                                            |
| _DefaultApi_ | [**fetch_media**](docs/DefaultApi.md#fetch_media)                                                                                     | **GET** /2010-04-01/Accounts/{AccountSid}/Messages/{MessageSid}/Media/{Sid}.json                                                     |
| _DefaultApi_ | [**fetch_member**](docs/DefaultApi.md#fetch_member)                                                                                   | **GET** /2010-04-01/Accounts/{AccountSid}/Queues/{QueueSid}/Members/{CallSid}.json                                                   |
| _DefaultApi_ | [**fetch_message**](docs/DefaultApi.md#fetch_message)                                                                                 | **GET** /2010-04-01/Accounts/{AccountSid}/Messages/{Sid}.json                                                                        |
| _DefaultApi_ | [**fetch_notification**](docs/DefaultApi.md#fetch_notification)                                                                       | **GET** /2010-04-01/Accounts/{AccountSid}/Notifications/{Sid}.json                                                                   |
| _DefaultApi_ | [**fetch_outgoing_caller_id**](docs/DefaultApi.md#fetch_outgoing_caller_id)                                                           | **GET** /2010-04-01/Accounts/{AccountSid}/OutgoingCallerIds/{Sid}.json                                                               |
| _DefaultApi_ | [**fetch_participant**](docs/DefaultApi.md#fetch_participant)                                                                         | **GET** /2010-04-01/Accounts/{AccountSid}/Conferences/{ConferenceSid}/Participants/{CallSid}.json                                    |
| _DefaultApi_ | [**fetch_queue**](docs/DefaultApi.md#fetch_queue)                                                                                     | **GET** /2010-04-01/Accounts/{AccountSid}/Queues/{Sid}.json                                                                          |
| _DefaultApi_ | [**fetch_recording**](docs/DefaultApi.md#fetch_recording)                                                                             | **GET** /2010-04-01/Accounts/{AccountSid}/Recordings/{Sid}.json                                                                      |
| _DefaultApi_ | [**fetch_recording_add_on_result**](docs/DefaultApi.md#fetch_recording_add_on_result)                                                 | **GET** /2010-04-01/Accounts/{AccountSid}/Recordings/{ReferenceSid}/AddOnResults/{Sid}.json                                          |
| _DefaultApi_ | [**fetch_recording_add_on_result_payload**](docs/DefaultApi.md#fetch_recording_add_on_result_payload)                                 | **GET** /2010-04-01/Accounts/{AccountSid}/Recordings/{ReferenceSid}/AddOnResults/{AddOnResultSid}/Payloads/{Sid}.json                |
| _DefaultApi_ | [**fetch_recording_transcription**](docs/DefaultApi.md#fetch_recording_transcription)                                                 | **GET** /2010-04-01/Accounts/{AccountSid}/Recordings/{RecordingSid}/Transcriptions/{Sid}.json                                        |
| _DefaultApi_ | [**fetch_short_code**](docs/DefaultApi.md#fetch_short_code)                                                                           | **GET** /2010-04-01/Accounts/{AccountSid}/SMS/ShortCodes/{Sid}.json                                                                  |
| _DefaultApi_ | [**fetch_signing_key**](docs/DefaultApi.md#fetch_signing_key)                                                                         | **GET** /2010-04-01/Accounts/{AccountSid}/SigningKeys/{Sid}.json                                                                     |
| _DefaultApi_ | [**fetch_sip_auth_calls_credential_list_mapping**](docs/DefaultApi.md#fetch_sip_auth_calls_credential_list_mapping)                   | **GET** /2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/Auth/Calls/CredentialListMappings/{Sid}.json                       |
| _DefaultApi_ | [**fetch_sip_auth_calls_ip_access_control_list_mapping**](docs/DefaultApi.md#fetch_sip_auth_calls_ip_access_control_list_mapping)     | **GET** /2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/Auth/Calls/IpAccessControlListMappings/{Sid}.json                  |
| _DefaultApi_ | [**fetch_sip_auth_registrations_credential_list_mapping**](docs/DefaultApi.md#fetch_sip_auth_registrations_credential_list_mapping)   | **GET** /2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/Auth/Registrations/CredentialListMappings/{Sid}.json               |
| _DefaultApi_ | [**fetch_sip_credential**](docs/DefaultApi.md#fetch_sip_credential)                                                                   | **GET** /2010-04-01/Accounts/{AccountSid}/SIP/CredentialLists/{CredentialListSid}/Credentials/{Sid}.json                             |
| _DefaultApi_ | [**fetch_sip_credential_list**](docs/DefaultApi.md#fetch_sip_credential_list)                                                         | **GET** /2010-04-01/Accounts/{AccountSid}/SIP/CredentialLists/{Sid}.json                                                             |
| _DefaultApi_ | [**fetch_sip_credential_list_mapping**](docs/DefaultApi.md#fetch_sip_credential_list_mapping)                                         | **GET** /2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/CredentialListMappings/{Sid}.json                                  |
| _DefaultApi_ | [**fetch_sip_domain**](docs/DefaultApi.md#fetch_sip_domain)                                                                           | **GET** /2010-04-01/Accounts/{AccountSid}/SIP/Domains/{Sid}.json                                                                     |
| _DefaultApi_ | [**fetch_sip_ip_access_control_list**](docs/DefaultApi.md#fetch_sip_ip_access_control_list)                                           | **GET** /2010-04-01/Accounts/{AccountSid}/SIP/IpAccessControlLists/{Sid}.json                                                        |
| _DefaultApi_ | [**fetch_sip_ip_access_control_list_mapping**](docs/DefaultApi.md#fetch_sip_ip_access_control_list_mapping)                           | **GET** /2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/IpAccessControlListMappings/{Sid}.json                             |
| _DefaultApi_ | [**fetch_sip_ip_address**](docs/DefaultApi.md#fetch_sip_ip_address)                                                                   | **GET** /2010-04-01/Accounts/{AccountSid}/SIP/IpAccessControlLists/{IpAccessControlListSid}/IpAddresses/{Sid}.json                   |
| _DefaultApi_ | [**fetch_transcription**](docs/DefaultApi.md#fetch_transcription)                                                                     | **GET** /2010-04-01/Accounts/{AccountSid}/Transcriptions/{Sid}.json                                                                  |
| _DefaultApi_ | [**fetch_usage_trigger**](docs/DefaultApi.md#fetch_usage_trigger)                                                                     | **GET** /2010-04-01/Accounts/{AccountSid}/Usage/Triggers/{Sid}.json                                                                  |
| _DefaultApi_ | [**list_account**](docs/DefaultApi.md#list_account)                                                                                   | **GET** /2010-04-01/Accounts.json                                                                                                    |
| _DefaultApi_ | [**list_address**](docs/DefaultApi.md#list_address)                                                                                   | **GET** /2010-04-01/Accounts/{AccountSid}/Addresses.json                                                                             |
| _DefaultApi_ | [**list_application**](docs/DefaultApi.md#list_application)                                                                           | **GET** /2010-04-01/Accounts/{AccountSid}/Applications.json                                                                          |
| _DefaultApi_ | [**list_authorized_connect_app**](docs/DefaultApi.md#list_authorized_connect_app)                                                     | **GET** /2010-04-01/Accounts/{AccountSid}/AuthorizedConnectApps.json                                                                 |
| _DefaultApi_ | [**list_available_phone_number_country**](docs/DefaultApi.md#list_available_phone_number_country)                                     | **GET** /2010-04-01/Accounts/{AccountSid}/AvailablePhoneNumbers.json                                                                 |
| _DefaultApi_ | [**list_available_phone_number_local**](docs/DefaultApi.md#list_available_phone_number_local)                                         | **GET** /2010-04-01/Accounts/{AccountSid}/AvailablePhoneNumbers/{CountryCode}/Local.json                                             |
| _DefaultApi_ | [**list_available_phone_number_machine_to_machine**](docs/DefaultApi.md#list_available_phone_number_machine_to_machine)               | **GET** /2010-04-01/Accounts/{AccountSid}/AvailablePhoneNumbers/{CountryCode}/MachineToMachine.json                                  |
| _DefaultApi_ | [**list_available_phone_number_mobile**](docs/DefaultApi.md#list_available_phone_number_mobile)                                       | **GET** /2010-04-01/Accounts/{AccountSid}/AvailablePhoneNumbers/{CountryCode}/Mobile.json                                            |
| _DefaultApi_ | [**list_available_phone_number_national**](docs/DefaultApi.md#list_available_phone_number_national)                                   | **GET** /2010-04-01/Accounts/{AccountSid}/AvailablePhoneNumbers/{CountryCode}/National.json                                          |
| _DefaultApi_ | [**list_available_phone_number_shared_cost**](docs/DefaultApi.md#list_available_phone_number_shared_cost)                             | **GET** /2010-04-01/Accounts/{AccountSid}/AvailablePhoneNumbers/{CountryCode}/SharedCost.json                                        |
| _DefaultApi_ | [**list_available_phone_number_toll_free**](docs/DefaultApi.md#list_available_phone_number_toll_free)                                 | **GET** /2010-04-01/Accounts/{AccountSid}/AvailablePhoneNumbers/{CountryCode}/TollFree.json                                          |
| _DefaultApi_ | [**list_available_phone_number_voip**](docs/DefaultApi.md#list_available_phone_number_voip)                                           | **GET** /2010-04-01/Accounts/{AccountSid}/AvailablePhoneNumbers/{CountryCode}/Voip.json                                              |
| _DefaultApi_ | [**list_call**](docs/DefaultApi.md#list_call)                                                                                         | **GET** /2010-04-01/Accounts/{AccountSid}/Calls.json                                                                                 |
| _DefaultApi_ | [**list_call_event**](docs/DefaultApi.md#list_call_event)                                                                             | **GET** /2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Events.json                                                                |
| _DefaultApi_ | [**list_call_notification**](docs/DefaultApi.md#list_call_notification)                                                               | **GET** /2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Notifications.json                                                         |
| _DefaultApi_ | [**list_call_recording**](docs/DefaultApi.md#list_call_recording)                                                                     | **GET** /2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Recordings.json                                                            |
| _DefaultApi_ | [**list_conference**](docs/DefaultApi.md#list_conference)                                                                             | **GET** /2010-04-01/Accounts/{AccountSid}/Conferences.json                                                                           |
| _DefaultApi_ | [**list_conference_recording**](docs/DefaultApi.md#list_conference_recording)                                                         | **GET** /2010-04-01/Accounts/{AccountSid}/Conferences/{ConferenceSid}/Recordings.json                                                |
| _DefaultApi_ | [**list_connect_app**](docs/DefaultApi.md#list_connect_app)                                                                           | **GET** /2010-04-01/Accounts/{AccountSid}/ConnectApps.json                                                                           |
| _DefaultApi_ | [**list_dependent_phone_number**](docs/DefaultApi.md#list_dependent_phone_number)                                                     | **GET** /2010-04-01/Accounts/{AccountSid}/Addresses/{AddressSid}/DependentPhoneNumbers.json                                          |
| _DefaultApi_ | [**list_incoming_phone_number**](docs/DefaultApi.md#list_incoming_phone_number)                                                       | **GET** /2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers.json                                                                  |
| _DefaultApi_ | [**list_incoming_phone_number_assigned_add_on**](docs/DefaultApi.md#list_incoming_phone_number_assigned_add_on)                       | **GET** /2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/{ResourceSid}/AssignedAddOns.json                                     |
| _DefaultApi_ | [**list_incoming_phone_number_assigned_add_on_extension**](docs/DefaultApi.md#list_incoming_phone_number_assigned_add_on_extension)   | **GET** /2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/{ResourceSid}/AssignedAddOns/{AssignedAddOnSid}/Extensions.json       |
| _DefaultApi_ | [**list_incoming_phone_number_local**](docs/DefaultApi.md#list_incoming_phone_number_local)                                           | **GET** /2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/Local.json                                                            |
| _DefaultApi_ | [**list_incoming_phone_number_mobile**](docs/DefaultApi.md#list_incoming_phone_number_mobile)                                         | **GET** /2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/Mobile.json                                                           |
| _DefaultApi_ | [**list_incoming_phone_number_toll_free**](docs/DefaultApi.md#list_incoming_phone_number_toll_free)                                   | **GET** /2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/TollFree.json                                                         |
| _DefaultApi_ | [**list_key**](docs/DefaultApi.md#list_key)                                                                                           | **GET** /2010-04-01/Accounts/{AccountSid}/Keys.json                                                                                  |
| _DefaultApi_ | [**list_media**](docs/DefaultApi.md#list_media)                                                                                       | **GET** /2010-04-01/Accounts/{AccountSid}/Messages/{MessageSid}/Media.json                                                           |
| _DefaultApi_ | [**list_member**](docs/DefaultApi.md#list_member)                                                                                     | **GET** /2010-04-01/Accounts/{AccountSid}/Queues/{QueueSid}/Members.json                                                             |
| _DefaultApi_ | [**list_message**](docs/DefaultApi.md#list_message)                                                                                   | **GET** /2010-04-01/Accounts/{AccountSid}/Messages.json                                                                              |
| _DefaultApi_ | [**list_notification**](docs/DefaultApi.md#list_notification)                                                                         | **GET** /2010-04-01/Accounts/{AccountSid}/Notifications.json                                                                         |
| _DefaultApi_ | [**list_outgoing_caller_id**](docs/DefaultApi.md#list_outgoing_caller_id)                                                             | **GET** /2010-04-01/Accounts/{AccountSid}/OutgoingCallerIds.json                                                                     |
| _DefaultApi_ | [**list_participant**](docs/DefaultApi.md#list_participant)                                                                           | **GET** /2010-04-01/Accounts/{AccountSid}/Conferences/{ConferenceSid}/Participants.json                                              |
| _DefaultApi_ | [**list_queue**](docs/DefaultApi.md#list_queue)                                                                                       | **GET** /2010-04-01/Accounts/{AccountSid}/Queues.json                                                                                |
| _DefaultApi_ | [**list_recording**](docs/DefaultApi.md#list_recording)                                                                               | **GET** /2010-04-01/Accounts/{AccountSid}/Recordings.json                                                                            |
| _DefaultApi_ | [**list_recording_add_on_result**](docs/DefaultApi.md#list_recording_add_on_result)                                                   | **GET** /2010-04-01/Accounts/{AccountSid}/Recordings/{ReferenceSid}/AddOnResults.json                                                |
| _DefaultApi_ | [**list_recording_add_on_result_payload**](docs/DefaultApi.md#list_recording_add_on_result_payload)                                   | **GET** /2010-04-01/Accounts/{AccountSid}/Recordings/{ReferenceSid}/AddOnResults/{AddOnResultSid}/Payloads.json                      |
| _DefaultApi_ | [**list_recording_transcription**](docs/DefaultApi.md#list_recording_transcription)                                                   | **GET** /2010-04-01/Accounts/{AccountSid}/Recordings/{RecordingSid}/Transcriptions.json                                              |
| _DefaultApi_ | [**list_short_code**](docs/DefaultApi.md#list_short_code)                                                                             | **GET** /2010-04-01/Accounts/{AccountSid}/SMS/ShortCodes.json                                                                        |
| _DefaultApi_ | [**list_signing_key**](docs/DefaultApi.md#list_signing_key)                                                                           | **GET** /2010-04-01/Accounts/{AccountSid}/SigningKeys.json                                                                           |
| _DefaultApi_ | [**list_sip_auth_calls_credential_list_mapping**](docs/DefaultApi.md#list_sip_auth_calls_credential_list_mapping)                     | **GET** /2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/Auth/Calls/CredentialListMappings.json                             |
| _DefaultApi_ | [**list_sip_auth_calls_ip_access_control_list_mapping**](docs/DefaultApi.md#list_sip_auth_calls_ip_access_control_list_mapping)       | **GET** /2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/Auth/Calls/IpAccessControlListMappings.json                        |
| _DefaultApi_ | [**list_sip_auth_registrations_credential_list_mapping**](docs/DefaultApi.md#list_sip_auth_registrations_credential_list_mapping)     | **GET** /2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/Auth/Registrations/CredentialListMappings.json                     |
| _DefaultApi_ | [**list_sip_credential**](docs/DefaultApi.md#list_sip_credential)                                                                     | **GET** /2010-04-01/Accounts/{AccountSid}/SIP/CredentialLists/{CredentialListSid}/Credentials.json                                   |
| _DefaultApi_ | [**list_sip_credential_list**](docs/DefaultApi.md#list_sip_credential_list)                                                           | **GET** /2010-04-01/Accounts/{AccountSid}/SIP/CredentialLists.json                                                                   |
| _DefaultApi_ | [**list_sip_credential_list_mapping**](docs/DefaultApi.md#list_sip_credential_list_mapping)                                           | **GET** /2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/CredentialListMappings.json                                        |
| _DefaultApi_ | [**list_sip_domain**](docs/DefaultApi.md#list_sip_domain)                                                                             | **GET** /2010-04-01/Accounts/{AccountSid}/SIP/Domains.json                                                                           |
| _DefaultApi_ | [**list_sip_ip_access_control_list**](docs/DefaultApi.md#list_sip_ip_access_control_list)                                             | **GET** /2010-04-01/Accounts/{AccountSid}/SIP/IpAccessControlLists.json                                                              |
| _DefaultApi_ | [**list_sip_ip_access_control_list_mapping**](docs/DefaultApi.md#list_sip_ip_access_control_list_mapping)                             | **GET** /2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/IpAccessControlListMappings.json                                   |
| _DefaultApi_ | [**list_sip_ip_address**](docs/DefaultApi.md#list_sip_ip_address)                                                                     | **GET** /2010-04-01/Accounts/{AccountSid}/SIP/IpAccessControlLists/{IpAccessControlListSid}/IpAddresses.json                         |
| _DefaultApi_ | [**list_transcription**](docs/DefaultApi.md#list_transcription)                                                                       | **GET** /2010-04-01/Accounts/{AccountSid}/Transcriptions.json                                                                        |
| _DefaultApi_ | [**list_usage_record**](docs/DefaultApi.md#list_usage_record)                                                                         | **GET** /2010-04-01/Accounts/{AccountSid}/Usage/Records.json                                                                         |
| _DefaultApi_ | [**list_usage_record_all_time**](docs/DefaultApi.md#list_usage_record_all_time)                                                       | **GET** /2010-04-01/Accounts/{AccountSid}/Usage/Records/AllTime.json                                                                 |
| _DefaultApi_ | [**list_usage_record_daily**](docs/DefaultApi.md#list_usage_record_daily)                                                             | **GET** /2010-04-01/Accounts/{AccountSid}/Usage/Records/Daily.json                                                                   |
| _DefaultApi_ | [**list_usage_record_last_month**](docs/DefaultApi.md#list_usage_record_last_month)                                                   | **GET** /2010-04-01/Accounts/{AccountSid}/Usage/Records/LastMonth.json                                                               |
| _DefaultApi_ | [**list_usage_record_monthly**](docs/DefaultApi.md#list_usage_record_monthly)                                                         | **GET** /2010-04-01/Accounts/{AccountSid}/Usage/Records/Monthly.json                                                                 |
| _DefaultApi_ | [**list_usage_record_this_month**](docs/DefaultApi.md#list_usage_record_this_month)                                                   | **GET** /2010-04-01/Accounts/{AccountSid}/Usage/Records/ThisMonth.json                                                               |
| _DefaultApi_ | [**list_usage_record_today**](docs/DefaultApi.md#list_usage_record_today)                                                             | **GET** /2010-04-01/Accounts/{AccountSid}/Usage/Records/Today.json                                                                   |
| _DefaultApi_ | [**list_usage_record_yearly**](docs/DefaultApi.md#list_usage_record_yearly)                                                           | **GET** /2010-04-01/Accounts/{AccountSid}/Usage/Records/Yearly.json                                                                  |
| _DefaultApi_ | [**list_usage_record_yesterday**](docs/DefaultApi.md#list_usage_record_yesterday)                                                     | **GET** /2010-04-01/Accounts/{AccountSid}/Usage/Records/Yesterday.json                                                               |
| _DefaultApi_ | [**list_usage_trigger**](docs/DefaultApi.md#list_usage_trigger)                                                                       | **GET** /2010-04-01/Accounts/{AccountSid}/Usage/Triggers.json                                                                        |
| _DefaultApi_ | [**update_account**](docs/DefaultApi.md#update_account)                                                                               | **POST** /2010-04-01/Accounts/{Sid}.json                                                                                             |
| _DefaultApi_ | [**update_address**](docs/DefaultApi.md#update_address)                                                                               | **POST** /2010-04-01/Accounts/{AccountSid}/Addresses/{Sid}.json                                                                      |
| _DefaultApi_ | [**update_application**](docs/DefaultApi.md#update_application)                                                                       | **POST** /2010-04-01/Accounts/{AccountSid}/Applications/{Sid}.json                                                                   |
| _DefaultApi_ | [**update_call**](docs/DefaultApi.md#update_call)                                                                                     | **POST** /2010-04-01/Accounts/{AccountSid}/Calls/{Sid}.json                                                                          |
| _DefaultApi_ | [**update_call_feedback**](docs/DefaultApi.md#update_call_feedback)                                                                   | **POST** /2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Feedback.json                                                             |
| _DefaultApi_ | [**update_call_recording**](docs/DefaultApi.md#update_call_recording)                                                                 | **POST** /2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Recordings/{Sid}.json                                                     |
| _DefaultApi_ | [**update_conference**](docs/DefaultApi.md#update_conference)                                                                         | **POST** /2010-04-01/Accounts/{AccountSid}/Conferences/{Sid}.json                                                                    |
| _DefaultApi_ | [**update_conference_recording**](docs/DefaultApi.md#update_conference_recording)                                                     | **POST** /2010-04-01/Accounts/{AccountSid}/Conferences/{ConferenceSid}/Recordings/{Sid}.json                                         |
| _DefaultApi_ | [**update_connect_app**](docs/DefaultApi.md#update_connect_app)                                                                       | **POST** /2010-04-01/Accounts/{AccountSid}/ConnectApps/{Sid}.json                                                                    |
| _DefaultApi_ | [**update_incoming_phone_number**](docs/DefaultApi.md#update_incoming_phone_number)                                                   | **POST** /2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/{Sid}.json                                                           |
| _DefaultApi_ | [**update_key**](docs/DefaultApi.md#update_key)                                                                                       | **POST** /2010-04-01/Accounts/{AccountSid}/Keys/{Sid}.json                                                                           |
| _DefaultApi_ | [**update_member**](docs/DefaultApi.md#update_member)                                                                                 | **POST** /2010-04-01/Accounts/{AccountSid}/Queues/{QueueSid}/Members/{CallSid}.json                                                  |
| _DefaultApi_ | [**update_message**](docs/DefaultApi.md#update_message)                                                                               | **POST** /2010-04-01/Accounts/{AccountSid}/Messages/{Sid}.json                                                                       |
| _DefaultApi_ | [**update_outgoing_caller_id**](docs/DefaultApi.md#update_outgoing_caller_id)                                                         | **POST** /2010-04-01/Accounts/{AccountSid}/OutgoingCallerIds/{Sid}.json                                                              |
| _DefaultApi_ | [**update_participant**](docs/DefaultApi.md#update_participant)                                                                       | **POST** /2010-04-01/Accounts/{AccountSid}/Conferences/{ConferenceSid}/Participants/{CallSid}.json                                   |
| _DefaultApi_ | [**update_payments**](docs/DefaultApi.md#update_payments)                                                                             | **POST** /2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Payments/{Sid}.json                                                       |
| _DefaultApi_ | [**update_queue**](docs/DefaultApi.md#update_queue)                                                                                   | **POST** /2010-04-01/Accounts/{AccountSid}/Queues/{Sid}.json                                                                         |
| _DefaultApi_ | [**update_short_code**](docs/DefaultApi.md#update_short_code)                                                                         | **POST** /2010-04-01/Accounts/{AccountSid}/SMS/ShortCodes/{Sid}.json                                                                 |
| _DefaultApi_ | [**update_signing_key**](docs/DefaultApi.md#update_signing_key)                                                                       | **POST** /2010-04-01/Accounts/{AccountSid}/SigningKeys/{Sid}.json                                                                    |
| _DefaultApi_ | [**update_sip_credential**](docs/DefaultApi.md#update_sip_credential)                                                                 | **POST** /2010-04-01/Accounts/{AccountSid}/SIP/CredentialLists/{CredentialListSid}/Credentials/{Sid}.json                            |
| _DefaultApi_ | [**update_sip_credential_list**](docs/DefaultApi.md#update_sip_credential_list)                                                       | **POST** /2010-04-01/Accounts/{AccountSid}/SIP/CredentialLists/{Sid}.json                                                            |
| _DefaultApi_ | [**update_sip_domain**](docs/DefaultApi.md#update_sip_domain)                                                                         | **POST** /2010-04-01/Accounts/{AccountSid}/SIP/Domains/{Sid}.json                                                                    |
| _DefaultApi_ | [**update_sip_ip_access_control_list**](docs/DefaultApi.md#update_sip_ip_access_control_list)                                         | **POST** /2010-04-01/Accounts/{AccountSid}/SIP/IpAccessControlLists/{Sid}.json                                                       |
| _DefaultApi_ | [**update_sip_ip_address**](docs/DefaultApi.md#update_sip_ip_address)                                                                 | **POST** /2010-04-01/Accounts/{AccountSid}/SIP/IpAccessControlLists/{IpAccessControlListSid}/IpAddresses/{Sid}.json                  |
| _DefaultApi_ | [**update_siprec**](docs/DefaultApi.md#update_siprec)                                                                                 | **POST** /2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Siprec/{Sid}.json                                                         |
| _DefaultApi_ | [**update_stream**](docs/DefaultApi.md#update_stream)                                                                                 | **POST** /2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Streams/{Sid}.json                                                        |
| _DefaultApi_ | [**update_usage_trigger**](docs/DefaultApi.md#update_usage_trigger)                                                                   | **POST** /2010-04-01/Accounts/{AccountSid}/Usage/Triggers/{Sid}.json                                                                 |

## Documentation For Models

- [AccountEnumStatus](docs/AccountEnumStatus.md)
- [AccountEnumType](docs/AccountEnumType.md)
- [ApiPeriodV2010PeriodAccount](docs/ApiPeriodV2010PeriodAccount.md)
- [ApiPeriodV2010PeriodAccountPeriodAddress](docs/ApiPeriodV2010PeriodAccountPeriodAddress.md)
- [ApiPeriodV2010PeriodAccountPeriodAddressPeriodDependentPhoneNumber](docs/ApiPeriodV2010PeriodAccountPeriodAddressPeriodDependentPhoneNumber.md)
- [ApiPeriodV2010PeriodAccountPeriodApplication](docs/ApiPeriodV2010PeriodAccountPeriodApplication.md)
- [ApiPeriodV2010PeriodAccountPeriodAuthorizedConnectApp](docs/ApiPeriodV2010PeriodAccountPeriodAuthorizedConnectApp.md)
- [ApiPeriodV2010PeriodAccountPeriodAvailablePhoneNumberCountry](docs/ApiPeriodV2010PeriodAccountPeriodAvailablePhoneNumberCountry.md)
- [ApiPeriodV2010PeriodAccountPeriodAvailablePhoneNumberCountryPeriodAvailablePhoneNumberLocal](docs/ApiPeriodV2010PeriodAccountPeriodAvailablePhoneNumberCountryPeriodAvailablePhoneNumberLocal.md)
- [ApiPeriodV2010PeriodAccountPeriodAvailablePhoneNumberCountryPeriodAvailablePhoneNumberMachineToMachine](docs/ApiPeriodV2010PeriodAccountPeriodAvailablePhoneNumberCountryPeriodAvailablePhoneNumberMachineToMachine.md)
- [ApiPeriodV2010PeriodAccountPeriodAvailablePhoneNumberCountryPeriodAvailablePhoneNumberMobile](docs/ApiPeriodV2010PeriodAccountPeriodAvailablePhoneNumberCountryPeriodAvailablePhoneNumberMobile.md)
- [ApiPeriodV2010PeriodAccountPeriodAvailablePhoneNumberCountryPeriodAvailablePhoneNumberNational](docs/ApiPeriodV2010PeriodAccountPeriodAvailablePhoneNumberCountryPeriodAvailablePhoneNumberNational.md)
- [ApiPeriodV2010PeriodAccountPeriodAvailablePhoneNumberCountryPeriodAvailablePhoneNumberSharedCost](docs/ApiPeriodV2010PeriodAccountPeriodAvailablePhoneNumberCountryPeriodAvailablePhoneNumberSharedCost.md)
- [ApiPeriodV2010PeriodAccountPeriodAvailablePhoneNumberCountryPeriodAvailablePhoneNumberTollFree](docs/ApiPeriodV2010PeriodAccountPeriodAvailablePhoneNumberCountryPeriodAvailablePhoneNumberTollFree.md)
- [ApiPeriodV2010PeriodAccountPeriodAvailablePhoneNumberCountryPeriodAvailablePhoneNumberVoip](docs/ApiPeriodV2010PeriodAccountPeriodAvailablePhoneNumberCountryPeriodAvailablePhoneNumberVoip.md)
- [ApiPeriodV2010PeriodAccountPeriodBalance](docs/ApiPeriodV2010PeriodAccountPeriodBalance.md)
- [ApiPeriodV2010PeriodAccountPeriodCall](docs/ApiPeriodV2010PeriodAccountPeriodCall.md)
- [ApiPeriodV2010PeriodAccountPeriodCallPeriodCallEvent](docs/ApiPeriodV2010PeriodAccountPeriodCallPeriodCallEvent.md)
- [ApiPeriodV2010PeriodAccountPeriodCallPeriodCallFeedback](docs/ApiPeriodV2010PeriodAccountPeriodCallPeriodCallFeedback.md)
- [ApiPeriodV2010PeriodAccountPeriodCallPeriodCallFeedbackSummary](docs/ApiPeriodV2010PeriodAccountPeriodCallPeriodCallFeedbackSummary.md)
- [ApiPeriodV2010PeriodAccountPeriodCallPeriodCallNotification](docs/ApiPeriodV2010PeriodAccountPeriodCallPeriodCallNotification.md)
- [ApiPeriodV2010PeriodAccountPeriodCallPeriodCallNotificationInstance](docs/ApiPeriodV2010PeriodAccountPeriodCallPeriodCallNotificationInstance.md)
- [ApiPeriodV2010PeriodAccountPeriodCallPeriodCallRecording](docs/ApiPeriodV2010PeriodAccountPeriodCallPeriodCallRecording.md)
- [ApiPeriodV2010PeriodAccountPeriodCallPeriodPayments](docs/ApiPeriodV2010PeriodAccountPeriodCallPeriodPayments.md)
- [ApiPeriodV2010PeriodAccountPeriodCallPeriodSiprec](docs/ApiPeriodV2010PeriodAccountPeriodCallPeriodSiprec.md)
- [ApiPeriodV2010PeriodAccountPeriodCallPeriodStream](docs/ApiPeriodV2010PeriodAccountPeriodCallPeriodStream.md)
- [ApiPeriodV2010PeriodAccountPeriodCallPeriodUserDefinedMessage](docs/ApiPeriodV2010PeriodAccountPeriodCallPeriodUserDefinedMessage.md)
- [ApiPeriodV2010PeriodAccountPeriodCallPeriodUserDefinedMessageSubscription](docs/ApiPeriodV2010PeriodAccountPeriodCallPeriodUserDefinedMessageSubscription.md)
- [ApiPeriodV2010PeriodAccountPeriodConference](docs/ApiPeriodV2010PeriodAccountPeriodConference.md)
- [ApiPeriodV2010PeriodAccountPeriodConferencePeriodConferenceRecording](docs/ApiPeriodV2010PeriodAccountPeriodConferencePeriodConferenceRecording.md)
- [ApiPeriodV2010PeriodAccountPeriodConferencePeriodParticipant](docs/ApiPeriodV2010PeriodAccountPeriodConferencePeriodParticipant.md)
- [ApiPeriodV2010PeriodAccountPeriodConnectApp](docs/ApiPeriodV2010PeriodAccountPeriodConnectApp.md)
- [ApiPeriodV2010PeriodAccountPeriodIncomingPhoneNumber](docs/ApiPeriodV2010PeriodAccountPeriodIncomingPhoneNumber.md)
- [ApiPeriodV2010PeriodAccountPeriodIncomingPhoneNumberPeriodIncomingPhoneNumberAssignedAddOn](docs/ApiPeriodV2010PeriodAccountPeriodIncomingPhoneNumberPeriodIncomingPhoneNumberAssignedAddOn.md)
- [ApiPeriodV2010PeriodAccountPeriodIncomingPhoneNumberPeriodIncomingPhoneNumberAssignedAddOnPeriodIncomingPhoneNumberAssignedAddOnExtension](docs/ApiPeriodV2010PeriodAccountPeriodIncomingPhoneNumberPeriodIncomingPhoneNumberAssignedAddOnPeriodIncomingPhoneNumberAssignedAddOnExtension.md)
- [ApiPeriodV2010PeriodAccountPeriodIncomingPhoneNumberPeriodIncomingPhoneNumberLocal](docs/ApiPeriodV2010PeriodAccountPeriodIncomingPhoneNumberPeriodIncomingPhoneNumberLocal.md)
- [ApiPeriodV2010PeriodAccountPeriodIncomingPhoneNumberPeriodIncomingPhoneNumberMobile](docs/ApiPeriodV2010PeriodAccountPeriodIncomingPhoneNumberPeriodIncomingPhoneNumberMobile.md)
- [ApiPeriodV2010PeriodAccountPeriodIncomingPhoneNumberPeriodIncomingPhoneNumberTollFree](docs/ApiPeriodV2010PeriodAccountPeriodIncomingPhoneNumberPeriodIncomingPhoneNumberTollFree.md)
- [ApiPeriodV2010PeriodAccountPeriodKey](docs/ApiPeriodV2010PeriodAccountPeriodKey.md)
- [ApiPeriodV2010PeriodAccountPeriodMessage](docs/ApiPeriodV2010PeriodAccountPeriodMessage.md)
- [ApiPeriodV2010PeriodAccountPeriodMessagePeriodMedia](docs/ApiPeriodV2010PeriodAccountPeriodMessagePeriodMedia.md)
- [ApiPeriodV2010PeriodAccountPeriodMessagePeriodMessageFeedback](docs/ApiPeriodV2010PeriodAccountPeriodMessagePeriodMessageFeedback.md)
- [ApiPeriodV2010PeriodAccountPeriodNewKey](docs/ApiPeriodV2010PeriodAccountPeriodNewKey.md)
- [ApiPeriodV2010PeriodAccountPeriodNewSigningKey](docs/ApiPeriodV2010PeriodAccountPeriodNewSigningKey.md)
- [ApiPeriodV2010PeriodAccountPeriodNotification](docs/ApiPeriodV2010PeriodAccountPeriodNotification.md)
- [ApiPeriodV2010PeriodAccountPeriodNotificationInstance](docs/ApiPeriodV2010PeriodAccountPeriodNotificationInstance.md)
- [ApiPeriodV2010PeriodAccountPeriodOutgoingCallerId](docs/ApiPeriodV2010PeriodAccountPeriodOutgoingCallerId.md)
- [ApiPeriodV2010PeriodAccountPeriodQueue](docs/ApiPeriodV2010PeriodAccountPeriodQueue.md)
- [ApiPeriodV2010PeriodAccountPeriodQueuePeriodMember](docs/ApiPeriodV2010PeriodAccountPeriodQueuePeriodMember.md)
- [ApiPeriodV2010PeriodAccountPeriodRecording](docs/ApiPeriodV2010PeriodAccountPeriodRecording.md)
- [ApiPeriodV2010PeriodAccountPeriodRecordingPeriodRecordingAddOnResult](docs/ApiPeriodV2010PeriodAccountPeriodRecordingPeriodRecordingAddOnResult.md)
- [ApiPeriodV2010PeriodAccountPeriodRecordingPeriodRecordingAddOnResultPeriodRecordingAddOnResultPayload](docs/ApiPeriodV2010PeriodAccountPeriodRecordingPeriodRecordingAddOnResultPeriodRecordingAddOnResultPayload.md)
- [ApiPeriodV2010PeriodAccountPeriodRecordingPeriodRecordingTranscription](docs/ApiPeriodV2010PeriodAccountPeriodRecordingPeriodRecordingTranscription.md)
- [ApiPeriodV2010PeriodAccountPeriodShortCode](docs/ApiPeriodV2010PeriodAccountPeriodShortCode.md)
- [ApiPeriodV2010PeriodAccountPeriodSigningKey](docs/ApiPeriodV2010PeriodAccountPeriodSigningKey.md)
- [ApiPeriodV2010PeriodAccountPeriodSipPeriodSipCredentialList](docs/ApiPeriodV2010PeriodAccountPeriodSipPeriodSipCredentialList.md)
- [ApiPeriodV2010PeriodAccountPeriodSipPeriodSipCredentialListPeriodSipCredential](docs/ApiPeriodV2010PeriodAccountPeriodSipPeriodSipCredentialListPeriodSipCredential.md)
- [ApiPeriodV2010PeriodAccountPeriodSipPeriodSipDomain](docs/ApiPeriodV2010PeriodAccountPeriodSipPeriodSipDomain.md)
- [ApiPeriodV2010PeriodAccountPeriodSipPeriodSipDomainPeriodSipAuthPeriodSipAuthCallsPeriodSipAuthCallsCredentialListMapping](docs/ApiPeriodV2010PeriodAccountPeriodSipPeriodSipDomainPeriodSipAuthPeriodSipAuthCallsPeriodSipAuthCallsCredentialListMapping.md)
- [ApiPeriodV2010PeriodAccountPeriodSipPeriodSipDomainPeriodSipAuthPeriodSipAuthCallsPeriodSipAuthCallsIpAccessControlListMapping](docs/ApiPeriodV2010PeriodAccountPeriodSipPeriodSipDomainPeriodSipAuthPeriodSipAuthCallsPeriodSipAuthCallsIpAccessControlListMapping.md)
- [ApiPeriodV2010PeriodAccountPeriodSipPeriodSipDomainPeriodSipAuthPeriodSipAuthRegistrationsPeriodSipAuthRegistrationsCredentialListMapping](docs/ApiPeriodV2010PeriodAccountPeriodSipPeriodSipDomainPeriodSipAuthPeriodSipAuthRegistrationsPeriodSipAuthRegistrationsCredentialListMapping.md)
- [ApiPeriodV2010PeriodAccountPeriodSipPeriodSipDomainPeriodSipCredentialListMapping](docs/ApiPeriodV2010PeriodAccountPeriodSipPeriodSipDomainPeriodSipCredentialListMapping.md)
- [ApiPeriodV2010PeriodAccountPeriodSipPeriodSipDomainPeriodSipIpAccessControlListMapping](docs/ApiPeriodV2010PeriodAccountPeriodSipPeriodSipDomainPeriodSipIpAccessControlListMapping.md)
- [ApiPeriodV2010PeriodAccountPeriodSipPeriodSipIpAccessControlList](docs/ApiPeriodV2010PeriodAccountPeriodSipPeriodSipIpAccessControlList.md)
- [ApiPeriodV2010PeriodAccountPeriodSipPeriodSipIpAccessControlListPeriodSipIpAddress](docs/ApiPeriodV2010PeriodAccountPeriodSipPeriodSipIpAccessControlListPeriodSipIpAddress.md)
- [ApiPeriodV2010PeriodAccountPeriodToken](docs/ApiPeriodV2010PeriodAccountPeriodToken.md)
- [ApiPeriodV2010PeriodAccountPeriodTranscription](docs/ApiPeriodV2010PeriodAccountPeriodTranscription.md)
- [ApiPeriodV2010PeriodAccountPeriodUsagePeriodUsageRecord](docs/ApiPeriodV2010PeriodAccountPeriodUsagePeriodUsageRecord.md)
- [ApiPeriodV2010PeriodAccountPeriodUsagePeriodUsageRecordPeriodUsageRecordAllTime](docs/ApiPeriodV2010PeriodAccountPeriodUsagePeriodUsageRecordPeriodUsageRecordAllTime.md)
- [ApiPeriodV2010PeriodAccountPeriodUsagePeriodUsageRecordPeriodUsageRecordDaily](docs/ApiPeriodV2010PeriodAccountPeriodUsagePeriodUsageRecordPeriodUsageRecordDaily.md)
- [ApiPeriodV2010PeriodAccountPeriodUsagePeriodUsageRecordPeriodUsageRecordLastMonth](docs/ApiPeriodV2010PeriodAccountPeriodUsagePeriodUsageRecordPeriodUsageRecordLastMonth.md)
- [ApiPeriodV2010PeriodAccountPeriodUsagePeriodUsageRecordPeriodUsageRecordMonthly](docs/ApiPeriodV2010PeriodAccountPeriodUsagePeriodUsageRecordPeriodUsageRecordMonthly.md)
- [ApiPeriodV2010PeriodAccountPeriodUsagePeriodUsageRecordPeriodUsageRecordThisMonth](docs/ApiPeriodV2010PeriodAccountPeriodUsagePeriodUsageRecordPeriodUsageRecordThisMonth.md)
- [ApiPeriodV2010PeriodAccountPeriodUsagePeriodUsageRecordPeriodUsageRecordToday](docs/ApiPeriodV2010PeriodAccountPeriodUsagePeriodUsageRecordPeriodUsageRecordToday.md)
- [ApiPeriodV2010PeriodAccountPeriodUsagePeriodUsageRecordPeriodUsageRecordYearly](docs/ApiPeriodV2010PeriodAccountPeriodUsagePeriodUsageRecordPeriodUsageRecordYearly.md)
- [ApiPeriodV2010PeriodAccountPeriodUsagePeriodUsageRecordPeriodUsageRecordYesterday](docs/ApiPeriodV2010PeriodAccountPeriodUsagePeriodUsageRecordPeriodUsageRecordYesterday.md)
- [ApiPeriodV2010PeriodAccountPeriodUsagePeriodUsageTrigger](docs/ApiPeriodV2010PeriodAccountPeriodUsagePeriodUsageTrigger.md)
- [ApiPeriodV2010PeriodAccountPeriodValidationRequest](docs/ApiPeriodV2010PeriodAccountPeriodValidationRequest.md)
- [ApiV2010AccountAvailablePhoneNumberCountryAvailablePhoneNumberLocalCapabilities](docs/ApiV2010AccountAvailablePhoneNumberCountryAvailablePhoneNumberLocalCapabilities.md)
- [ApiV2010AccountIncomingPhoneNumberCapabilities](docs/ApiV2010AccountIncomingPhoneNumberCapabilities.md)
- [ApiV2010AccountTokenIceServersInner](docs/ApiV2010AccountTokenIceServersInner.md)
- [AuthorizedConnectAppEnumPermission](docs/AuthorizedConnectAppEnumPermission.md)
- [CallEnumEvent](docs/CallEnumEvent.md)
- [CallEnumStatus](docs/CallEnumStatus.md)
- [CallFeedbackEnumIssues](docs/CallFeedbackEnumIssues.md)
- [CallFeedbackSummaryEnumStatus](docs/CallFeedbackSummaryEnumStatus.md)
- [CallRecordingEnumSource](docs/CallRecordingEnumSource.md)
- [CallRecordingEnumStatus](docs/CallRecordingEnumStatus.md)
- [ConferenceEnumReasonConferenceEnded](docs/ConferenceEnumReasonConferenceEnded.md)
- [ConferenceEnumStatus](docs/ConferenceEnumStatus.md)
- [ConferenceRecordingEnumSource](docs/ConferenceRecordingEnumSource.md)
- [ConferenceRecordingEnumStatus](docs/ConferenceRecordingEnumStatus.md)
- [ConnectAppEnumPermission](docs/ConnectAppEnumPermission.md)
- [DependentPhoneNumberEnumAddressRequirement](docs/DependentPhoneNumberEnumAddressRequirement.md)
- [DependentPhoneNumberEnumEmergencyStatus](docs/DependentPhoneNumberEnumEmergencyStatus.md)
- [IncomingPhoneNumberEnumAddressRequirement](docs/IncomingPhoneNumberEnumAddressRequirement.md)
- [IncomingPhoneNumberEnumEmergencyAddressStatus](docs/IncomingPhoneNumberEnumEmergencyAddressStatus.md)
- [IncomingPhoneNumberEnumEmergencyStatus](docs/IncomingPhoneNumberEnumEmergencyStatus.md)
- [IncomingPhoneNumberEnumVoiceReceiveMode](docs/IncomingPhoneNumberEnumVoiceReceiveMode.md)
- [IncomingPhoneNumberLocalEnumAddressRequirement](docs/IncomingPhoneNumberLocalEnumAddressRequirement.md)
- [IncomingPhoneNumberLocalEnumEmergencyAddressStatus](docs/IncomingPhoneNumberLocalEnumEmergencyAddressStatus.md)
- [IncomingPhoneNumberLocalEnumEmergencyStatus](docs/IncomingPhoneNumberLocalEnumEmergencyStatus.md)
- [IncomingPhoneNumberLocalEnumVoiceReceiveMode](docs/IncomingPhoneNumberLocalEnumVoiceReceiveMode.md)
- [IncomingPhoneNumberMobileEnumAddressRequirement](docs/IncomingPhoneNumberMobileEnumAddressRequirement.md)
- [IncomingPhoneNumberMobileEnumEmergencyAddressStatus](docs/IncomingPhoneNumberMobileEnumEmergencyAddressStatus.md)
- [IncomingPhoneNumberMobileEnumEmergencyStatus](docs/IncomingPhoneNumberMobileEnumEmergencyStatus.md)
- [IncomingPhoneNumberMobileEnumVoiceReceiveMode](docs/IncomingPhoneNumberMobileEnumVoiceReceiveMode.md)
- [IncomingPhoneNumberTollFreeEnumAddressRequirement](docs/IncomingPhoneNumberTollFreeEnumAddressRequirement.md)
- [IncomingPhoneNumberTollFreeEnumEmergencyAddressStatus](docs/IncomingPhoneNumberTollFreeEnumEmergencyAddressStatus.md)
- [IncomingPhoneNumberTollFreeEnumEmergencyStatus](docs/IncomingPhoneNumberTollFreeEnumEmergencyStatus.md)
- [IncomingPhoneNumberTollFreeEnumVoiceReceiveMode](docs/IncomingPhoneNumberTollFreeEnumVoiceReceiveMode.md)
- [ListAccountResponse](docs/ListAccountResponse.md)
- [ListAddressResponse](docs/ListAddressResponse.md)
- [ListApplicationResponse](docs/ListApplicationResponse.md)
- [ListAuthorizedConnectAppResponse](docs/ListAuthorizedConnectAppResponse.md)
- [ListAvailablePhoneNumberCountryResponse](docs/ListAvailablePhoneNumberCountryResponse.md)
- [ListAvailablePhoneNumberLocalResponse](docs/ListAvailablePhoneNumberLocalResponse.md)
- [ListAvailablePhoneNumberMachineToMachineResponse](docs/ListAvailablePhoneNumberMachineToMachineResponse.md)
- [ListAvailablePhoneNumberMobileResponse](docs/ListAvailablePhoneNumberMobileResponse.md)
- [ListAvailablePhoneNumberNationalResponse](docs/ListAvailablePhoneNumberNationalResponse.md)
- [ListAvailablePhoneNumberSharedCostResponse](docs/ListAvailablePhoneNumberSharedCostResponse.md)
- [ListAvailablePhoneNumberTollFreeResponse](docs/ListAvailablePhoneNumberTollFreeResponse.md)
- [ListAvailablePhoneNumberVoipResponse](docs/ListAvailablePhoneNumberVoipResponse.md)
- [ListCallEventResponse](docs/ListCallEventResponse.md)
- [ListCallNotificationResponse](docs/ListCallNotificationResponse.md)
- [ListCallRecordingResponse](docs/ListCallRecordingResponse.md)
- [ListCallResponse](docs/ListCallResponse.md)
- [ListConferenceRecordingResponse](docs/ListConferenceRecordingResponse.md)
- [ListConferenceResponse](docs/ListConferenceResponse.md)
- [ListConnectAppResponse](docs/ListConnectAppResponse.md)
- [ListDependentPhoneNumberResponse](docs/ListDependentPhoneNumberResponse.md)
- [ListIncomingPhoneNumberAssignedAddOnExtensionResponse](docs/ListIncomingPhoneNumberAssignedAddOnExtensionResponse.md)
- [ListIncomingPhoneNumberAssignedAddOnResponse](docs/ListIncomingPhoneNumberAssignedAddOnResponse.md)
- [ListIncomingPhoneNumberLocalResponse](docs/ListIncomingPhoneNumberLocalResponse.md)
- [ListIncomingPhoneNumberMobileResponse](docs/ListIncomingPhoneNumberMobileResponse.md)
- [ListIncomingPhoneNumberResponse](docs/ListIncomingPhoneNumberResponse.md)
- [ListIncomingPhoneNumberTollFreeResponse](docs/ListIncomingPhoneNumberTollFreeResponse.md)
- [ListKeyResponse](docs/ListKeyResponse.md)
- [ListMediaResponse](docs/ListMediaResponse.md)
- [ListMemberResponse](docs/ListMemberResponse.md)
- [ListMessageResponse](docs/ListMessageResponse.md)
- [ListNotificationResponse](docs/ListNotificationResponse.md)
- [ListOutgoingCallerIdResponse](docs/ListOutgoingCallerIdResponse.md)
- [ListParticipantResponse](docs/ListParticipantResponse.md)
- [ListQueueResponse](docs/ListQueueResponse.md)
- [ListRecordingAddOnResultPayloadResponse](docs/ListRecordingAddOnResultPayloadResponse.md)
- [ListRecordingAddOnResultResponse](docs/ListRecordingAddOnResultResponse.md)
- [ListRecordingResponse](docs/ListRecordingResponse.md)
- [ListRecordingTranscriptionResponse](docs/ListRecordingTranscriptionResponse.md)
- [ListShortCodeResponse](docs/ListShortCodeResponse.md)
- [ListSigningKeyResponse](docs/ListSigningKeyResponse.md)
- [ListSipAuthCallsCredentialListMappingResponse](docs/ListSipAuthCallsCredentialListMappingResponse.md)
- [ListSipAuthCallsIpAccessControlListMappingResponse](docs/ListSipAuthCallsIpAccessControlListMappingResponse.md)
- [ListSipAuthRegistrationsCredentialListMappingResponse](docs/ListSipAuthRegistrationsCredentialListMappingResponse.md)
- [ListSipCredentialListMappingResponse](docs/ListSipCredentialListMappingResponse.md)
- [ListSipCredentialListResponse](docs/ListSipCredentialListResponse.md)
- [ListSipCredentialResponse](docs/ListSipCredentialResponse.md)
- [ListSipDomainResponse](docs/ListSipDomainResponse.md)
- [ListSipIpAccessControlListMappingResponse](docs/ListSipIpAccessControlListMappingResponse.md)
- [ListSipIpAccessControlListResponse](docs/ListSipIpAccessControlListResponse.md)
- [ListSipIpAddressResponse](docs/ListSipIpAddressResponse.md)
- [ListTranscriptionResponse](docs/ListTranscriptionResponse.md)
- [ListUsageRecordAllTimeResponse](docs/ListUsageRecordAllTimeResponse.md)
- [ListUsageRecordDailyResponse](docs/ListUsageRecordDailyResponse.md)
- [ListUsageRecordLastMonthResponse](docs/ListUsageRecordLastMonthResponse.md)
- [ListUsageRecordMonthlyResponse](docs/ListUsageRecordMonthlyResponse.md)
- [ListUsageRecordResponse](docs/ListUsageRecordResponse.md)
- [ListUsageRecordThisMonthResponse](docs/ListUsageRecordThisMonthResponse.md)
- [ListUsageRecordTodayResponse](docs/ListUsageRecordTodayResponse.md)
- [ListUsageRecordYearlyResponse](docs/ListUsageRecordYearlyResponse.md)
- [ListUsageRecordYesterdayResponse](docs/ListUsageRecordYesterdayResponse.md)
- [ListUsageTriggerResponse](docs/ListUsageTriggerResponse.md)
- [MessageEnumDirection](docs/MessageEnumDirection.md)
- [MessageEnumStatus](docs/MessageEnumStatus.md)
- [MessageEnumTrafficType](docs/MessageEnumTrafficType.md)
- [MessageFeedbackEnumOutcome](docs/MessageFeedbackEnumOutcome.md)
- [ParticipantEnumStatus](docs/ParticipantEnumStatus.md)
- [RecordingAddOnResultEnumStatus](docs/RecordingAddOnResultEnumStatus.md)
- [RecordingEnumSource](docs/RecordingEnumSource.md)
- [RecordingEnumStatus](docs/RecordingEnumStatus.md)
- [RecordingTranscriptionEnumStatus](docs/RecordingTranscriptionEnumStatus.md)
- [SiprecEnumStatus](docs/SiprecEnumStatus.md)
- [StreamEnumStatus](docs/StreamEnumStatus.md)
- [TranscriptionEnumStatus](docs/TranscriptionEnumStatus.md)
- [UsageRecordAllTimeEnumCategory](docs/UsageRecordAllTimeEnumCategory.md)
- [UsageRecordDailyEnumCategory](docs/UsageRecordDailyEnumCategory.md)
- [UsageRecordEnumCategory](docs/UsageRecordEnumCategory.md)
- [UsageRecordLastMonthEnumCategory](docs/UsageRecordLastMonthEnumCategory.md)
- [UsageRecordMonthlyEnumCategory](docs/UsageRecordMonthlyEnumCategory.md)
- [UsageRecordThisMonthEnumCategory](docs/UsageRecordThisMonthEnumCategory.md)
- [UsageRecordTodayEnumCategory](docs/UsageRecordTodayEnumCategory.md)
- [UsageRecordYearlyEnumCategory](docs/UsageRecordYearlyEnumCategory.md)
- [UsageRecordYesterdayEnumCategory](docs/UsageRecordYesterdayEnumCategory.md)
- [UsageTriggerEnumRecurring](docs/UsageTriggerEnumRecurring.md)
- [UsageTriggerEnumTriggerField](docs/UsageTriggerEnumTriggerField.md)
- [UsageTriggerEnumUsageCategory](docs/UsageTriggerEnumUsageCategory.md)

To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author

ldeconantsesznak@gmail.com
