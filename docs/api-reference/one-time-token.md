# One Time TokenCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/api-reference/one-time-token

---

One Time Token
Copy

These endpoints are deprecated. Please refer to strong-customer-authentication section to integrate with SCA.

Represents a list of challenges that a user needs to clear in order to access protected resources.

Commonly used for Strong Customer Authentication & 2FA.

Learn from our guide to understand One Time Token Framework.

To ease reading in this document, we will use OTT as an abbreviation for one time token.

The Verify PIN/FaceMap/Device Fingerprint APIs are currently in closed Beta and subject to change. Please speak with your implementation manager if you would like to use these APIs

Operations

GET
/v1/one-time-token/status
POST
/v1/one-time-token/pin/verify
POST
/v1/one-time-token/partner-device-fingerprint/verify
POST
/v1/one-time-token/facemap/verify
POST
/v1/one-time-token/sms/trigger
POST
/v1/one-time-token/sms/verify
POST
/v1/one-time-token/whatsapp/trigger
POST
/v1/one-time-token/whatsapp/verify
POST
/v1/one-time-token/voice/trigger
POST
/v1/one-time-token/voice/verify
GET
/v1/identity/one-time-token/status - Deprecated
The One Time Token resource

We suggest the following method to check if the OTT is ready to access an SCA protected endpoint.

Iterate through challenges array and look for any challenge that has the following properties:

required is true
passed is false
Fields
oneTimeTokentext

Unique identifier of a one time token.

challengesChallengeObject[]

Array of ChallengeObject.

validitynumber

Seconds until the one time token become expired.

actionTypetext

The action bound to the one time token.

For example: BALANCE__GET_STATEMENT when we want to retrieve a balance account statement.

userIdnumber

Creator of this one time token.

One Time Token Object
{
  "oneTimeToken": "5932d5b5-ec13-452f-8688-308feade7834",
  "challenges": [
    {
      "primaryChallenge": {
        "type": "PIN",
        "viewData": {
          "attributes": {
            "userId": 6146956
          }
        }
      },
      "alternatives": [],
      "required": true,
      "passed": false
    }
  ],
  "validity": 3600,
  "actionType": "BALANCE__GET_STATEMENT",
  "userId": 6146956
}
The Challenge Object resource
Fields
primaryChallengeChallenge

Type of challenge user can do.

alternativesChallenge[]

Alternative challenges that user can do instead of the primary ones.

requiredboolean

Required (or not) to pass the OTT.

passedboolean

Status of this challenge.

Challenge Object
{
  "primaryChallenge": {
    "type": "PIN",
    "viewData": {
      "attributes": {
        "userId": 6146956
      }
    }
  },
  "alternatives": [],
  "required": true,
  "passed": false
}
The Challenge resource
Fields
typeChallengeType

Type of the challenge

viewDataobject

An object that provides data required to present a challenge window. It can be messages, IDs, or other attributes.

Challenge
{
  "type": "PIN",
  "viewData": {
    "attributes": {
      "userId": 6146956
    }
  }
}
The Challenge Type resource

Enumerated string that indicates what sort of challenge user can do to pass the associated OTT.

Type	Pre-requisite	Endpoint to trigger challenge	Endpoint to perform challenge
PIN	Create Pin	-	Verify Pin
FACE_MAP	Enrol FaceMap	-	Verify FaceMap
SMS	Create Phone Number	Trigger SMS Challenge	Verify SMS
WHATSAPP	Create Phone Number	Trigger WhatsApp Challenge	Verify WhatsApp
VOICE	Create Phone Number	Trigger Voice Challenge	Verify Voice
PARTNER_DEVICE_FINGERPRINT	Create Device Fingerprint	-	Verify Device Fingerprint
Get status of a one time token

GET /v1/one-time-token/status

Notes:

GET /v1/identity/one-time-token/status will be deprecated soon.
Please use the new endpoint /v1/one-time-token/status instead.

Retrieve necessary information to clear a OTT.

Request
Header
One-Time-Tokentext

Text value of a OTT.

Response
oneTimeTokenPropertiesOneTimeToken

Properties of OneTimeToken

Example Request
curl -X GET \
  https://api.wise-sandbox.com/v1/one-time-token/status \
  -H 'Authorization: Bearer <your api token>'
  -H 'One-Time-Token: <one time token>'
Example Response
{
  "oneTimeTokenProperties": {
    "oneTimeToken": "9f5f5812-2609-4e48-8418-b64437c0c7cd",
    "challenges": [
      {
        "primaryChallenge": {
          "type": "PIN",
          "viewData": {
            "attributes": {
              "userId": 6146956
            }
          }
        },
        "alternatives": [],
        "required": true,
        "passed": false
      }
    ],
    "validity": 3600,
    "actionType": "BALANCE__GET_STATEMENT",
    "userId": 6146956
  }
}
Verify PIN

POST /v1/one-time-token/pin/verify

To clear a PIN challenge listed in a OTT.

Notes:

User is required to create pin before the verification can be successful.
Rate limit may be applied if there are 5 continuous unsuccessful attempts and OTT creation will be blocked for 15 minutes.
Request
Header
One-Time-Tokentext

Text value of a OTT.

Body
pintext

PIN that is setup using create pin endpoint.

Response
oneTimeTokenPropertiesOneTimeToken

Take note that the raw response body will be a string. Please refer to our JOSE guide on how should you decrypt this.

Properties of OneTimeToken.

Example Request
curl -X POST \
  https://api.wise-sandbox.com/v1/one-time-token/pin/verify \
  -H 'Authorization: Bearer <your api token>' \
  -H 'Accept: application/jose+json' \
  -H 'Accept-Encoding: identity' \
  -H 'Content-Type: application/jose+json' \
  -H 'Content-Encoding: identity' \
  -H 'X-TW-JOSE-Method: jwe' \
  -H 'One-Time-Token: <one time token>' \
  -d 'eyJlbmMiOiJBMjU2R0NNIiwi...'
Example Response
eyJlbmMiOiJBMjU2R0NNIiwi...
Verify Device Fingerprint

POST /v1/one-time-token/partner-device-fingerprint/verify

To clear a Device Fingerprint challenge listed in an OTT.

Notes:

User is required to create a device fingerprint before the verification can be successful.
Rate limit may be applied if there are 5 continuous unsuccessful attempts and OTT creation will be blocked for 15 minutes.
Request
Header
One-Time-Tokentext

Text value of a OTT.

Body
deviceFingerprinttext

Device Fingerprint previously set.

Response
oneTimeTokenPropertiesOneTimeToken

Take note that the raw response body will be a string. Please refer to our JOSE guide on how should you decrypt this.

Properties of OneTimeToken.
When successful, response may return the next challenge in challenges array. If challenges array is empty. You may now use the OTT to access an SCA protected endpoint.

Example Request
curl -X POST \
  'https://api.wise-sandbox.com/v1/one-time-token/partner-device-fingerprint/verify' \
  -H 'Authorization: Bearer <your api token>' \
  -H 'Accept: application/jose+json' \
  -H 'Accept-Encoding: identity' \
  -H 'Content-Type: application/jose+json' \
  -H 'Content-Encoding: identity' \
  -H 'X-TW-JOSE-Method: jwe' \
  -H 'One-Time-Token: <one time token>' \
  -d 'eyJlbmMiOiJBMjU2R0NNIiwi...'
Example Response
eyJlbmMiOiJBMjU2R0NNIiwi...
Verify FaceMap

POST /v1/one-time-token/facemap/verify

To clear a FACE_MAP challenge listed in a OTT.

Notes:

User is required to enrol facemap before the verification can be successful.
Rate limit may be applied if there are 5 continuous unsuccessful attempts and OTT creation will be blocked for 15 minutes.
Request
Header
One-Time-Tokentext

Text value of a OTT.

Body
faceMaptext

Base64-encoded binary data as a string.

For more details how to get this binary, please read FaceTec's export API.

To retrieve Wise's FaceTec public key, please refer to our FaceTec's Get Public Key API.

Response
oneTimeTokenPropertiesOneTimeToken

Properties of OneTimeToken.


When successful, response may return the next challenge in challenges array.

If challenges array is empty. You may now use the OTT to access an SCA protected endpoint.

Example Request
curl -X POST \
  https://api.wise-sandbox.com/v1/one-time-token/facemap/verify \
  -H 'Authorization: Bearer <your api token>' \
  -H 'One-Time-Token: <one time token>' \
  -d '{
    "faceMap": "<base64_encoded_string>"
  }'
Example Response
{
  "oneTimeTokenProperties": {
    "oneTimeToken": "9f5f5812-2609-4e48-8418-b64437c0c7cd",
    "challenges": [],
    "validity": 3600
  }
}
Trigger SMS Challenge

POST /v1/one-time-token/sms/trigger

To trigger a SMS challenge by sending SMS to user verified phone number containing a 6 digit one time password (OTP).

This OTP code can be used to clear a SMS challenge by using the verify sms endpoint.

The Trigger SMS Challenge API is currently in closed Beta and subject to change. Please speak with your implementation manager if you would like to use this API
Request
Header
One-Time-Tokentext

Text value of a OTT.

Response
obfuscatedPhoneNotext

Obfuscated phone number that can be used as a hint for the end customer regarding which phone number the SMS was sent to.

Example Request
curl -X POST \
  https://api.wise-sandbox.com/v1/one-time-token/sms/trigger \
  -H 'Authorization: Bearer <your api token>' \
  -H 'One-Time-Token: <one time token>'
Example Response
{
  "obfuscatedPhoneNo": "*********8888"
}
Verify SMS Challenge

POST /v1/one-time-token/sms/verify

To clear a SMS challenge listed in a OTT.

Notes:

User is required have a verified phone number. See create phone number for more information.
Trigger SMS Challenge is required to be called first.
Since we won't be sending real SMS on sandbox, the OTP Code will always be 111111.

The Verify SMS Challenge API is currently in closed Beta and subject to change. Please speak with your implementation manager if you would like to use this API

Request
Header
One-Time-Tokentext

Text value of a OTT.

Body
otpCodetext

6 digit OTP code in text format.

Response
oneTimeTokenPropertiesOneTimeToken

Properties of OneTimeToken.

Example Request
curl -X POST \
  https://api.wise-sandbox.com/v1/one-time-token/sms/verify \
  -H 'Authorization: Bearer <your api token>' \
  -H 'One-Time-Token: <one time token>' \
  -d '{
    "otpCode": "111111"
  }'
Example Response
{
  "oneTimeTokenProperties": {
    "oneTimeToken": "9f5f5812-2609-4e48-8418-b64437c0c7cd",
    "challenges": [],
    "validity": 3600
  }
}
Trigger WhatsApp Challenge

POST /v1/one-time-token/whatsapp/trigger

To trigger a WhatsApp challenge by sending WhatsApp message to user verified phone number containing a 6 digit one time password (OTP).

This OTP code can be used to clear a WHATSAPP challenge by using the verify whatsapp endpoint.

The Trigger WhatsApp Challenge API is currently in closed Beta and subject to change. Please speak with your implementation manager if you would like to use this API

Request
Header
One-Time-Tokentext

Text value of a OTT.

Response
obfuscatedPhoneNotext

Obfuscated phone number that can be used as a hint for the end customer regarding which phone number the WhatsApp message was sent to.

Example Request
curl -X POST \
  https://api.wise-sandbox.com/v1/one-time-token/whatsapp/trigger \
  -H 'Authorization: Bearer <your api token>' \
  -H 'One-Time-Token: <one time token>'
Example Response
{
  "obfuscatedPhoneNo": "*********8888"
}
Verify WhatsApp Challenge

POST /v1/one-time-token/whatsapp/verify

To clear a WHATSAPP challenge listed in a OTT.

Notes:

User is required have a verified phone number. See create phone number for more information.
Trigger WhatsApp Challenge is required to be called first.
Since we won't be sending real WhatsApp message on sandbox, the OTP Code will always be 111111.

The Verify WhatsApp Challenge API is currently in closed Beta and subject to change. Please speak with your implementation manager if you would like to use this API

Request
Header
One-Time-Tokentext

Text value of a OTT.

Body
otpCodetext

6 digit OTP code in text format.

Response
oneTimeTokenPropertiesOneTimeToken

Properties of OneTimeToken.

Example Request
curl -X POST \
  https://api.wise-sandbox.com/v1/one-time-token/whatsapp/verify \
  -H 'Authorization: Bearer <your api token>' \
  -H 'One-Time-Token: <one time token>' \
  -d '{
    "otpCode": "111111"
  }'
Example Response
{
  "oneTimeTokenProperties": {
    "oneTimeToken": "9f5f5812-2609-4e48-8418-b64437c0c7cd",
    "challenges": [],
    "validity": 3600
  }
}
Trigger Voice Challenge

POST /v1/one-time-token/voice/trigger

To trigger a WhatsApp challenge by sending voice message to user verified phone number containing a 6 digit one time password (OTP).

This OTP code can be used to clear a VOICE challenge by using the verify voice endpoint.

The Trigger Voice Challenge API is currently in closed Beta and subject to change. Please speak with your implementation manager if you would like to use this API

Request
Header
One-Time-Tokentext

Text value of a OTT.

Response
obfuscatedPhoneNotext

Obfuscated phone number that can be used as a hint for the end customer regarding which phone number the WhatsApp message was sent to.

Example Request
curl -X POST \
  https://api.wise-sandbox.com/v1/one-time-token/voice/trigger \
  -H 'Authorization: Bearer <your api token>' \
  -H 'One-Time-Token: <one time token>'
Example Response
{
  "obfuscatedPhoneNo": "*********8888"
}
Verify Voice Challenge

POST /v1/one-time-token/voice/verify

To clear a VOICE challenge listed in a OTT.

Notes:

User is required have a verified phone number. See create phone number for more information.
Trigger Voice Challenge is required to be called first.
Since we won't be sending real voice message on sandbox, the OTP Code will always be 111111.

The Verify Voice Challenge API is currently in closed Beta and subject to change. Please speak with your implementation manager if you would like to use this API

Request
Header
One-Time-Tokentext

Text value of a OTT.

Body
otpCodetext

6 digit OTP code in text format.

Response
oneTimeTokenPropertiesOneTimeToken

Properties of OneTimeToken.

Example Request
curl -X POST \
  https://api.wise-sandbox.com/v1/one-time-token/voice/verify \
  -H 'Authorization: Bearer <your api token>' \
  -H 'One-Time-Token: <one time token>' \
  -d '{
    "otpCode": "111111"
  }'
Example Response
{
  "oneTimeTokenProperties": {
    "oneTimeToken": "9f5f5812-2609-4e48-8418-b64437c0c7cd",
    "challenges": [],
    "validity": 3600
  }
}