# Send moneyCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/send-money/use-cases/enterprise/send-money

---

Send money
Copy

Sending money with Wise Platform is a process with four stages. You start by getting a quote which tells you the cost of the transaction, and the exchange rate.

When you create a transfer, you do so from your own Wise profile, and must include your profileId in the path of the API request.

Overview of API requests

To send money:

Create an authenticated quote Use POST /profiles to create an authenticated quote. Whether or not you share the information in the quote with your customer, a quote is required to create the transfer itself.
Add a recipient's account details using POST /accounts - the recipient is the party receiving the money being transferred. You receive a recipientId in the response to this request.
Create the transfer using POST /transfers. In this request, as well as details of the transaction, you must include:
quoteUuid - the ID of the quote you have created for this transaction.
targetAccount - the recipientId of the recipient of this transfer.
Fund the transfer using POST /profiles/{{profileId}}/transfers/{{transferId}}/payments once this step is complete the money is sent.
Your system
Wise Platform
Create an authenticated quote - POST /quotes
200 Response returns: quoteId
Create a recipient - POST /accounts
200 Response returns: recipientId
Create transfer including quoteId, recipientId - POST /transfers
200 Response returns: transferId
Fund transfer including transferId - POST profiles/{{profileId}}/transfers/{{transferId}}/payments
200 Response returns: transfer status
Your system
Wise Platform
Create an authenticated quote

POST /v3/profiles/{{profileId}}/quotes

You must use a user access token to authenticate this call and supply the profile with which the quote should be associated. This ensures that quote can be used to create a transfer.

Request Fields
profileIdinteger

Personal or business profile ID of the sender - required.

sourceCurrencytext

Source (sending) currency code.

targetCurrencytext

Target (receiving) currency code.

targetAmountdecimal

Amount in target currency.

sourceAmountdecimal

Amount in source currency.
Either sourceAmount or targetAmount is required, never both.

targetAccountinteger

Optional. This is the ID of transfer recipient, found in response from POST v1/accounts (recipient creation). If provided can be used as an alternative to updating the quote.

payOuttext

Optional. Preferred payout method. Default value is BANK_TRANSFER. Other possible values are BALANCE, SWIFT, SWIFT_OUR and INTERAC.

preferredPayIntext

Optional. Preferred payin method. Use BANK_TRANSFER to return this method at the top of the response's results.

paymentMetadataobject

Optional. Used to pass additional metadata about the intended transfer.

paymentMetadata.transferNaturestring

Optional. Used to pass transfer nature for calculating proper tax amounts (IOF) for transfers to and from BRL. Accepted values are shown dynamically in transfer requirements.

pricingConfigurationobject

Required when configured for your client ID. includes a pricingConfiguration to be used for pricing calculations with the quote.

pricingConfiguration.fee.typetext

Identifies the type of fee that will be configured. Options include only OVERRIDE

pricingConfiguration.fee.variabledecimal

The selected variable percentage (in decimal format) that should be used in the pricingConfiguration

pricingConfiguration.fee.fixeddecimal

The selected fixed fee that should be used in the pricingConfiguration. Always considered in source currency.

If you are funding the transfer from a Multi Currency Balance, you must set the payIn as BALANCE to get the correct pricing in the quote. Not doing so will default to BANK_TRANSFER and the fees will be inconsistent between quote and transfer.

When SWIFT_OUR is set as payOut value, it enables payment protection for swift recipients for global currency transfers. By using this payOut method, you can guarantee your customers that the fee will be charged to the sender and can ensure that the recipient gets the complete target amount.

Response

The following describes the fields of the quote response that may be useful when building your integration.

The payOut field is used to select the correct entry in the paymentOptions array in order to know which fees to display to your customer. Find the paymentOption that matches the payOut field shown at the top level of the quote resource and payIn based on the settlement model the bank is using. By default, this is BANK_TRANSFER, unless you are using a prefunded or bulk settlement model. The payOut field will change based on the type of recipient you add to the quote in the PATCH /quote call, for example to-USD swift_code or to-CAD interac have different fees.

For example sending USD to a country other than the United States is supported but with different fees to domestic USD transfers. Please see the later section on Global Currencies to learn more about how to offer this useful feature.

For each paymentOption there is a price field. It gives a full breakdown of all the taxes, fees and discounts. It is preferable to refer to this structure to show breakdowns and totals, rather than the fee structure, found as well in each paymentOption element, that only gives a summary and is not able to surface important specifics such as taxes.

When showing the price of a transfer always show the 'price.total.value.amount' of a payment option.

Disabled Payment Options

Each payment option is either enabled or disabled based on the disabled value. Disabled payment options should be shown to the user in a disabled state in most cases. This ensures users are given the options that they are familiar with regardless of their availability, as well as with options that can be beneficial to their accounts.

The option.disabledReason contains both the code and message, with the message being the user-friendly text to surface to the user if necessary.

Transfer Nature for BRL

When creating or updating a quote, the transfer nature can be set. This is a requirement for transfers to or from BRL and impacts the fees we charge on the quote, specifically the IOF.

Note that IOF is determined based on the transfer nature, sender information, and recipient information. The default IOF will be included in a quote until all relevant information has been included.

Omitting transfer nature will not prevent the transfer from being created or even funded. However, when attempting to process the transfer, it will be blocked and the money will be refunded to the sender.

Pricing Configuration

When creating or updating a quote, partners that have flexible partner pricing enabled are able to override the pricing configuration dynamically.

To learn more on how to use this feature, please see the Flexible Partner Pricing Guide

Example Request
curl -X POST \
  https://api.wise-sandbox.com/v3/profiles/{{profileId}}/quotes \
  -H 'Authorization: Bearer <your api token>' \
  -H 'Content-Type: application/json' \
  -d '{
    "sourceCurrency": "GBP",
    "targetCurrency": "USD",
    "sourceAmount": 100,
    "targetAmount": null,
    "payOut": null,
    "preferredPayIn": null,
    "targetAccount": 12345,
    "paymentMetadata": {
      "transferNature": "MOVING_MONEY_BETWEEN_OWN_ACCOUNTS"
    },
    "pricingConfiguration": {
      "fee": {
        "type": "OVERRIDE",
        "variable": 0.011,
        "fixed": 15.42
      }
    }
  }'
Create a recipient account

POST /v1/accounts

A recipient is a person or institution who is the ultimate beneficiary of your payment.

Recipient data includes three data blocks:

General data - the personal details of an individual and basic information about a business.
Bank details - account numbers, routing numbers, and other region-specific bank details.
Address details - country and street address of an individual or business.
General Data
Recipient full name
Legal type (private/business)
Currency
Date of Birth
Owned by customer

Date of Birth is optional. Its usually not required, but consult with the /account-requirements APIs to confirm if it is needed, or contact Wise Support.

The ownedbycustomer is an optional boolean that indicates whether the recipient is the same entity (person or business) as the sender. Set it to true for self-transfers, such as a user sending money to their own account in another country or currency. We strongly recommend setting this field, as distinguishing self-transfers from third-party transfers improves routing and processing efficiency.

Bank account data

There are many different variations of bank account details needed depending on recipient target currency. For example:

GBP — sort code and account number
BGN, CHF, DKK, EUR, GEL, GBP, NOK, PKR, PLN, RON, SEK — IBAN
USD — routing number, account number, account type
INR — IFSC code, account number
etc.
Address data

Recipient address data is required only if target currency is USD, PHP, THB or TRY, or if the source currency is USD or AUD.

Country
State (US, Canada, Brazil)
City
Address line
Zip code

When creating recipient, the following general rules should be applied to the accountHolderName field:

Full names for personal recipients. They must include more than one name, and both first and last name must have more than one character. Numbers are not allowed in personal recipient names.
Business names must be in full, but can be just a single name. The full name cannot be just a single character but can be made up of a set of single characters. e.g. "A" is not permitted but "A 1" or "A1" is permitted.
Special characters _()'*,. are allowed for personal and business names.
In general the following regex describes our permitted characters for a name: [0-9A-Za-zÀ-ÖØ-öø-ÿ-_()'*,.\s].

Recipient requirements will vary depending on recipient type. A GBP example is provided here.
As you can see many of the fields are null, in order to know which fields are required for which currency we expose the Recipients Requirements endpoint.

Request
currencytext

3 character currency code

typetext

Recipient type

profileinteger

Personal or business profile ID. It is highly advised to pass the business profile ID in this field if your business account is managed by multiple users, so that the recipient can be accessed by all users authorized on the business account.

accountHolderNametext

Recipient full name

ownedByCustomerboolean

Whether this account is owned by the sending user

detailsobject

Currency specific fields

details.legalTypetext

Recipient legal type: PRIVATE or BUSINESS

details.sortCodetext

Recipient bank sort code (GBP example)

details.accountNumbertext

Recipient bank account no (GBP example)

details.dateOfBirthtext

Optional. Recipient Date of Birth in ISO 8601 date format.

Response

A complete Recipient object is returned when created.

Example Request - GBP Recipient
curl -X \
  POST https://api.wise-sandbox.com/v1/accounts \
  -H 'Authorization: Bearer <your api token>' \
  -H 'Content-Type: application/json' \
  -d '{
    "currency": "GBP",
    "type": "sort_code",
    "profile": 30000000,
    "ownedByCustomer": true,
    "accountHolderName": "John Doe",
      "details": {
        "legalType": "PRIVATE",
        "sortCode": "040075",
        "accountNumber": "37778842",
        "dateOfBirth": "1961-01-01"
      }
    }'
Create a transfer

POST /v1/transfers

Request
sourceAccount (optional)integer

Refund recipient account ID

targetAccountinteger

Recipient account ID. You can create multiple transfers to same recipient account.

quoteUuidtext

V2 quote ID. You can only create one transfer per one quote. You cannot use same quote ID to create multiple transfers.

customerTransactionIduuid

This is required to perform idempotency check to avoid duplicate transfers in case of network failures or timeouts.

details.referencetext

Recipient will see this reference text in their bank statement. Maximum allowed characters depends on the currency route. Business Payments Tips article has a full list.

details.transferPurpose (conditionally required)text

For example when target currency is THB. See more about conditions at Transfers.Requirements

details.transferPurposeSubTransferPurpose (conditionally required)text

For example when target currency is CNY. See more about conditions at Transfers.Requirements

details.transferPurposeInvoiceNumber (conditionally required)text

For example when target currency is INR. See more about conditions at Transfers.Requirements

details.sourceOfFunds (conditionally required)text

For example when target currency is USD and transfer amount exceeds 80k. See more about conditions at Transfers.Requirements

There are options to deal with conditionally required fields:

Always call transfer-requirements endpoint and submit values only if indicated so.
Always provide values for these fields based on a dynamically retrieved list (transfer-requirements endpoint). It is possible these fields change over time, so take this into account if hard coding the options.

Contact api@wise.com if you have questions about this property.

Response

Returns a standard transfer object.

Avoiding duplicate transfers

We use customerTransactionId field to avoid duplicate transfer requests. If your initial call to create a transfer fails (error or timeout) then you should retry the call using the same value in the customerTransactionId field that you used in the original call. This way we can treat subsequent retry messages as repeat messages and will not create duplicate transfers to your account should one have succeeded before. You should not retry indefinitely but use a sensible limit, perhaps with a back-off approach.

Payment Approvals

Business Payment Approvals created on your wise.com settings page are not compatible with creating transfers over the API.

If you use personal tokens and do not use client credentials, and if your business account has payment approvals, your application will run in to this error when attempting to create a transfer

Quote cannot be accepted with this request due to missing approval.

Consider removing the payment rule if you are going to use the API to create transfers.

Example Request
curl -X POST \
  https://api.wise-sandbox.com/v1/transfers \
  -H 'Authorization: Bearer <your api token>' \
  -H 'Content-Type: application/json' \
  -d '{
    "sourceAccount": <refund recipient account ID>,
    "targetAccount": <recipient account ID>,
    "quoteUuid": <quote ID>,
    "customerTransactionId": "<the unique identifier you generated for the transfer attempt>",
    "details" : {
      "reference" : "to my friend",
      "transferPurpose": "verification.transfers.purpose.pay.bills",
      "transferPurposeSubTransferPurpose": "verification.sub.transfers.purpose.pay.interpretation.service",
      "sourceOfFunds": "verification.source.of.funds.other"
    }
  }'
Fund a transfer

POST /v3/profiles/{{profileId}}/transfers/{{transferId}}/payments

This endpoint is SCA protected when it applies. If your profile is registered within the UK and/or EEA, SCA most likely applies to you. Please read more about implementing SCA. If you use mTLS and client credentials to make API requests, SCA does not apply.

This API call is the final step for executing payouts when using a balance with Wise. Upon calling the endpoint, Wise will begin the processing of the transfer, depending on the status of funds.

When using the transfer by transfer settlement model, the following funding type(s) must be used:

BALANCE - Funds are pulled from a multi-currency account held with Wise.
BANK_TRANSFER - Manually send funds from your business bank account to pay for any transfers. Only applicable when using the Batch Group API.

When funding through the Bulk Settlement model, the following funding type(s) must be used:

TRUSTED_PRE_FUND_BULK - Funds for the transfer will be settled through a bulk payment at a later date. This method is not applicable to first party Payouts.

If funding from BALANCE, and your multi-currency account does not have the required funds to complete the action, then this call will fail with an "insufficient funds" error. Once funds are added and available, you must call this endpoint again.

{{profileId}} refers to the profile that created the transfer. It can be either your personal profile ID, or your business profile ID.

Request
typetext

"BALANCE"
This indicates the type of funding you would like to apply to the transfer.

partnerReference (conditionally required)string

The transaction/payment identifier in your system, uniquely identifies the transfer in your platform. This is required for the Cross Currency Bulk Settlement model.

Response
typetext

"BALANCE"
This indicates the type of funding you would like to apply to the transfer.

statustext

"COMPLETED" or "REJECTED"

errorCodetext

Failure reason. For example "balance.payment-option-unavailable".

Example Request
curl -X POST \
  https://api.wise-sandbox.com/v3/profiles/{{profileId}}/transfers/{{transferId}}/payments \
  -H 'Authorization: Bearer <your api token>' \
  -H 'Content-Type: application/json' \
  -d '{
    "type": "BALANCE"
  }'
Example Request - Balance - Completed
{
  "type": "BALANCE",
  "status": "COMPLETED",
  "errorCode": null
}
Example Request - Balance - Insufficient Funds
{
  "type": "BALANCE",
  "status": "REJECTED",
  "errorCode": "transfer.insufficient_funds"
}
Webhooks

Webhooks provide real-time updates on status changes for your Payment Instruments and Payins. You can subscribe to these events to avoid constantly polling the API.

Payment Instrument Status Change: The payment-instruments#status-change webhook notifies you when an instrument's status changes (e.g., from PENDING to CONNECTED or DISCONNECTED).
Payin Status Change: The payment-instruments-payins#status-change webhook informs you of every status change for a Payin, including PROCESSING, CONFIRMED, SETTLED, or FAILED.
Payment Reversals: A Payment Reversal webhook is available to notify you of any chargebacks. This happens after the original payment and requires you to contact the customer to resolve the dispute.