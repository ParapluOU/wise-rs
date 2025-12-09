# Balance AccountsCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/accounts/balance-accounts

---

Balance Accounts
Copy

Balance accounts are the accounts that sit inside the multi-currency account and hold the currency balance, ledger of transactions, and other details specifically for that account. For the purpose of this guide, only STANDARD balances will be included/discussed. Additional types, such as jars (type = SAVINGS) will be ignored.

The balance account endpoints are provided to create, view and manage the balance accounts for a particular profile and a particular multi-currency account. They also are what is used to create a multi-currency account when one does not exist for a profile.

Via the Multi-Currency Account Management section, you should know the profileId you want to operate on, what balance account currencies you want to open, and should have created the multi-currency account and first balance account for the profile. The next steps are to:

Create a balance account - Create the first or subsequent balance accounts in a different currencies.
List all balance accounts - List the balance accounts currently included in the multi-currency account for this profile.
Get balance account by ID - Get the details of a balance account by its ID.
Delete balance account - Remove a balance account from a multi-currency account.
Get balance account statement - Get a balance account statement for a specified time range.
Prerequisites & Notes
Investment State

Balance accounts include a field of investmentState. This can include multiple different statuses, however the one to watch for is NOT_INVESTED. If a balance has any other value, then the balance should not and currently cannot be operated on via the API.

If a balance is invested, it should still be shown, however it should be shown as an un-operable balance in your integration. This should only be the case for user accounts that are linked and already have balances, but should be taken into account when building your integration.

Create a balance account

POST /v4/profiles/{{profileId}}/balances

This endpoint opens a balance within the specified profile, in the currency and type specified in the request.

For STANDARD balances, only one can be created for a currency. For SAVINGS balances, multiples in the same currency can be opened.

When sending the request, the currency and type are required. If creating a SAVINGS type balance, a name is also required.

Request
currencytext

Currency code (ISO 4217 Alphabetic Code)

typetext

Type (STANDARD, SAVINGS)

nametext

Name of the balance

Response

Returns a balance object.

Example Request
curl -X POST \
  https://api.wise-sandbox.com/v4/profiles/{{profileId}}/balances \
  -H 'Authorization: Bearer <your api token>' \
  -H 'Content-Type: application/json' \
  -H 'X-idempotence-uuid: <generated uuid>' \
  -d '{
    "currency": "EUR",
    "type": "STANDARD"
  }'
List balances for a profile

GET /v4/profiles/{{profileId}}/balances?types=STANDARD

Retrieve the user's multi-currency account balance accounts. It returns all balance accounts the profile has in the types specified.

A parameter of type must be passed and include at least a single type. To return more than one type, comma separate the values. Acceptable values are STANDARD and SAVINGS.

Returns an array of balance objects.

Example Request
curl -X GET \
  https://api.wise-sandbox.com/v4/profiles/{{profileId}}/balances?types=STANDARD \
  -H 'Authorization: Bearer <your api token>'
Retrieve a balance by ID

GET /v4/profiles/{{profileId}}/balances/{{balanceId}}

This endpoint returns a balance based on the specified balance ID.

Returns a balance object.

Example Request
curl -X GET \
  https://api.wise-sandbox.com/v4/profiles/{{profileId}}/balances/{{balanceId}} \
  -H 'Authorization: Bearer <your api token>'
Remove a balance account

DELETE /v4/profiles/{{profileId}}/balances/{{balanceId}}

Close a balance account for the users profile. Balance accounts must have a zero balance in order for it to be closed. Bank account details for a balance account will also be deactivated and may not be restored in the future.

Returns a balance object.

Example Request
curl -X DELETE \
  https://api.wise-sandbox.com/v4/profiles/{{profileId}}/balances/{{balanceId}} \
  -H 'Authorization: Bearer <your api token>'
Retrieving a balance account statement

GET /v1/profiles/{{profileId}}/balance-statements/{{balanceId}}/statement.json?currency=EUR&intervalStart=2018-03-01T00:00:00.000Z&intervalEnd=2018-03-15T23:59:59.999Z&type=COMPACT

This endpoint allows for statements to be generated for the provided balanceId, with the response in JSON. To generate in CSV, PDF, XLSX, CAMT.053, MT940 or QIF, replace statement.json with statement.csv, statement.pdf, statement.xlsx, statement.xml, statement.mt940 or statement.qif respectively in the above URL. Note that the PDF includes Wise branding.

The period between intervalStart and intervalEnd cannot exceed 469 days (around 1 year 3 months).

This endpoint is SCA protected when it applies. If your profile is registered within the UK and/or EEA, SCA most likely applies to you. The additional authentication is only required once every 90 days, viewing the statement on the website or in the mobile app counts towards that as well.
Learn more

Request
currencytext

Currency of the balance statement requested

intervalStarttimestamp

Statement start time in UTC time

intervalEndtimestamp

Statement end time in UTC time

typetext
COMPACT for a single statement line per transaction
FLAT for accounting statements where transaction fees are on a separate line
statementLocaletext

Language that you wish the statement to be in. Supports 2 character language codes

Response

Returns a balance statement object.

Example Request
curl -X GET \
  https://api.wise-sandbox.com/v1/profiles/{{profileId}}/balance-statements/{{balanceId}}/statement.json
    ?currency=EUR
    &intervalStart=2018-03-01T00:00:00.000Z
    &intervalEnd=2018-03-15T23:59:59.999Z
    &type=COMPACT \
  -H 'Authorization: Bearer <your api token>'