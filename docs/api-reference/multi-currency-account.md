# Multi Currency AccountCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/api-reference/multi-currency-account

---

Multi Currency Account
Copy

The Wise multi-currency account (MCA) enables users to hold, convert, and fund transfers (single or batches) with balances in up to 56 currencies. Please note that of the 50+ currency balances that we support, 10+ of these come with local account details.

Please refer to our multi-currency account guide for more information on the general use of the endpoints included below.

Please see the Balances APIs for more details.

Operations

GET
/v2/borderless-accounts-configuration/profiles/{{profileId}}/available-currencies
GET
/v2/borderless-accounts-configuration/profiles/{{profileId}}/payin-currencies
GET
/v4/profiles/{{profileId}}/multi-currency-account
GET
/v4/multi-currency-account/eligibility?profileId={{profileId}}
GET
/v4/multi-currency-account/eligibility?country={{country-ISO-Code}}&state={{state-ISO-code}}
The Multi Currency Account resource
Fields
idinteger

Multi currency account ID

profileIdinteger

Profile ID the multi currency account is attributed to.

recipientIdinteger

Recipient ID of the multi currency account, to be used for transfer recipient

creationTimedatetime

Datetime when multi currency account was created

modificationTimedatetime

Datetime when multi currency account was last modified

activebool

Whether multi currency account is active or not

eligiblebool

Whether multi currency account is eligible or not

Multi Currency Account Object
{
  "id": 1,
  "profileId": 33333333,
  "recipientId": 12345678,
  "creationTime": "2020-05-20T14:43:16.658Z",
  "modificationTime": "2020-05-20T14:43:16.658Z",
  "active": true,
  "eligible": true
}
Retrieve available currencies

Two endpoints exist to retrieve all the currencies available for balance accounts. You can use this list to create a balance account for the currency included.

Available Currencies

GET /v2/borderless-accounts-configuration/profiles/{{profileId}}/available-currencies

Lists all currencies that are available for balance accounts. This includes those that can have funds added from external accounts, as well as those that a balance can be held in.

Payin Currencies

GET /v2/borderless-accounts-configuration/profiles/{{profileId}}/payin-currencies

Lists all the currencies available for balance accounts that are also available to have bank account details. You can use this list to create a balance account for the currency included and then subsequently create bank account details.

Response

Returns a list of currencies supported for that option.

Example Request - Available Currencies
curl -X GET \
  https://api.wise-sandbox.com/v2/borderless-accounts-configuration/profiles/{{profileId}}/available-currencies \
  -H 'Authorization: Bearer <your api token>' 
Example Request - Payin Currencies
curl -X GET \
  https://api.wise-sandbox.com/v2/borderless-accounts-configuration/profiles/{{profileId}}/payin-currencies \
  -H 'Authorization: Bearer <your api token>' 
Example Response
[
  "EUR",
  "GBP",
  "USD",
  ...
]
Retrieve multi currency account for a profile

GET /v4/profiles/{{profileId}}/multi-currency-account

This endpoint returns the multi-currency account details for the specified profileId. If the user does not yet have a multi-currency account, a 404 Not Found will be returned.

Response

Returns a multi currency account object.

Example Request
curl -X GET \
  https://api.wise-sandbox.com/v4/profiles/{{profileId}}/multi-currency-account \
  -H 'Authorization: Bearer <your api token>'
Retrieve multi currency account eligibility

GET /v4/multi-currency-account/eligibility

This endpoint checks eligibility for a multi-currency account for either a specific profile or for a location. Customers in some countries and states/provinces may not be eligible for a multi currency account.

To check a profile, the profileId should be passed as a parameter.

To check a specific location, the country the user is in should be passed as country using 2-letter ISO 3166 codes. If the country is US, a valid 2 letter state parameter must also be passed.

Ex 1: France: /v4/multi-currency-account/eligibility?country=FR

Ex 2: USA, California: /v4/multi-currency-account/eligibility?country=US&state=CA

Response
eligibleboolean

Profile is eligible for MCA Account

eligibilityCode"eligible", "invalid.profile.type", "invalid.country", or "invalid.state"

Reason for the ineligibility

accountType"ineligible", "receive_only" or "full". generally this will be returned as "full".

Account type available

ineligibilityReasonstring

Reason the profile is not eligible

Example Request - Profile Eligibility
curl -X GET \
  https://api.wise-sandbox.com/v4/multi-currency-account/eligibility?profileId={{profileId}} \
  -H 'Authorization: Bearer <your api token>'
Example Request - Country/State Eligibility
curl -X GET \
  https://api.wise-sandbox.com/v4/multi-currency-account/eligibility?country={{country-ISO-Code}}&state={{state-ISO-code}} \
  -H 'Authorization: Bearer <your api token>'
Example Response
{
  "eligible": true,
  "eligibilityCode": "eligible",
  "accountType": "FULL",
  "ineligibilityReason": null
}