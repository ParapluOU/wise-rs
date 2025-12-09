# Balance LimitsCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/accounts/balance-limits

---

Balance Limits
Copy

Certain countries have regulatory hold limits, please speak to the implementation manager to know if your integration applies.

Singapore

Malaysia

Japan

To mitigate any risks that the hold limits are breached, we have 2 APIs that should be integrated.

Check the deposit limit - Use the balance capacity API to check the deposit limits of a balance before making a top up to a balance. Only top up the balance by the deposit limit amount or less.

Set an excess money account - Set an excess money account to the profile so that any excess money (money that exceeds the limit) will be transferred out of the Wise balance at the end of the day.

Retrieve the deposit limits of an account

GET /v1/profiles/{{profileId}}/balance-capacity

This endpoint allows a user to check how much money they are able to deposit in their balance based on regulatory limits.

It is useful for personal profiles located in countries that have hold limits.

We advise to call this API before depositing money into an account if the profile is located in Singapore or Malaysia.

Request
currencytext

Currency code (ISO 4217 Alphabetic Code) - The deposit limit will be returned in this currency

Response
hasLimitboolean

True if there is a regulatory hold limit for that profile's country

depositLimit.amountdecimal

Amount of money that can be added to the account

depositLimit.currencytext

Currency code (ISO 4217 Alphabetic Code) of money that can be added to the account. Specified in the request

Example Request
curl -X GET \
  https://api.wise-sandbox.com/v1/profiles/{{profileId}}/balance-capacity?currency=SGD \
  -H 'Authorization: Bearer <your api token>'
Example Response
{
  "hasLimit": true,
  "depositLimit": {
    "amount": 2000.00,
    "currency": "SGD"
  }
}
Add an excess money account to a profile

POST /v1/profiles/{profileId}/excess-money-account

If a balance goes over the regulatory hold limit, we need to automatically move the excess amount to another account at the end of the day.

To do this, we require a recipient where the money will be moved to be added to the profile.

This API is primarily used for SG and MY customers.

Request
recipientIdinteger

ID of the recipient for excess money transfers (Recipient)

Response
Response
userProfileIdinteger

ID of the profile

recipientIdinteger

ID of the recipient for excess money transfers

Example Request
curl -X POST \
  https://api.wise-sandbox.com/v1/profiles/{{profileId}}/excess-money-account \
  -H 'Authorization: Bearer <your api token> \'
  -d '{
    "recipientId": 148393305
  }'
Example Response
{
  "userProfileId": 12321323,
  "recipientId": 148393305
}