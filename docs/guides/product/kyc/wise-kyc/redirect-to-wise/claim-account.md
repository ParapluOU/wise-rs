# Claiming AccountCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/kyc/wise-kyc/redirect-to-wise/claim-account

---

Claiming Account
Copy
Generate Claim Account Code

Once the customer's account has been set up you can generate a claim account code in order to allow the customer to take control of their account. The claim_account_code is a short-lived token (5 minutes) that allows the customer to claim the account created by the Onboarding API as their own. In case the customer fails to claim the account within the short-lived validity window, a new claim_account_code can be generated.

Redirect to Wise

You will need to redirect the customer to Wise in order to finalize the account setup by confirming that the data submitted by the Onboarding API is accurate, and filling in any missing data where applicable.

List of available parameters
client_idtext

The client ID you received from us.

redirect_uritext

The preconfigured URL in your application where users will be sent after authorization.

state (optional)text

An opaque value, used for security purposes. If this parameter is set in the request, then it is returned to the application as part of the redirect URL.

claim_account_code (optional)text

Short lived security code generated at Step 3. This helps us identify who the connecting user.

On Wise, there are a couple of steps that need to be completed for the journey to complete:

Securing the account with a password and 2FA
Reviewing data uploaded via the Onboarding API
Authorize access to the account

Once a user gives your application authorization to connect to Wise and access their data, they will be redirected back to your redirect_uri with some additional query parameters.

Parameters included in the response
codetext

Authorization code used to complete the authorization code grant-based OAuth 2.0 flow

statetext

Any state parameter you initially passed to us when initiating the flow

profileIdtext

The profile ID that the Wise user granted you access to

The received code parameter should then be used to generate a User Token

Users may skip important steps when attempting to access the customer account. If the user wishes to link into the Wise account again, you will need to ensure an appropriate flow.
The `redirect_uri` has to be preconfigured before it can be used. The value used should be equal to the value preconfigured, including the query parameters.
Example of Wise OAuth page redirect URL in Sandbox

https://wise-sandbox.com/oauth/authorize/?client_id=yourapp&redirect_uri=https://www.yourapp.com&state=f6027a42-344d-4a4d-9f8a-39e42acf9887&claim_account_code=c228bc78-9cb8-11ed-a8fc-0242ac120002

Example of Wise OAuth page redirect URL in Production

https://wise.com/oauth/authorize/?client_id=yourapp&redirect_uri=https://www.yourapp.com&state=f6027a42-344d-4a4d-9f8a-39e42acf9887&claim_account_code=c228bc78-9cb8-11ed-a8fc-0242ac120002

Wise redirects back to

https://www.yourapp.com/?code=ABCDEF&state=f6027a42-344d-4a4d-9f8a-39e42acf9887&profileId=30000000