# QuoteCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/api-reference/quote

---

Quote
Copy

The quote resource defines the basic information required for a Wise transfer - the currencies to send between, the amount to send and the profile who is sending the money. The profile must be included when creating a quote.

Quote is one of the required resources to create a transfer, along with the recipient who is to receive the funds.

The quote response contains other information such as the exchange rate, the estimated delivery time and the methods the user can pay for the transfer. Not all of this information may apply to your use case.

Upon creating a quote the current mid-market exchange rate is locked and will be used for the transfer that is created from the quote. The rate will be locked for 30 minutes to give a user time to complete the transfer creation flow.

Operations

POST
/v3/quotes
POST
/v3/profiles/{{profileId}}/quotes
PATCH
/v3/profiles/{{profileId}}/quotes/{{quoteId}}
GET
/v3/profiles/{{profileId}}/quotes/{{quoteId}}
The Quote resource
Fields
idtext

ID of this quote (GUID format).

sourceCurrencytext

Source (sending) currency code.

targetCurrencytext

Target (receive) currency code.

sourceAmountdecimal

Amount in source currency to send.

targetAmountdecimal

Amount in target currency to be received by the recipient.

payOuttext

Mechanism we use to deliver the transfer. Not usually of interest to the user.

ratedecimal

Exchange rate value used for the conversion.

createdTimetimestamp

Quote created timestamp.

userinteger

User ID who created the quote.

profileinteger

Personal or business profile ID.

rateTypetext

Whether the rate is "FIXED" or "FLOATING".

rateExpirationTimetimestamp

Time the locked rate will expire.

providedAmountTypetext

Whether the quote was created as "SOURCE" or "TARGET".

pricingConfigurationobject

Allows for pricing configurations to be overridden by partners on a transfer level. Mirrors what was sent in the request.

pricingConfiguration.fee.typetext

Identifies the type of fee that will be configured. Options include only OVERRIDE

pricingConfiguration.fee.variabledecimal

The selected variable percentage (in decimal format) that should be used in the pricingConfiguration

pricingConfiguration.fee.fixeddecimal

The selected fixed fee (in source currency) that should be used in the pricingConfiguration

paymentOptionsarray

List of the methods a user can pay for the transfer. See above for help on choosing the correct one to display.

option.disabledboolean

Whether this option is enabled or not for this quote.

option.estimatedDeliverytimestamp

The estimated delivery time for this combination of payIn and payOut methods, assuming payIn is performed now.

option.formattedEstimatedDeliverytext

A string to display to users for the estimated delivery date.

option.estimatedDeliveryDelaysarray

Array of objects for delivery delays to display to users.

option.estimatedDeliveryDelays.reasontext

Reason of the delivery delay.

option.feeobject

Object containing fee information.

option.fee.transferwisedecimal

The fee to be paid by the sender based on the current state of the quote.

option.fee.payIndecimal

The fee for this payment option, based on the product type of the payment option.

option.fee.discountdecimal

Any discounts that have been applied to this quote for the user.

option.fee.partnerdecimal

If you have agreed a custom price, it will be displayed here.

option.fee.totaldecimal

The total fees to be paid - use this figure when displaying fees on your app.

option.priceobject

Object containing the price information.

option.price.priceSetIdinteger

ID if the price structure.

option.price.totalobject

The total fees to be paid - use this figure when displaying fees on your app.

option.price.total.idinteger

ID of this structure.

option.price.total.typetext

Type of the pricing element - "TOTAL" in this case.

option.price.total.labeltext

Short text describing the price structure this field is nested in.

option.price.total.valueobject

Object containing value elements.

option.price.total.value.amountdecimal

Amount to be paid.

option.price.total.value.currencytext

Currency of the amount to be paid.

option.price.total.value.labeltext

Text version of the price.

option.price.total.explanationobject

Object element giving more details about the price.

option.price.itemslist

Object containing the details of the different elements of the total price.

option.price.items[n].idinteger

ID of this item.

option.price.items[n].typetext

Type of the pricing item. It could be "DISCOUNT" for example.

option.price.items[n].labeltext

Short text describing the pricing element.

option.price.items[n].valuevalue

Object containing value elements.

option.price.items[n].value.amountdecimal

Amount associated to this pricing element. Can be negative for discounts.

option.price.items[n].value.currencytext

Currency on the pricing element.

option.price.items[n].value.labeltext

Text field containing the price and its currency.

option.price.items[n].explanationobject

Additional information on a price breakdown item.

option.price.items[n].explanation.plainTexttext

Text element giving more details about the item.

option.price.items[n].explanation.markdowntext

Formatted textual information

option.price.deferredFee.amountdecimal

The amount of fees that has been deferred by a priceConfiguration override.

option.price.deferredFee.currencytext

The currency of the amount of fees that has been deferred by a priceConfiguration override.

option.price.calculatedOn.unroundedAmountToConverttext

This is the amount, unrounded, that fees were calculated on and built up from.

option.sourceAmountdecimal

sourceAmount when using this payment option.

option.targetAmountdecimal

targetAmount when using this payment option.

option.payIntext

Type of pay in method for this payment option.

option.payOuttext

Type of pay out method for this payment option.

option.allowedProfileTypesarray

Array of the allowed types of profile to use this payment option for this quote "PERSONAL", "BUSINESS" or both.

option.disabledReasonobject

Object present if a payment option is disabled.

option.disabledReason.codetext

Code to denote the reason a payment method is unavailable.

option.disabledReason.messagetext

User friendly message to display when a method is unavailable.

statustext

Current status of this quote, one of: "PENDING", "ACCEPTED", "FUNDED" or "EXPIRED".

expirationTimetimestamp

The time the quote expires.

noticesarray

Array of messages to display to the user in case of useful information based on their selections. May be empty ([]) if there are no messages.

notice.texttext

The message to display.

notice.linktext

URL that provides more information to the message. May be null if there's no URL.

notice.typetext

Type of message, WARNING or INFO or BLOCKED. If it is BLOCKED, don't allow the quote to be used to create the transfer.

Quote Object
{
  "id": "11144c35-9fe8-4c32-b7fd-d05c2a7734bf",
  "sourceCurrency": "GBP",
  "targetCurrency": "USD",
  "sourceAmount": 100,
  "payOut": "BANK_TRANSFER",
  "preferredPayIn": "BANK_TRANSFER",
  "rate": 1.30445,
  "createdTime": "2019-04-05T13:18:58Z",
  "user": 55,
  "profile": 101,
  "rateType": "FIXED",
  "rateExpirationTime": "2019-04-08T13:18:57Z",
  "guaranteedTargetAmountAllowed": true,
  "targetAmountAllowed": true,
  "guaranteedTargetAmount": false,
  "providedAmountType": "SOURCE",
  "pricingConfiguration": {
    "fee": {
      "type": "OVERRIDE",
      "variable": 0.011,
      "fixed": 15.42
    }
  },
  "paymentOptions": [
    {
      "disabled": false,
      "estimatedDelivery": "2019-04-08T12:30:00Z",
      "formattedEstimatedDelivery": "by Apr 8",
      "estimatedDeliveryDelays": [
        {
          "reason": "sample reason"
        }
      ],
      "fee": {
        "transferwise": 3.04,
        "payIn": 0,
        "discount": 2.27,
        "partner": 0,
        "total": 0.77
      },
      "price": {
        "priceSetId": 238,
        "total": {
          "type": "TOTAL",
          "label": "Total fees",
          "value": {
            "amount": 0.77,
            "currency": "GBP",
            "label:": "0.77 GBP"
          }
        },
        "items": [
          {
            "type": "FEE",
            "label": "fee",
            "value": {
              "amount": 0,
              "currency": "GBP",
              "label": "0 GBP"
            }
          },
          {
            "type": "TRANSFERWISE",
            "label": "Our fee",
            "value": {
              "amount": 3.04,
              "currency": "GBP",
              "label": "3.04 GBP"
            }
          },
          {
            "id": 123,
            "type": "DISCOUNT",
            "value": {
              "amount": -2.27,
              "currency": "GBP",
              "label": "2.27 GBP"
            },
            "label": "Discount applied",
            "explanation": {
              "plainText": "You can have a discount for a number of reasons...",
              "markdown": "You can have a **discount** for a number of reasons..."
            }
          }
        ],
        "deferredFee": {
          "amount": 14.79,
          "currency": "BRL"
        },
        "calculatedOn": {
          "unroundedAmountToConvert": {
            "amount": 179.97342,
            "currency": "BRL"
          }
        }
      },
      "sourceAmount": 100,
      "targetAmount": 129.24,
      "sourceCurrency": "GBP",
      "targetCurrency": "USD",
      "payIn": "BANK_TRANSFER",
      "payOut": "BANK_TRANSFER",
      "allowedProfileTypes": [
        "PERSONAL",
        "BUSINESS"
      ],
      "payInProduct": "CHEAP",
      "feePercentage": 0.0092
    },
    ...
  ],
  "status": "PENDING",
  "expirationTime": "2019-04-05T13:48:58Z",
  "notices": [
    {
      "text": "You can have a maximum of 3 open transfers with a guaranteed rate. After that, they'll be transferred using the live rate. Complete or cancel your other transfers to regain the use of guaranteed rate.",
      "link": null,
      "type": "WARNING"
    }
  ]
}
Create an un-authenticated quote

POST /v3/quotes

Use this endpoint to get example quotes for people to see the exchange rate and fees Wise offers before a user has created or linked an account. This can drive a version of the quote screen that shows the user what Wise offers before they sign up. Note that this endpoint does not require a token to create the resource, however, since it is just an example, the returned quote has no ID so can't be used later to create a transfer.

In order to get an accurate partner fee, we require a client credentials token to be provided. If you are a partner and would like your fee to be included in the quote returned, you must provide your auth token. If not, you do not require the Authorization header.

Unauthenticated quotes cannot be used to create transfers, they are meant for illustration purposes in partner interfaces only. Make sure to create an authenticated quote during the transfer creation flow.

Request Fields
sourceCurrencytext

Source (sending) currency code

targetCurrencytext

Target (receiving) currency code

sourceAmountdecimal

Amount in source currency.
Either sourceAmount or targetAmount is required, never both.

targetAmountdecimal

Amount in target currency

pricingConfigurationobject

Required when configured for your client ID. includes a pricingConfiguration to be used for pricing calculations with the quote.

pricingConfiguration.fee.typetext

Identifies the type of fee that will be configured. Options include only OVERRIDE

pricingConfiguration.fee.variabledecimal

The selected variable percentage (in decimal format) that should be used in the pricingConfiguration

pricingConfiguration.fee.fixeddecimal

The selected fixed fee that should be used in the pricingConfiguration. Always considered in source currency.

Response

Returns a quote object.

Example Request
curl -X POST \
  https://api.wise-sandbox.com/v3/quotes/ \
  -H 'Authorization: Bearer <client credential token>' \
  -H 'Content-type: application/json' \
  -d '{
    "sourceCurrency": "GBP",
    "targetCurrency": "USD",
    "sourceAmount": null,
    "targetAmount": 110,
    "pricingConfiguration": {
      "fee": {
        "type": "OVERRIDE",
        "variable": 0.011,
        "fixed": 15.42
      }
    }
  }'
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
Update a quote

PATCH /v3/profiles/{{profileId}}/quotes/{{quoteId}}

You should update a quote using a PATCH call to add a recipient. This will update the saved quote's data based on who and where the money will be sent to.

Updating the quote with a recipient may cause the available payment options, prices and estimated delivery times to change from the original quoted amounts. This is due to the fact that sending some currencies to some destinations costs a different amount based on the payment networks we use, for example sending USD to a country outside the USA uses international rather than domestic payment networks and as such costs more, or sending CAD over the Interac network is a more expensive operation.

When updating a quote the payOut field may change to denote the payment option you should select when sending to this recipient. For example sending USD to a swift_code recipient or CAD to an interac recipient with change payOut to SWIFT or INTERAC respectively. This field defaults to BANK_TRANSFER so it can be used in all cases to help select the correct paymentOption and hence show the correct pricing to users.

If you want to provide more transparency in terms of fees charged when your customers create quote with swift recipient for global currencies, you might consider to set payOut field with SWIFT_OUR value. This will ensure that the recipient receives complete target amount.

In this case, where pricing changes after a user selects recipient, you should show a message to your customer before confirming the transfer. Please see the section on Global Currencies to learn more how and why this works and the messaging you need to display.

Request
targetAccountinteger

ID of transfer recipient, found in response from POST v1/accounts (recipient creation)

payOuttext

Optional. Preferred payout method. Default value is BANK_TRANSFER. Other possible values are BALANCE, SWIFT, SWIFT_OUR and INTERAC.

paymentMetadataobject

Optional. Used to pass additional metadata about the intended transfer.

paymentMetadata.transferNaturestring

Optional. Used to pass transfer nature for calculating proper tax amounts (IOF) for transfers to and from BRL. Accepted values are shown dynamically in transfer requirements.

pricingConfigurationobject

Required when configured for your client ID. Includes a pricingConfiguration to be used for pricing calculations with the quote. If previously passed, the existing pricingConfiguration will remain and not be updated.

pricingConfiguration.fee.typetext

Identifies the type of fee that will be configured. Options include only OVERRIDE

pricingConfiguration.fee.variabledecimal

The selected variable percentage (in decimal format) that should be used in the pricingConfiguration

pricingConfiguration.fee.fixeddecimal

The selected fixed fee (in source currency) that should be used in the pricingConfiguration

Transfer Nature for BRL

When creating or updating a quote, the transfer nature can be set. This is a requirement for transfers to or from BRL and impacts the fees we charge on the quote, specifically the IOF.

Note that IOF is determined based on the transfer nature, sender information, and recipient information. The default IOF will be included in a quote until all relevant information has been included.

Omitting transfer nature will not prevent the transfer from being created or even funded. However, when attempting to process the transfer, it will be blocked and the money will be refunded to the sender.

Example Request
curl -X PATCH \
  https://api.wise-sandbox.com/v3/profiles/{{profileId}}/quotes/{{quoteId}} \
  -H 'Authorization: Bearer <your api token>' \
  -H 'Content-Type: application/merge-patch+json' \
  -d '{
    "targetAccount": 12345,
    "payOut": "SWIFT_OUR",
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
Retrieve a quote by ID

GET /v3/profiles/{{profileId}}/quotes/{{quoteId}}

Get quote info by ID. If the quote has expired (not used to create a transfer within 30 minutes of quote creation), it will only be accessible for approximately 48 hours via this endpoint.

Response

Returns a quote object.

Example Request
curl -X GET \
  https://api.wise-sandbox.com/v3/profiles/{{profileId}}/quotes/{{quoteId}} \
  -H 'Authorization: Bearer <your api token>'