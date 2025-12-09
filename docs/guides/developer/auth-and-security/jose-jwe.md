# JSON Web Encryption (JWE)Copy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/developer/auth-and-security/jose-jwe

---

JSON Web Encryption (JWE)
Copy

Learn how JOSE JWE works with Wise Platform.

The encryption is completed via a JWE mechanism, supported by JOSE. Generally, this means:

The client creates a key pair and delivers the public key to Wise. Wise will use this public key for response encryption. The format, type, and details required are included below.
The client requests Wise's public key and uses it to encrypt the request.
Once steps (1) and (2) are complete, the client encrypts a request body and makes a call to the Wise API.
Wise will decrypt the encrypted request using its private key and then perform additional checks.
Wise will return the response encrypted with the provided public key. The partner’s system must accept the media type specified (see details below).
The partner decrypts payload from the returned JWE.
Providing a public key

Wise allows to upload public keys via API. See Add a new client public key. To use JWE, the scope should be set to PAYLOAD_ENCRYPTION, and the algorithm should be RSA_OAEP_256.

Using JWE Endpoints Workflow

Here is a step-by-step workflow with the required information for successful request encryption.

Wise uses a standard implementation of the JOSE JWE standard. For more information on constructing, encrypting, and decrypting JWE, please see [JSON Web Encryption (JWE)](rfc-editor.org/rfc/rfc7516).
1. Create request JWE headers

Create a request body as-per our usual API reference (see example).

List of required JWE headers:

Header	RFC
alg	RFC-7516 section-4.1.1
enc	RFC-7516 section-4.1.2

Currently, we only support the RSA-OAEP-256 algorithm and the A256GCM content encryption algorithm. For further support of other algorithms and encryption methods please contact your implementation manager.

JOSE Headers
{
  "alg": "RSA-OAEP-256",
  "enc": "A256GCM"
}
Payload To Be Encrypted
{
  "type": "GENERAL_ENQUIRY",
  "subject": "Inquiry about Transfer 12345",
  "details": {
    "transferId": 58114690,
    "profileId": 14556049,
    "userId": 1234
  },
  "externalId": "partner_external_id_12345",
  "description": "Initial summary of the issue"
}
2. Message Encryption

For instructions on encrypting messages, please refer to RFC-7516 section-5.1.

Wise only supports JWE with Key Encryption employed. A Key Management Mode in which the CEK value is encrypted to the intended recipient using an asymmetric encryption algorithm.

List of supported cryptographic algorithms:

Name	RFC
RSA-OAEP-256	RFC-7518 section-4.1

List of supported content encryption algorithms:

Name	RFC
A256GCM	RFC-7518 section-5.1
Wise does not re-use the request CEK for response encryption; instead, we generate a new CEK for every response.
Encrypted API HTTP Request Body Example
eyJlbmMiOiJBMjU2R0NNIiwiYWxnIjoiUlNBLU9BRVAtMjU2In0
.nzXdYfoI1X-4RVKuPp4U1b7oRGjz8zp5mqU-Otag0GUczxmifX3OqxBqMirfvdZXLdV6GgIyxZxlFeuF9C94cuFaIkA1frZy6bp-gY6dNrXTWebka34SWFwCNL8hfyzDRcWs6mQ0BNL5ALUfA3NO8Xh98yUSk1dnIMSHZvLd0feIP-1rYYDJsacIMnWA9zUjzY-rTsQbrzziGKtCvmRyzJAaCYfiYOAh-UjkmAhc5-xd3XPzqKeF-QaIGd0KTrr_Wucdq9CZsuEjXhKWfU27hacDtKaG_khUOKWfWTwoThRjfO9ckuO2oeB4UGLZ36IFSlMQ48p9s4PgnMg2YDCOrg
.fx2-MHbRsutByyOm
.d8MhWJo5Ih_fvTAmBGjfUMR5wc1V74ZS65Xy7Xx5MEAU8DEPbu1cQBu9U9l4BUZ2iR637C_JSo44EqDE26dDiow6LM-GIM-WWsR5f50A1_7vf89fJR4zfH6hob6g9YC-gMMTJ50x6fhspI1vBmXWvt4uXPmO4HTs06Rl3gs1jXyoA5_3A58IsFV1POMX3DUVPhKEVQwtlUtXgfEdyXXHnO854wWNbi7_pyn2vPABS_CyZy74JJGjDWovONRFZ81OkLGmyb2jQcQxlfetZz5wTbcQ86m-AGcf3nw41y4xXn8hODjGwnYY1BiSvZOt2kOmEGRaMHr2s3kTlhZ6rPJoLjodBkbUBM_W0g
.Sewt5vM_AClXhdCBVNbWyQ
3. Submit an authenticated API request

The Wise public API supports the use of JWE for all HTTP request methods, including those without an expected HTTP request body, such as GET requests. When we receive an HTTP request without a body, our system performs only response payload encryption.

The request must include the following headers (all are required):

Content-Type: application/jose+json
Accept: application/jose+json
X-TW-JOSE-Method: jwe
Complete API POST request
curl -X POST 'https://api.transferwise.com/v1/cases' \
     -H 'Authorization: Bearer XXXXXXX' \
     -H 'Content-Type: application/jose+json' \
     -H 'Accept: application/jose+json' \
     -H 'X-TW-JOSE-Method: jwe' \
     -H 'Accept-Encoding: identity' \
     -d 'eyJlbmMiOiJBMjU2R0NNIiwiYWxnIjoiUlNBLU9BRVAtMjU2IiwidXJsIjoiL3YxL2Nhc2VzIn0.B9g-Z3eJrKFK7eALNjL363ZXHn_us_axnfIJqB4GH8ayvL2QvK8OVlrONI3HGlq80zNzaFGSugzMzptJeQxa3jz92QK3Ks-JVRUH9vEMe99fDEuvvIWHKRA-yuCs1UBRSYjt1NxskX9Vo97xmjzLyy5ysOrz5SJYJj2okKnZ7DZuNNfZ7m6wKqxFJDJ2S_y6uJQXGiv5isip1WGd54qagiK5L_9pACZNBAG9IBErojY8L0xruCjkt1-2jktyDa6t1euER_CSstiYM55Lie2EZKnDKVIJ1zOuOEqMPOFEbwNhZaJyspNiWeu0l87zo7K8_Vu6VCeSaVo0fvH7Fm6Dtg.yenUpQ_KISjo0JMk.nk5h-UKewy4wJsCk2prAK5bmUbc-PQEPHW3YtxUMBk8u5X59bHXOsol5H4foKPeRvUIw8lx-yOLck2pvh7QESUoiWSwVguuKn3w-V3AdhhTXm6Bzij_6wndusILAHThBHlwRrUuU1amG69nNxDsLNv_zmZ1ZSZfwJz01sy6dcKEolD4xH72hPIBYKrNk5ihkiV8DgzTVF-rAcBODTQ10PTd4ZWIDsvTGL49neD9WcLMVpaUwG8NMOARY4w_pr0ikTlcjFUP2vOFhq6I1KwxrRJFRNb60ifY2w-_-2Dmab4kWtwTFrkFCj2T8hUV0vZQcYldxUGCDPnSIPaHk_x9O-y66lMwyyi70ug.AMyL0XyQJ6roHeGAc_Du1Q'
Complete API GET request
curl -X GET 'https://api.transferwise.com/v1/cases/123456789' \
     -H 'Authorization: Bearer XXXXXXX' \
     -H 'Content-Type: application/jose+json' \
     -H 'Accept: application/jose+json' \
     -H 'X-TW-JOSE-Method: jwe' \
     -H 'Accept-Encoding: identity'
4. Decrypt the response
Our system does not apply encryption or modify HTTP headers for empty responses.

Our system is designed to enhance security by encrypting HTTP responses. When a JWE (JSON Web Encryption) is utilized in an HTTP request, we respond with a JWE message. The response encryption algorithm and the content encryption algorithm are matched with those used in the initial HTTP request. Additionally, we include the public key ID, which was used for encrypting the response payload, as part of the kid (Key Identifier) in the JOSE Header.

For instructions on decrypting messages, please refer to RFC-7516 section-5.2

Response JOSE Headers
{
  "alg": "RSA-OAEP-256",
  "enc": "A256GCM",
  "kid": "663a0e44-aa4a-4ff0-a9f8-cd99f5fbad71"
}
4a. Decrypt directly encrypted response

Our system supports end-to-end application-level encryption, which can be used for securing requests between mobile clients and Wise. In this scenario, the response encryption is implicitly handled using direct encryption. Our system employs the original content encryption key, used in the request, to encrypt the response.

For more information about direct encryption, please refer to RFC-7518 section-4.5.

Response JOSE Headers
{
  "enc":"A256GCM",
  "alg":"dir"
}