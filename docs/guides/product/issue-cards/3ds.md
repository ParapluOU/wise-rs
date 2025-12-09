# 3D Secure AuthenticationCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/issue-cards/3ds

---

3D Secure Authentication
Copy

Please contact your Wise sales team to use this feature

3D Secure (3DS) authentication adds an extra layer of protection to online shopping, all Wise issued cards have 3DS enabled by default.

This process can be entirely managed by Wise and requiring no implementation from your side. When 3DS authentication is triggered, Wise sends a one-time password (OTP) to the customer's registered phone number through SMS or voice call. The authentication is completed only after the customer submits the OTP within the specified time frame.

For partners onboarded after 1/3/2025, we will use the registered phone number from the profile, which we will obtained through the create profile endpoint or update profile endpoint. For existing and new partners our support team will need to whitelist your clientId for us to start using the registered phone number from the profile.

We also offer an additional authentication flow by sending a push notification to the customer’s mobile app. For this flow, you need to subscribe to the webhook event and send a push notification using the transaction information provided in the event. Once the customer has approved or rejected the transaction, you must inform us of the result by calling this API.

The diagram below illustrates how push notifications works. If the customer falls back to use OTP as the challenge method, everything will continue to be managed by Wise.

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