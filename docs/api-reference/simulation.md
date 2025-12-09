# SimulationCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/api-reference/simulation

---

Simulation
Copy

You can simulate payment processing by changing transfer statuses using these endpoints.

These features are limited to sandbox only.

Operations

GET
/v1/simulation/transfers/{{transferId}}/processing
POST
/v1/simulation/profiles/{{profileId}}/verifications
POST
/v1/simulation/balance/topup
POST
/v2/simulation/spend/profiles/{{profileId}}/cards/{{cardToken}}/transactions/authorisation
POST
/v1/simulation/spend/profiles/{{profileId}}/cards/{{cardToken}}/transactions/clearing
POST
/v1/simulation/spend/profiles/{{profileId}}/cards/{{cardToken}}/transactions/reversal
POST
/v2/simulation/spend/profiles/{{profileId}}/cards/{{cardToken}}/transactions/authorisation
GET
/v2/simulation/spend/profiles/{{profileId}}/cards/{{cardToken}}/transaction
POST
/v1/simulation/spend/profiles/{{profileId}}/cards/{{cardToken}}/production
GET
/v2/simulation/profiles/{{profileId}}/kyc-reviews/{{kycReviewId}}/requirements
POST
/v2/simulation/profiles/{{profileId}}/kyc-reviews/{{kycReviewId}}/requirements/add
POST
/v2/simulation/profiles/{{profileId}}/kyc-reviews/{{kycReviewId}}/requirements/submit
POST
/v2/simulation/profiles/{{profileId}}/kyc-reviews/{{kycReviewId}}/verify
POST
/v1/simulation/profiles/{{profileId}}/bank-transactions/import
Simulate transfer state change
Simulation doesn't work with email transfers.
You need to fund transfer before calling simulation endpoints. Calling of processing is required even after the funding call transfer state has changed to processing automatically.
While transfer state simulation calls will respond with 200 in real time, the process internally is asynchronous. Please ensure you give at least 5 seconds in between simulation calls

GET /v1/simulation/transfers/{{transferId}}/processing

Changes transfer status from incoming_payment_waiting to processing.

GET /v1/simulation/transfers/{{transferId}}/funds_converted

Changes transfer status from processing to funds_converted. Please refer to our regional guides for any special regional requirements when simulating payments.


GET /v1/simulation/transfers/{{transferId}}/outgoing_payment_sent

Changes transfer status from funds_converted to outgoing_payment_sent.


GET /v1/simulation/transfers/{{transferId}}/bounced_back

Changes transfer status from outgoing_payment_sent to bounced_back.


GET /v1/simulation/transfers/{{transferId}}/funds_refunded

Changes transfer status from bounced_back to funds_refunded. Please note that this simulation will not trigger a refund webhook.

Response

Transfer entity with changed status.

Example Request
curl -X GET \
  https://api.wise-sandbox.com/v1/simulation/transfers/{{transferId}}/processing \
  -H 'Authorization: Bearer <your api token>'
Example Response
{
  "id": 15574445,
  "user": 294205,
  "targetAccount": 7993919,
  "sourceAccount": null,
  "quote": 113379,
  "status": "processing",
  "reference": "good times",
  "rate": 1.2151,
  "created": "2017-03-14 15:25:51",
  "business": null,
  "transferRequest": null,
  "details": {
    "reference": "good times"
  },
  "hasActiveIssues": false,
  "sourceValue": 1000,
  "sourceCurrency": "EUR",
  "targetValue": 895.32,
  "targetCurrency": "GPB"
}
Simulate verification state change

Use these endpoints to simulate a profile's verification state changing to PASSED.

This is useful for testing the profiles#verification-state-change webhook and is a prerequisite for setting up a Multi-Currency Account (MCA).

Verify a specific profile

POST /v1/simulation/profiles/{{profileId}}/verifications

Accepts both user and application API tokens.

Path parameter profileId is required to identify the profile you wish to verify.

No content is returned with a 200 response code.

Verify all profiles for a user

POST /v1/simulation/verify-profile

This endpoint verifies all profiles associated with the authenticated user.

Accepts user API tokens only. The profiles are identified via the token.

No content is returned with a 200 response code.

Example Request
curl -X POST \
  https://api.wise-sandbox.com/v1/simulation/profiles/{{profileId}}/verifications \
  -H 'Authorization: Bearer <your api token>'
Example Response - 200
No Content
Simulate a balance top-up

POST /v1/simulation/balance/topup

Simulates a top-up so that a balance can be used to fund transfers and/or card spend.

Request
profileId (required)string

The profile ID linked to the balance account

balanceId (required)string

The ID of the balance account that is going to receive the funds

currency (required)string

The currency to top up the balance account in. Must be the same currency as the balance account

amount (required)float

The amount to top up the balance account with

channel (optional)string

Type of top-up. Available values: TRANSFER or CARD. Not providing a channel will default to CARD

Response

Returns a simulated response for a successful balance topup.

transactionIdstring

The ID of the top up transaction

statestring

The state of the transaction. COMPLETED is always returned when using this endpoint

balancesAfter.idstring

The ID of the balance account

balancesAfter.valuefloat

The new amount available in the balance account

balancesAfter.currencystring

The currency of the balance account

Example Request
curl -X POST \
  https://api.wise-sandbox.com/v1/simulation/balance/topup \
  -H 'Authorization: Bearer <your api token>' \
  -H 'Content-Type: application/json' \
  -d '{
    "profileId": 2,
    "balanceId": 5,
    "currency": "EUR",
    "amount": 100,
    "channel": "TRANSFER"
  }'
Example Response
{
  "transactionId": 5,
  "state": "COMPLETED",
  "balancesAfter": [
    {
      "id": 5,
      "value": 100.00,
      "currency": "EUR"
    }
  ]
}
Simulate a card transaction authorisation

POST /v2/simulation/spend/profiles/{{profileId}}/cards/{{cardToken}}/transactions/authorisation

Simulates a card transaction authorisation request in the sandbox environment. It can simulate ATM withdrawals, purchasing and ECOM transactions. This is an authorisation hold, where funds are held, but not yet captured by the acquirer.

View Request for Transaction Type:

Select a Transaction Type
Response

Returns a simulated card authorisation.

referenceObject

The transaction reference. This can be used in a following request for Reversal and Clearing

errortext

An error returned by the transaction, if it exists

Example Response
{
  "reference": {
    "transaction": {
      "acquirer": {
        "institutionId": "430010",
        "name": "ACQUIRER NAME",
        "city": "CITY NAME",
        "merchantCategoryCode": 5499,
        "country": "GB",
        "acceptorTerminalId": "TERMID01",
        "acceptorIdCode": "CARD ACCEPTOR",
        "forwardingInstitutionId": "400050"
      },
      "card": {
        "token": "59123122-223d-45f9-b840-0ad4a4f80937",
        "schemeName": "VISA",
        "pan": "4242424242424242",
        "pin": "1234",
        "cvv1": "123",
        "icvv": "456",
        "cvv2": "789",
        "expiration": [
          2029,
          7
        ],
        "sequenceNumber": 1,
        "profileId": 2,
        "userId": 5,
        "cardStatus": "ACTIVE",
        "country": "SG",
        "currencies": [
          "SGD"
        ]
      },
      "pos": {
        "type": "CHIP_AND_PIN",
        "acceptsOnlinePins": true,
        "maxPinLength": 12,
        "supports3ds": false,
        "hasChip": true
      },
      "transactionStartTime": 1667541087.047643305,
      "stan": "363054",
      "schemeTransactionId": "932290252416153",
      "retrievalReferenceNum": "230805363054"
    },
    "requestMti": "0200",
    "authorizationIdResponse": "123646"
  },
  "error": null
}
Simulate a card transaction clearing

POST /v1/simulation/spend/profiles/{{profileId}}/cards/{{cardToken}}/transactions/clearing

Clearing simulation doesn't work with Mastercard.

Simulates a transaction clearing request in the sandbox environment. This is done after the authorisation. The ref field can be copied from the reference object in the authorisation response.

To clear a previous authorisation, the ref details must match the previous authorisation request. The amount does not have to match the previous authorisation request, it can be more or less than the authorisation request amount.

Request
amount.valuedecimal

Transaction amount

amount.currencytext

Currency code

transactionTypetext

The type of the transaction. Use the same transaction type from the previous Authorisation request

refObject

The transaction reference. This can be obtained from a previous Authorisation request

Response

Simulates a transaction clearing request in the sandbox environment. This is done after the authorisation.

referenceObject

The transaction reference

errortext

An error returned by the transaction, if it exists

Example Request
curl -X POST \
  'https://api.wise-sandbox.com/v1/simulation/spend/profiles/{{profileId}}/cards/{{cardToken}}/transactions/clearing' \
  -H 'Authorization: Bearer <your api token>' \
  -H 'Content-Type: application/json' \
  -d '{
    "amount": {
      "value": 10.00
      "currency": "SGD"
    },
    "transactionType": "GOODS_AND_SERVICES",
    "ref": {
      "transaction": {
        "acquirer": {
          "institutionId": "430010",
          "name": "ACQUIRER NAME",
          "city": "CITY NAME",
          "merchantCategoryCode": 5499,
          "country": "GB",
          "acceptorTerminalId": "TERMID01",
          "acceptorIdCode": "CARD ACCEPTOR",
          "forwardingInstitutionId": "400050"
        },
        "card": {
          "token": "59123122-223d-45f9-b840-0ad4a4f80937",
          "schemeName": "VISA",
          "pan": "4242424242424242",
          "pin": "1234",
          "cvv1": "123",
          "icvv": "456",
          "cvv2": "789",
          "expiration": [
            2029,
            7
          ],
          "sequenceNumber": 1,
          "profileId": 2,
          "userId": 5,
          "cardStatus": "ACTIVE",
          "country": "SG",
          "currencies": [
            "SGD"
          ]
        },
        "pos": {
          "type": "CHIP_AND_PIN",
          "acceptsOnlinePins": true,
          "maxPinLength": 12,
          "supports3ds": false,
          "hasChip": true
        },
        "transactionStartTime": 1667541087.047643305,
        "stan": "363054",
        "schemeTransactionId": "932290252416153",
        "retrievalReferenceNum": "230805363054"
      },
      "requestMti": "0200",
      "authorizationIdResponse": "123646"
    }
  }'
Example Response
{
  "reference": {
    "transaction": {
      "acquirer": {
        "institutionId": "430010",
        "name": "ACQUIRER NAME",
        "city": "CITY NAME",
        "merchantCategoryCode": 5499,
        "country": "GB",
        "acceptorTerminalId": "TERMID01",
        "acceptorIdCode": "CARD ACCEPTOR",
        "forwardingInstitutionId": "400050"
      },
      "card": {
        "token": "59123122-223d-45f9-b840-0ad4a4f80937",
        "schemeName": "VISA",
        "pan": "4242424242424242",
        "pin": "1234",
        "cvv1": "123",
        "icvv": "456",
        "cvv2": "789",
        "expiration": [
          2029,
          7
        ],
        "sequenceNumber": 1,
        "profileId": 2,
        "userId": 5,
        "cardStatus": "ACTIVE",
        "country": "SG",
        "currencies": [
          "SGD"
        ]
      },
      "pos": {
        "type": "CHIP_AND_PIN",
        "acceptsOnlinePins": true,
        "maxPinLength": 12,
        "supports3ds": false,
        "hasChip": true
      },
       "transactionStartTime": 1667541087.047643305,
        "stan": "363054",
        "schemeTransactionId": "932290252416153",
        "retrievalReferenceNum": "230805363054"
      },
      "requestMti": "0200",
      "authorizationIdResponse": "123646"
    }
  "error": null
}
Simulate a card transaction reversal

POST /v1/simulation/spend/profiles/{{profileId}}/cards/{{cardToken}}/transactions/reversal

Simulates a transaction reversal request in the sandbox environment. The amount.value field can be set to 0 for a full reversal, or a positive value that represents the intended final value of the transaction after a partial reversal.

For example, if a transaction value was originally 10 SGD, and the intended final value is 3 SGD, amount.value should be set to 3. This means there is a partial refund of 7 SGD.

Request
amount.valuedecimal

Transaction amount

amount.currencytext

Currency code

transactionTypetext

The type of the transaction. Use the same transaction type from the previous Authorisation request

refObject

The transaction reference. This can be obtained from a previous Authorisation request

Response

Returns a simluated card reversal, while the funds are not yet cleared.

referenceObject

The transaction reference

errortext

An error returned by the transaction, if it exists

Example Request
curl -X POST \
  'https://api.wise-sandbox.com/v1/simulation/spend/profiles/{{profileId}}/cards/{{cardToken}}/transactions/reversal' \
  -H 'Authorization: Bearer <your api token>' \
  -H 'Content-Type: application/json' \
  -d '{
    "amount": {
      "value": 3.00
      "currency": "SGD"
    },
    "transactionType": "GOODS_AND_SERVICES",
    "ref": {
      "transaction": {
        "acquirer": {
          "institutionId": "430010",
          "name": "ACQUIRER NAME",
          "city": "CITY NAME",
          "merchantCategoryCode": 5499,
          "country": "GB",
          "acceptorTerminalId": "TERMID01",
          "acceptorIdCode": "CARD ACCEPTOR",
          "forwardingInstitutionId": "400050"
        },
        "card": {
          "token": "59123122-223d-45f9-b840-0ad4a4f80937",
          "schemeName": "VISA",
          "pan": "4242424242424242",
          "pin": "1234",
          "cvv1": "123",
          "icvv": "456",
          "cvv2": "789",
          "expiration": [
            2029,
            7
          ],
          "sequenceNumber": 1,
          "profileId": 2,
          "userId": 5,
          "cardStatus": "ACTIVE",
          "country": "SG",
          "currencies": [
            "SGD"
          ]
        },
        "pos": {
          "type": "CHIP_AND_PIN",
          "acceptsOnlinePins": true,
          "maxPinLength": 12,
          "supports3ds": false,
          "hasChip": true
        },
        "transactionStartTime": 1667541087.047643305,
        "stan": "363054",
        "schemeTransactionId": "932290252416153",
        "retrievalReferenceNum": "230805363054"
      },
      "requestMti": "0200",
      "authorizationIdResponse": "123646"
    }
  }'
Example Response
{
  "reference": {
    "transaction": {
      "acquirer": {
        "institutionId": "430010",
        "name": "ACQUIRER NAME",
        "city": "CITY NAME",
        "merchantCategoryCode": 5499,
        "country": "GB",
        "acceptorTerminalId": "TERMID01",
        "acceptorIdCode": "CARD ACCEPTOR",
        "forwardingInstitutionId": "400050"
      },
      "card": {
        "token": "59123122-223d-45f9-b840-0ad4a4f80937",
        "schemeName": "VISA",
        "pan": "4242424242424242",
        "pin": "1234",
        "cvv1": "123",
        "icvv": "456",
        "cvv2": "789",
        "expiration": [
          2029,
          7
        ],
        "sequenceNumber": 1,
        "profileId": 2,
        "userId": 5,
        "cardStatus": "ACTIVE",
        "country": "SG",
        "currencies": [
          "SGD"
        ]
      },
      "pos": {
        "type": "CHIP_AND_PIN",
        "acceptsOnlinePins": true,
        "maxPinLength": 12,
        "supports3ds": false,
        "hasChip": true
      },
       "transactionStartTime": 1667541087.047643305,
        "stan": "363054",
        "schemeTransactionId": "932290252416153",
        "retrievalReferenceNum": "230805363054"
      },
      "requestMti": "0200",
      "authorizationIdResponse": "123646"
    }
  "error": null
}
Simulate a card transaction refund

POST /v2/simulation/spend/profiles/{{profileId}}/cards/{{cardToken}}/transactions/authorisation

Refund simulation doesn't work with Mastercard.

Simulates a transaction refund request in the sandbox environment. A refund is a 2-step process which involves authorisation and clearing.

Authorisation
Request
postext

The point-of-sale used for the transaction. For refund transactions, use CHIP_AND_PIN or E_COMMERCE_NO_3DS

transactionTypetext

The type of the transaction. For refund transactions, use REFUND

amount.valuedecimal

Transaction amount

amount.currencytext

Currency

mccnumber

Merchant category code

cardNumbertext

Primary Account Number of the card

Response

Returns a simulated card authorisation.

referenceObject

The transaction reference. This can be used in a following request for Reversal and Clearing

errortext

An error returned by the transaction, if it exists

Example Request
curl -X POST \
  'https://api.wise-sandbox.com/v2/simulation/spend/profiles/{{profileId}}/cards/{{cardToken}}/transactions/authorisation' \
  -H 'Content-Type: application/json' \
  -H 'Authorization: Bearer <your api token>' \
  -d '{
    "pos": "CHIP_AND_PIN",
    "transactionType": "REFUND",
    "amount": {
      "value": 10,
      "currency": "SGD"
    },
    "mcc": 5499,
    "cardNumber": "4242424242424242"
  }'
Example Response
{
  "reference": {
    "transaction": {
      "acquirer": {
        "institutionId": "430010",
        "name": "ACQUIRER NAME",
        "city": "CITY NAME",
        "merchantCategoryCode": 5499,
        "country": "GB",
        "acceptorTerminalId": "TERMID01",
        "acceptorIdCode": "CARD ACCEPTOR",
        "forwardingInstitutionId": "400050"
      },
      "card": {
        "token": "59123122-223d-45f9-b840-0ad4a4f80937",
        "schemeName": "VISA",
        "pan": "4242424242424242",
        "pin": "1234",
        "cvv1": "123",
        "icvv": "456",
        "cvv2": "789",
        "expiration": [
          2029,
          7
        ],
        "sequenceNumber": 1,
        "profileId": 2,
        "userId": 5,
        "cardStatus": "ACTIVE",
        "country": "SG",
        "currencies": [
          "SGD"
        ]
      },
      "pos": {
        "type": "CHIP_AND_PIN",
        "acceptsOnlinePins": true,
        "maxPinLength": 12,
        "supports3ds": false,
        "hasChip": true
      },
      "transactionStartTime": 1667541087.047643305,
      "stan": "363054",
      "schemeTransactionId": "932290252416153",
      "retrievalReferenceNum": "230805363054"
    },
    "requestMti": "0200",
    "authorizationIdResponse": "123646"
  },
  "error": null
}
Clearing

A refund transaction can be cleared after the initial authorisation request has been made.

POST /v1/simulation/spend/profiles/{{profileId}}/cards/{{cardToken}}/transactions/clearing

Simulates a transaction clearing request in the sandbox environment. The ref field can be copied from the reference object in the authorisation response. In the clearing request, the transactionType must be set to REFUND.

To clear a previous authorisation, the ref details must match the previous authorisation request. It may take a few seconds for the transaction status to be updated.

Request
amount.valuedecimal

Transaction amount

amount.currencytext

Currency code

transactionTypetext

The type of the transaction. For refund transactions, use REFUND

refObject

The transaction reference. This can be obtained from a previous Authorisation request

Response

Returns a simluated card authorisation, after the funds have been cleared.

referenceObject

The transaction reference

errortext

An error returned by the transaction, if it exists

Example Request for Clearing after Refund
curl -X POST \
  'https://api.wise-sandbox.com/v1/simulation/spend/profiles/{{profileId}}/cards/{{cardToken}}/transactions/clearing' \
  -H 'Authorization: Bearer <your api token>' \
  -H 'Content-Type: application/json' \
  -d '{
    "amount": {
      "value": 10.00
      "currency": "SGD"
    },
    "transactionType": "REFUND",
    "ref": {
      "transaction": {
        "acquirer": {
          "institutionId": "430010",
          "name": "ACQUIRER NAME",
          "city": "CITY NAME",
          "merchantCategoryCode": 5499,
          "country": "GB",
          "acceptorTerminalId": "TERMID01",
          "acceptorIdCode": "CARD ACCEPTOR",
          "forwardingInstitutionId": "400050"
        },
        "card": {
          "token": "59123122-223d-45f9-b840-0ad4a4f80937",
          "schemeName": "VISA",
          "pan": "4242424242424242",
          "pin": "1234",
          "cvv1": "123",
          "icvv": "456",
          "cvv2": "789",
          "expiration": [
            2029,
            7
          ],
          "sequenceNumber": 1,
          "profileId": 2,
          "userId": 5,
          "cardStatus": "ACTIVE",
          "country": "SG",
          "currencies": [
            "SGD"
          ]
        },
        "pos": {
          "type": "CHIP_AND_PIN",
          "acceptsOnlinePins": true,
          "maxPinLength": 12,
          "supports3ds": false,
          "hasChip": true
        },
        "transactionStartTime": 1667541087.047643305,
        "stan": "363054",
        "schemeTransactionId": "932290252416153",
        "retrievalReferenceNum": "230805363054"
      },
      "requestMti": "0200",
      "authorizationIdResponse": "123646"
    }
  }'
Example Response
{
  "reference": {
    "transaction": {
      "acquirer": {
        "institutionId": "430010",
        "name": "ACQUIRER NAME",
        "city": "CITY NAME",
        "merchantCategoryCode": 5499,
        "country": "GB",
        "acceptorTerminalId": "TERMID01",
        "acceptorIdCode": "CARD ACCEPTOR",
        "forwardingInstitutionId": "400050"
      },
      "card": {
        "token": "59123122-223d-45f9-b840-0ad4a4f80937",
        "schemeName": "VISA",
        "pan": "4242424242424242",
        "pin": "1234",
        "cvv1": "123",
        "icvv": "456",
        "cvv2": "789",
        "expiration": [
          2029,
          7
        ],
        "sequenceNumber": 1,
        "profileId": 2,
        "userId": 5,
        "cardStatus": "ACTIVE",
        "country": "SG",
        "currencies": [
          "SGD"
        ]
      },
      "pos": {
        "type": "CHIP_AND_PIN",
        "acceptsOnlinePins": true,
        "maxPinLength": 12,
        "supports3ds": false,
        "hasChip": true
      },
       "transactionStartTime": 1667541087.047643305,
        "stan": "363054",
        "schemeTransactionId": "932290252416153",
        "retrievalReferenceNum": "230805363054"
      },
      "requestMti": "0200",
      "authorizationIdResponse": "123646"
    }
  "error": null
}
List simulated transactions for a card

GET /v2/simulation/spend/profiles/{{profileId}}/cards/{{cardToken}}/transaction?limit=10

Request
limit (optional)integer

The maximum number of transactions to return. The default value is 10.

Response

Returns a list of transactions, in descending order of creation time. To retrieve more details of a transaction, use the get card transaction by ID endpoint.

transactionIdnumber

ID of the transaction

creationTimetext

Time when the card transaction was created

Example Request
curl -X GET \
  'https://api.wise-sandbox.com/v2/simulation/spend/profiles/{{profileId}}/cards/{{cardToken}}/transactions?limit=10' \
  -H 'Authorization: Bearer <your api token>'
Example Response
[
  {
    "transactionId": 5028,
    "creationTime": 1723600773.748887000
  },
  {
    "transactionId": 5027,
    "creationTime": 1723600659.095692000
  },
  {
    "transactionId": 5026,
    "creationTime": 1723600648.133955000
  },
  {
    "transactionId": 5025,
    "creationTime": 1723545470.449374000
  },
  {
    "transactionId": 5024,
    "creationTime": 1723545430.790566000
  },
  {
    "transactionId": 5023,
    "creationTime": 1723545400.717954000
  },
  {
    "transactionId": 5022,
    "creationTime": 1723538943.303342000
  },
  {
    "transactionId": 5021,
    "creationTime": 1723538794.965883000
  },
  {
    "transactionId": 5020,
    "creationTime": 1723538784.678004000
  },
  {
    "transactionId": 5019,
    "creationTime": 1723538626.199095000
  }
]
Simulate card production state change

POST /v1/simulation/spend/profiles/{{profileId}}/cards/{{cardToken}}/production

Please ensure that you have created a physical card order with KIOSK_COLLECTION delivery method and that you produce the card before calling this endpoint to simulate various card production statuses.

Simulate a card production status in the sandbox environment.

Request
statustext

The intended production status of the card. The status can be whether PRODUCED or PRODUCTION_ERROR.

errorCode (optional)text

The error code to simulate. This field should be set only if the status is PRODUCTION_ERROR.

Example Request
curl -X POST \
  'https://api.wise-sandbox.com/v1/simulation/spend/profiles/{{profileId}}/cards/{{cardToken}}/production' \
  -H 'Authorization: Bearer <your api token>' \
  -H 'Content-Type: application/json' \
  -d '{
    "status": "PRODUCTION_ERROR", 
    "errorCode": "PRT_NOT_REACHABLE" 
  }'
Simulate Listing all KYC requirements

GET /v2/simulation/profiles/{{profileId}}/kyc-reviews/{{kycReviewId}}/requirements

This endpoint lists all blocking requirements for a KYC (Know Your Customer) review. These outstanding KYC requirements typically act as "blocking requirements", preventing actions like creating transfers.

Request
profileId (required)String

The profile ID linked to the

kycReviewId (required)UUID

The ID of the existing Kyc review you want to get the blocking requirement

Response

List of KYC review items.

keytext

KYC Review Item Key

statetext

State of the KYC Review Item. Could be NOT_PROVIDED, IN_REVIEW, VERIFIED, REJECTED.

descriptiontext

Description of the KYC Review Item. Can be null.

Example Request
curl -X GET \
  https://api.wise-sandbox.com/v2/simulation/profiles/{{personal-profile-id}}/kyc-reviews/{{kyc-review-id}}/requirements \
  -H 'Authorization: Bearer <user access token>'
Example Response - 200 OK
{
  "requirements": [
    {
      "key": "ID_DOCUMENT_WITH_LIVENESS",
      "state": "IN_REVIEW",
      "description": null
    }
  ]
}
Example Response - 404 NOT FOUND
When KYC Review ID is not found
{
  "code": "kyc_review.not_found",
  "category": "CLIENT",
  "description": "KycReview not found",
  "details": null
}
Simulate adding new requirement

POST /v2/simulation/profiles/{{profileId}}/kyc-reviews/{{kycReviewId}}/requirements/add

This endpoint creates new requirement for a KYC (Know Your Customer) review for a business profile.

This can only be used for business profiles, it creates a static BUSINESS_USE_CASE requirement only.

Request
profileId (required)String

The profile ID requiring the new requirement to be added

kycReviewId (required)UUID

The ID of the existing Kyc review you want to get the blocking requirement

requestedAction (required)String

The action to be carried out, should be either APPROVE or REJECT

requirement (required)String

Requirement to simulate hosted journey for

Response

No content is returned with a 202 response code.

Example Request
curl -X POST \
  https://api.wise-sandbox.com/v2/simulation/profiles/{{business-profile-id}}/kyc-reviews/{{kyc-review-id}}/requirements/add \
  -H 'Authorization: Bearer <user access token>'
Example Response - 202 ACCEPTED
No Content
Example Response - 400 BAD REQUEST
When the profile is not a business profile
{
  "code": "invalid-request",
  "category": "CLIENT",
  "description": "Profile 123L is a personal type, not permitted to simulate evidence request",
  "details": null
}
Simulate requirement submission

POST /v2/simulation/profiles/{{profileId}}/kyc-reviews/{{kycReviewId}}/requirements/submit

This endpoint simulates providing everything needed for a particular KYC requirement, this can be used in place of going through a hosted journey on the browser.

This functionality only works for the following requirements: TRANSFER_PURPOSE, ID_DOCUMENT_WITH_LIVENESS, SOURCE_OF_FUNDS, and any other that has the ___ delimiter in them

Request
profileId (required)String

The profile ID requiring the new requirement to be added

kycReviewId (required)UUID

The ID of the existing Kyc review you want to get the blocking requirement

requirement (required)String

Requirement to simulate hosted journey for

Response

No content is returned with a 202 response code.

Example Request
curl -X GET \
  https://api.wise-sandbox.com/v2/simulation/profiles/{{personal-profile-id}}/kyc-reviews/{{kyc-review-id}}/requirements/submit \
  -H 'Authorization: Bearer <user access token>' \
  -D '{
    "requirement": "ACCOUNT_PURPOSE"
  }'
Example Response - 202 ACCEPTED
No Content
Example Response - 400 BAD REQUEST
When the requirement is not requested for the Kyc review
{
  "code": "invalid-request",
  "category": "CLIENT",
  "description": "Invalid request. KycRequirement {ACCOUNT_PURPOSE} was not found for KycReview",
  "details": null
}
Simulate verifying a KYC review

POST /v2/simulation/profiles/{{profileId}}/kyc-reviews/{{kycReviewId}}/verify

This endpoint verifies requirement(s) on an existing KYC (Know Your Customer) review, can be used after a user has provided input for requirement needed through the hosted journey or simulated the requirement submission.

Request
profileId (required)String

The profile ID

kycReviewId (required)UUID

The ID of the existing Kyc review you want to get the blocking requirement

requestedAction (required)String

The action to be carried out, should be either APPROVE or REJECT

requirements (optional)List<String>

Zero or more requirements to be verified, if this is not passed it verifies all blocking requirements

Response

No content is returned with a 202 response code.

Example Request
curl -X POST \
  https://api.wise-sandbox.com/v2/simulation/profiles/{{personal-profile-id}}/kyc-reviews/{{kyc-review-id}}/verify \
  -H 'Authorization: Bearer <user access token>'
  -D '{
    "requestedAction": "APPROVE",
    "requirements": ["ACCOUNT_PURPOSE_V2"]
  }'
Example Response - 202 ACCEPTED
No Content
Example Response - 400 BAD REQUEST
When any unrelated action passed in request body
{
  "code": "kyc_review.state_update_not_allowed",
  "category": "CLIENT",
  "description": "Invalid action passed in request, action: APPROVED",
  "details": null
}
Simulate bank transaction import

POST /v1/simulation/profiles/{{profileId}}/bank-transactions/import

This endpoint simulates a bank transfer into a profile's account details. This will create a payment into the user's account details and balance for the specified amount and currency.

Please refer to bank-account-details api reference for more information on setting up bank details to receive money.

This functionality only works for the following currencies: USD, EUR, GBP

Request
currency (required)String

The currency of the bank account details that you want to make payment to

amount (required)Double

The payment amount

Response

No content is returned with a 201 response code.

Example Request
curl -X POST \
  https://api.wise-sandbox.com/v1/simulation/profiles/{{profileId}}/bank-transactions/import \
  -H 'Authorization: Bearer <your api token>' \
  -D '{ 
    "currency" : "USD", 
    "amount": 2000.0
  }'
Example Response - 201 CREATED
No Content
Example Response - 400 BAD REQUEST
When account details for the specified currency do not exist
{
  "code": "not.valid",
  "message": "No account details found for profile currency pair"
}