# JSON Web Signature (JWS)Copy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/developer/auth-and-security/jose-jws

---

JSON Web Signature (JWS)
Copy

Learn how JOSE JWS works with Wise Platform.

The signing is completed via a JWS mechanism, supported by JOSE. Generally, this means:

The client creates a key pair and delivers the public key to Wise (please contact your implementation manager for details). The format, type, and details required are included below.
Wise installs the public key to be consumed by our API. Multiple keys can be installed to enable rotation.
Once installed, the partner can now call the funding endpoint with signed request body.
Wise will verify the signature using the public key provided runs other checks including the validity of the URL.
Wise will return the response encoded and signed, the partner’s system must accept the media type provided (see details below).
The partner verifies and decodes the header and payload from the returned JWS
Providing a public key

Contact your implementation manager about submitting your public key details to Wise.

When delivering the public key, we require the following information:

Key Material
Algorithm used
KeyID: UUID (Optional)
Expiration Timestamp (Optional)
Deletion Timestamp (Optional)
Activation Timestamp (Optional)

Expiration, deletion, and activation timestamps are all optional. If not included, once installed, the key will remain active until Wise is instructed to remove it.

The KeyID, denoted in the JWS header as kid is an indicator of which key to use to validate the signature. This should be supplied if the caller plans to have multiple keys active at once, either for multiple services or rotation.

Using JWS Endpoints Workflow

Here is a step-by-step workflow with example commands to manually sign a request.

Wise uses a standard implementation of the JOSE JWS standard. For more information on constructing, signing, and validating JWS, please see [JSON Web Signature (JWS)](https://openid.net/specs/draft-jones-json-web-signature-04.html).
1. Create your request header & body

Create a request body as-per our usual API reference, and sign it with the request URL in the header block (see example).

It’s very important that the url be included since we use this to verify the request path has not been tampered with. Both the headers and payload are signed and verified against the actual call.
Header To be Signed
{
  "alg": "ES512",
  "typ": "JWT",
  "kid": "663a0e44-aa4a-4ff0-a9f8-cd99f5fbad71",
  "url": "/v3/profiles/12345/transfers/12345/payments"
}
Payload To Be Signed
{
  "type": "BALANCE"
}
2. Sign with your private key

The header and body should then be base64url encoded and signed, with the signature appended as per the JOSE standard for JWS. It is recommended that an established library be used to complete this process to avoid difficulties and errors in the encoding and signing process.

The example here uses the header and body as shown above and signs it with a private key. The three parts of this JWS are:

Base64(base64url) Encoded Header
Base64(base64url) Encoded Body
Signed header + Body
Signed API Example
ewogICJhbGciOiAiRVM1MTIiLAogICJ0eXAiOiAiSldUIiwKICAia2lkIjogIjY2M2EwZTQ0LWFhNGEtNGZmMC1hOWY4LWNkOTlmNWZiYWQ3MSIsCiAgInVybCI6ICIvdjMvcHJvZmlsZXMvMTIzNDUvdHJhbnNmZXJzLzEyMzQ1L3BheW1lbnRzIgp9
.ewogICJ0eXBlIjogIkJBTEFOQ0UiCn0=
.Z-B7ScaZ37U_0CKrv03LTi0O0GWR2Hm5shzoEj8xRmM
3. Submit an authenticated API request

The request must include the following headers (all are required):

Content-Type: application/jose+json
Accept: application/jose+json
X-TW-JOSE-Method: jws
Complete API request
curl -X POST 'https://api.transferwise.com/v3/profiles/12345/transfers/12345/payments' \
     -H 'Authorization: Bearer XXXXXXX' \
     -H 'Content-Type: application/jose+json' \
     -H 'Accept: application/jose+json' \
     -H 'X-TW-JOSE-Method: jws' \
     -H 'Accept-Encoding: identity' \
     -d 'ewogICJhbGciOiAiRVM1MTIiLAogICJ0eXAiOiAiSldUIiwKICAia2lkIjogIjY2M2EwZTQ0LWFhNGEtNGZmMC1hOWY4LWNkOTlmNWZiYWQ3MSIsCiAgInVybCI6ICIvdjMvcHJvZmlsZXMvMTIzNDUvdHJhbnNmZXJzLzEyMzQ1L3BheW1lbnRzIgp9.ewogICJ0eXBlIjogIkJBTEFOQ0UiCn0=.Z-B7ScaZ37U_0CKrv03LTi0O0GWR2Hm5shzoEj8xRmM'
4. Verify the response

Verify the response of the API call by fetching the latest Wise public key and checking the signature. See Get Wise JWT public key

The response is a fully formed JWS with a Base64 encoded JSON body.

Request
  $ curl -i -X GET 'https://api.wise-sandbox.com/v1/auth/jwt-public-key?algorithm=ES512&scope=PAYLOAD_SIGNING' \
         -H 'Content-Type: application/json' \
         -H 'Authorization: Bearer <your api token>'