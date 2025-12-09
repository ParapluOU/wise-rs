# SCA over APICopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/developer/auth-and-security/sca-over-api

---

SCA over API
Copy

The term SCA is essentially a form of MFA designed for the financial industry. MFA, which stands for Multi-Factor Authentication, is a broader technical concept whereas SCA defines the business feature.

We provide endpoints that allow your customers to complete Strong Customer Authentication (SCA), adding an extra layer of security to help reduce fraud. SCA is required for both high-risk and low-risk operations.

Examples of high-risk operations include fund a transfer, 3DS result notification, or fund a batch group, while retrieving a balance statement is considered a low-risk operation.

All SCA-protected endpoints can be accessed using the authentication flow described below. Alternatively, for lower-risk operations, we introduce the concept of SCA session. This allows users to perform lower-risk actions within a 5-minute window after successfully completing SCA once.

Please reach out to your Implementation Manager before starting implementing SCA. We may require you to complete a survey before granting access to these endpoints.
Strong Authentication Methods

Wise offers four authentication methods categorized by authentication type. To perform a higher-risk operation, at least two authentication methods of different types must be registered with Wise. We recommend enrolling in a third authentication method to serve as an alternative in case one of the primary methods cannot be used.

Method	Authentication Type	Description	Example
PIN	Knowledge	A 4-digit code only known by the customer	pin example
Device Fingerprint	Possession	A unique identifier generated on the device that is tracked as a proof of authenticity showing that the device is owned by the customer	device fingerprint example
Phone Based OTP	Possession	An OTP code sent to a pre-registered phone number with Wise	phone OTP example
FaceTec	Inherence	A digital representation of a customer's facial features used as biometric authentication	facemap example
How it works

A SCA flow is composed of 2 parts:

Enrollment: First, you should determine which authentication factors you want your customers to use for accessing SCA-protected endpoints. These factors should be registered with Wise for each customer by calling these endpoints. They will serve as a reference when customers are required to complete SCA challenges.

Verification: Certain high-risk endpoints require customers to verify their identity through SCA challenges. These endpoints remain inaccessible until a cleared authentication credential is provided. To clear this credential, the customer must successfully complete at least two challenges by verifying pre-enrolled authentication factors. Once the required challenges are passed, the authentication credential can be used to access the restricted resource.

Enrollment

Strong Customer Authentication (SCA) requires a set up process. The customer has to provide authentication factors that inherently are used to prove their identity. These are commonly described by something they are, something they know and something they have.

It's important to provide an onboarding flow in your application that sets up SCA enrollment for your end customers.

We strongly recommend providing fallback challenges for your customers in case they are unable to complete one of the primary challenge methods.

PIN: For PIN-based authentication, it's important to collect the PIN information directly from the end customer through a secure client-side encryption method. This ensures that the PIN is not compromised during transmission.

Device Fingerprint: For device fingerprint-based authentication, it's important to use client-side encryption to collect the information directly from the device, and then forward the encrypted information securely to Wise. You should never persist the device fingerprint in any form. This ensures that the fingerprint is collected securely and accurately represents the user's device.

Phone Based OTP: For phone-based OTP authentication, Wise offers multiple OTP methods such as SMS, Voice, and WhatsApp. Use a secure endpoint provided by Wise to update the end user's phone number, and perform phone number verification on your end before updating the information with Wise. Please reach out to your implementation manager if you would like to use this method.

FaceTec: To implement FaceTec-based authentication, integrate with FaceTec and propagate the FaceMap to us for verification.

Verification
Restricted access endpoint

When your users are interacting with Wise's endpoint, a high-risk endpoint might return a 403 status code along with these two fields in the response header:

x-2fa-approval-result with value REJECTED that indicates the request requires additional authentication.
x-2fa-approval returning a unique identifier text that represent the authentication credentials (OTT) to be verified and cleared.

For more information on how to manage strong customer authentication using the response header values, you can refer to our one-time token guide.

It is recommended to always intercept responses from Wise and read these headers to enable an automated SCA flow in your application, thus ensuring that your end users can benefit from the latest protection that Wise has put in place.
Get SCA challenges list

To obtain the list of challenges linked to the authentication credentials or to check the status of SCA challenges, please use /one-time-token/status endpoint and set the One-Time-Token field in the request header.

Verify authentication factors

Please use these verification endpoints to verify a authentication factor and clear a challenge:

PIN verification
Device Fingerprint verification
Phone OTP verification
FaceTec verification

A verified authentication factor will return a successful HTTP status code. You can verify this by calling GET /one-time-token/status and check the passed field for the associated challenge.

Please note that for phone-based OTP authentication method, you have to call a trigger endpoint. This allows end users to choose when to send the OTP code and which phone number the message should be sent to.

Authorised access

Once at least two authentication methods are verified, the authentication credential (OTT) is cleared.

You can now access the restricted endpoint by including the cleared authentication credential in the x-2fa-approval header of your request.

Strong Authentication Session

A Strong Customer Authentication session can be set up for low-risk operations. Once the customer successfully performs SCA, this session remains valid for 5 minutes until it expires. During this 5-minute period, customers can access any low-risk endpoints without needing to perform Strong Customer Authentication for each protected endpoint.

Please be aware that high-risk transactions like payment transfers may still require to perform Strong Customer Authentication (SCA).

Pin Example

This is a walkthrough of how to use PIN as an authentication method for SCA including setting it up for your end customers.

Please note that all PIN-related endpoints use Jose direct encryption for secure data encryption in transit. It is essential to never persist this sensitive data in your application and only retrieve it directly from the customer

Setting Up

Create PIN API allow customers to setup their PIN with Wise directly.

This will allow customers to send their desired PIN to Wise in a direct encrypted manner which not even Wise will know about its value.

Verify Flow

Once an end customer's PIN is set up. It can now be used as an authentication factor to clear an SCA challenge.

Image below illustrates the interaction between Partner and Wise.

Steps
Partner makes an HTTP call to get balance account statement which is a SCA protected endpoint.
Get Balance Account Statement - Request
curl -X GET https://api.wise-sandbox.com/v1/profiles/{{profileId}}/balance-statements/{{balanceId}}/statement.json \
      ?currency=EUR \
      &intervalStart=2023-01-01T00:00:00.000Z \
      &intervalEnd=2023-01-15T23:59:59.999Z \
      &type=COMPACT \
      -H 'Authorization: Bearer <your api token>'
Wise rejects the request with status 403 Forbidden. Please see the example response on the right.
Get Balance Account Statement - Response
HTTP/1.1 403 Forbidden
Date: Wed, 06 Dec 2023 08:57:34 GMT
x-2fa-approval: bb676aeb-7c4d-4930-bb55-ab949fd3fd87
x-2fa-approval-result: REJECTED
...other headers
Partner gets status of a one time token to get all required challenges to clear this OTT.
Get Status of One Time Token - Request
curl -X GET https://api.wise-sandbox.com/v1/identity/one-time-token/status \
     -H 'Authorization: Bearer <your api token>' \
     -H 'One-Time-Token: bb676aeb-7c4d-4930-bb55-ab949fd3fd87'

Wise returns one time token that describes all required challenges.


For the complete list of challenges available please refer to ChallengeType.

Get Status of One Time Token - Response
{
  "oneTimeTokenProperties": {
    "oneTimeToken": "bb676aeb-7c4d-4930-bb55-ab949fd3fd87",
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
Partner verify PIN by passing OTT acquired in step 2.
Verify PIN - Request
curl -X POST https://api.wise-sandbox.com/v1/one-time-token/pin/verify \
     -H 'Authorization: Bearer <your api token>' \
     -H 'Accept: application/jose+json' \
     -H 'Accept-Encoding: identity' \
     -H 'Content-Type: application/jose+json' \
     -H 'Content-Encoding: identity' \
     -H 'X-TW-JOSE-Method: jwe' \
     -H 'One-Time-Token: <one time token>'
     -d 'eyJlbmMiOiJBMjU2R0NNIiwi...'
Wise returns one time token properties after a successful pin verification.

Assuming that the challenges array field is empty, indicating that the OTT is now usable.

If you are unsure, you can always get status of a one time token again.

It is possible that the challenges array returns type of challenge. In that case, please perform the verification flow as written in our guides.

Verify PIN - Response
eyJlbmMiOiJBMjU2R0NNIiwi...
Partner calls Get Balance Statement with the approved OTT.
Get Balance Account Statement - Request
curl -X GET https://api.wise-sandbox.com/v1/profiles/{{profileId}}/balance-statements/{{balanceId}}/statement.json \
         ?currency=EUR \
         &intervalStart=2023-01-01T00:00:00.000Z \
         &intervalEnd=2023-01-15T23:59:59.999Z \
         &type=COMPACT \
     -H 'Authorization: Bearer <your api token>' \
     -H 'x-2fa-approval: bb676aeb-7c4d-4930-bb55-ab949fd3fd87'
Wise returns Balance Account Statement.
Get Balance Account Statement - Response
{
   "accountHolder": {
      "type": "PERSONAL",
      "address": {
         "addressFirstLine": "Veerenni 24",
         "city": "Tallinn",
         "postCode": "12112",
         "stateCode": "",
         "countryName": "Estonia"
      },
      "firstName": "Oliver",
      "lastName": "Wilson"
   },
   "otherFields": "..."
}
Facemap Example

This is a walkthrough of how to use FaceMap as an authentication method for SCA including setting it up for your end customers.

By following these steps, you can enable secure and convenient authentication for your users while meeting the regulatory requirements for online transactions.

Limitations
Wise only supports the 3D FaceMap Interoperability Between Organizations, which requires integration with FaceTec. This means that your organization must also be a consumer of FaceTec services in order to enable this flow.
This authentication method will only be enabled when requested. Please reach out to your implementation manager for inquiry.
Setting Up

Wise leverages the import functionality of FaceTec when receiving 3D FaceMaps from customers, enabling seamless integration and secure authentication processes.

Image below illustrates the interaction between Frontend, Backend and Wise.

Steps

Frontend makes an HTTP call to enrol FaceMap to your Backend application.

This is where the customer submits a FaceScan to your application for FaceTec enrolment.

Take note that you should have a **FaceMap** after successful enrolment. This will be used in Step (4) for export.

Backend makes an HTTP call to get Wise's FaceTec Public Key to acquire public key required to do a FaceTec export.

Get FaceTec Public Key - Request
curl -X GET https://api.wise-sandbox.com/v1/facetec/public-key \
     -H 'Authorization: Bearer <your api token>' 
Wise returns the public key to Backend in plain text. You are encouraged to cache this public key to minimize latency, as it is not subject to frequent rotation.
Get FaceTec Public Key - Response
-----BEGIN PUBLIC KEY-----
Public Key Content
-----END PUBLIC KEY----- 

The Backend application will utilize the previously stored FaceMap from Step (1) and the acquired Wise's FaceTec public key from Step (3) as input parameters for the export function provided by FaceTec.

Upon a successful export, we will possess an encrypted FaceMap that is ready to be transmitted to Wise.

Backend makes an HTTP call to Enrol FaceMap.

Enrol FaceMap - Request
curl -X POST https://api.wise-sandbox.com/v1/users/facemap/enrol \
     -H 'Authorization: Bearer <your api token>' 
     -d '{
        "faceMap": "<encrypted_face_map_in_base64_string>"
     }'

Wise responds with a 204 - No Content status code upon successful enrollment.

Please note that a 409 - Conflict response indicates that the enrollment already exists and cannot be repeated.

Backend should responds with a successful HTTP status code to customer upon successful enrollment.

Verify Flow

This guide uses retrieving balance account statement as an example.

Image below illustrates the interaction between Frontend, Backend, and Wise.

Steps
Frontend makes an HTTP call to get balance account statement which is a SCA protected endpoint.
Get Balance Account Statement - Request
curl -X GET https://api.wise-sandbox.com/v1/profiles/{{profileId}}/balance-statements/{{balanceId}}/statement.json \
      ?currency=EUR \
      &intervalStart=2023-01-01T00:00:00.000Z \
      &intervalEnd=2023-01-15T23:59:59.999Z \
      &type=COMPACT \
      -H 'Authorization: Bearer <your api token>'
Wise rejects the request with status 403 Forbidden. Please see the example response on the right.
Get Balance Account Statement - Response
HTTP/1.1 403 Forbidden
Date: Wed, 06 Dec 2023 08:57:34 GMT
x-2fa-approval: bb676aeb-7c4d-4930-bb55-ab949fd3fd87
x-2fa-approval-result: REJECTED
...other headers
Frontend gets status of a one time token to get all required challenges to clear this OTT.
Get Status of One Time Token - Request
curl -X GET https://api.wise-sandbox.com/v1/identity/one-time-token/status \
     -H 'Authorization: Bearer <your api token>' \
     -H 'One-Time-Token: bb676aeb-7c4d-4930-bb55-ab949fd3fd87'

Wise returns one time token that describes all required challenges.


For the complete list of challenges available please refer to ChallengeType.

Get Status of One Time Token - Response
{
   "oneTimeTokenProperties": {
      "oneTimeToken": "bb676aeb-7c4d-4930-bb55-ab949fd3fd87",
      "challenges": [
         {
            "primaryChallenge": {
               "type": "FACE_MAP",
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

Frontend makes an HTTP call to Backend to perform a match 3d 3d check.

Take note that you should have a **FaceMap** after successful match3d3d. This will be used in Step (8) for export.

Backend makes an HTTP call to get Wise's FaceTec Public Key to acquire public key required to do a FaceTec export.

Get FaceTec Public Key - Request
curl -X GET https://api.wise-sandbox.com/v1/facetec/public-key \
     -H 'Authorization: Bearer <your api token>' 
Wise returns the public key to Backend in plain text. You are encouraged to cache this public key to minimize latency, as it is not subject to frequent rotation.
Get FaceTec Public Key - Response
-----BEGIN PUBLIC KEY-----
Public Key Content
-----END PUBLIC KEY----- 

The Backend application will utilize the previously stored FaceMap from Step (5) and the acquired Wise's FaceTec public key from Step (7) as input parameters for the export function provided by FaceTec.

Upon a successful export, we will possess an encrypted FaceMap that is ready to be transmitted to Wise.

Backend makes an HTTP call to Verify FaceMap.

Verify FaceMap - Request
curl -X GET https://api.wise-sandbox.com/v1/identity/one-time-token/facemap/verify \
     -H 'Authorization: Bearer <your api token>'
     -H 'One-Time-Token: <one time token>'
     -d '{
        "faceMap": "<base64_encoded_string>"
    }'
Wise returns one time token properties after a successful FaceMap verification.

Assuming that the challenges array field is empty, indicating that the OTT is now usable.

If you are unsure, you can always get status of a one time token again.

It is possible that the challenges array returns type of challenge. In that case, please perform the verification flow as written in our guides.

Verify FaceMap - Response
{
   "oneTimeTokenProperties": {
      "oneTimeToken": "bb676aeb-7c4d-4930-bb55-ab949fd3fd87",
      "challenges": [],
      "validity": 3600,
      "actionType": null,
      "userId": null
   }
}

Backend should respond with a successful HTTP status code to customer upon successful verification.

Frontend calls Get Balance Statement with the approved OTT.

Get Balance Account Statement - Request
curl -X GET https://api.wise-sandbox.com/v1/profiles/{{profileId}}/balance-statements/{{balanceId}}/statement.json \
         ?currency=EUR \
         &intervalStart=2023-01-01T00:00:00.000Z \
         &intervalEnd=2023-01-15T23:59:59.999Z \
         &type=COMPACT \
     -H 'Authorization: Bearer <your api token>' \
     -H 'x-2fa-approval: bb676aeb-7c4d-4930-bb55-ab949fd3fd87'
Wise returns Balance Account Statement.
Get Balance Account Statement - Response
{
   "accountHolder": {
      "type": "PERSONAL",
      "address": {
         "addressFirstLine": "Veerenni 24",
         "city": "Tallinn",
         "postCode": "12112",
         "stateCode": "",
         "countryName": "Estonia"
      },
      "firstName": "Oliver",
      "lastName": "Wilson"
   },
   "otherFields": "..."
}
Device Fingerprint Example

This is a walkthrough of how to use Device Fingerprint as an authentication method for SCA including setting it up for your end customers.

Please note that all related endpoints use Jose direct encryption for secure data encryption in transit. It is essential to never persist this sensitive data in your application and only retrieve it directly from the customer

Setting Up

Create Device Fingerprint API allow customers to setup their device fingerprint with Wise directly.

This will allow customers to send their device fingerprint to Wise in a direct encrypted manner to Wise and be used as an authentication method in the future.

Please be aware that Wise allows up to three device fingerprints to be registered at any given time. If you have more than three devices connected to your account, you will need to delete unused device fingerprints using our deletion API.

Verify Flow

Once an end customer's device fingerprint is set up. It can now be used as an authentication factor to clear an SCA challenge.

Image below illustrates the interaction between Partner and Wise.

Steps
Partner makes an HTTP call to get balance account statement which is a SCA protected endpoint.
Get Balance Account Statement - Request
curl -X GET https://api.wise-sandbox.com/v1/profiles/{{profileId}}/balance-statements/{{balanceId}}/statement.json \
      ?currency=EUR \
      &intervalStart=2023-01-01T00:00:00.000Z \
      &intervalEnd=2023-01-15T23:59:59.999Z \
      &type=COMPACT \
      -H 'Authorization: Bearer <your api token>'
Wise rejects the request with status 403 Forbidden. Please see the example response on the right.
Get Balance Account Statement - Response
HTTP/1.1 403 Forbidden
Date: Wed, 06 Dec 2023 08:57:34 GMT
x-2fa-approval: bb676aeb-7c4d-4930-bb55-ab949fd3fd87
x-2fa-approval-result: REJECTED
...other headers
Partner gets status of a one time token to get all required challenges to clear this OTT.
Get Status of One Time Token - Request
curl -X GET https://api.wise-sandbox.com/v1/identity/one-time-token/status \
     -H 'Authorization: Bearer <your api token>' \
     -H 'One-Time-Token: bb676aeb-7c4d-4930-bb55-ab949fd3fd87'

Wise returns one time token that describes all required challenges.


For the complete list of challenges available please refer to ChallengeType.

Get Status of One Time Token - Response
{
  "oneTimeTokenProperties": {
    "oneTimeToken": "bb676aeb-7c4d-4930-bb55-ab949fd3fd87",
    "challenges": [
      {
        "primaryChallenge": {
          "type": "PARTNER_DEVICE_FINGERPRINT",
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
Partner verify device fingerprint by passing OTT acquired in step 2.
Verify Device Fingerprint - Request
curl -X POST https://api.wise-sandbox.com/v1/one-time-token/partner-device-fingerprint/verify \
     -H 'Authorization: Bearer <your api token>' \
     -H 'Accept: application/jose+json' \
     -H 'Accept-Encoding: identity' \
     -H 'Content-Type: application/jose+json' \
     -H 'Content-Encoding: identity' \
     -H 'X-TW-JOSE-Method: jwe' \
     -H 'One-Time-Token: <one time token>'
     -d 'eyJlbmMiOiJBMjU2R0NNIiwi...'
Wise returns one time token properties after successful verification.

Assuming that the challenges array field is empty, indicating that the OTT is now usable.

If you are unsure, you can always get status of a one time token again.

It is possible that the challenges array returns type of challenge. In that case, please perform the verification flow as written in our guides.

Verify Device Fingerprint - Response
eyJlbmMiOiJBMjU2R0NNIiwi...
Partner calls Get Balance Statement with the approved OTT.
Get Balance Account Statement - Request
curl -X GET https://api.wise-sandbox.com/v1/profiles/{{profileId}}/balance-statements/{{balanceId}}/statement.json \
         ?currency=EUR \
         &intervalStart=2023-01-01T00:00:00.000Z \
         &intervalEnd=2023-01-15T23:59:59.999Z \
         &type=COMPACT \
     -H 'Authorization: Bearer <your api token>' \
     -H 'x-2fa-approval: bb676aeb-7c4d-4930-bb55-ab949fd3fd87'
Wise returns Balance Account Statement.
Get Balance Account Statement - Response
{
   "accountHolder": {
      "type": "PERSONAL",
      "address": {
         "addressFirstLine": "Veerenni 24",
         "city": "Tallinn",
         "postCode": "12112",
         "stateCode": "",
         "countryName": "Estonia"
      },
      "firstName": "Oliver",
      "lastName": "Wilson"
   },
   "otherFields": "..."
}
Phone OTP Example

This is a walkthrough of how to use phone as an authentication method for SCA including setting it up for your end customers.

Wise provides three methods for delivering one-time password (OTP) codes to your end-user devices.

SMS
WhatsApp
Voice
Setting Up

Please note that you will be responsible for managing the lifecycle of the phone number, so that Wise can send one-time password (OTP) codes to your end customers when necessary.

Lifecycle	Description
Create	After successfully onboarding a customer and verifying their phone number through your application, you should the respective phone number on Wise.
Update	If customer updated their phone number, update the respective phone number on Wise.
[Delete]((/api-reference/user-security#delete-phone-number)	If customer has removed or lose access to the phone number, delete the respective phone number on Wise.
We only allow one phone number per customer. And we do not allow multiple customer to share the same phone number.
Verify Flow

Once an end customer's phone number is set up. It can now be used as an authentication factor to clear an SCA challenge.

Image below illustrates the interaction between Partner and Wise.

Steps
Partner makes an HTTP call to get balance account statement which is a SCA protected endpoint.
Get Balance Account Statement - Request
curl -X GET https://api.wise-sandbox.com/v1/profiles/{{profileId}}/balance-statements/{{balanceId}}/statement.json \
      ?currency=EUR \
      &intervalStart=2023-01-01T00:00:00.000Z \
      &intervalEnd=2023-01-15T23:59:59.999Z \
      &type=COMPACT \
      -H 'Authorization: Bearer <your api token>'
Wise rejects the request with status 403 Forbidden. Please see the example response on the right.
Get Balance Account Statement - Response
HTTP/1.1 403 Forbidden
Date: Wed, 06 Dec 2023 08:57:34 GMT
x-2fa-approval: bb676aeb-7c4d-4930-bb55-ab949fd3fd87
x-2fa-approval-result: REJECTED
...other headers
Partner gets status of a one time token to get all required challenges to clear this OTT.
Get Status of One Time Token - Request
curl -X GET https://api.wise-sandbox.com/v1/identity/one-time-token/status \
     -H 'Authorization: Bearer <your api token>' \
     -H 'One-Time-Token: bb676aeb-7c4d-4930-bb55-ab949fd3fd87'

Wise returns one time token that describes all required challenges.


For the complete list of challenges available please refer to ChallengeType.

The 'alternatives' field suggests that the end user has the option to select a trigger channel to receive the OTP code, rather than receiving it via the default channel.

Get Status of One Time Token - Response
{
  "oneTimeTokenProperties": {
    "oneTimeToken": "bb676aeb-7c4d-4930-bb55-ab949fd3fd87",
    "challenges": [
      {
        "primaryChallenge": {
          "type": "SMS",
          "viewData": {
            "attributes": {
              "userId": 6146956
            }
          }
        },
        "alternatives": [
          {
            "type": "WHATSAPP",
            "viewData": {
              "attributes": {
                "userId": 6146956
              }
            }
          },
          {
            "type": "VOICE",
            "viewData": {
              "attributes": {
                "userId": 6146956
              }
            }
          }
        ],
        "required": true,
        "passed": false
      }
    ],
    "validity": 3600,
    "actionType": "BALANCE__GET_STATEMENT",
    "userId": 6146956
  }
}
Partner verify otp by passing OTT acquired in step 2.
Verify SMS - Request
curl -X POST https://api.wise-sandbox.com/v1/one-time-token/sms/verify \
     -H 'Authorization: Bearer <your api token>'
     -H 'One-Time-Token: <one time token>'
     -d '{
        "otpCode": "111111"
    }'
Wise returns one time token properties after a successful OTP code verification.

Assuming that the challenges array field is empty, indicating that the OTT is now usable.

If you are unsure, you can always get status of a one time token again.

It is possible that the challenges array returns type of challenge. In that case, please perform the verification flow as written in our guides.

Verify SMS - Response
{
  "oneTimeTokenProperties": {
    "oneTimeToken": "9f5f5812-2609-4e48-8418-b64437c0c7cd",
    "challenges": [],
    "validity": 3600
  }
}
Partner calls Get Balance Statement with the approved OTT.
Get Balance Account Statement - Request
curl -X GET https://api.wise-sandbox.com/v1/profiles/{{profileId}}/balance-statements/{{balanceId}}/statement.json \
         ?currency=EUR \
         &intervalStart=2023-01-01T00:00:00.000Z \
         &intervalEnd=2023-01-15T23:59:59.999Z \
         &type=COMPACT \
     -H 'Authorization: Bearer <your api token>' \
     -H 'x-2fa-approval: bb676aeb-7c4d-4930-bb55-ab949fd3fd87'
Wise returns Balance Account Statement.
Get Balance Account Statement - Response
{
   "accountHolder": {
      "type": "PERSONAL",
      "address": {
         "addressFirstLine": "Veerenni 24",
         "city": "Tallinn",
         "postCode": "12112",
         "stateCode": "",
         "countryName": "Estonia"
      },
      "firstName": "Oliver",
      "lastName": "Wilson"
   },
   "otherFields": "..."
}