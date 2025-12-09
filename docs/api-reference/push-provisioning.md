# Push ProvisioningCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/api-reference/push-provisioning

---

Push Provisioning
Copy

These APIs provide encrypted cardholder information which are needed to implement push provisioning in your own mobile App.

Operations

POST
/v3/spend/profiles/{{profileId}}/cards/{{cardToken}}/payment-tokens
POST
/twcard-data/v1/push-provisioning/encrypted-payload/google-pay
POST
/twcard-data/v1/push-provisioning/encrypted-payload/apple-pay
The Payment Token resource
cardTokentext

Card token

walletNametext

Wallet name, e.g. GOOGLE_PAY or APPLE_PAY

paymentTokenUniqueReferencetext

Unique reference of the payment token

panUniqueReferencetext

Unique reference of the PAN

provisioningStatustext

Payment token status, e.g. ACTIVE, SUSPENDED or INACTIVE

creationTimetext

Time when the payment token is created

modificationTimetext

Time when the payment token was last modified

Table of available provisioning status and descriptions
Code	Description
ACTIVE	The token is active and available for payments
SUSPENDED	The token has been temporarily suspended
INACTIVE	The token is in the active wallet, but requires additional user authentication for use
DEACTIVATED	The status will not be visible, you can safely ignore it
Payment Token Resource
{
  "cardToken": "a3f90c98-1cd1-4488-9050-2e32c696f8fa",
  "walletName": "GOOGLE_PAY",
  "paymentTokenUniqueReference": "DSHRMC0000255389d1106783179b420aa5b5ca09ba2f12b8",
  "panUniqueReference": "FSHRMC000025538959679bc73c9d4fdc8031b20fa91d0e3b",
  "provisioningStatus": "ACTIVE",
  "creationTime": "2025-10-02T21:26:00.377756354Z",
  "modificationTime": "2025-10-02T21:26:00.378182498Z"
}
Retrieve payment tokens for a card

POST /v3/spend/profiles/{{profileId}}/cards/{{cardToken}}/payment-tokens

You have the option to obtain the payment tokens associated with a card, which can assist in determining if it's already linked to a specific wallet. Note that the payment token status associated with the card may not be accurate (for example, after a phone factory reset). Therefore, it's advisable to consistently check the wallet instance for the most up-to-date token status.

Request
walletNamesarray of texts

List of supported wallet types, namely APPLE_PAY and GOOGLE_PAY

Example Request
curl -X POST \
  'https://api.wise-sandbox.com/v3/spend/profiles/{{profile_id}}/cards/{{card_token}}/payment-tokens'
  -H 'Authorization: Bearer <your api token>'
  -H 'Content-Type: application/json'
  -d '{
    "walletNames": ["APPLE_PAY", "GOOGLE_PAY"]
  }'
Response

Returns all the payment tokens except deactivated

Example Response
{
  "paymentTokens": [
    {
      "cardToken": "a3f90c98-1cd1-4488-9050-2e32c696f8fa",
      "walletName": "GOOGLE_PAY",
      "paymentTokenUniqueReference": "DSHRMC0000255389d1106783179b420aa5b5ca09ba2f12b8",
      "panUniqueReference": "FSHRMC000025538959679bc73c9d4fdc8031b20fa91d0e3b",
      "provisioningStatus": "ACTIVE",
      "creationTime": "2025-02-06T15:34:23.877041000Z",
      "modificationTime": "2025-02-06T15:34:23.877343000Z"
    },
    {
      "cardToken": "a3f90c98-1cd1-4488-9050-2e32c696f8fa",
      "walletName": "GOOGLE_PAY",
      "paymentTokenUniqueReference": "DSHRMC0000255389e2f7133754ab4e2f87fc323730fdfd8e",
      "panUniqueReference": "",
      "provisioningStatus": "INACTIVE",
      "creationTime": "2025-06-25T22:05:47.116016000Z",
      "modificationTime": "2025-06-25T22:05:47.116077000Z"
    },
    {
      "cardToken": "a3f90c98-1cd1-4488-9050-2e32c696f8fa",
      "walletName": "APPLE_PAY",
      "paymentTokenUniqueReference": "DAPLMC000025538967ea1d7972414a93a6544a07dae27a8c",
      "panUniqueReference": "FAPLMC0000255389d7df244df7244803aa2c4ce2a29c242a",
      "provisioningStatus": "ACTIVE",
      "creationTime": "2025-08-19T21:28:28.434472000Z",
      "modificationTime": "2025-08-19T21:28:28.434738000Z"
    },
    {
      "cardToken": "a3f90c98-1cd1-4488-9050-2e32c696f8fa",
      "walletName": "APPLE_PAY",
      "paymentTokenUniqueReference": "DAPLMC0000255389c25cd65bf8084c168dbc4aa59708841a",
      "panUniqueReference": "FAPLMC0000255389d7df244df7244803aa2c4ce2a29c242a",
      "provisioningStatus": "SUSPENDED",
      "creationTime": "2025-08-04T01:37:30.658315000Z",
      "modificationTime": "2025-08-04T01:37:30.658552000Z"
    }
  ]
}
Retrieve encrypted cardholder information for Google Pay

POST /twcard-data/v1/push-provisioning/encrypted-payload/google-pay

This API is not available for sandbox testing.

Request
clientDeviceIdtext

Stable device identification set by Wallet Provider. Could be computer identifier or ID tied to hardware such as TEE_ID or SE_ID. This field must match the clientDeviceId wallet provider will send in token provision request

clientWalletAccountIdtext

Client-provided consumer ID that identifies the Wallet Account Holder entity

Example Request
curl -X POST \
  'https://twcard.wise.com/twcard-data/v1/push-provisioning/encrypted-payload/google-pay'
  -H 'Authorization: Bearer <your api token>'
  -H 'x-tw-twcard-card-token: <your card token>'
  -H 'Content-Type: application/json'
  -d '{
    "clientDeviceId": "ed6abb56323ba656521ac476",
    "clientWalletAccountId: "walletid"
  }'
Response

Returns encrypted cardholder information and other metadata

opaquePaymentCardtext

Encrypted authentication and activation data following card scheme and wallet provider specifications. The response is encoded in Base64.

cardNetworktext

CARD_NETWORK_VISA or CARD_NETWORK_MASTERCARD

tokenServiceProvidertext

TOKEN_PROVIDER_VISA or TOKEN_PROVIDER_MASTERCARD

cardDisplayNametext

Default card name that will be displayed in wallet

cardLastDigitstext

Last 4 digits of the card PAN

userAddresstext

Entire address and phone number associated with the card

Example Response
{
  "opaquePaymentCard":"eyJraWQiOiIwQk...",                    
  "cardNetwork":"CARD_NETWORK_VISA",            
  "tokenServiceProvider": "TOKEN_PROVIDER_VISA",
  "cardDisplayName":"Wise Card",
  "cardLastDigits":"1234",
  "userAddress": {
    "addressLine1":"56 Shoreditch High St",
    "addressLine2": "The Tea Bldg",
    "countryCode":"GB",
    "locality":"London",
    "administrativeArea":"",
    "name":"John Smith",
    "phoneNumber":"+441234567890",
    "postalCode":"E1 6JJ"
  }
}
Retrieve encrypted cardholder information for Apple Pay

POST /twcard-data/v1/push-provisioning/encrypted-payload/apple-pay

This API is not available for sandbox testing.

Request
certificatesarray of texts

DER encoded X.509 ECC leaf and sub CA certificate, the value is then encoded in Base64.

noncetext

One time use nonce generated by Apple Servers and HEX encoded on iOS app

nonceSignaturetext

The device and account specific signature of the nonce generated by Apple and HEX encoded on iOS app

Example Request
curl -X POST \
  'https://twcard.wise.com/twcard-data/v1/push-provisioning/encrypted-payload/apple-pay'
  -H 'Authorization: Bearer <your api token>'
  -H 'x-tw-twcard-card-token: <your card token>'
  -H 'Content-Type: application/json'
  -d '{
    "certificates": [ 
      <DER encoded X.509 ECC leaf certificate and then encoded in Base64>,
      <DER encoded X.509 ECC sub CA certificate and then encoded in Base64>
    ],
    "nonce": "HEX encoded nonce",
    "nonceSignature": "HEX encoded nonce signature"
  }'
Response

Returns encrypted cardholder information and other metadata

encryptedPassDatatext

Encrypted authentication data following card scheme and wallet provider specifications. The response is encoded in Base64.

activationDatatext

Encrypted activation data following card scheme and wallet provider specifications. The response is encoded in Base64.

ephemeralKeytext

Ephemeral key used to encrypt authentication data. The response is encoded in Base64.

Example Response
{
  "encryptedPassData": "443232323637393045DDE321469537FE461E824AA55BA67BF645454330A32433610DE1D1461475BEB6D815F31764DDC20298BD779FBE37EE5AB3CBDA9F9825E1",
  "activationData": "KDlTthhZTGufMY…….xPSUrfmqCHXaI9wOGY=",
  "ephemeralKey": "A1B2C3D4E5F6112233445566"
}