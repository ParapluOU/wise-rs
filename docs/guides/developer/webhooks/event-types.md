# Event Types

Source: https://docs.wise.com/guides/developer/webhooks/event-types

---

Event Types

Learn the different event types.

The following event types are available. Please note that some event types are restricted to certain types of webhook subscription.

Event type	Profile subscriptions	Application subscriptions
transfers#state-change	supported	supported
transfers#active-cases	supported	not supported
account-details-payment#state-change	supported	supported
balances#credit	supported	not supported
balances#update	supported	supported
balances#account-state-change	not supported	supported
profiles#verification-state-change	not supported	supported
batch-payment-initiations#state-change	not supported	supported
transfers#payout-failure	supported	supported
transfers#refund	supported	supported
swift-in#credit	supported	supported
cards#transaction-state-change	not supported	supported
profiles#cdd-check-state-change	not supported	supported
cards#card-status-change	not supported	supported
cards#card-order-status-change	not supported	supported
cards#card-production-status-change	not supported	supported
partner-support#case-changed	not supported	supported
transaction-disputes#update	not supported	supported
bulk-settlement#payment-received	not supported	supported
users#state-change	not supported	supported
kyc-reviews#state-change	not supported	supported
cards#3ds-challenge	not supported	supported
profiles#overdraft-limit-threshold	not supported	supported
account-details-order#order-state-change	not supported	supported
Transfer state change event
Event type: transfers#state-change
Profile level subscriptions: Supported
Application level subscriptions: Supported

This event will be triggered every time a transfer's status is updated. Each event contains a timestamp. As we do not guarantee the order of events, please use data.occurred_at to reconcile the order.

If you would like to subscribe to transfer state change events, please use transfers#state-change when creating your subscription.

Note that for all topup-to-balance transfers created, transfers#state-change events will NOT be triggered. To listen to these, subscribe to balances#update webhook.

Schema
data.resource.typestring

Transfer resource type (always transfer)

data.resource.idinteger (long)

ID of the transfer

data.resource.profile_idinteger (long)

ID of the profile that owns the transfer

data.resource.account_idinteger (long)

ID of transfer's recipient account

data.current_statestring

Current transfer state (see transfer statuses)

data.previous_statestring or null

Previous transfer state (see transfer statuses) - Note that this will be null for newly created transfers.

data.occurred_atdatetime

When the state change occurred.

Transfer Statuses

You should use a subscription to application webhooks to keep the statuses of your local transfer data storage up to date. The transfers will move through the following statuses in the happy path.

Incoming Payment Waiting ⇒ Processing ⇒ Funds Converted ⇒ Outgoing Payment Sent

Outgoing Payment Sent is the final state of the normal flow. If the payment fails, the following problematic flow will occur. An example would be if the recipient bank account doesn’t exist or the data is entered incorrectly and the payment is returned to Wise. The problematic state flow of transfers is:

Outgoing Payment Sent ⇒ Bounced Back ⇒ Processing ⇒ Cancelled ⇒ Funds Refunded

Most bounce backs occur within 2-3 business days, however this could happen up to several weeks after a transfer is sent.

Transfer state flow

NB: Transfers support rollback transitions. This allows transfers to return back to previous states.

See below for the full list of transfer statuses and what they mean in the order of occurrence.

incoming_payment_waiting – The transfer has been submitted and Wise is waiting for funding to be initiated.

incoming_payment_initiated - The funding has been initiated but the money has not yet arrived in Wise's account.

processing – We have received the funds and are processing the transfer. Processing is a generic term that means we’re doing behind-the-scene activities before the money gets sent to the recipient, such as AML, compliance and fraud checks.

funds_converted – All compliance checks have been completed for the transfer and funds have been converted from the source currency to the target currency.

outgoing_payment_sent – Wise has paid out funds to the recipient. This is the final state of the transfer, assuming funds will not be returned. When a transfer has this state, it doesn’t mean the money has arrived in the recipient’s bank account, just that we have sent it from ours. Note: Payment systems in different countries operate in different speeds and frequency. For example, in the UK, the payment will reach the recipient bank account within few minutes after we send the outgoing payment. However, in the US it usually takes a day until funds are received.

cancelled – This status is used when the transfer was never funded and therefore never processed. This is a final state of the transfer.

funds_refunded – Transfer has been refunded. This is a final state of the transfer.

bounced_back – Transfer has bounced back but has not been cancelled nor refunded yet. This is not a final transfer state, it means the transfer will either be delivered after a delay or it will turn to funds_refunded state.

charged_back - This status is used when we have a problem debiting payer's account or the payer requested money back. Chargeback can happen from any other state.

unknown - This status is used when we don’t have enough information to move the transfer into a final state. We send out an email for more information. For example sender account details to refund money back.

Keep in mind the transfer statuses in our API have different names than what you’ll see on our website or app. That’s because we use more consumer friendly language in the front end of our products. For example "Completed" on our website means outgoing_payment_sent in the API.

You should use the following descriptions in your website or app for the potential statuses we return:

Status	Description
incoming_payment_waiting	"On its way to Wise"
incoming_payment_initiated	"On its way to Wise"
processing	"Processing"
funds_converted	"Processing"
outgoing_payment_sent	"Sent"
charged_back	"Charged back"
cancelled	"Cancelled"
funds_refunded	"Refunded"
bounced_back	"Bounced back"
unknown	"Unknown"
Example `transfers#state-change` event
{
  "data": {
    "resource": {
      "type": "transfer",
      "id": 111,
      "profile_id": 222,
      "account_id": 333
    },
    "current_state": "processing",
    "previous_state": "incoming_payment_waiting",
    "occurred_at": "2020-01-01T12:34:56Z"
  },
  "subscription_id": "01234567-89ab-cdef-0123-456789abcdef",
  "event_type": "transfers#state-change",
  "schema_version": "2.0.0",
  "sent_at": "2020-01-01T12:34:56Z"
}
Transfer active cases event
Event type: transfers#active-cases
Profile level subscriptions: Supported
Application level subscriptions: Not Supported

This event will be triggered every time a transfer's list of active cases is updated. Active cases indicate potential problems with transfer processing.

If you would like to subscribe to transfer active cases events, please use transfers#active-cases when creating your subscription.

Schema
data.resource.typestring

Transfer resource type (always transfer)

data.resource.idinteger

ID of the transfer

data.resource.profile_idinteger

ID of the profile that owns the transfer

data.resource.account_idinteger

ID of transfer's recipient account

data.active_caseslist of strings

Ongoing issues related to the transfer

Example `transfers#active-cases` event
{
  "data": {
    "resource": {
      "type": "transfer",
      "id": 111,
      "profile_id": 222,
      "account_id": 333
    },
    "active_cases": ["deposit_amount_less_invoice"]
  },
  "subscription_id": "01234567-89ab-cdef-0123-456789abcdef",
  "event_type": "transfers#active-cases",
  "schema_version": "2.0.0",
  "sent_at": "2020-01-01T12:34:56Z"
}
Transfer payout failure events
Event type: transfers#payout-failure
Profile level subscriptions: Supported
Application level subscriptions: Supported

This event will be triggered every time a payout fails. The event can be used to get further information about a failed payment.

transfers#state-change event provides high level information about the state of transfers. But it doesn't provide details about payout failures.

While the transfers#state-change event is in outgoing_payment_sent state, the payout could fail for certain reasons listed below.

Using this event, you can understand the reason behind the failure and even ask your customers to fix the problem by themselves. If the failure reason code is WRONG_ACCOUNT_NUMBER for example.

Please note that not every payout failure will trigger a transfers#state-change. For example a payout might fail with MANDATE_NOT_FILLED_IN code but the corresponding transfer might stay in the same state.

We recommend to process both event types(transfers#payout-failure and transfers#state-change) separate from each other.

If you would like to subscribe to payout failure events, please use transfers#payout-failure when creating your subscription.

Schema
data.transfer_idinteger

ID of the transfer

data.profile_idinteger

ID of the profile that initiated the transfer

data.failure_reason_codestring

Code of the failure error

data.failure_descriptionstring

Description of the failure code

data.occurred_atdatetime

When the state change occurred

Table of available failure reason codes and descriptions
Code	Description
ACCOUNT_CLOSED	The recipient details are correct, but beneficiary account is closed
ACCOUNT_FROZEN	The recipient details are correct, but beneficiary account is frozen
ACCOUNT_BLOCKED	The recipient details are correct, but beneficiary account is blocked
ACCOUNT_LIMIT_REACHED	The recipient details are correct, but beneficiary account has a limit that can be unblocked by the recipient
WRONG_ACCOUNT_NUMBER	Invalid account number
WRONG_CARD_NUMBER	Invalid card number
WRONG_ACCOUNT_DETAILS	Invalid account number AND/OR invalid bank code
WRONG_ACCOUNT_TYPE	Incorrect account type
WRONG_BANK_CODE	Invalid sort/BIC/routing/etc number
WRONG_BRANCH_CODE	Invalid branch code
WRONG_NAME	Account number matches, but the name is not 100% correct
WRONG_PHONE_NUMBER	Recipient phone number is incorrect
WRONG_ID_NUMBER	Invalid recipient's ID document number
WRONG_RUT_NUMBER	Invalid recipient's RUT (Rol Único Tributario) number
TAX_ID_NOT_MATCHING	Tax ID doesn't match recipient's name
TAX_ID_SUSPENDED	Tax ID is suspended
WRONG_REFERENCE	Invalid payment reference
WRONG_PAYMENT_PURPOSE	Invalid or not accepted payment purpose
ACCOUNT_DOES_NOT_EXIST	Format is correct but this account does not exist in recipient bank
WRONG_CURRENCY	Recipient account is in different currency
WRONG_CARD_TYPE	Recipient account doesn't support payments to this card type
CANNOT_ACCEPT_FROM_3RD_PARTY	Recipient bank can't accept the payments from 3rd party
CREDITING_ACCOUNT_FORBIDDEN	Terms and Conditions of Account do not permit crediting of these funds
DUPLICATE_ENTRY	Recipient bank informs that there has been another payment in the same amount
FUNDS_NOT_EXPECTED_RETURNED	Beneficiary not expecting funds/instructed return
MANDATE_NOT_FILLED_IN	Recipient didn't fill out the mandate form on time or the email address is incorrect
BUSINESS_PAYMENTS_FORBIDDEN	Payment to business accounts are not allowed
DUPLICATE_ENTRY	Recipient bank informs that there has been another payment in the same amount
SENDER_REQUESTED_TO_CANCEL	Sender requested cancellation
REQUEST_FOR_INFORMATION_EXPIRED	Request for information expired
RETURN_REQUESTED_BY_RECIPIENT	Recipient requested return
EXTERNAL_IDENTIFIER_DETAILS_HAVE_CHANGED	External identifier details have changed
REASON_NOT_SPECIFIED	Reason not specified

Wise could add new failure codes to this list and your system should be able to handle the events even if the failure reason code is not recognised.

Example `transfers#payout-failure` event
{
  "data": {
    "transfer_id": 111,
    "profile_id": 222,
    "failure_reason_code": "WRONG_ID_NUMBER",
    "failure_description": "Invalid recipient's ID document number",
    "occurred_at": "2023-08-10T10:17:23.000+00:00"
  },
  "subscription_id": "01234567-89ab-cdef-0123-456789abcdef",
  "event_type": "transfers#payout-failure",
  "schema_version": "2.0.0",
  "sent_at": "2023-08-10T10:17:28Z"
}
Transfer refund event
Event type: transfers#refund
Profile level subscriptions: Supported
Application level subscriptions: Supported

This event will be triggered every time a transfer's status becomes funds_refunded. Each event contains a timestamp. As we do not guarantee the order of events, please use data.occurred_at to reconcile the order.

This event provides information about refund amount and currency.

If you would like to subscribe to transfer refund events, please use transfers#refund when creating your subscription.

Schema
data.resource.typestring

Transfer resource type (always transfer)

data.resource.idinteger (long)

ID of the transfer

data.resource.profile_idinteger (long)

Identifies the Wise profile associated with this refund

data.resource.account_idinteger (long)

ID referring to the initial beneficiary of the payment. The ultimate beneficiary is the sender the funds are being refunded to.

data.resource.refund_amountdouble

The total amount that was refunded to the customer. Includes any applicable fees.

data.resource.refund_currencystring

The three-letter ISO currency code representing the currency in which the refund was issued.

data.occurred_atdatetime

The date and time the refund was triggered

Example `transfers#refund` event
{
  "data": {
    "resource": {
      "type": "transfer",
      "id": 111,
      "profile_id": 222,
      "account_id": 333,
      "refund_amount": 5000,
      "refund_currency": "EUR"
    },
    "occurred_at": "2024-01-01T12:34:56Z"
  },
  "subscription_id": "01234567-89ab-cdef-0123-456789abcdef",
  "event_type": "transfers#refund",
  "schema_version": "1.0.0",
  "sent_at": "2024-01-01T12:34:56Z"
}
Currently this webhook does not work in sandbox.
Account details payment state change event
Event type: account-details-payment#state-change
Profile level subscriptions: Supported
Application level subscriptions: Supported

This event will be triggered every time a pay-in is made into the specified account details.

Schema
data.account_details_idstring

Account detail ID where the pay-in was received

data.target_account_idstring

Balance ID which is linked to the account detail

data.resource.idlong

Balance ID which is linked to the account detail

data.resource.profile_idlong

ID of the profile that owns the payment

data.transfer.idinteger

ID of the incoming transfer

data.transfer.typestring

Type of the transfer

data.transfer.amountdecimal

Transfer amount

data.transfer.currencystring

Currency code

data.sender.namestring

Sender name

data.sender.account_numberstring

Sender account number

data.sender.bank_codestring

Sender bank code

data.sender.addressstring

Sender address

data.current_statusstring

Status of either PROCESSING, COMPLETED, CANCELLED or REFUNDED

data.previous_statusstring

Status of either PROCESSING, COMPLETED, CANCELLED or REFUNDED

data.occured_atstring

Timestamp when the event occurred

Example `account-details-payment#state-change` event
{
  "data": {
    "account_details_id": "1",
    "target_account_id": "12345",
    "resource": {
      "id": 12345,
      "profile_id": 1
    },
    "transfer": {
        "id": "36c3f762-560d-4f07-84f9-5d8a3cacabbb",
        "type": "credit",
        "amount": 1.23,
        "currency": "EUR"
    },
    "sender": {
      "name": "Test Sender",
      "account_number": "12345678",
      "bank_code": "TESTBANK",
        "address": "Test Address"
    },
    "current_status": "PENDING",
    "previous_status": null,
    "occurred_at": "2024-01-01T12:34:56Z"
  },
  "subscription_id": "36c3f762-560d-4f07-84f9-5d8a3cacabbb",
  "event_type": "account-details-payment#state-change",
  "schema_version": "2.0.0",
  "sent_at": "2024-01-01T12:34:56Z"
}
Balance credit event

New balance webhook is released. The new webhook is triggered every time a multi-currency account is credited or debited. For more information: Balance update event.

Event type: balances#credit
Profile level subscriptions: Supported
Application level subscriptions: Not Supported

This event will be triggered every time a multi-currency account is credited.

If you would like to subscribe to balance credit events, please use balances#credit when creating your subscription.

Schema
data.resource.typestring

Resource type (always balance-account)

data.resource.idinteger

ID of the account

data.resource.profile_idinteger

ID of the profile that owns the account

data.transaction_typestring

Always credit

data.amountdecimal

Deposited amount

data.currencystring

Currency code

data.post_transaction_balance_amountdecimal

Balance after the credit was applied

data.occurred_atdatetime

When the credit occurred

Example `balances#credit` event
{
   "data":{
      "resource":{
         "type":"balance-account",
         "id":111,
         "profile_id":222
      },
      "transaction_type":"credit",
      "amount":1.23,
      "currency":"EUR",
      "post_transaction_balance_amount":2.34,
      "occurred_at":"2020-01-01T12:34:56Z"
   },
   "subscription_id":"01234567-89ab-cdef-0123-456789abcdef",
   "event_type":"balances#credit",
   "schema_version":"2.0.0",
   "sent_at":"2020-01-01T12:34:56Z"
}
Balance update event
v3.0.0
Event type: balances#update
Profile level subscriptions: Supported
Application level subscriptions: Supported

This event will be triggered every time a multi-currency account is credited or debited. If you would like to subscribe to balance update events, please use balances#update when creating your subscription.

For tracking events related to a single transaction such as a cancelled card transaction, please use data.step_id to reconcile the order.

Schema, version 3.0.0
data.resource.typestring

Resource type (always balance-account)

data.resource.idinteger (long)

ID of the account

data.resource.profile_idinteger (long)

ID of the profile that owns the account

data.amountdecimal

Transaction amount

data.balance_idinteger (long)

ID of the balance credited or debited

data.channel_namestring

Transfer category

data.currencystring

Currency code

data.occurred_atdatetime

When the transaction occurred

data.post_transaction_balance_amountdecimal

Available balance after current transaction

data.step_idinteger (long)

Unique identifier for tracking sequence of transaction events

data.transaction_typestring

Either credit or debit

data.transfer_referencestring

ID of the transfer

Example `balances#update event` credit notification
{
  "data": {
    "resource": {
      "id": 2,
      "profile_id": 2,
      "type": "balance-account"
    },
    "amount": 70,
    "balance_id": 111,
    "channel_name": "TRANSFER",
    "currency": "GBP",
    "occurred_at": "2023-03-08T14:55:38Z",
    "post_transaction_balance_amount": 88.93,
    "step_id": 1234567,
    "transaction_type": "credit",
    "transfer_reference": "BNK-1234567"
  },
  "subscription_id": "f2264fe5-a0f5-4dab-a1b4-6faa87761425",
  "event_type": "balances#update",
  "schema_version": "3.0.0",
  "sent_at": "2023-03-08T14:55:39Z"
}
Example `balances#update event` debit notification
{
  "data": {
    "resource": {
      "id": 2,
      "profile_id": 2,
      "type": "balance-account"
    },
    "amount": 9.6,
    "balance_id": 111,
    "channel_name": "TRANSFER",
    "currency": "GBP",
    "occurred_at": "2023-03-08T15:26:07Z",
    "post_transaction_balance_amount": 106.93,
    "step_id": 1234567,
    "transaction_type": "debit",
    "transfer_reference": "47500002"
  },
  "subscription_id": "f2264fe5-a0f5-4dab-a1b4-6faa87761425",
  "event_type": "balances#update",
  "schema_version": "3.0.0",
  "sent_at": "2023-03-08T15:26:07Z"
}
Balance account state change event
Event type: balances#account-state-change
Profile level subscriptions: Not Supported
Application level subscriptions: Supported

This event will be triggered every time a balance account is activated or deactivated

If you would like to subscribe to balance account state change events, please use balances#account-state-change when creating your subscription.

Schema
data.resource.typestring

Resource type (always balance-account)

data.resource.idinteger

ID of the account

data.resource.profile_idinteger

ID of the profile that owns the account

data.resource.stateinteger

State of the account.
Values:

ACTIVE
INACTIVE
data.occurred_atdatetime

Timestamp when the event occurred

Example `balances#account-state-change` event
{
  "data":{
    "resource":{
      "type":"balance-account",
      "id": 123,
      "profile_id": 555,
      "state": "INACTIVE"
    },
    "occurred_at":"2020-01-01T12:34:56Z"
  },
  "subscription_id":"01234567-89ab-cdef-0123-456789abcdef",
  "event_type":"balances#account-state-changed",
  "schema_version":"2.0.0",
  "sent_at":"2020-01-01T12:34:56Z"
}
Verification state change event
Event type: profiles#verification-state-change
Profile level subscriptions: Not Supported
Application level subscriptions: Supported

This event will be triggered when the verification state of a connected profile changes.

A profile's verification state can be verified or not_verified. If the state is verified, then the user is ready to make payments using Wise. If the state is not_verified, then we advise not to initiate any transfers with the user's access token as the payments will not be processed until the verification is completed.

If you would like to subscribe to verification state change events, please use profiles#verification-state-change when creating your subscription.

Schema
data.resource.typestring

Profile resource type (always profile)

data.resource.idinteger

ID of the profile

data.current_statestring

Current verification state of the profile (see discussion above)

data.occurred_atdatetime

When the verification state change occurred

Example `profiles#verification-state-change` event
{
  "data": {
    "resource": {
      "type": "profile",
      "id": 111
    },
    "current_state": "verified",
    "occurred_at": "2020-01-01T12:34:56Z"
  },
  "subscription_id": "01234567-89ab-cdef-0123-456789abcdef",
  "event_type": "profiles#verification-state-change",
  "schema_version": "2.0.0",
  "sent_at": "2020-01-01T12:34:56Z"
}
Batch Payment Initiations state change event
Event type: batch-payment-initiations#state-change
Profile level subscriptions: Not Supported
Application level subscriptions: Supported

This event will be triggered when the payment initiation state of a batch group payment changes.

The expected state of a batch payment initiation can be one of: NEW, PROCESSING, COMPLETED, FAILED, CHARGED_BACK.

If you would like to subscribe to batch payment initiation state change events, please use batch-payment-initiations#state-change when creating your subscription.

Schema
data.resource.idnumber

Payment initiation ID

data.resource.batchGroupIdtext

Batch group ID

data.resource.profileIdnumber

The ID of the profile this payment belongs to

data.previousStatustext

Previous payment initiation state

data.currentStatustext

Current payment initiation state

data.occurredAttimestamp

When the payment initiation state change occurred

data.returnCodetext

Return code of the underlying payment system

Example `batch-payment-initiations#state-change` event
{
  "data": {
    "resource": {
      "id": 12345,
      "batchGroupId": "068e186d-9632-4937-b753-af3e53f4d0b0",
      "profileId": 2
    },
    "previousStatus": "NEW",
    "newStatus": "PROCESSING",
    "occurredAt": "2021-04-13T19:51:41.423404Z" ,
    "returnCode": "200"
  },
  "subscription_id": "01234567-89ab-cdef-0123-456789abcdef",
  "event_type": "batch-payment-initiations#state-change",
  "schema_version": "2.0.0",
  "sent_at": "2020-01-01T12:34:56Z"
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
Card transaction state change events
Event type: cards#transaction-state-change
Profile level subscriptions: Not Supported
Application level subscriptions: Supported

This event will be triggered whenever a new card transaction occurs or is modified.

Transactions can be categorized based on the guidelines below, though the results may not always be consistent.

Active card check – transaction_amount is 0 or 1 and merchant.category.code not equals 6011
ATM balance check – transaction_amount is 0 and merchant.category.code equals 6011
Pre-Authorisation – is_amount_confirmed is false and transaction_state equals IN_PROGRESS
Schema
data.resource.profile_idinteger

ID of the profile that owns the card

data.resource.client_idstring

Your api_client_id

data.resource.card_tokenstring

Unique identifier of the card

data.resource.card_last_digitsstring

Last 4 digits of the card

data.resource.typestring

Resource type (always card)

data.transaction_idinteger

ID of the transaction

data.transaction_typestring

Type of the transaction

data.is_debit (Deprecated)boolean

Indicates whether the transaction type is a debit. This field is deprecated, use data.transaction_amount.value to determine if the transaction is a debit (positive) or a credit into the balance (negative).

data.transaction_step_typestring

Step type of the transaction

data.decline_reasonstring

Code of the decline reason if applicable

data.transaction_statestring

The current state of the transaction

data.transaction_amount.valuedecimal

Transaction amount

data.transaction_amount.currencystring

Currency code

data.is_amount_confirmedboolean

Whether the transaction amount is confirmed

data.fees[0].amountdecimal

Fee amount

data.fees[0].currencystring

Currency code

data.fees[0].fee_typestring

Fee type

data.transaction_amount_with_fees.valuedecimal

Transaction amount including fees

data.transaction_amount_with_fees.currencystring

Currency code

data.billing_amount.valuedecimal

Billing amount

data.billing_amount.currencystring

Currency code

data.authorisation_methodstring

Authorisation method

data.pin_validation_resultstring

PIN validation result. Possible values: ONLINE_PIN_VALIDATED, ONLINE_PIN_INVALID, OFFLINE_PIN_VALIDATED, OFFLINE_PIN_INVALID, NOT_RECEIVED

data.approval_codestring

Also called authorization code. This can be used to prove ownership of a customer's card/account to a merchant

data.purge_timedatetime

Time at which reserved funds will be released after the authorisation hold expires

data.balance_transaction_idinteger

Associated balance transaction id if applicable

data.balance_movements[0].creation_timedatetime

When the balance movement occurred

data.balance_movements[0].balance_idstring

ID of the balance credited or debited

data.balance_movements[0].typestring

credit or debit

data.balance_movements[0].amount.valuedecimal

Credited or debited amount

data.balance_movements[0].amount.currencystring

Currency code

data.debits[0].balance_idinteger

Balance ID

data.debits[0].debited_amount.valuedecimal

Amount taken from the balance

data.debits[0].debited_amount.currencystring

Currency code

data.debits[0].for_amount.valuedecimal

Amount converted to

data.debits[0].for_amount.currencystring

Currency code

data.debits[0].ratedecimal

Exchange rate

data.debits[0].fee.valuedecimal

Conversion fee amount

data.debits[0].fee.currencystring

Currency code

data.credit.balance_idinteger

Balance ID

data.credit.credited_amount.valuedecimal

Amount credited to the balance

data.credit.credited_amount.currencystring

Currency code

data.merchant.namestring

Name of the merchant

data.merchant.location.countrystring

Country where merchant is located

data.merchant.location.citystring

City where merchant is located

data.merchant.location.postCodestring

Post code where merchant is located

data.merchant.location.statestring

State where merchant is located

data.merchant.category.codestring

Merchant category code

data.merchant.category.descriptionstring

Merchant category description

data.arnstring

Acquirer reference number

data.creation_timedatetime

When transaction was created

data.occurred_atdatetime

When transaction or transaction state change occurred

Table of available transaction state and descriptions
Code	Description
IN_PROGRESS	The transaction is still in progress
COMPLETED	The transaction is completed
DECLINED	The transaction has been declined
CANCELLED	The transaction has been cancelled
UNKNOWN	Default fallback status
Table of available transaction type and descriptions
Code	Description
ACCOUNT_CREDIT	Receiving money on the card, excluding Visa OCT or Mastercard MoneySend
ACCOUNT_FUNDING	Sending money to another card or e-wallet
CASH_ADVANCE	Cash disbursement
CASH_WITHDRAWAL	ATM withdrawal
CHARGEBACK	Currently unused. Reserved for future use.
CREDIT_TRANSACTION	Visa OCT and Mastercard MoneySend
ECOM_PURCHASE	Online purchase
POS_PURCHASE	Purchase via a POS Terminal
REFUND	Partial or full refund of an existing card transaction
Table of available transaction step type and descriptions
Code	Description
AUTH	Transaction authorization which is usually the first step
PARTIAL_REVERSAL	Transaction has been partially reversed
FULL_REVERSAL	Transaction has been fully reversed
CAPTURE	Transaction has been caputured
SETTLE	Currently unused. Reserved for future use.
REFUND	Currently unused. Reserved for future use.
RECONCILIATION	Currently unused. Reserved for future use.
Table of available fee type and descriptions
Code	Description
ATM_WITHDRAWAL	Fee charged by Wise
ATM_MACHINE	Fee charged by the ATM owner
Example `cards#transaction-state-change` event, completed transaction
{
  "data": {
    "resource": {
      "profile_id": 123456,
      "client_id": "your-bank",
      "card_token": "136b29e4-7eab-4dac-a017-438d494ef6cb",
      "card_last_digits": "1234",
      "type": "card"
    },
    "transaction_id": 12345,
    "transaction_type": "CASH_WITHDRAWAL",
    "is_debit": true,
    "transaction_step_type": "CAPTURE",
    "decline_reason": null,
    "transaction_state": "COMPLETED",
    "transaction_amount": {
      "value": 100.00,
      "currency": "EUR"
    },
    "is_amount_confirmed": true,
    "fees": [
      {
        "amount": 1.00,
        "currency": "EUR",
        "fee_type": "ATM_WITHDRAWAL"
      }
    ],
    "transaction_amount_with_fees": {
      "value": 101.00,
      "currency": "EUR"
    },
    "billing_amount": {
      "value": 100.00,
      "currency": "EUR"
    },
    "authorisation_method": "CHIP_AND_PIN",
    "pin_validation_result": "ONLINE_PIN_VALIDATED",
    "approval_code": "913647",
    "purge_time": "2022-08-22T11:10:41Z",
    "balance_transaction_id": 12345,
    "balance_movements": [
      {
        "creation_time": "2024-12-02T04:17:40Z",
        "balance_id": 123,
        "type": "debit",
        "amount": {
          "currency": "AUD",
          "value": 165.96
        }
      }
    ],
    "debits": [
      {
        "balance_id": 123,
        "debited_amount": {
          "value": 165.96,
          "currency": "AUD"
        },
        "for_amount": {
          "value": 101.00,
          "currency": "EUR"
        },
        "rate": 0.61223252,
        "fee": {
          "value": 0.99,
          "currency": "AUD"
        }
      }
    ],
    "credit": null,
    "merchant": {
      "name": "Test Payment",
      "location": {
        "country": "France",
        "city": "Rouen",
        "postCode": "00000",
        "state": null
      },
      "category": {
        "code": "6011",
        "description": "6011 Z Member Financial Institution"
      }
    },
    "arn": "04300014127798385983852",
    "creation_time": "2022-08-15T11:10:41Z",
    "occurred_at": "2022-08-15T11:10:41Z"
  },
  "subscription_id": "568da36b-7335-4608-ad1b-e5c2d32273b6",
  "event_type": "cards#transaction-state-change",
  "schema_version": "2.0.0",
  "sent_at": "2022-08-15T11:10:46Z"
}
Example cards#transaction-state-change event, cancelled transaction
{
  "data": {
    "arn": null,
    "authorisation_method": "CHIP_AND_PIN",
    "pin_validation_result": "ONLINE_PIN_VALIDATED",
    "purge_timestamp": "2024-11-13T06:40:49Z",
    "purge_time": "2024-11-13T06:40:49Z",
    "approval_code": "913647",
    "balance_transaction_id": 5543970,
    "balance_movements": [
      {
        "creation_time": "2024-11-13T06:41:20Z",
        "balance_id": 124741,
        "type": "debit",
        "amount": {
          "currency": "SGD",
          "value": 10
        }
      },
      {
        "creation_time": "2024-11-13T06:41:48Z",
        "balance_id": 124741,
        "type": "credit",
        "amount": {
          "currency": "SGD",
          "value": 10
        }
      }
    ],
    "billing_amount": {
      "currency": "SGD",
      "value": 10
    },
    "creation_time": "2024-11-13T06:41:20Z",
    "credit": null,
    "debits": [
      {
        "balance_id": 124741,
        "debited_amount": {
          "currency": "SGD",
          "value": 0
        },
        "fee": {
          "currency": "SGD",
          "value": 0
        },
        "for_amount": {
          "currency": "SGD",
          "value": 0
        },
        "rate": 1
      }
    ],
    "decline_reason": null,
    "fees": [],
    "is_amount_confirmed": true,
    "is_debit": true,
    "merchant": {
      "category": {
        "code": "5912",
        "description": "Drug Stores, Pharmacies"
      }
    },
    "occurred_at": "2024-11-13T06:41:49Z",
    "resource": {
      "card_last_digits": "7223",
      "card_token": "495980f9-719e-4e60-0d3b-812ab0c5c5a4",
      "client_id": "tw-test-connected-apps",
      "profile_id": 16605997,
      "type": "card"
    },
    "transaction_amount": {
      "currency": "SGD",
      "value": 10
    },
    "transaction_amount_with_fees": {
      "currency": "SGD",
      "value": 10
    },
    "transaction_id": 5255,
    "transaction_state": "CANCELLED",
    "transaction_step_type": "FULL_REVERSAL",
    "transaction_type": "POS_PURCHASE"
  },
  "event_type": "cards#transaction-state-change",
  "schema_version": "2.0.0",
  "sent_at": "2024-11-13T06:41:49Z",
  "subscription_id": "568da36b-7335-4608-ad1b-e5c2d32273b6"
}
Additional verification state change event
Event type: profiles#cdd-check-state-change
Profile level subscriptions: Not Supported
Application level subscriptions: Supported

This event pushes updates for Additional Customer Verification process.

Schema
data.resource.typestring

Profile resource type (always set to profile)

data.resource.idinteger

ID of the profile

data.current_statestring

Current verification state of the profile (see table below)

data.review_outcomestring

Optional

Reason the verification review did not pass.

Refer to the table below for description on these reasons.

data.required_evidenceslist of strings

List of required evidences for verification.

Refer to the table below for description on how to provide the missing evidence.

data.occurred_atdatetime

When the CDD check state change occurred

data.source_of_incomestring

Optional

Source of income for a personal profile submitted via the /upload-evidences API.

Use this to upload the correct Source of Wealth document.

data.source_of_fundingstring

Optional

Source of funding for a business profile submitted via the /upload-evidences API.

Use this to upload the correct Source of Wealth document.

Verification state values
Code	Description
VERIFIED	Additional verification check has passed
EVIDENCE_REQUIRED	Additional verification check has failed, evidences are required
Review outcome values
Code	Description
DOCUMENT_POOR_QUALITY	Document quality poor
DOCUMENT_OUT_OF_DATE	Document out of date
DOCUMENT_NAME_MISMATCH	Name on document does not match records
DOCUMENT_MISSING_NAME	Name on document missing
DOCUMENT_MISSING_ISSUE_DATE	Document issue date missing
DOCUMENT_MISSING_COMPANY_LOGO_LETTERHEAD	Company logo or letterhead missing from document
DOCUMENT_NOT_COMPLETE	Document is partially cut-off and does not contain full information
DOCUMENT_MISSING_SIGNATURE	Document missing signature
DOCUMENT_MISSING_INCOME	Document missing income information or income is not clear from the document
DOCUMENT_TYPE_UNACCEPTABLE	Submitted document type is not acceptable
INVOICE_UNACCEPTABLE	Invoice is not an accepted document
PHOTO_ID_UNACCEPTABLE	Photo ID is not an accepted document
TRANSACTION_UNACCEPTABLE	Transaction (or a screenshot of a transaction) is not accepted
PHOTO_UNACCEPTABLE	Photos (or computer screenshots) are not acceptable
OTHER	Reason unknown
Required evidence values

Go to the list of evidences to find all possible values.

Example `profiles#cdd-check-state-change` event
{
  "data": {
    "resource": {
      "type": "profile",
      "id": 111
    },
    "current_state": "VERIFIED",
    "required_evidences": [],
    "occurred_at": "2020-01-01T12:34:56Z"
  },
  "subscription_id": "01234567-89ab-cdef-0123-456789abcdef",
  "event_type": "profiles#cdd-check-state-change",
  "schema_version": "2.0.0",
  "sent_at": "2020-01-01T12:34:56Z"
}
Another example `profiles#cdd-check-state-change` event
{
  "data": {
    "resource": {
      "type": "profile",
      "id": 111
    },
    "current_state": "EVIDENCE_REQUIRED",
    "review_outcome": "Document is not acceptable",
    "required_evidences": [
      "SOURCE_OF_INCOME"
    ],
    "source_of_income": "INVESTMENTS",
    "occurred_at": "2020-01-01T12:34:56Z"
  },
  "subscription_id": "01234567-89ab-cdef-0123-456789abcdef",
  "event_type": "profiles#cdd-check-state-change",
  "schema_version": "2.0.0",
  "sent_at": "2020-01-01T12:34:56Z"
}
Card status change events
Event type: cards#card-status-change
Profile level subscriptions: Not Supported
Application level subscriptions: Supported

This event is triggered for the initial card status as well as any subsequent updates.

Schema
data.resource.profile_idinteger

ID of the profile that owns the card

data.resource.client_idstring

Your api_client_id

data.resource.card_tokenstring

Unique identifier of the card

data.resource.typestring

Resource type (always card)

data.card_statusstring

The updated card status

data.changed_bystring

The identifier of the entity that updated the card status. If changed by Wise, the value is set to internal_system, otherwise, it is set to your api_client_id

data.occurred_atdatetime

When the card status change occurred

Table of available card status and descriptions
Code	Description
ACTIVE	Card is active and can be used
INACTIVE	Card is inactive and all transactions will be declined
BLOCKED	Card is blocked and cannot be reversed back to any state
FROZEN	Card is “blocked” but temporarily
PARTNER_SUSPENDED	Card is suspended by Wise temporarily due to e.g. fraud reasons
EXPIRED	Card is expired
PURGED	The cardhoder data (e.g. PAN, PIN) have been purged after exceeds the retention period (555 days after the card's expiry date)
Example `cards#card-status-change` event
{
  "data": {
    "resource": {
      "profile_id": 123456,
      "client_id": "your-bank",
      "card_token": "ABCD-1234-ABCD-1234-ABCD",
      "type": "card"
    },
    "card_status": "FROZEN",
    "changed_by": "internal_system",
    "occurred_at": "2022-08-22T07:49:50Z"
  },
  "subscription_id": "ABCD-1234-ABCD-1234-ABCD",
  "event_type": "cards#card-status-change",
  "schema_version": "2.0.0",
  "sent_at": "2022-08-22T07:59:50Z"
}
Card order status change events
Event type: cards#card-order-status-change
Profile level subscriptions: Not Supported
Application level subscriptions: Supported

This event will be triggered every time a card order status is updated. Please note that statuses depend on the type of card (virtual/physical). Additional statuses related to delivery exist for physical cards.

Schema
data.resource.profile_idinteger

ID of the profile that owns the card

data.resource.typestring

Webhook notification of type 'card'

data.resource.client_idstring

Api client_id

data.resource.card_tokenstring or null

Unique identifier of the card

data.order_idstring

Card order ID associated with the status change

data.order_statusstring

Updated card order status

data.delivery_vendorstring or null

Delivery vendor used to dispatch the order (physical card only)

data.occurred_atdatetime

When the card order status change occurred

Example `cards#card-order-status-change` event
{
   "data":{
      "resource":{ 
        "type": "card",
        "profile_id": "123456",
        "client_id": "your-bank",
        "card_token": "35050a4a-9521-426e-8109-1396e3687a3e",
        "card_program": "VISA_DEBIT_CONSUMER_UK_1_PHYSICAL_CARDS_API"
      },
      "order_id" : "1001L",
      "order_status": "PRODUCED",
      "delivery_vendor": "DHL",
      "occurred_at":"2023-01-01T12:24:56Z"
   },
   "subscription_id":"01234567-89ab-cdef-0123-456789abcdef",
   "event_type":"cards#card-order-status-change",
   "schema_version":"2.0.0",
   "sent_at":"2023-01-01T12:34:56Z"
}
Table of available card order status and description

The initial status is PLACED or REQUIREMENT_FULFILLED depending on the requirement fulfillment state.

The possible values are shown in the table below:

Status	Definition
PLACED	The card order is created. The card will be generated once it has fulfilled all the requirements
REQUIREMENTS_FULFILLED	The card order has fulfilled all the requirements and the card should be generated in a short while
CARD_DETAILS_CREATED	The card has been generated
PRODUCED	The physical card has been produced and waiting to be picked up by delivery vendor (physical card only)
COMPLETED	The card has been activated and is ready to use. The card order is completed
CANCELLED	The card order has been cancelled. This can happen if you reach out to Wise Support to cancel a card order
RETURNED	Delivery failed, the physical card has been returned and will be blocked (physical card only)
Card order status transitions
Card production status change events
Event type: cards#card-production-status-change
Profile level subscriptions: Not Supported
Application level subscriptions: Supported

This event will be triggered every time a card production status is updated. Please note that this webhook is solely used for physical card with KIOSK_COLLECTION delivery method.

Refer back to the Card kiosk collection guide for more details.

Schema
data.resource.profile_idinteger

ID of the profile that owns the card

data.resource.typestring

Webhook notification of type 'card'

data.resource.client_idstring

API client_id

data.resource.card_tokenstring or null

Unique identifier of the card

data.statusstring

Current production status. The possible values can be found in the production status flow diagram.

data.kiosk_idstring or null

Identifier that specifies which kiosk machine is producing the card.

data.occurred_atdatetime

Time when the card production request has been sent to the kiosk machine.

data.error_codestring or null

Code returned when card production is not successful. The possible values can be found in production errors.

data.descriptionstring

Detailed description of the error code.

Example `cards#card-production-status-change` event
{
  "data": {
    "resource": {
      "type": "card",
      "profile_id": "123456",
      "client_id": "your-bank",
      "card_token": "35050a4a-9521-426e-8109-1396e3687a3e"
    },
    "status": "PRODUCED",
    "kiosk_id": "WIS00001",
    "error_code": null,
    "description": "Card produced",
    "occurred_at": "2024-01-01T12:24:560Z"
  },
  "subscription_id": "01234567-89ab-cdef-0123-456789abcdef",
  "event_type": "cards#card-production-status-change",
  "schema_version": "2.0.0",
  "sent_at": "2024-01-01T12:34:56Z"
}
Partner Support Case Changed
Event type: partner-support#case-changed
Profile level subscriptions: Not supported
Application level subscriptions: Supported

This event will be triggered every time a partner case is updated, including if commented on or if the status is changed.

Note that you will also receive a webhook response when you have created a new case or updated a case.

As this the Partner Support API is currently in beta, the information below may change. We will be adding in new types of cases, so please ensure that you handle additional undocumented types.
Schema
data.resource.typestring

Type of the partner case. Value must be GENERAL_ENQUIRY. More case types will be added in the future.

data.resource.case_idlong

ID of the partner case

data.resource.details.transfer_idlong

ID of the transfer that is associated with the partner case. This can be null.

data.resource.details.profile_idinteger (long)

ID of the profile that is associated with the partner case.

data.resource.details.user_idinteger (long)

ID of the user account that is associated with the partner case.

statusstring

Status of the case. Can include OPEN, PENDING, SOLVED and CLOSED.

data.resource.typestring

Type of resource for webhook. Will always be partner-support-case

data.typestring

The type of the webhook being sent. Will be STATUS_UPDATED , NEW_COMMENT , or NEW_CASE

data.occurred_atdatetime

When the partner case update occurred.

Example `partner-support#case-changed` event
{
  "data": {
    "resource": {
      "case_id": 136,
      "case_type": "GENERAL_ENQUIRY",
      "details": {
        "transfer_id": 12345678,
        "user_id": 12345678,
        "profile_id": 12345678
      },
      "status": "PENDING",
      "type": "partner-support-case"
    },
    "type": "NEW_CASE",
    "occurred_at": "2023-06-23T09:45:34Z"
  },
  "subscription_id": "017631af-326c-4a69-93f3-bd1ce987a743",
  "event_type": "partner-support#case-changed",
  "schema_version": "2.0.0",
  "sent_at": "2023-06-23T09:45:36Z"
}
Transaction disputes update events
Event type: transaction-disputes#update
Profile level subscriptions: Not Supported
Application level subscriptions: Supported

This event will be triggered every time a transaction dispute is submitted or updated.

You may receive update events even when there are no apparent changes in the dispute status or subStatus, as certain underlying modifications might not directly impact these visible indicators
Schema
data.resource.idstring

Unique ID of the dispute

data.resource.profile_idinteger

ID of the profile that owns the card

data.resource.transaction_idinteger

ID of the card transaction

data.resource.typestring

Resource type (always transaction-dispute)

data.reasonstring

Dispute reason, you can find all the possible values here

data.statusstring

Dispute overall status, it is either ACTIVE or CLOSED

data.sub_statusstring

Dispute detailed status, you can find all the possible values here

data.status_messagestring

Explanation for subStatus

data.created_atdatetime

Time when the dispute was created

data.created_bystring

Creator of the dispute, it is currently set to the user id

data.can_withdrawboolean

Whether the dispute can be withdrawn

data.occurred_atdatetime

When the dispute updates occurred

Example `transaction-disputes#update` event
{
  "data": {
    "resource": {
      "id": "39f893e3-4b0c-4850-9c5c-8cb8f4798a43",
      "profile_id": 16605997,
      "transaction_id": 4337,
      "type": "transaction-dispute"
    },
    "reason": "WRONG_AMOUNT",
    "status": "CLOSED",
    "sub_status": "WITHDRAWN",
    "status_message": "Withdrawn",
    "created_at": "2024-04-18T06:17:12Z",
    "created_by": "6097861",
    "can_withdraw": false,
    "occurred_at": "2024-04-18T06:36:15Z"
  },
  "event_type": "transaction-disputes#update",
  "schema_version": "2.0.0",
  "sent_at": "2024-04-18T06:36:17Z",
  "subscription_id": "7bb32a11-74ad-43b6-9505-3f5facbc87ed"
}
Bulk settlement payment received events
v3.0.0
Event type: bulk-settlement#payment-received
Profile level subscriptions: Not Supported
Application level subscriptions: Supported

This event will be triggered when a bulk settlement fund is received. Once there is a match, meaning the amount for this settlement journal is fully settled, the amount_matched will be set to true. If it is not matched, check the difference between source_amount and target_amount to determine how much more needs to be paid. Please note that this event can also be triggered when our operations team reallocates funds between settlements. Therefore, the received_amount in version 2.0.0 of the webhook is inaccurate. Please use the latest version, 3.0.0.

Schema, version 3.0.0
data.resource.settlement_referencestring

Reference on settlement journal

data.resource.source_currencystring

Settlement currency

data.resource.source_amountdecimal

The amount Wise paid out on behalf of partner

data.resource.target_amountdecimal

Total settlement fund received for this settlement journal

data.resource.amount_matchedboolean

Whether amount received matchs what is expected

data.occurred_atdatetime

When the settlement fund was received

Example `bulk-settlement#payment-received` event, version 3.0.0
{
  "data": {
    "resource": {
      "settlement_reference": "TPFB1111111",
      "source_currency": "GBP",
      "source_amount": -100.1,
      "target_amount": 100.1,
      "amount_matched": true
    },
    "occurred_at": "2024-04-18T06:36:15Z"
  },
  "event_type": "bulk-settlement#payment-received",
  "schema_version": "3.0.0",
  "sent_at": "2024-04-18T06:36:17Z",
  "subscription_id": "f5b51f77-e14a-433b-9f7c-fc2834ffcff5"
}
User state change events
Event type: users#state-change
Profile level subscriptions: Not Supported
Application level subscriptions: Supported

This event will be triggered every time a user state is updated.

Schema
data.resource.idinteger

ID of the user

data.resource.typestring

Resource type (always user)

data.previous_statestring

The previous state of the user

data.current_statestring

The current state of the user

data.deactivation_type (optional)string

The type of deactivation, ACCOUNT_SUSPENSION or ACCOUNT_CLOSURE. If the type is ACCOUNT_SUSPENSION, it is possible for your customer support teams to appeal the deactivation by reaching out to Wise.

data.deactivation_reason (optional)string

The reason for deactivation

data.occurred_atdatetime

When the user state change occurred

Example `users#state-change` event
{
  "data": {
    "resource": {
      "id": 1234,
      "type": "user"
    },
    "previous_state": "ACTIVE",
    "current_state": "WITHDRAW_ONLY",
    "deactivation_type": "ACCOUNT_SUSPENSION",
    "deactivation_reason": "REQUESTED_BY_CUSTOMER_CS",
    "occurred_at": "2020-01-01T12:34:567Z"
  },
  "event_type": "users#state-change",
  "schema_version": "2.0.0",
  "sent_at": "2020-01-01T12:34:56Z",
  "subscription_id": "f5b51f77-e14a-433b-9f7c-fc2834ffcff5"
}

The possible previous_state and current_state values are:

ACTIVE - The user's account is active.
WITHDRAW_ONLY - The user has 90 calendar days to remove their money from their balance before we fully close their account. This can be done through sending from their balance to themselves and others or spending with their card. The user will still be able to order new cards, convert between balances, open and close balances, and download balance statements. After 90 calendar days, the account will move into a DEACTIVATED state.
DEACTIVATED - The user's account is deactivated and they will not be able to perform any actions on their account. The end-user tokens will be revoked and you will receive a 401 Unauthorized response for API calls.
User state change transitions
KYC review state change event
Event type: kyc-reviews#state-change
Profile level subscriptions: Not Supported
Application level subscriptions: Supported

This event is triggered when a KYC Review state has changed.

Schema
data.resource.iduuid

ID of the KYC Review.

data.resource.statestring

Status of the KYC Review. See KYC Review Status for possible values and state diagram.

data.resource.profileIdinteger

ID of the profile KYC Review belongs to.

data.resource.requiredBydatetime

Timestamp by which the underlying requirement set needs to be verified to not block the customer. Only relevant if the status is PASSED_WITH_REQUIREMENTS.

data.resource.createdAtdatetime

Timestamp marking the creation of the KYC Review.

data.resource.updatedAtdatetime

Timestamp marking the last update of the KYC Review.

data.resource.triggerReference[n].typestring

Type of the underlying action/process this KYC Review is for. Usually a reference to which product this KYC Review is for (like QUOTE or TRANSFER) or a reference to a KYC process on the profile that isn’t related to a specific product (like REFRESH_CYCLE or REPAPERING). Currently, only types QUOTE and TRANSFER are supported.

data.resource.triggerReference[n].triggerData.idstring

ID of the underlying product object. e.g. if type is TRANSFER then this would be transfer ID. This ID might be null if underlying action is a process like REPAPERING.

Example `kyc-reviews#state-change` event
{
  "data": {
    "resource": {
      "id": "46e1a5c4-4a9b-4563-39d3-18174d3ac0f8",
      "state": "WAITING_CUSTOMER_INPUT",
      "profileId": 22016766,
      "requiredBy": "2024-09-03T16:22:02.257725",
      "createdAt": "2024-09-03T16:22:02.257725",
      "updatedAt": "2024-09-03T16:29:41.147522",
      "triggerReferences": [
        {
          "type": "QUOTE",
          "triggerData": {
            "id": "ba83s43a-f623-46f0-956d-196c13e2ab01"
          }
        }
      ]
    }
  },
  "subscription_id": "8df95817-8085-40aa-9bda-e3bf46e7a21a",
  "event_type": "kyc-reviews#state-change",
  "schema_version": "2.0.0",
  "sent_at": "2024-09-03T16:29:42Z"
}
3DS challenge events
Event type: cards#3ds-challenge
Profile level subscriptions: Not Supported
Application level subscriptions: Supported

This event is triggered when a customer initiates a push notification for 3DS. The provided information can be used to send a push notification to the customer's mobile app.

Schema
data.resource.client_idstring

Your api_client_id

data.resource.card_tokenstring

Card token

data.resource.profile_idinteger

Profile ID

data.resource.typestring

Resource type (always card)

data.challenge_expires_afterinteger

The number of seconds before the challenge expires

data.occurred_atdatetime

When the challenge is triggered

data.challenge_methodstring

The challenge method chosen by customer (always PUSH)

data.transaction.referencestring

Transaction reference, you should use this field when notifying us of the challenge result.

data.transaction.channelstring

User agent (usually browser or app)

data.transaction.merchant.categorystring

Merchant category code, this field could be null

data.transaction.merchant.countrystring

Merchant country, it follows ISO 3166-1 alpha-2 standard

data.transaction.merchant.namestring

Merchant name

data.transaction.merchant.urlstring

Merchant url, this field could be null

data.transaction.money_value.valuedecimal

Transaction amount

data.transaction.money_value.currencystring

Currency code

Example `cards#3ds-challenge` event
{
  "data": {
    "challenge_expires_after": 300,
    "challenge_method": "PUSH",
    "occurred_at": "2024-09-05T06:47:15Z",
    "resource": {
      "card_token": "3ab81206-d258-4caf-cea5-60067ba82404",
      "client_id": "tw-test-card-issuance",
      "profile_id": 29751964,
      "type": "card"
    },
    "transaction": {
      "channel": "browser",
      "merchant": {
        "category": null,
        "country": "AD",
        "name": "Cards API BBT",
        "url": "http://intrepid-fav.net"
      },
      "money_value": {
        "currency": "EUR",
        "value": 100
      },
      "reference": "148990660"
    }
  },
  "event_type": "cards#3ds-challenge",
  "schema_version": "2.0.0",
  "sent_at": "2024-09-05T06:47:15Z",
  "subscription_id": "ec6d37c2-4611-457a-b210-d1a5f4e354c7"
}
Overdraft limit threshold event
Event type: profiles#overdraft-limit-threshold
Profile level subscriptions: Not Supported
Application level subscriptions: Supported

This event is triggered when the overdraft limit usage has reached 70% threshold. The event at each threshold is triggered maximum once per hour.

Schema
data.resource.typestring

Profile resource type (always profile)

data.resource.idinteger

ID of the profile.

data.overdraft.useddecimal

Overdraft limit used.

data.overdraft.limitdecimal

Overdraft limit approved.

data.overdraft.currencystring

Currency in which the amounts are expressed in.

Example `profiles#overdraft-limit-threshold` event
{
  "data": {
    "resource": {
      "type": "profile",
      "id": 111
    },
    "overdraft": {
        "used": 12000,
        "limit": 20000,
        "currency": "EUR"
    }
  },
  "subscription_id": "01234567-89ab-cdef-0123-456789abcdef",
  "event_type": "profiles#overdraft-limit-threshold",
  "schema_version": "2.0.0",
  "sent_at": "2020-01-01T12:34:56Z"
}
Account details order state change event
Event type: account-details-order#order-state-change
Profile level subscriptions: Not Supported
Application level subscriptions: Supported

This event is triggered when there is a change in the state of an account details order. The event contains information about the order state and requirement state.

The possible values for an order are:

PENDING_USER: One or more requirements has some pending action from the user.
PENDING_TW: One or more requirements has some pending action from Wise.
REQUIREMENTS_FULFILLED: All requirements are completed and pending account details to be created.
DONE: All requirements are completed and account details has also been created.

The more common requirements are:

Please reach out to your Implementation Manager for more information
VERIFICATION: The user needs to be fully verified before completing this requirement.
Schema
data.creation_timedatetime

When the order was created

data.currencystring

Currency code for the account details order

data.is_account_details_issuedboolean

Whether account details have been issued for this order

data.modification_timedatetime

When the order was last modified

data.order_idstring

Unique identifier for the account details order

data.order_statusstring

Current status of the order (PENDING_USER, PENDING_TW, REQUIREMENTS_FULFILLED, DONE)

data.profile_idinteger

ID of the profile associated with this order

data.requirementsarray

List of requirements for this order

data.requirements[].statusstring

Status of the individual requirement (DONE, PENDING, etc.)

data.requirements[].typestring

Type of requirement (VERIFICATION)

Example `account-details-order#order-state-change` event
{
  "data": {
    "creation_time": "2025-08-08T07:49:27Z",
    "currency": "CAD",
    "is_account_details_issued": false,
    "modification_time": "2025-08-08T07:49:30Z",
    "order_id": "01989c58-45e4-71dd-9373-7d999e992f99",
    "order_status": "REQUIREMENTS_FULFILLED",
    "profile_id": 28835473,
    "requirements": [
      {
        "status": "DONE",
        "type": "VERIFICATION"
      }
    ]
  },
  "event_type": "account-details-order#order-state-change",
  "schema_version": "2.0.0",
  "sent_at": "2025-08-08T07:49:31Z",
  "subscription_id": "01989c58-7171-7276-9a4c-c9b4cefc0897"
}
Account details order status transitions