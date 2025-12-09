# Get notifications about transfersCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/send-money/use-cases/correspondent/webhooks

---

Get notifications about transfers
Copy

To track the status of your payments, you should subscribe to the following webhooks:

transfers#state-change: This is the primary webhook you need. It provides notifications whenever a transfer's status changes, such as from "processing" to "completed."
transfers#payout-failure: This event will notify you if a transfer fails during the payout process, which is crucial for handling failed payments.
transfers#refund: This webhook is important for cases where a transfer is refunded, allowing you to track the return of funds.
balances#update: To track the status of funds. If funding your transfer from a Wise Nostro account, this webhook tracks updates to your balance.
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