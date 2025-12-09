# Sensitive card detailsCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/issue-cards/sensitive-card-details

---

Sensitive card details
Copy

Securely handle card data with end-to-end encryption

Card details (PAN, CVV, PIN) are sensitive data that are strongly secured by RSA and AES encryption algorithms.

To minimize risks of exposure, we take the stand to set up an end to end encrypted communication between the Client and Wise. The encryption should be performed on the client side to ensure data integrity. The encrypted payload should be proxied to the backend which initiate the API call to Wise with the user token.

We are using the JWE standard to pass encrypted data. A JWE is composed of 5 parts that ensure the payload integrity and authentication checks. Each part is required and need to be base64 encoded.

Details to generate a valid JWE:

Our RSA key is using RSAES-PKCS1-V1_5 encryption algorithm.
The AES key should be of length 256 and use AES-GCM encryption algorithm.
The length of the Initialisation Vector should be of length 12.

Please reach out to our team to get an implementation code example.

Card PIN encryption

This section explains how to set a card PIN for your customer when ordering a card. If you don't need this feature, a PIN will be automatically generated instead.

Please contact Wise to enable this feature.

Fetch the RSA key from Wise's endpoint.
Generate an AES key directly from your client.
Use the AES key to encrypt your pin.
Use the RSA Key to encrypt your AES key.
Build an encrypted JSON (JWE). The payload part encapsulate the AES encrypted PIN (step 3) and the RSA encrypted AES key (step 4).
Proxy the call to your backend. Call /POST set pin with a user token.
Sensitive card data decryption
Fetch the RSA key from Wise's endpoint.
Generate an AES key directly in your client.
Use the RSA Key to encrypt your AES key.
Build an encrypted JSON (JWE) from step 3.
Proxy the call to your backend. Call one of sensitive card details endpoints with a user token.
The response is an JWE. Use your same key generated previously (step 2) to decrypt the response.