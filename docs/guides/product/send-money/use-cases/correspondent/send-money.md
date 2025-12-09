# Send moneyCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/send-money/use-cases/correspondent/send-money

---

Send money
Copy

Sending money with Wise Platform is a process with four stages. The journey for your customers starts with a quote which indicates the potential cost of the transaction, and the exchange rate.

Once the quote is agreed, and more information about the transaction has been added, you can give an updated, authenticated quote.

To send money:

Create an authenticated quote using POST /quotes - this optional step allows your customer can see how much the transaction is likely to cost, and the exchange rate.
Authenticate the quote. At this stage, you need to use POST /profiles to create an authenticated quote. This quote is more accurate than the unauthenticated quote, and is required to create the transfer itself.
Add a recipient's account details using POST /accounts - the recipient is the party receiving the money being transferred. You receive a recipientId in the response to this request.
Create the transfer using POST /v2/profiles/{{profileId}}/third-party-transfers. In this request, you must include:
The authenticated quote ID.
Recipient account ID.
Fund the transfer using POST /profiles/{{profileId}}/transfers/{{transferId}}/payments once this step is complete the money is sent.
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
Create a third party transfer

POST /v2/profiles/{{profileId}}/third-party-transfers

When creating a transfer on behalf of a third party, you must take note that:

The originator datablock is required. This details the ultimate sender of funds in the transfer.
Depending on the legal entity type of the originator (PRIVATE or BUSINESS), the required fields vary. Please refer the two sample request examples on the right.
OriginalTransferId field must be used. This is your own ID for the transfer.
Request
targetAccountinteger

Recipient account ID. You can create multiple transfers to same recipient account.

quotetext

V2 quote ID. You can only create one transfer per one quote.
You cannot use same quote ID to create multiple transfers.

originalTransferIdtext

Unique transfer ID in your system. We use this field also to perform idempotency check to avoid duplicate transfers in case of network failures or timeouts. You can only submit one transfer with same originalTransferId.

details.reference (optional)text

Recipient will see this reference text in their bank statement. Maximum allowed characters depends on the currency route. Business Payments Tips article has a full list.

originatorgroup

Data block to capture payment originator details.

originator.legalEntityTypetext

PRIVATE or BUSINESS. Payment originator legal type.

originator.referencetext

Unique customer ID in your system. This allows us to uniquely identify each originator. Required.

originator.nametext

Data block to capture the originator name details.
Depends on the type of legal entity (PRIVATE or BUSINESS), the required fields and inputs are different.

originator.name.givenNametext

Payment originator first name. Required if legalEntityType = PRIVATE.

originator.name.middleNames (optional)text array

Payment originator middle name(s). Used only if legalEntityType = PRIVATE.

originator.name.familyNametext

Payment originator family name. Required if legalEntityType = PRIVATE.

originator.name.patronymicName (optional)text

Payment originator patronymic name. Used only if legalEntityType = PRIVATE.

originator.name.fullNametext

Payment originator full legal name. Required if legalEntityType = BUSINESS.

originator.dateOfBirthyyyy-mm-dd

Payment originator date of birth. Required if legalEntityType = PRIVATE.

originator.businessRegistrationCodetext

Payment originator business registry number / incorporation number. Required if legalEntityType = BUSINESS.

originator.address.firstLinetext

Payment originator address first line. Required

originator.address.citytext

Payment originator address city. Required

originator.address.stateCodetext

Payment originator address state code. Required if address country code in (US, CA, BR, AU).

originator.address.countryCodetext

Payment originator address first line. Required

originator.address.postCode (optional)text

Originator address zip code.

originator.accountDetails (optional)text

Originator account number.

Response

Returns an originator transfer object.

You need to save the transfer ID for tracking its status later via webhooks.

Avoiding duplicate transfers

We use originalTransferId field to avoid duplicate transfer requests. When your first call fails (error or timeout) then you should use the same value in originalTransferId field that you used in the original call when you are submitting a retry message. This way we can treat subsequent retry messages as repeat messages and will not create duplicate transfers to your account.

Example Request - Personal
curl -X POST \
  https://api.wise-sandbox.com/v2/profiles/{{profileId}}/third-party-transfers \
  -H 'Authorization: Bearer <your api token>' \
  -H 'Content-Type: application/json' \
  -d '{
    "targetAccount": <recipient account ID>,
    "quote": "<quote ID>",
    "originalTransferId": "<unique transfer ID in your system>",
    "details" : {
      "reference" : "Ski trip"
    },
    "originator" : {
      "legalEntityType" : "PRIVATE",
      "reference" : "<unique customer ID in your system>",
      "name" : {
        "givenName": "John",
        "middleNames": ["Ryan"],
        "familyName": "Godspeed"
      },
      "dateOfBirth": "1977-07-01",
      "address" : {
        "firstLine": "Salu tee 100, Apt 4B",
        "city": "Tallinn",
        "countryCode": "EE",
        "postCode": "12112"
      }
    }
  }'
Example Request - Business
curl -X POST \
  https://api.wise-sandbox.com/v2/profiles/{{profileId}}/third-party-transfers \
  -H 'Authorization: Bearer <your api token>' \
  -H 'Content-Type: application/json' \
  -d '{
    "targetAccount": <recipient account ID>,
    "quote": "<quote ID>",
    "originalTransferId": "<unique transfer ID in your system>",
    "details" : {
        "reference" : "Payment for invoice 22092"
    },
    "originator" : {
      "legalEntityType" : "BUSINESS",
      "reference" : "<originator customer ID in your system>",
      "name" : {
        "fullName": "Hot Air Balloon Services Ltd"
      },
      "businessRegistrationCode": "1999212",
      "address" : {
        "firstLine": "Aiandi tee 1431",
        "city": "Tallinn",
        "countryCode": "EE",
        "postCode": "12112"
      }
    }
  }'
Example Request - Canadian Personal
curl -X POST \
  https://api.wise-sandbox.com/v2/profiles/{{profileId}}/third-party-transfers \
  -H 'Authorization: Bearer <your api token>' \
  -H 'Content-Type: application/json' \
  -d '{
    "targetAccount": <recipient account ID>,
    "quote": "<quote ID>",
    "originalTransferId": "<unique transfer ID in your system>",
    "details" : {
      "reference" : "Ski trip"
    },
    "originator" : {
      "legalEntityType" : "PRIVATE",
      "reference" : "<unique customer ID in your system>",
      "name" : {
        "givenName": "John",
        "middleNames": ["Ryan"],
        "familyName": "Godspeed"
      },
      "dateOfBirth": "1977-07-01",
      "address" : {
        "firstLine": "102-3393 Capilano Road",
        "city": "North Vancouver",
        "countryCode": "CA",
        "postCode": "V7R 4W7"
      },
      "accountDetails": "<the unique account number of the customer>"
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