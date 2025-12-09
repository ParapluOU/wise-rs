# Raise dispute via APICopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/issue-cards/card-disputes-api

---

Raise dispute via API
Copy

Using dynamic flow is the preferred way of implementing dispute submission. Alternatively you can submit disputes directly using our APIs.

Do not use the get reasons API for the moment with this approach. Instead get dispute reasons directly from this list then upload any required files and submit the dispute.

Submitting files

POST /v4/spend/profiles/{{profileId}}/dispute-form/file

Submit a file for disputes

Use the fileId in the response for the dispute submission.

NB: A dispute referencing the returned fileId must be submitted no later than two hours after the file submission, otherwise the file will expire and must be re-submitted.

Example Request
curl -X POST \
  'https://api.wise.com/v4/spend/profiles/{{profileId}}/dispute-form/file' \
  -H 'Authorization: Bearer <your api token>' \
  -H 'Content-Type: multipart/form-data'  \
  -F 'receipt=@"<your file>"'
Response
[fileName: text]text

ID of the file to be used on dispute submission.

Response
{
    "receipt" : "ab4f5d2"
}
Submitting disputes

POST /v3/spend/profiles/{{profileId}}/dispute-form/flows/{{scheme}}/{{reason}}

Submit the dispute.

Path Variables
schemetext

The network of the card that was used to make this transaction. One of MASTERCARD or VISA

reasontext

Dispute reason code supplied by the dispute reasons API

Information required for a dispute submission is different per dispute reason. For more information, please use the dropdown and select a dispute reason.

View Request for Dispute Reason:

Select a Dispute Reason
Response

The submit API will return back a response to be used with dynamic flow. If you are using the API without dynamic flow, the response can be ignored.

Example Response
{
  "key": "final",
  "type": "form",
  "title": "Done!",
  "actions": [
    {
      "title": "Continue",
      "exit": true,
      "$id": "continue"
    }
  ],
  "schemas": [],
  "layout": [
    {
      "width": "md",
      "components": [
        {
          "url": "https://wise.com/web-art/assets/illustrations/email-success-large%402x.png",
          "type": "image"
        } 
      ],
      "type": "box"
    },
    {
      "margin": "lg",
      "align": "center",
      "type": "info",
      "markdown": "Thanks for reporting this transaction. It's pre-authorised right now, but as soon as it becomes \"spent\" we'll begin our investigation."
    },
    {
      "type": "button",
      "action": {
        "$ref": "continue"
      }
    }
  ]
}