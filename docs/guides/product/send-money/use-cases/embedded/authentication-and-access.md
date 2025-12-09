# Authentication & AccessCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/send-money/use-cases/embedded/authentication-and-access

---

Authentication & Access
Copy

You must have first generated Client Credentials and stored them securely. Client credentials consist of a client-id and client-secret. You will need a separate set of credentials for our Sandbox and Production environments.

It is also strict requirement to set up and use mutual TLS (mTLS).

Using these you can retrieve a client credentials token, which grants you time-limited access to the API. You will need to do this prior to any other API activity such as creating profiles and sending money.

Your client credentials token should not be persisted and is valid for a 12 hour session (by default). You'll need to get a new token if it expires.

It is important not to assume the size and format of the access token provided as this may change in future.

Please also refer to the security section in our going live checklist.

Getting Client Credentials and mTLS certificates

To generate your Client Credentials, you must use Developer Hub.

Your Client Credentials are valid indefinitely, but can be rotated or revoked via Developer Hub. Access to Developer Hub is granted by Wise; you will need to speak with your implementation team in order to get access.

The Developer Hub also provides you with the means to upload your mTLS certificate and retrieve the Wise certificate for your truststore.

Retrieve a client credentials token

POST /oauth/token

Obtain access_token based on your API client credentials.

Request

Use Basic Authentication with your api-client-id/api-client-secret as the username/password. The body of the request must be sent as x-www-form-urlencoded.

grant_typetext

"client_credentials"

Response

Returns a client credentials token object

Example Request
curl \
  https://api.wise-sandbox.com/oauth/token \
  -u '<client id>:<client secret>' \
  -d 'grant_type=client_credentials'
Using tokens and managing expiry

We recommend retrieving a separate client credentials token per thread/worker to avoid concurrency issues.

Each worker must manage its own token lifecycle via one of the following strategies:

Ensuring the task is completed within the expiry period of the token.
By tracking the lifetime and retrieving a new client credentials token before the token expires.

Client credentials tokens do not have a corresponding refresh token, the normal process is to fetch a new token via the same API when required.