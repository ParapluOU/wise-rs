# Claim AccountCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/api-reference/claim-account

---

Claim Account
Copy

Operations

POST
/v1/user/claim-account
Generate a claim account code

POST /v1/user/claim-account

The claim_account_code is meant to be used as a request parameter when redirecting a new customer to Wise, effectively allowing the customer to claim the account in question as their own.

Use the registration_code used for creating the user

Request
registrationCodetext

The registration_code belonging to the user

Response
claimAccountCodetext

The claim_account_code to be used in the redirect to Wise

Example Request
curl -X POST \
  https://api.wise-sandbox.com/v1/user/claim-account \
  -H 'Authorization: Bearer <your user token>' \
  -H 'Content-Type: application/json' \
  -d '{
    "registrationCode": <registration code>
  }'
Example Response
{
  "claimAccountCode": "<claim_account_code>"
}