# Direct Debit AccountCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/api-reference/direct-debit-account

---

Direct Debit Account
Copy

Operations

POST
/v1/profiles/{{profileId}}/direct-debit-accounts
GET
/v1/profiles/{{profileId}}/direct-debit-accounts?type={{type}}&currency={{currencyCode}}
The Direct Debit Account resource

A direct debit account represents details of your external bank account. It is used for funding batches via direct debit.

Properties
idnumber

Direct debit account ID

currencytext

Currency of a bank account. 3 character currency code. Should be "USD" for ACH direct debits.

typetext

Payment type for which account is used

detailsobject

Currency specific fields

details.routingNumbertext

A valid ABA routing number. For CAD direct debits, the routing number is a combination of an institution number + transit number.

details.accountNumbertext

A valid bank account number

details.accountTypetext

Bank account type

Bank Account Type
Name	Description	Currencies
CHECKING	Checking account	USD, CAD
SAVINGS	Savings account	USD, CAD
Direct Debit Account Payment Type
Name	Description	Currencies
ACH	US Direct debit	USD
EFT	CA Direct debit	CAD
Direct Debit Account Object
{
  "id": 1,
  "currency": "USD",
  "type": "ACH",
  "details": {
    "routingNumber": "000000000",
    "accountNumber": "0000000000",
    "accountType": "CHECKING"
  }
}
Create a direct debit account

POST /v1/profiles/{{profileId}}/direct-debit-accounts

Create a new direct debit account.

Request
currencytext

Currency of a bank account. 3 character currency code. Should be "USD" for ACH direct debits.

typetext

"ACH" for USD direct debits; "EFT" for CAD direct debits.

detailsobject

Currency specific fields

details.routingNumbertext

A valid ABA routing number. For CAD direct debits, the routing number is a combination of an institution number + transit number.

details.accountNumbertext

A valid bank account number

details.accountTypetext

Bank account type (Either CHECKING or SAVINGS)

Response

Returns a direct debit account object.

Example Request
curl -X POST \
  https://api.wise-sandbox.com/v1/profiles/{{profileId}}/direct-debit-accounts \
  -H 'Authorization: Bearer <your api token>' \
  -H 'Content-Type: application/json' \
  -d '{
    "currency": "USD",
    "type": "ACH",
    "details": {
      "routingNumber": "000000000",
      "accountNumber": "0000000000",
      "accountType": "CHECKING"
    }
  }'
Retrieve all direct debit accounts

GET /v1/profiles/{{profileId}}/direct-debit-accounts?type={{type}}&currency={{currencyCode}}

Get a list of your direct debit accounts. Use "type" and "currency" query parameters to filter accounts by type and currency.

Request
typetext

Required. A payment type for which account is used.

currencytext

Required. Currency of a bank account. 3 character currency code. Should be "USD" for ACH direct debits; "CAD" for EFT direct debits.

Response

Returns a direct debit account object.

Example Request
curl -X GET \
  https://api.wise-sandbox.com/v1/profiles/{{profileId}}/direct-debit-accounts?type={{type}}&currency={{currencyCode}} \
  -H 'Authorization: Bearer <your api token>'