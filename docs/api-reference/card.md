# CardCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/api-reference/card

---

Card
Copy

These APIs are designed for you to be able to manage your consumer's cards easily, allowing you to set spending limits, modify the status of the cards, or view a list of cards that belong to a specific profile.

Operations

GET
/v3/spend/profiles/{{profileId}}/cards?pageNumber=1&pageSize=10
GET
/v3/spend/profiles/{{profileId}}/cards/{{cardToken}}
PUT
/v3/spend/profiles/{{profileId}}/cards/{{cardToken}}/status
POST
/v3/spend/profiles/{{profileId}}/cards/{{cardToken}}/reset-pin-count
GET
/v3/spend/profiles/{{profileId}}/cards/{{cardToken}}/spending-permissions
PATCH
/v3/spend/profiles/{{profileId}}/cards/{{cardToken}}/spending-permissions
POST
/twcard-data/v1/sensitive-card-data/details
POST
/twcard-data/v1/sensitive-card-data/pin
The Card resource

The primary resource that you will be interacting with when managing your user's cards.

tokentext

Token

profileIdnumber

Profile ID

clientIdtext

Client ID

status.valuetext

Status Text

cardHolderNametext

Name of the card holder

expiryDatetext

Date when the card will expire

lastFourDigitstext

Last 4 digits of the card number

bankIdentificationNumbertext

Bank identification number of the card

phoneNumbertext

Phone number of the card

cardProgram.nametext

Name of the card program

cardProgram.schemetext

Scheme of the card program. E.g. VISA or MASTERCARD

cardProgram.defaultCurrencytext

Default currency of the card. E.g. GBP

cardProgram.cardTypetext

Type of the card. E.g. PHYSICAL or VIRTUAL

unlockSpendingPermissions (deprecated)text

Method with which physical card spending permissions can be unlocked. One of WITH_PARTNER_API, WITH_FIRST_CHIP_AND_PIN_TRANSACTION or NONE. Digital cards will have NONE as the method since there are no applicable physical spending permissions. For more information, refer to this guide

creationTimetext

Time when the card is created

modificationTimetext

Time when the card was last modified

Table of available card status and descriptions
`PARTNER_SUSPENDED` status should not be displayed to end customers
Code	Description
ACTIVE	Card is active and can be used
INACTIVE	Card is inactive and all transactions will be declined
BLOCKED	Card is blocked and cannot be reversed back to any state
FROZEN	Card is “blocked” but temporarily
PARTNER_SUSPENDED	Card is suspended by Wise temporarily due to e.g. fraud reasons.
EXPIRED	Card is expired
PURGED	The cardholder data (e.g. PAN, PIN) has been purged after the retention period (555 days after the card's expiry date)
Card Resource
{
  "token": "ca0c8154-1e14-4464-a1ce-dcea7dc3de52",
  "profileId": 123456,
  "clientId": "wise_api_docs",
  "status": {
    "value": "ACTIVE"
  },
  "cardHolderName": "John Smith",
  "expiryDate": "2028-05-31T00:00:00Z",
  "lastFourDigits": "6320",
  "bankIdentificationNumber": "459661",
  "phoneNumber": "+441234567890",
  "cardProgram": {
    "name": "VISA_DEBIT_BUSINESS_UK_1",
    "scheme": "VISA",
    "defaultCurrency": "GBP",
    "cardType" : "VIRTUAL_NON_UPGRADEABLE"
  },
  "unlockSpendingPermissions": "WITH_PARTNER_API", // Deprecated
  "creationTime": "2022-05-31T01:43:24.596321434Z",
  "modificationTime": "2022-05-31T01:43:24.596321825Z"
}
The Permissions resource

The Permissions resource returns information on the existing permissions that are configured on your user's cards.

typetext

The type of transaction. One of ECOM, POS_CHIP, ATM_WITHDRAWAL, MOBILE_WALLETS, POS_CONTACTLESS or POS_MAGSTRIPE

isEnabledboolean

A flag to indicate if a specific type of permission is enabled

isLockedboolean

A flag to indicate if a specific type of permission is locked. If locked, the permission cannot be enabled

Permissions Resource
{
  "type": "ECOM",
  "isEnabled": false,
  "isLocked": false
}
Retrieve sensitive card details

Wise is a PCI DSS compliant provider, and stores all of your cards' data securely. The scope for PCI compliance depends on your use case and will impact how you integrate with Wise.

Sensitive card details endpoints allow you to retrieve card data such as Primary Account Number, CVV or PIN. You can use it to set a pin if required in the card order flow. For all sensitive card details endpoints, please follow the detailed guide.

Fetch RSA encryption key

This endpoint is used to fetch our RSA public key. This key is required in the sensitive card details flow.

Response
versionnumber

Version of the key

keytext

The RSA public key

Example Request
curl -X GET \
  https://twcard.wise-sandbox.com/twcard-data/v1/clientSideEncryption/fetchEncryptingKey
  -H 'Authorization: Bearer <your api token>'
Example Response
{
    "version": 1,
    "key": "<encryption key>"
}
Get Card Sensitive Details

This endpoint is used to fetch the card Primary Account Number, security code (CVV2), expiry date and cardholder name. It requires an api token and a card token set in the headers.

To retrieve sensitive card details, the card must be in either ACTIVE or FROZEN status. A 403 response will be returned when getting sensitive card details for a card in any other status.

This endpoint is SCA protected when it applies. If your profile is registered within the UK and/or EEA, SCA most likely applies to you. For more information please read more implementing SCA.

Request
keyVersionnumber

The version of the key to use. It is always set to 1.

encryptedPayloadtext

Your JWE payload.

Response
noncetext

An arbitrary UUID issued from the cryptographic communication

cvv2text

Your card CVV2

pantext

Your card Primary Account Number

expiryDatetext

Your card expiry date

cardholderNametext

Name on the card

Example Request
curl -X POST \
  https://twcard.wise-sandbox.com/twcard-data/v1/sensitive-card-data/details 
  -H 'Authorization: Bearer <your api token>' 
  -H 'x-tw-twcard-card-token: <your card token>'
  -d '{
    "keyVersion": 1,
    "encryptedPayload": <your JWE> 
  }'
Example Response
{ 
  "nonce": "33d51227-9ad6-4624-b4b7-7853b56076dd",
  "cvv2": "111",
  "pan": "4396910000012345",
  "expiryDate": "10/31",
  "cardholderName": "John Smith"
}
Get Card PIN

This endpoint is used to fetch the card PIN. It requires an api token and a card token set in the headers.

This endpoint is SCA protected when it applies. If your profile is registered within the UK and/or EEA, SCA most likely applies to you. For more information please read more implementing SCA.

Request
keyVersionnumber

The version of the key to use. It is always set to 1.

encryptedPayloadtext

Your JWE payload.

Response
noncetext

An arbitrary UUID issued from the cryptographic communication

pintext

Your card pin

Example Request
curl -X POST \
  https://twcard.wise-sandbox.com/twcard-data/v1/sensitive-card-data/pin 
  -H 'Authorization: Bearer <your api token>' 
  -H 'x-tw-twcard-card-token: <your card token>'
  -d '{
    "keyVersion": 1,
    "encryptedPayload": <your JWE> 
  }'
Example Response
{ 
  "nonce": "33d51227-9ad6-4624-b4b7-7853b56076dd",
  "pin": "1234",
}
Reset pin count

POST /v3/spend/profiles/{{profileId}}/cards/{{cardToken}}/reset-pin-count

If the wrong PIN have been entered more that 3 times, future transactions on the card will be blocked with PIN_ENTRY_TRIES_EXCEEDED error message.

To unblock your transactions, use this endpoint to reset the PIN count to 0.

Additionally, there are instances where you will also need to reset the PIN count directly at the ATM.

Response

200 - No Content

Example Request
curl -X POST \
  https://api.wise-sandbox.com/v3/spend/profiles/{{profileId}}/cards/{{cardToken}}/reset-pin-count \
  -H 'Authorization: Bearer <your api token>' \
  -H 'Content-Type: application/json' \
Retrieve a list of cards linked to a profile

GET /v3/spend/profiles/{{profileId}}/cards?pageSize=10&pageNumber=1

Returns a list of cards that belong to a specific profileId.

Request
pageSize (optional)integer

The maximum number of cards to return per page. This number can be between 10 - 100, and will default to 10

pageNumber (optional)integer

The page number to retrieve the next set of cards. The number has to be greater than 1, and will default to 1

Response
totalCountinteger

The total number of cards for this profileId

cardslist of cards

A collection of Cards for this profileId

Example Request
curl -X GET \
  https://api.wise-sandbox.com/v3/spend/profiles/{{profileId}}/cards?pageSize=10&pageNumber=1 \
  -H 'Authorization: Bearer <your api token>'
Example Response
{
  "cards": [
    {
      "token": "ca0c8154-1e14-4464-a1ce-dcea7dc3de52",
      "profileId": 123456,
      "clientId": "wise_api_docs",
      "status": {
        "value": "ACTIVE"
      },
      "cardHolderName": "John Smith",
      "expiryDate": "2028-05-31T00:00:00Z",
      "lastFourDigits": "6320",
      "bankIdentificationNumber": "459661",
      "phoneNumber": "+441234567890",
      "cardProgram": {
        "name": "VISA_DEBIT_BUSINESS_UK_1",
        "scheme": "VISA",
        "defaultCurrency": "GBP",
        "cardType": "VIRTUAL_NON_UPGRADEABLE"
      },
      "creationTime": "2022-05-31T01:43:24.596321434Z",
      "modificationTime": "2022-05-31T01:43:24.596321825Z"
    }
  ],
  "totalCount": 1
}
Retrieve details for a card

GET /v3/spend/profiles/{{profileId}}/cards/{{cardToken}}

Retrieves a card based on the cardToken provided.

Response

Returns a Card resource.

Example Request
curl -X GET \
  https://api.wise-sandbox.com/v3/spend/profiles/{{profileId}}/cards/{{cardToken}} \
  -H 'Authorization: Bearer <your api token>'
Example Response
{
  "token": "ca0c8154-1e14-4464-a1ce-dcea7dc3de52",
  "profileId": 123456,
  "clientId": "wise_api_docs",
  "status": {
    "value": "ACTIVE"
  },
  "cardHolderName": "John Smith",
  "expiryDate": "2028-05-31T00:00:00Z",
  "lastFourDigits": "6320",
  "bankIdentificationNumber": "459661",
  "phoneNumber": "+441234567890",
  "cardProgram": {
    "name": "VISA_DEBIT_BUSINESS_UK_1",
    "scheme": "VISA",
    "defaultCurrency": "GBP",
    "cardType" : "VIRTUAL_NON_UPGRADEABLE"
  },
  "creationTime": "2022-05-31T01:43:24.596321434Z",
  "modificationTime": "2022-05-31T01:43:24.596321825Z"
}
Update the status of a card

PUT /v3/spend/profiles/{{profileId}}/cards/{{cardToken}}/status

Update the status of a card based on its cardToken. For cards issued with an INACTIVE card status, the card status can be updated from INACTIVE to ACTIVE to activate the card, which also moves the card order status to COMPLETED.

Request
statustext

The status that you want to update the card to. One of ACTIVE, FROZEN or BLOCKED.

The definition for the status values are:

ACTIVE - the card is active and usable
FROZEN - the card is temporarily frozen resulting in all authorisation requests to be declined
BLOCKED - the card is irreversibly blocked and is no longer usable
Response

Returns a Card object.

Example Request
curl -X PUT \
  https://api.wise-sandbox.com/v3/spend/profiles/{{profileId}}/cards/{{cardToken}}/status \
  -H 'Authorization: Bearer <your api token>' \
  -H 'Content-Type: application/json' \
  -d '{
    "status": "ACTIVE"
  }'
Example Response
{
  "token": "ca0c8154-1e14-4464-a1ce-dcea7dc3de52",
  "profileId": 123456,
  "clientId": "wise_api_docs",
  "status": {
    "value": "ACTIVE"
  },
  "cardHolderName": "John Smith",
  "expiryDate": "2028-05-31T00:00:00Z",
  "lastFourDigits": "6320",
  "bankIdentificationNumber": "459661",
  "phoneNumber": "+441234567890",
  "cardProgram": {
    "name": "VISA_DEBIT_BUSINESS_UK_1",
    "scheme": "VISA",
    "defaultCurrency": "GBP",
    "cardType" : "VIRTUAL_NON_UPGRADEABLE"
  },
  "creationTime": "2022-05-31T01:43:24.596321434Z",
  "modificationTime": "2022-05-31T01:43:24.596321825Z"
}
Retrieve permissions for a card

GET /v3/spend/profiles/{{profileId}}/cards/{{cardToken}}/spending-permissions

Retrieves permissions for a card.

Response

A collection of Permissions

Example Request
curl -X GET \
  https://api.wise-sandbox.com/v3/spend/profiles/{{profileId}}/cards/{{cardToken}}/spending-permissions \
  -H 'Authorization: Bearer <your api token>'
Example Response
{
  "permissions": [
    {
      "type": "ECOM",
      "isEnabled": false,
      "isLocked": false
    },
    {
      "type": "POS_CHIP",
      "isEnabled": true,
      "isLocked": false
    },
    {
      "type": "ATM_WITHDRAWAL",
      "isEnabled": false,
      "isLocked": false
    },
    {
      "type": "MOBILE_WALLETS",
      "isEnabled": true,
      "isLocked": false
    },
    {
      "type": "POS_CONTACTLESS",
      "isEnabled": false,
      "isLocked": true
    },
    {
      "type": "POS_MAGSTRIPE",
      "isEnabled": false,
      "isLocked": true
    }
  ]
}
Modify permissions for a card

PATCH /v3/spend/profiles/{{profileId}}/cards/{{cardToken}}/spending-permissions

Enable or disable permissions on a card.

Request
typetext

One of ECOM, POS_CHIP, POS_MAGSTRIPE,ATM_WITHDRAWAL, POS_CONTACTLESS,MOBILE_WALLETS

isEnabledboolean

A flag indicating if the permissions for a specific type are enabled

Response

Returns a 200 - No Content

Example Request
curl -X PATCH \
  https://api.wise-sandbox.com/v3/spend/profiles/{{profileId}}/cards/{{cardToken}}/spending-permissions \
  -H 'Authorization: Bearer <your api token>'
  -d '{
    "type": <permission type>,
    "isEnabled": <true or false>
  }'