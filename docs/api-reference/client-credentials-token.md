# Client Credentials TokenCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/api-reference/client-credentials-token

---

Client Credentials Token
Copy

Operations

POST
/oauth/token
The Client Credentials Token resource
Fields
access_tokentext

Access token to be used when calling "create user" endpoint. Valid for 12 hours.

token_typetext

Type (bearer)

expires_ininteger

Expiry time in seconds

scopetext

Scope (transfers)

Client Credentials Token Object
{
  "access_token": "ba8k1234-00f2-475a-60d8-6g45377b4062",
  "token_type":"bearer",
  "expires_in": 43199,
  "expires_at": "2025-04-11T03:43:28.148Z",
  "scope":"transfers"
}
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