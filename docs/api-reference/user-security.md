# User SecurityCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/api-reference/user-security

---

User Security
Copy

These endpoints are deprecated. Please refer to strong-customer-authentication section to integrate with SCA.

User security allow users to set up security related protections over API.

Operations

POST
/v1/user/pin
DELETE
/v1/users/{{userId}}/pin
POST
/v1/user/facemap/enrol
DELETE
/v1/users/{{userId}}/facemap/enrol
POST
/v1/user/partner-device-fingerprints
GET
/v1/users/{{userId}}/partner-device-fingerprints
DELETE
/v1/users/{{userId}}/partner-device-fingerprints/{{deviceFingerprintId}}
GET
/v1/application/users/{{userId}}/phone-numbers
POST
/v1/application/users/{{userId}}/phone-numbers
PUT
/v1/application/users/{{userId}}/phone-numbers/{{phoneNumberId}}
DELETE
/v1/application/users/{{userId}}/phone-numbers/{{phoneNumberId}}
Phone Number

A resource used to define phone number stored in Wise.

Fields
idnumber

ID of the phone number

phoneNumbertext

A text representation of phone number.

typetext

Type of phone number when used in authentication.

Only PRIMARY is supported at the moment.

verifiedboolean

Indicator if phone number is verified.

Note that only verified phone number will be used as a form of authentication.

clientIdtext

Client ID of which this phone number belongs to.

Phone Number
{
  "id": 1230944,
  "phoneNumber": "+6588888888",
  "type": "PRIMARY",
  "verified": true,
  "clientId": "clientId"
}
Create PIN

POST /v1/user/pin

Create PIN for a user as a form of authentication.

Can be used to verify pin when accessing a strongly protected endpoint via One Time Token Framework.

Request
pintext

A four digits string.

Response

Possible HTTP status codes

204 - No Content

PIN is created successfully.

409 - Conflict

PIN has already been created.

Example Request
curl -X POST \
  https://api.wise-sandbox.com/v1/user/pin \
  -H 'Authorization: Bearer <your api token>' \
  -H 'Accept: application/jose+json' \
  -H 'Accept-Encoding: identity' \
  -H 'Content-Type: application/jose+json' \
  -H 'Content-Encoding: identity' \
  -H 'X-TW-JOSE-Method: jwe' \
  -d 'eyJlbmMiOiJBMjU2R0NNIiwi...'
Example Response
eyJlbmMiOiJBMjU2R0NNIiwi...
Delete PIN

DELETE /v1/users/{{userId}}/pin

Can be used to remove the PIN from the user's account.

This endpoint is restricted and requires both a client credentials token and additional access to use. Please speak with your implementation manager if you would like to use this API
Path Variable
userIdtext

User ID.

Response

Possible HTTP status codes

204 - No Content

PIN is deleted successfully.

404 - PIN Not Setup

PIN is not setup for this user.

Example Request
curl -X DELETE \
  https://api.wise-sandbox.com/v1/users/{{userId}}/pin \
  -H 'Authorization: Bearer <your client credentials token>' 
Response - PIN Not Setup (404)
{
  "errors": [{
    "code": "pin.not.setup",
    "message": "PIN has not been setup."
  }]
}
Enrol FaceMap

POST /v1/user/facemap/enrol

Enrol FaceMap: Facial biometric enrolment for Strong Customer Authentication (SCA).

Can be used to verify facemap when accessing a strongly protected endpoint via One Time Token Framework.

Request
faceMaptext

Base64-encoded binary data as a string.

For more details how to get this binary, please read FaceTec's export API.

To retrieve Wise's FaceTec public key, please refer to our FaceTec's Get Public Key API.

Response

Possible HTTP status codes

204 - No Content

Enrollment is successful.

409 - Conflict

FaceMap has already been enrolled.

Example Request
curl -X POST \
  https://api.wise-sandbox.com/v1/user/facemap/enrol \
  -H 'Authorization: Bearer <your api token>' \
  -d '{
    "faceMap": "<encrypted_face_map_in_base64_string>"
  }'
Delete FaceMap

DELETE /v1/users/{{userId}}/facemap/enrol

Can be used to remove the FaceMap from the user's account.

This endpoint is restricted and requires both a client credentials token and additional access to use. Please speak with your implementation manager if you would like to use this API
Path Variable
userIdtext

User ID.

Response

Possible HTTP status codes

204 - No Content

FaceMap is deleted successfully.

404 - FaceMap Not Setup

FaceMap is not setup for this user.

Example Request
curl -X DELETE \
  https://api.wise-sandbox.com/v1/users/{{userId}}/facemap/enrol \
  -H 'Authorization: Bearer <your client credentials token>' 
Response - FaceMap Not Setup (404)
{
  "errors": [{
    "code": "facemap.not.setup",
    "message": "FaceMap has not been setup."
  }]
}
Create Device Fingerprint

POST /v1/user/partner-device-fingerprints

A device fingerprint represents a string that identifies a unique device.

This endpoint is used to register the fingerprint of the device as one of the allowed devices used during an One Time Token (OTT) challenge.

This can be used to verify device fingerprint when clearing a OTT.

Request
deviceFingerprinttext

A string that is used as a device fingerprint

Response
deviceFingerprintIdUUID

Identifier of the device fingerprint

createdAtdate

Timestamp on when the device fingerprint was created

Possible HTTP status codes

200 - HTTP OK

The device fingerprint has been successfully created.

409 - Conflict

The device fingerprint has already been created.

400 - Bad Request

Maximum number of device fingerprints reached (defaulted to 3).

Example Request
curl -X POST \
  'https://api.wise-sandbox.com/v1/user/partner-device-fingerprints' \
  -H 'Authorization: Bearer <your api token>' \
  -H 'Content-Type: application/jose+json' \
  -H 'X-TW-JOSE-Method: jwe' \
  -H 'Accept: application/jose+json' \
  -H 'Accept-Encoding: *' \
  -d '{
    "deviceFingerprint": "3207da22-a0d3-4b6b-a591-6297e646fe32" 
  }'
Example Response
{
  "deviceFingerprintId": "636a5514-aa86-4719-8700-e9a9a0ae7ea7",
  "createdAt": "2024-05-24T07:27:58.273205554Z"
}
Get Device Fingerprints

POST /v1/users/{{userId}}/partner-device-fingerprints

Returns a list of device fingerprints created for this user.

Example Request
curl -X POST \
  'https://api.wise-sandbox.com/v1/users/{{userId}}/partner-device-fingerprints' \
  -H 'Authorization: Bearer <your client credentials token>'
Response
List of device fingerprints
deviceFingerprintIdUUID

Identifier of the device fingerprint

createdAtdate

Timestamp on when the device fingerprint was created

Possible HTTP status codes
200 - HTTP OK
404 - User Not Found

The user is not found

Example Response
{
  [
    {
      "deviceFingerprintId": "636a5514-aa86-4719-8700-e9a9a0ae7ea7",
      "createdAt": "2024-05-24T07:27:58.273205554Z"
    }
  ]
}
Delete Device Fingerprint

DELETE /v1/users/{{userId}}/partner-device-fingerprints/{{deviceFingerprintId}}

Can be used to remove a specific device fingerprint from the allowed devices of a user.

This endpoint is restricted and requires both a client credentials token and additional access to use. Please speak with your implementation manager if you would like to use this API
Path Variable
deviceFingerprintIdtext

Device fingerprint ID.

Response

Possible HTTP status codes

204 - No Content

Device fingerprint has been successfully removed.

404 - Not found

User or deviceFingerprintId is not found.

Example Request
curl -X DELETE \
  https://api.wise-sandbox.com/v1/users/{{userId}}/partner-device-fingerprints/{{deviceFingerprintId}} \
  -H 'Authorization: Bearer <your client credentials token>' 
List Phone Numbers

GET /v1/application/users/{{userId}}/phone-numbers

This endpoint is restricted and requires both a client credentials token and additional access to use. Please speak with your implementation manager if you would like to use this API

List verified phone numbers for a user.

Request
Parameters
userIdtext

User ID.

Response

Returns a list of phone numbers.

Example Request
curl -X GET \
  https://api.wise-sandbox.com/v1/application/users/{{user_id}}/phone-numbers \
  -H 'Authorization: Bearer <your api token>'
Example Response
[
  {
    "id": 1230944,
    "phoneNumber": "+6588888888",
    "type": "PRIMARY",
    "verified": true,
    "clientId": "clientId"
  }
]
Create Phone Number

POST /v1/application/users/{{userId}}/phone-numbers

This endpoint is restricted and requires both a client credentials token and additional access to use. Please speak with your implementation manager if you would like to use this API

Create a verified phone number for a user.

Request
phoneNumbertext

A valid phone number in string.

Response
HTTP Status Codes
200 - OK

Returns phone number

422 - Unprocessable Entity

The phone number is already associated with another account.

To authenticate users and prevent unauthorized access, we require each user to have a unique phone number that can be verified.

Example Request
curl -X POST \
  https://api.wise-sandbox.com/v1/application/users/{{user_id}}/phone-numbers \
  -H 'Authorization: Bearer <your api token>' \
  -d '{
    "phoneNumber": "+6588888888"
  }'
Example Response - 200
{
    "id": 1230944,
    "phoneNumber": "+6588888888",
    "type": "PRIMARY",
    "verified": true,
    "clientId": "clientId"
}
Example Response - 422
{
  "errors": [{
    "code": "phone.number.repeated",
    "message": "It's linked to an account with the email ****@wise.com"
  }]
}
Update Phone Number

PUT /v1/application/users/{{userId}}/phone-numbers/{{phoneNumberId}}

This endpoint is restricted and requires both a client credentials token and additional access to use. Please speak with your implementation manager if you would like to use this API

Update a verified phone number for a user.

Request
phoneNumbertext

A valid phone number in string.

Response
HTTP Status Codes
200 - OK

Returns phone number

422 - Unprocessable Entity

The phone number is already associated with another account.

To authenticate users and prevent unauthorized access, we require each user to have a unique phone number that can be verified.

Example Request
curl -X PUT \
  https://api.wise-sandbox.com/v1/application/users/{{user_id}}/phone-numbers/{{phoneNumberId}} \
  -H 'Authorization: Bearer <your api token>' \
  -d '{
    "phoneNumber": "+6588888888"
  }'
Example Response - 200
{
    "id": 1230944,
    "phoneNumber": "+6588888888",
    "type": "PRIMARY",
    "verified": true,
    "clientId": "clientId"
}
Example Response - 422
{
  "errors": [{
    "code": "phone.number.repeated",
    "message": "It's linked to an account with the email ****@wise.com"
  }]
}
Delete Phone Number

DELETE /v1/application/users/{{userId}}/phone-numbers/{{phoneNumberId}}

This endpoint is restricted and requires both a client credentials token and additional access to use. Please speak with your implementation manager if you would like to use this API

Deletes a verified phone number for a user.

Request
Parameters
userIdtext

User ID.

phoneNumberIdtext

ID of a phone number.

Response
HTTP Status Codes
204 - No Content

No Content.

Example Request
curl -X DELETE \
  https://api.wise-sandbox.com/v1/application/users/{{user_id}}/phone-numbers/{{phoneNumberId}} \
  -H 'Authorization: Bearer <your api token>'