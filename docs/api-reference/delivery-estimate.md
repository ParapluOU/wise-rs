# Delivery EstimateCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/api-reference/delivery-estimate

---

Delivery Estimate
Copy

Operations

GET
/v1/delivery-estimates/{{transferId}}
Get the delivery estimate for a transfer

GET /v1/delivery-estimates/{{transferId}}?timezone=Asia/Singapore

Get the live delivery estimate for a transfer by the transfer ID.

The delivery estimate is the time at which we currently expect the transfer to arrive in the beneficiary's bank account.

This is not a guaranteed time but we are working hard to make these estimates as accurate as possible.

Request
timezonetext

Timezone ID for the formatted text. Example: UTC, Asia/Singapore. Defaults to UTC if not provided.

Response
estimatedDeliveryDatetimestamp

Estimated time when funds will arrive to recipient's bank account

formattedEstimatedDeliveryDatetext

A string to display to users for the estimated delivery date.

Example Request
curl -X GET \
  https://api.wise-sandbox.com/v1/delivery-estimates/{{transferId}}?timezone=Asia/Singapore \
  -H 'Authorization: Bearer <your api token>' 
Example Response
{
  "estimatedDeliveryDate" : "2018-01-10T12:15:00.000+0000",
  "formattedEstimatedDeliveryDate" : "in seconds"
}