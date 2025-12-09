# Refreshing AccessCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/kyc/partner-kyc/refreshing-access

---

Refreshing Access
Copy
User Tokens resource
Fields
access_tokentext

Access token to be used when calling API endpoints on behalf of user. Valid for 12 hours.

token_typetext

"bearer"

refresh_tokentext

Refresh token which you need to use in order to request new access_token. The lifetime of refresh tokens is 20 years.

expires_ininteger

Access Token expiry time in seconds

expires_attext

Access Token expiration timestamp (UTC)

refresh_token_expires_ininteger

Refresh Token expiry time in seconds

refresh_token_expires_attext

Refresh Token expiration timestamp (UTC)

scopetext

"transfers"

created_atdatetime

Creation time in ISO 8601 format

User Tokens Object
{
  "access_token": "01234567-89ab-cdef-0123-456789abcdef",
  "token_type": "bearer",
  "refresh_token": "01234567-89ab-cdef-0123-456789abcdef",
  "expires_in": 43199,
  "expires_at": "2025-04-11T03:43:28.148Z",
  "refresh_token_expires_in": 628639555,
  "refresh_token_expires_at": "2045-03-12T13:49:23.552Z",
  "scope": "transfers",
  "created_at": "2020-01-01T12:33:33.12345Z"
}

Access tokens are designed to expire after a shot period of time, representing a login session to the Wise Platform API. This increases security of the user's account in case the token is leaked.

In order to maintain an uninterrupted connection, you can request a new access token whenever the previous one is close to expiring. There is no need to wait for the actual expiration to happen first.

It's important to note that using a refresh_token to generate a new access_token automatically invalidates the previously granted access_token. Therefore, it is important to manage the access tokens carefully and to use central storage if using a stateless system.

Retrieve user tokens with refresh token

POST /oauth/token

Access tokens are valid for 12 hours, so upon expiry you need to use the refresh token to generate a new access token.

In order to maintain an uninterrupted connection, you can request a new access token whenever it’s close to expiring. There is no need to wait for the actual expiration to happen first.

Depending on how your application uses the Wise Platform API, you may find that requesting a new access token before attempting a series of API calls on behalf of an individual user will avoid issues with expired access tokens.

Request
grant_typetext

"refresh_token"

refresh_tokentext

User's refresh token obtained from creating or linking to a Wise user.

Response

Returns a user tokens object

Example Request
curl \
  https://api.wise-sandbox.com/oauth/token \
  -u '<client id>:<client secret>' \
  -d 'grant_type=refresh_token' \
  -d 'refresh_token=<user refresh token>'
Token Expiry

It is also possible that a user's refresh token will become invalid. This could happen for a number of reasons, for example:

The refresh token's validity period expires (usually set at three months or more)
The user revokes the access of your application to their account.
The user enables enhanced security on their Wise account.
Wise revoke a token due to a suspected security breach of the token or your client secret.

Due to this possibility your application should handle the scenario where you fail to generate a new access token from the refresh token. Correctly handling this depends on how you originally gained access to the user.

1. An existing user granted your application access to the account

If you were granted access by an existing user then you should send the user through the same flow as you initially did to generate tokens described in linking to an existing Wise account. You will then have new access and refresh tokens generated which you can now store and use as before.

2. Your application created the user

In the case you created the user using the user creation over API flow then the mechanism for regenerating tokens is dependent on whether the user you created has "reclaimed" their Wise account by using our website or apps directly.

If the user has NOT reclaimed their account then the original registration_code you generated should still be able to generate new tokens for the user. Because of this you should store this code alongside the created user ID in your database at the point of user generation, encrypted for security purposes.

If the user has reclaimed their account, the previously stored token fails with an error code 400 and error:

{
  "error": "invalid_grant",
  "error_description": "Invalid user credentials."
}

then push them through the linking to an existing Wise account flow.