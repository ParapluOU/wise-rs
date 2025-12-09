# Test your integrationCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/issue-cards/card-testing

---

Test your integration
Copy

Simulate card transactions in sandbox environment

Once everything's been set-up, we also provide you with endpoints that allow you to simulate transactions on your card, providing you with data that you can use to better optimize your flows in production.

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