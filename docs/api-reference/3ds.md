# 3D Secure AuthenticationCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/api-reference/3ds

---

3D Secure Authentication
Copy

To manage certain aspects of the 3D Secure (3DS) authentication, you will need to integrate with the following APIs.

Operations

POST
/v3/spend/profiles/{{profileId}}/3dsecure/challenge-result
Inform Challenge Result

POST /v3/spend/profiles/{{profileId}}/3dsecure/challenge-result

Once the customer has accepted or rejected the push notification for a 3DS challenge, you can use this endpoint to notify us of the result.

You must call this endpoint before the expiration time, otherwise it will return a 400 error. You can find the expiration information from the webhook event

Only the first call to this endpoint will be processed, any subsequent duplicate requests will be ignored, although you will still receive a success response.

This endpoint is SCA protected when it applies. If your profile is registered within the UK and/or EEA, SCA most likely applies to you. For more information please read more implementing SCA.

Request
transactionReferencetext

Transaction reference as received in the webhook event

challengeStatustext

One of APPROVED or REJECTED

Example Request
curl -X POST \
  https://api.wise-sandbox.com/v3/spend/profiles/{{profile_id}}/3dsecure/challenge-result \
  -H 'Authorization: Bearer <your api token>' \
  -H 'Content-Type: application/json' \
  -d '{
    "transactionReference": "148579538",
    "challengeStatus": "APPROVED"
  }'
Response

204 - No Content