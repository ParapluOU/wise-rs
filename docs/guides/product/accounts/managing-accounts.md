# Multi-Currency AccountsCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/accounts/managing-accounts

---

Multi-Currency Accounts
Copy

Multi-currency account management endpoints allow you to manage the user's multi-currency account overall. We have also included information on getting profile information for an authenticated customer.

Generally, the following flow should be taken to open a multi-currency account:

Get profile - Get the profile ID of the profile you want to operate on.
List available currencies - Determine which currencies are supported for the user.
Get multi-currency Account - Get current multi-currency accounts and balance accounts for the profile, determine next steps.
Check eligibility - Check the eligibility of the profile for multi-currency accounts.
Create a multi-currency account - Create a multi-currency account and first balance account for the profile, if not already created.
Retrieve a profile by ID

GET /v2/profiles/{{profileId}}

Get profile info by ID.

Example Request
curl -X GET \
  https://api.wise-sandbox.com/v2/profiles/{{profileId}} \
  -H 'Authorization: Bearer <your api token>'
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
Create a multi-currency account

To create a multi-currency account, you simply need to open a balance in at least one currency. When doing so, a multi-currency account will automatically be created. To move forward, you should create a balance account.