# Bank Account DetailsCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/accounts/bank-account-details

---

Bank Account Details
Copy

Bank account detail endpoints are provided to create, view, and manage bank account details for an account balance.

Via the Multi-Currency Accounts and Balance Accounts sections of this guide, you will know the profile and multi-currency account, as well as the specific balance accounts created. The next optional steps are:

List current bank account details - View a list of all existing bank account details for the multi-currency account.
Request bank account details - Create a request for bank account details to be added to the profile and multi-currency account.
List bank account detail orders - View open requests for bank account details on the profile and multi-currency account.
List bank account detail orders
v3

GET /v3/profiles/{{profileId}}/account-details-orders?currency=GBP

This endpoint returns the bank account assignment requests for a profile and multi-currency account. The response includes bank‑details orders in the following statuses: (PENDING_USER, PENDING_TW, REQUIREMENTS_FULFILLED, DONE)

currencytext

Currency code (ISO 4217 Alphabetic Code)

Example Request
curl -X GET \
  https://api.wise-sandbox.com/v3/profiles/{{profileId}}/account-details-orders?currency=EUR \
  -H 'Authorization: Bearer <your api token>'
Example Response
[
  {
    "status": "DONE",
    "currency": "EUR",
    "requirements": [
      {
        "type": "TOP_UP",
        "status": "DONE",
      },
      {
        "type": "VERIFICATION",
        "status": "DONE",
      }
    ]
  },
  ...
]
Create a bank account details order

POST /v1/profiles/{{profileId}}/account-details-orders

Creates an order which will issue account details. It should use the same currency as the balance previously created. Fulfilling all the requirements will complete the order. That means reaching status DONE.

The possible values for a requirement are:

PENDING_USER: The requirement has some pending action from the user.
PENDING_TW: The requirement has some pending action from Wise.
DONE: The requirement is completed

The more common requirements are:

VERIFICATION: The user needs to be fully verified before complete this requirement.
TOP_UP: A fee will be charged and must be paid using through wise.com before complete this requirement.
Request
currencytext

balance currency (ISO 4217 Alphabetic Code)

Response
statustext

Status (PENDING_USER, PENDING_TW, DONE)

currencytext

currency (ISO 4217 Alphabetic Code)

requirements.typetext

Requirement type (VERIFICATION, TOP_UP)

requirement.statustext

Requirement status (PENDING_USER, PENDING_TW, DONE)

Example Request
curl -X POST \
  https://api.wise-sandbox.com/v1/profiles/{{profileId}}/account-details-orders \
  -H 'Authorization: Bearer <your api token>' \
  -H 'Content-Type: application/json' \
  -d '{
    "currency": "EUR"
  }'
Example Response
{
  "status": "PENDING_USER",
  "currency": "EUR",
  "requirements": [
    {
      "type": "VERIFICATION",
      "status": "PENDING_USER"
    }
  ]
}
Retrieve bank account details for a profile

GET /v1/profiles/{{profileId}}/account-details

This endpoint returns a list with all the AVAILABLE and ACTIVE account details for the given profile, including examples. Account receive options can also include local and international details to receive money on the currency balance.

Example bank account details are returned for any currency where bank account details have not been requested and issued. Examples will always include an id of null.

Response

Returns a list of bank account details objects.

Example Request
curl -X GET \
  https://api.wise-sandbox.com/v1/profiles/{{profileId}}/account-details \
  -H 'Authorization: Bearer <your api token>'