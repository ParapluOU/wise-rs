# Manage disputesCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/issue-cards/card-disputes-management

---

Manage disputes
Copy

These APIs allow you to manage disputes raised via either API or dynamic flow.

The Dispute resource

This primary resource that you will be interacting with when managing your disputes.

idtext

Unique identifier of the dispute

transactionIdnumber

Card transaction ID

profileIdnumber

Profile ID

reasontext

Dispute reason, you can find all the possible values here

statustext

Dispute overall status, it is either ACTIVE or CLOSED

subStatustext

Dispute detailed status

statusMessagetext

Explanation for subStatus

createdAttext

Time when the dispute was created

createdBytext

Creator of the dispute, it is currently set to the user id

lastUpdatedAttext

Time when the dispute was last updated

canWithdrawboolean

Whether the dispute can be withdrawn

Dispute Resource
{
  "id": "b4eae16c-b3a9-4327-b0bd-6a2ad430d803",
  "transactionId": 476873,
  "profileId": 14547572,
  "reason": "WRONG_AMOUNT",
  "status": "CLOSED",
  "subStatus": "WITHDRAWN",
  "statusMessage": "Withdrawn",
  "createdAt": "2024-03-18T03:47:52.493Z",
  "createdBy": "9867661",
  "lastUpdatedAt": "2024-03-27T02:30:27.374Z",
  "canWithdraw": false
}
List disputes for a profile

GET /v3/spend/profiles/{{profileId}}/disputes?status=ACTIVE

Request
status (optional)text

Dispute status, can be either ACTIVE or CLOSED

transactionId (optional)number

Card transaction id

pageSize (optional)integer

The maximum number of disputes to return per page. This number can be between 10 - 100, and will default to 10

pageNumber (optional)integer

The page number to retrieve the next set of disputes. The number has to be greater than 1, and will default to 1

Response

Returns a list of disputes

Example Request
curl -X GET \
  https://api.wise-sandbox.com/v3/spend/profiles/{{profileId}}/disputes?status=ACTIVE \
  -H 'Authorization: Bearer <your api token>'
Example Response
{
  "totalCount": 2,
  "disputes": [
    {
      "id": "51e2dc60-9e4b-4ff5-b917-b90cc5e1ecfb",
      "transactionId": 2040,
      "profileId": 16605997,
      "reason": "NEVER_ARRIVED",
      "status": "ACTIVE",
      "subStatus": "SUBMITTED",
      "statusMessage": "Submitted",
      "createdAt": "2024-03-08T08:30:14.989Z",
      "createdBy": "6097861",
      "lastUpdatedAt": "2024-03-08T08:30:14.989Z",
      "canWithdraw": true
    },
    {
      "id": "9c5ca0cb-00d6-41f7-8214-8cfdb11388a7",
      "transactionId": 3405,
      "profileId": 16605997,
      "reason": "CANCELLED_ORDER",
      "status": "ACTIVE",
      "subStatus": "SUBMITTED",
      "statusMessage": "Submitted",
      "createdAt": "2024-03-27T02:24:16.878Z",
      "createdBy": "6097861",
      "lastUpdatedAt": "2024-03-27T02:24:16.878Z",
      "canWithdraw": true
    }
  ]
}
Retrieve a dispute by ID

GET /v3/spend/profiles/{{profileId}}/disputes/{{disputeId}}

Retrieves a dispute based on the disputeId.

Response

Returns a Dispute.

Example Request
curl -X GET \
  https://api.wise-sandbox.com/v3/spend/profiles/{{profileId}}/disputes/{{disputeId}} \
  -H 'Authorization: Bearer <your api token>'
Example Response
{
    "id": "51e2dc60-9e4b-4ff5-b917-b90cc5e1ecfb",
    "transactionId": 2040,
    "profileId": 16605997,
    "reason": "NEVER_ARRIVED",
    "status": "ACTIVE",
    "subStatus": "SUBMITTED",
    "statusMessage": "Submitted",
    "createdAt": "2024-03-08T08:30:14.989Z",
    "createdBy": "6097861",
    "lastUpdatedAt": "2024-03-08T08:30:14.989Z",
    "canWithdraw": true
}
Withdraw a dispute by ID

You can only withdraw a dispute if canWithdraw is set to true

PUT /v3/spend/profiles/{{profileId}}/disputes/{{disputeId}}/status

Request
statusmoney

The value must be set to CLOSED

Response

Returns a Dispute.

Example Request
curl -X PUT \
  https://api.wise-sandbox.com/v3/spend/profiles/{{profileId}}/disputes/{{disputeId}}/status \
  -H 'Authorization: Bearer <your api token>' \
  -H 'Content-Type: application/json' \
  -d `{
    "status": "CLOSED"
  }`    
Example Response
{
  "id": "51e2dc60-9e4b-4ff5-b917-b90cc5e1ecfb",
  "transactionId": 2040,
  "profileId": 16605997,
  "reason": "NEVER_ARRIVED",
  "status": "CLOSED",
  "subStatus": "WITHDRAWN",
  "statusMessage": "Withdrawn",
  "createdAt": "2024-03-08T08:30:14.989Z",
  "createdBy": "6097861",
  "lastUpdatedAt": "2024-03-27T13:12:28.390Z",
  "canWithdraw": false
}