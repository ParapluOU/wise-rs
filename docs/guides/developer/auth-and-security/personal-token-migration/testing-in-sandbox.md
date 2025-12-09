# Testing in SandboxCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/developer/auth-and-security/personal-token-migration/testing-in-sandbox

---

Testing in Sandbox
Copy
Prerequisites

If you are using Wise OAuth credentials, you can proceed straight to Set up mutual authentication.

You will need to fill out the form your Customer Success Manager sent you to get client credentials. Please also be sure to include User IDs for Sandbox accounts, which you can create if you do not have any. Please contact your CSM if you're not sure how to get client credentials.

In order to create a Sandbox account, please use the Sandbox UI.

Setup Steps
1. Go to Wise authorization page

Construct a sandbox authorization page URL with your client_id and redirect_uri and navigate to that page.

Only redirect URIs configured by Wise should be used. Otherwise, we will not be able to redirect you to the provided URI and you will get an error.
Sandbox authorization page URL

https://wise-sandbox.com/oauth/authorize/?client_id=yourapp&redirect_uri=https://www.yourapp.com

2. Select an account and grant access

Select your business account on Wise authorization page and give your application authorization to connect to Wise.

3. We forward you to your redirect_url

You will be redirected back to your redirect URL with some additional query parameters.

If you get an error at this point, this could be due to an incorrect redirect URI (see step 1).

You're not required to build an actual service to handle that redirect as this flow is only accessed once. But you must be able to obtain the authorization code.
List of query parameters
codestring

Authorization code. You can use it to generate a User Token.

profileIdstring

The profile ID that the Wise user granted you access to.

Wise redirects back to

https://www.yourapp.com/?code=ABCDEF&profileId=30000000

The auth code provided in the parameter is one time use only expires within 30 minutes.
4. Exchange the authorization code for API tokens

You can use the authorization code to obtain user access token and refresh token.

You can also have a look at the example in Postman.

Example Request
curl https://api.wise-sandbox.com/oauth/token \
     -u '<client id>:<client secret>' \
     -d 'grant_type=authorization_code' \
     -d 'client_id=<client id>' \
     -d 'code=<code from redirect uri>' \
     -d 'redirect_uri=https://www.yourapp.com'
Example Response (200 - OK)
{
  "access_token": {access-token},
  "token_type": "bearer",
  "refresh_token": {refresh-token},
  "expires_in": 43199,
  "expires_at": "2025-04-11T03:43:28.148Z",
  "refresh_token_expires_in": 628639555,
  "refresh_token_expires_at": "2045-03-12T13:49:23.552Z",
  "scope": "transfers",
  "created_at": "2023-12-06T18:28:14.206824830Z"
}
5. Set up mutual authentication

Please reference generating a CSR and getting a certificate.

Testing in Sandbox

When successfully linked the integration will be visible on the settings page.

You can now test your business flows in sandbox.

The partner account integration guide contains information and tips for using the user access token for authentication.

Once you've downloaded your mTLS certificate, you will need to use the following URL when making API calls in Sandbox: https://api-mtls.wise-sandbox.com.

Please note that some profile webhook subscriptions are not available as application subscriptions. To see the full list of available options, please refer to our Webhooks & Notifications documentation. If you're currently using personal tokens for any webhooks, you can continue to do so without any changes.

Successfully linked account on the settings page.