# Subscribe to swift-in creditCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/receive-money/subscribe-to-swift-in-credit

---

Subscribe to swift-in credit
Copy

The webhook subscription you need to get updates about Swift payments reaching your Wise account is called swift-in.

In the payload you can see:

sender - details of the originating account from which the payment has been made.
recipient - details of the ultimate recipient of the payment - which is your customer.
fees - how much has been taken in fees by Wise and other correspondents through the Swift payment journey.
data.action.id - the ID of the transfer. You need this for reconciliation or to trigger a return.

All of these help you determine how to make the payment to your customer, the ultimate beneficiary of the payment in most cases.

Customer details in the notification payload

In the resource object in the swift-in#credit notification payload includes information about the recipient.

The recipient is the party who ultimately is to receive this payment, via your account.

Here you can see the name and account details of John Smith. Including the address and account details, and reference IDs for the transaction.

You can use this information to move money to your customer when needed.

Example `swift-in#credit` event
{
  "data": {
    "action": {
      "type": "credit",
      "id": 12345,
      "profile_id": 222,
      "account_id": 333
    },
    "resource": {
      "id": "55555",
      "uetr": "f875814b-7d44-4d1b-a499-123456789abcde",
      "reference": "/RFB/BET072",
      "recipient": {
        "name": "JOHN SMITH",
        "address": "EVERGREEN AVE, 6, BRUSSELS, BE",
        "account": "BE1234567891234"
      },
      "sender": {
        "name": "GEORGE SMITH",
        "address": "EVERGREEN STREET, 10, BRUSSELS, BE",
        "account": "EE947700771111111111",
        "bank_code": {
          "value": "ABNABE2AIDJ",
          "type": "BIC"
        }
      },
      "exchange_rate": 0.8,
      "instructed_amount": {
        "value": 1000,
        "currency": "USD"
      },
      "settled_amount": {
        "value": 786.54,
        "currency": "GBP"
      },
      "fee": {
        "wise": [
          {
              "type": "conversion",
              "value": 2.76,
              "currency": "GBP"
          }
        ],
        "correspondent": [
          {
            "value": 4.5,
            "currency": "USD"
          },
          {
            "value": 2.1,
            "currency": "GBP"
          },
          {
            "value": 5,
            "currency": "GBP"
          }
        ]
      },
      "transaction_time": "2023-08-21T12:34:56Z"
    },
    "occurred_at": "2023-08-21T12:34:56Z"
  },
  "subscription_id": "01234567-89ab-cdef-0123-456789abcdef",
  "event_type": "swift-in#credit",
  "schema_version": "3.0.0",
  "sent_at": "2023-08-21T12:34:58Z"
}

Swift receive credit events
Triggering event type: swift-in#credit
Profile level subscriptions: Supported
Application level subscriptions: Supported

This webhook will be sent every time a Swift message is received and a credit is deposited into a balance account.

If you also subscribe to balances#credit or transfers#state-change on the same profile or application you may experience some duplication of webhook events. You should make allowances for this and handle it in your implementation.

Version 3.0.0
Schema
data.action.typestring

Type of action.
Mandatory
Format: always credit

data.action.idinteger

TransferID of the credit.
Mandatory

data.action.profile_idinteger

ProfileID for the transfer.
Mandatory

data.action.account_idinteger

AccountID for the transfer.
Mandatory

data.resource.idstring

ID of the webhook.
Mandatory

data.resource.uetrguid

Unique identifier created by the scheme for each payment and passed through the scheme to Wise.
Mandatory
Length: 36 Characters

data.resource.referencestring

Reference provided by the ultimate sender for the payment.
Optional
Length: Min 0 Max 140 Characters

data.resource.recipient.namestring

Name of the ultimate recipient.
Optional
Length: Min 1 Max 140 Characters

data.resource.recipient.addressstring

Address of the ultimate recipient.
Optional
Length: Min 1 Max 140 Characters

data.resource.recipient.accountstring

This can be either an IBAN, BBAN, AccountId or a Descriptor (another kind of identifier).
Optional
Length: Min 1 Max 36 Characters

data.resource.sender.namestring

Name of the ultimate sender.
Optional
Length: Min 1 Max 140 Characters

data.resource.sender.addressstring

Address of the ultimate sender.
Optional
Length: Min 1 Max 140 Characters

data.resource.sender.accountstring

This can be either an IBAN, BBAN, AccountId or a Descriptor (another kind of identifier).
Optional
Length: Min 1 Max 36 Characters

data.resource.sender.bank_code.valuestring

Value of the ultimate sender’s bank code. This can be either a BIC code or bank institution name.
Optional

data.resource.sender.bank_code.typestring

Type of the ultimate sender’s bank code. This can be either BIC or BANK NAME.
Optional

data.resource.exchange_ratedouble

In case the originally instructed amount was converted by Wise or any other intermediary agent, this field contains the exchange rate used. If no conversion took place, the exchange rate will be 1.0.
Mandatory

data.resource.instructed_amountobject

Amount originally instructed by the ultimate sender.
Optional

data.resource.instructed_amount.valuedouble

Value of the amount originally instructed by the ultimate sender.
Mandatory

data.resource.instructed_amount.currencystring

Currency of the amount originally instructed by the ultimate sender.
Mandatory
Length: Exactly 3 Characters
Format: 3-letter ISO currency code

data.resource.settled_amount.valuedouble

Value of the amount settled to the balance account.
Mandatory

data.resource.settled_amount.currencystring

Currency of the amount settled to the balance account.
Mandatory
Length: Exactly 3 Characters
Format: 3-letter ISO currency code

data.resource.fee.wise[]list

List of fees applied by Wise (e.g., conversion fee). If this list is empty, no fees were applied by Wise.
Optional

data.resource.fee.wise[].typestring

Type of fee.
Mandatory
Format: conversion,

data.resource.fee.wise[].valuedouble

Value of fee.
Mandatory

data.resource.fee.wise[].currencystring

Currency of fee.
Mandatory
Length: Exactly 3 Characters
Format: 3-letter ISO currency code

data.resource.fee.correspondent[]list

List of fees declared by correspondents (e.g., handling fee). If this list is empty, no fees were declared by correspondents.
Optional

data.resource.fee.correspondent[].valuedouble

Value of fee.
Mandatory

data.resource.fee.correspondent[].currencystring

Currency of fee.
Mandatory
Length: Exactly 3 Characters
Format: 3-letter ISO currency code

data.resource.transaction_timedatetime

Date and time at which the associated transaction was created. Time in UTC.
Mandatory
Length: Exactly 20 Characters
Format: YYYY-MM-DDTHH:MM:SSZ

data.occurred_atdatetime

Date and time at which the amount was credited to the balance account. Time in UTC.
Mandatory
Length: Exactly 20 Characters
Format: YYYY-MM-DDTHH:MM:SSZ

Example `swift-in#credit` event (JSON format)
{
  "data": {
    "action": {
      "type": "credit",
      "id": 12345,
      "profile_id": 222,
      "account_id": 333
    },
    "resource": {
      "id": "55555",
      "uetr": "f875814b-7d44-4d1b-a499-123456789abcde",
      "reference": "/RFB/BET072",
      "recipient": {
        "name": "JOHN SMITH",
        "address": "EVERGREEN AVE, 6, BRUSSELS, BE",
        "account": "BE1234567891234"
      },
      "sender": {
        "name": "GEORGE SMITH",
        "address": "EVERGREEN STREET, 10, BRUSSELS, BE",
        "account": "EE947700771111111111",
        "bank_code": {
          "value": "ABNABE2AIDJ",
          "type": "BIC"
        }
      },
      "exchange_rate": 0.8,
      "instructed_amount": {
        "value": 1000,
        "currency": "USD"
      },
      "settled_amount": {
        "value": 786.54,
        "currency": "GBP"
      },
      "fee": {
        "wise": [
          {
              "type": "conversion",
              "value": 2.76,
              "currency": "GBP"
          }
        ],
        "correspondent": [
          {
            "value": 4.5,
            "currency": "USD"
          },
          {
            "value": 2.1,
            "currency": "GBP"
          },
          {
            "value": 5,
            "currency": "GBP"
          }
        ]
      },
      "transaction_time": "2023-08-21T12:34:56Z"
    },
    "occurred_at": "2023-08-21T12:34:56Z"
  },
  "subscription_id": "01234567-89ab-cdef-0123-456789abcdef",
  "event_type": "swift-in#credit",
  "schema_version": "3.0.0",
  "sent_at": "2023-08-21T12:34:58Z"
}
Create Application Webhook Subscription

POST /v3/applications/{{clientKey}}/subscriptions

{{clientKey}} can be received upon obtaining client credentials from our tech support.

All fields listed below are required for creating a webhook subscription.

Request
nametext

A custom name for your webhook to ease with identification

trigger_ontext

Choose from a list of available events

delivery.versiontext

The event representation semantic version

delivery.urltext

Required. The URL where your server will be listening for events.

Response
IDtext

UUID that uniquely identifies the subscription

nametext

A custom name for your webhook to ease with identification

trigger_ontext

Choose from a list of available events

delivery.versiontext

The event representation semantic version

delivery.urltext

The URL where your server will be listening for events.

scope.domaintext

Scope of this subscription, always "application" in this case

scope.idtext

Client key used to create this subscription

created_by.typetext

Creator type. Always application in this case

created_by.idtext

Client ID of the creator. Not always the same as the client key

created_attext

Timestamp of when the subscription was created

Example Request
curl -X POST \
  "https://api.wise-sandbox.com/v3/applications/{{clientKey}}/subscriptions" \
  -H "Authorization: Bearer <your client credentials token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Webhook Subscription #1",
    "trigger_on": "transfers#state-change",
    "delivery": {
      "version": "2.0.0",
      "url": "https://your.webhook.url/12345"
    }
  }'
Example Response
{
  "id": "72195556-e5cb-495e-a010-b37a4f2a3043",
  "name": "Webhook Subscription #1",
  "delivery": {
    "version": "2.0.0",
    "url": "https://your.webhook.url/12345"
  },
  "trigger_on": "transfers#state-change",
  "scope": {
    "domain": "application",
    "id": "<your client key>"
  },
  "created_by": {
    "type": "application",
    "id": "<your client ID>"
  },
  "created_at": "2019-10-10T13:55:57Z"
}