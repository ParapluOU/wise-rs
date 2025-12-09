# ActivityCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/api-reference/activity

---

Activity
Copy

Activity represents a snapshot of a performed action for a profile.

Operations

GET
/v1/profiles/{{profileId}}/activities
The Activity resource
Parameters
idtext

Activity id

typetext

Type of Activity. Refer here for the full list.

resource.typetext

Type of Activity Monetary Resource. Refer here for the full list.

titletext

Title of the Activity.
Value can be formatted with custom tags to put emphasis on important wordings.
Supported custom tags:


<strong>: Indicating strong emphasis on words that end user should pay attention on
<positive>: Indicating positive transaction (Example: Top up to balance is successful)
<negative>: Indicating negative transaction (Example: Amount is deducted from a balance)
<strikethrough>: (Coming soon) Indicating the negation of an activity (Example: Transfer is cancelled)
descriptiontext

A short description that briefly summarizes the activity.

primaryAmounttext

A currency formatted text that describe the primary amount of transaction.
Value of this field is intended to have units in it and should not be treated as a numeric value.
One example of primaryAmount would be: "Topping up 100 USD balance with 80 GBP"
In this case 100 USD would be the primaryAmount of the activity.


secondaryAmounttext

A currency formatted text that describe the secondary amount of transaction.
Value of this field is intended to have units in it and should not be treated as a numeric value.
Value can be empty if there is no good candidate as secondary amount
One example of secondaryAmount would be: "Topping up 100 USD balance with 80 GBP"
In this case 80 GBP would be the secondaryAmount of the activity.


statustext

Status of the Activity. Possible values are:

REQUIRES_ATTENTION: Requires an end user attention
IN_PROGRESS: Indicate that this activity has yet to be completed. (Example: In progress Top Up)
UPCOMING: Indicate that this activity is scheduled to be happen in the future. By default these activities will only be shown 2 days before the date. (Example: A scheduled transfer)
COMPLETED: Indicate that this activity is at its end state. (Example: A completed Top Up)
CANCELLED: Indicate that this activity is cancelled. (Example: A Top Up is cancelled)
createdOntext

Timestamp when the activity was created

updatedOntext

Timestamp when the activity was last modified

Activity type

The following activity types are available.

ACQUIRING_PAYMENT
AUTO_CONVERSION
BALANCE_ADJUSTMENT
BALANCE_ASSET_FEE
BALANCE_CASHBACK
BALANCE_DEPOSIT
BALANCE_HOLD_FEE
BALANCE_INTEREST
BANK_DETAILS_ORDER
BATCH_TRANSFER
CARD_CASHBACK
CARD_CHECK
CARD_ORDER
CARD_PAYMENT
CASH_WITHDRAWAL
CLAIMABLE_SEND_ORDER
DIRECT_DEBIT_TRANSACTION
EXCESS_REFUND
FEE_REFUND
INCORPORATION_ORDER
INTERBALANCE
PAYMENT_REQUEST
PREFUNDING_TRANSFER
REWARD
SCHEDULED_SEND_ORDER
TRANSFER
Activity Resource Type

The following activity resource types are available.

ACCRUAL_CHARGE
ACQUIRING_PAYMENT
ASSETS_WITHDRAWAL
BALANCE_CASHBACK
BALANCE_INTEREST
BALANCE_TRANSACTION
BANK_DETAILS_ORDER
BATCH_TRANSFER
CARD_CASHBACK
CARD_ORDER
CARD_TRANSACTION
DIRECT_DEBIT_INSTRUCTION
DIRECT_DEBIT_TRANSACTION
FEE_REFUND
INCIDENT_REFUND
INCORPORATION_ORDER
OPERATIONAL_TRANSACTION
PAYMENT_REQUEST
REWARD
REWARDS_REDEMPTION
SEND_ORDER
SEND_ORDER_EXECUTION
TRANSFER
Activity Object
{
  "id": "TU9ORVRBUllfQUNUSVZJVFk6OjE0NTU4OTk4OjpDQVJEX1RSQU5TQUNUSU9OOjozNDMwNDk=",
  "type": "CARD_PAYMENT",
  "resource": {
    "type": "CARD_TRANSACTION",
    "id": "343049"
  },
  "title": "<strong>Test Payment</strong>",
  "description": "",
  "primaryAmount": "150 JPY",
  "secondaryAmount": "1.50 SGD",
  "status": "COMPLETED",
  "createdOn": "2023-01-01T00:00:00.000Z",
  "updatedOn": "2023-01-01T00:00:00.000Z"
}
List activities for a profile

GET /v1/profiles/{{profileId}}/activities

List of activities belonging to user profile. The following parameters are optional:

monetaryResourceType - Filter activity by resource type. Refer here for the full list.
status - Filter by activity status
since - Filter activity list after a certain timestamp
until - Filter activity list until a certain timestamp
nextCursor - Can be used to get next page of activities.
size - Desired size of query. Min 1, max 100, and default value is 10 if not specified
Response

The returned response contains 2 elements:

cursor - A nullable text field that its value can be used as nextCursor in query parameter to get the next page of this query.
activities - Array of activity objects.
Example Request
curl -X GET \
  https://api.wise-sandbox.com/v1/profiles/{{profileId}}/activities \
  -H 'Authorization: Bearer <your api token>'
Example Response
{
  "cursor": "WyJQUklPUklUWSIsMTAwMDAxNjY5NzA4MjI0MDAwMDE2MDk5OV0=",
  "activities": [{
    "id": "TU9ORVRBUllfQUNUSVZJVFk6OjE0NTU4OTk4OjpDQVJEX1RSQU5TQUNUSU9OOjozNDMwNDk=",
    "type": "CARD_PAYMENT",
    "resource": {
      "type": "CARD_TRANSACTION",
      "id": "343049"
    },
    "title": "<strong>Test Payment</strong>",
    "description": "",
    "primaryAmount": "150 JPY",
    "secondaryAmount": "1.50 SGD",
    "status": "COMPLETED",
    "createdOn": "2023-01-01T00:00:00.000Z",
    "updatedOn": "2023-01-01T00:00:00.000Z"
  }]
}