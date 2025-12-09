# Security & AccessCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/developer/auth-and-security

---

Security & Access
Copy

Learn how Wise Platform handles security.

Wise supports two ways to authenticate to the Wise Platform API.

Use a personal API token if you’re a small business user automating your own Wise account.
Use OAuth 2.0 if you’re a partner building for end customers or a large enterprise
Personal API token (Small Business Users)

A personal token authenticates requests for a single Wise.com user and has limited API access compared to OAuth credentials.

To generate an API token go to Wise.com select your business profile then go to Your Account > Integrations and Tools > API tokens > Add new Token (2‑step login is required).

You can then pass this token with Authorization: Bearer token in your API calls.

Important personal token limitations

Some endpoints and actions aren’t available with personal tokens. EU/UK: Due to PSD2, you cannot fund transfers or view balance statements via API with a personal token.

If in doubt, contact your CSM to confirm the correct auth type.

OAuth 2.0 (Partners & Enterprises)

Wise uses standard OAuth 2.0 protocol for partner authentication and authorisation.

Once our partnership begins, we’ll send you API access credentials for the sandbox environment consisting of a Client ID and a Client Secret. The credentials are needed to either create users over API or complete the authorization_code OAuth 2.0 grant type through which the customer will allow your application access to their account.

Keep your Client Secret Safe

The Client Secret is highly sensitive data that can impersonate your institution on the Wise Platform API. Handle it with utmost care, limit access, and store it securely.

Need a refresher on the protocol? See: https://auth0.com/docs/protocols/oauth2 Not sure which method to use? Contact your CSM.

We also need redirect_url from your technical team which is used to forward users to after successfully granting your application access to their Wise account. Specifying this explicitly makes the integration more secure. This article about OAuth 2.0 framework is a great way to refresh your knowledge about the protocol itself

Enhanced security

Wise takes security as a paramount concern in its APIs, employing technologies like mTLS (mutual TLS), JOSE (JSON Object Signing and Encryption), Strong Customer Authentication and 2FA to further secure communications.

Enhanced Security is only available for selected partners. Please speak with your implementation manager to request access to these features.

Follow the guides below to make your integration with Wise API more secure:

SCA & 2FA

Learn how to set-up Strong customer authentication and two factor authentication.

mTLS

Learn how to set-up mTLS to support stronger security in API calls and webhooks

JOSE JWE

Implementing JWS provides a robust mechanism for creating tamper-proof requests, safeguarding data integrity and authenticity during critical transactional processes.

JOSE JWS

Implementing JWE ensures only the intended recipient can read the JSON request.