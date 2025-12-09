# Card TransactionsCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/api-reference/card-transaction

---

Card Transactions
Copy

These APIs allow you to retrieve information on transactions that are made on your user's cards.

Operations

GET
/v3/spend/profiles/{{profileId}}/cards/transactions/{{transactionId}}
Transaction states

The possible state values are:

IN_PROGRESS - The transaction has been authorized but not captured.
COMPLETED - The transaction has been captured and/or settled.
DECLINED - The transaction has been declined.
CANCELLED - The transaction has been cancelled.
UNKNOWN - Default fallback status if the state can't be confirmed.

The transition from CANCELLED to COMPLETED is an edge case. Wise is releasing the customer funds after 7 days (30 days for pre-authorization) if the merchant has not capture the transaction, the state becomes CANCELLED. But the merchant can decide to capture the transaction on a later date, the state will then be COMPLETED.

Decline Reasons

The decline reason field provides information about the specific issue that led to the transaction being declined, helping the merchant and the customer troubleshoot the problem.

While the 'decline reason' field provides valuable information about the specific issue that led to the transaction being declined, it may not cover all possible reasons for a decline, such as technical issues or unforeseen circumstances.

Please be aware that new decline reasons may be added in the future, and not all decline reasons are currently documented. Therefore, it is important to exercise caution and regularly review the documentation to ensure accuracy.

Please exercise caution when communicating reasons to your customers, as some may not be relevant or may cause confusion. If you have any questions or concerns, we are here to assist you.
ACCOUNT_INACTIVE

Balance related to the transaction is not active.

Please ensure that all outstanding actions have been completed before using your card, as this may help avoid any potential issues or declines.

ACCOUNT_SUSPENDED

This transaction has been declined pending further compliance checks.

It may have been flagged for potential sanctions issues.

ATM_PIN_CHANGE_NOT_ALLOWED

PIN change via ATM terminal is not allowed.

BLOCKED_COUNTRY

Transactions were made in unsupported countries.

Check out this link to see if the country is included in our list of supported nations.

It is possible for a merchant to be based in a supported country and have an address registered in a blocked country, albeit infrequently.

BLOCKED_SUBSCRIPTION

Our system cannot facilitate this transaction as the customer has opted out of recurring payments with this merchant.

CARD_EXPIRED

The card provided has reached its expiration date, making it invalid for this transaction.

CARD_FROZEN

The customer or our customer service team has put this card on a temporary hold.

If the card has not been frozen by the customer, it may be worth investigating further.

To resume spending, advise customer to unfreeze the card.

CARD_INACTIVE

The card is either not active or has not been received by the customer, so we are unable to proceed with the transaction.

CARD_BLOCKED

The card has been blocked and can't be used anymore.

CARD_PARTNER_SUSPENDED

Our internal team has deactivated the account for compliance reasons related to AML, fraud, or EDD.

Please contact our team if you believe this is an error.

CHIP_PIN_ENTRY_TRIES_EXCEEDED

The PIN is restricted on the chip of the card due to excessive incorrect entries.

The blocked PIN can be unlocked at an ATM using specific steps that vary depending on the machine and country, such as PIN management or PIN operations followed by unblocking the PIN.

CONNECTION_ISSUE

There's a connection problem occurring during the transaction.

CONTACTLESS_PIN_ENTRY_REQUIRED

Contactless payment systems sometimes require a PIN for authentication purposes to protect users' accounts from potential fraud or tampering.

In Europe, contactless payment transactions that follow one after the other require PIN verification as an additional security measure.

CREDIT_TRANSACTIONS_NOT_SUPPORTED

Credit is not supported for this specific transaction. Please review our Acceptable Use Policy for further information.

CUMULATIVE_LIMIT_EXCEEDED

In certain jurisdictions, there are restrictions on the amount that can be spent. For further details, please refer to the following resource about spending limits.

DECLINED_ADVICE

There's a problem with the message from the processor, so we might not be able to accept it because it could be incomplete or unsafe.

Just because we can't process the payment right now doesn't mean there's anything wrong with the card.

Tell the customer that there was a technical issue with the payment and to try again later.

INCORRECT_CVV

The customer accidentally entered the wrong security code.

Please advise the customer to check their card details and try again.

If the customer's saved card details are correct, they should remove their card details from the merchant's website and then add them back again.

INCORRECT_EXPIRY_DATE

The customer made a mistake and entered the wrong expiration date for their card.

If the customer's saved card details are correct, they should remove their card details from the merchant's website and then add them back again.

INCORRECT_PIN

The customer entered their PIN incorrectly.

Please advise the customer to check their PIN and try again.

If the PIN is correct and still fails, kindly suggest resetting the PIN.

INSUFFICIENT_FUNDS

The customer doesn't have enough money in their account to make the payment.

Please advise the customer to add money to their account and try again. In most cases, this will resolve the issue.

INVALID_3DS_UCAF

The 3D Secure checks failed during the transaction, so the customer should try again and ask for authentication.

INCORRECT_ARQC

ARQC (Authorization Request Cryptogram) is a cryptogram generated by the card during a transaction, which we validate on our end. If it's incorrect, it could indicate a faulty card, a fraudulent attack, or an issue with the POS terminal.

INCORRECT_ICVV

ICVV (Integrated Circuit Card Verification Value) is a security feature used to validate the authenticity of a card during chip-based transactions. There were problems reading the chip on the card, which may indicate an issue with the card's chip, the terminal, or the transaction process. It is best to wait a bit and try again.

INVALID_MERCHANT

Transaction from a specific merchant is declined by scheme. The merchant should clarify the exact cause with scheme.

INVALID_TRANSACTION

We don't support certain types of transactions, so the customer should ask the merchant to use a different payment method or try a different merchant.

MANDATE_DCC_NON_SUPPORTED_FOR_CARD_COUNTRY

The transaction was declined because our system doesn't support conversions for Brazilian cards when BRL is involved.

The BRL won't be automatically exchanged to other currencies.

If the customer wants to continue with the payment, they need to change the currency they're using.

MANDATE_LOCAL_CASH_WITHDRAWAL_NOT_ALLOWED

Currently, we do not provide ATM withdrawal services in the country where your transaction is taking place using your card.

NON_SUPPORTED_CURRENCY

At present, we do not offer support for this currency in this transaction.

NON_SUPPORTED_MCC_FOR_COUNTRY

At present, we do not offer support for transactions in this category for customers in the country of purchase. Please consider using an alternative payment method or changing merchant.

PAYMENT_METHOD_DAILY_LIMIT_EXCEEDED

The customer has reached daily spending limit for the card or their profile.

Please advise if they would like to update card or profile limit.

PAYMENT_METHOD_LIFETIME_LIMIT_EXCEEDED

The customer has reached lifetime spending limit.

Please advise if they would like to increase their lifetime limit.

PAYMENT_METHOD_MONTHLY_LIMIT_EXCEEDED

The customer has reached monthly spending limit for the card or their profile.

Please advise if they would like to update card or profile limit.

PAYMENT_METHOD_NOT_ALLOWED

This payment type has been disabled

Please advise if they would like to enable the payment type.

PAYMENT_METHOD_TRANSACTION_LIMIT_EXCEEDED

The customer has exceeded transaction limit for the card.

Please advise if they would like to update their card limit.

PIN_ENTRY_TRIES_EXCEEDED

The customer has reached the maximum number of allowed online PIN entry attempts.

To avoid further inconvenience, we recommend implementing a reset PIN feature within your app to help the customer regain access to their card.

PRE_ACTIVATED_CARD_PIN_ENTRY_REQUIRED

The customer has attempted to make a contactless payment at a Point of Sale (POS) or ATM, but their card has not been activated for chip and PIN transactions.

If you would like to modify the card activation strategy for all your cards, please contact your implementation manager for assistance.

PROCESSING_ERROR

Our system is currently experiencing technical difficulties.

Please advise the customer to try again after a brief period of time.

RESTRICTED_MODE

Although rare, restricted mode can happen, and it's advisable to advise the customer to replace their card promptly as our system should have already informed them.

In this mode, more secure payment methods like chip and PIN, contactless, mobile wallets, and online payments with 3DS are allowed, while less secure methods like magnetic stripe and online payments without 3DS are not permitted.

REVERSAL_NOT_MATCHING_AUTH_CURRENCY

The merchant has issued a reversal instruction for a different currency than what was originally requested during the authorization process.

SCA_SOFT_DECLINE

We cannot proceed with the transaction due to SCA regulations.

Kindly suggest to the customer that they contact the merchant and use a more secure authentication method such as 3DS.

For example, customer can try to use chip & pin, or mobile wallet like (Apply Pay or Google Pay).

SCHEME_BLOCKED_TRANSACTION

This transaction has been flagged by the scheme and cannot be processed.

SECURITY_CVM_FAILURE

Our system has detected that the POS terminal was misconfigured and failed our security checks.

Kindly suggest to the customer that they use an alternative payment method like contactless or mobile wallets, or recommend that they ask the merchant to accept a signature instead.

SECURITY_MAGSTRIPE_SECURE_ELEMENTS_INCORRECT_OR_MISSING

The merchant has entered the wrong type of purchase.

Please advise the customer to contact the merchant and ask them to correct this issue.

SECURITY_PIN_ENTRY_REQUIRED

To proceed with this transaction, we require customer to enter their PIN.

Please advise the customer to enter their PIN to continue.

SUSPECTED_FRAUD

This transaction has been labeled as high-risk by Wise.

If you have any questions or concerns, please reach out to us.

SUSPECTED_FRAUD_AML

This transaction has been flagged as high-risk based on our AML compliance protocols.

Please be aware that we cannot disclose this reason to your end customers.

Please contact us if you believe this classification is incorrect.

SUSPECTED_FRAUD_COMPLIANCE

Our compliance system has flagged this transaction as high-risk.

Please be aware that we cannot disclose this reason to your end customers.

If you believe this classification is in error, please reach out to us for further assistance.

SUSPECTED_FRAUD_CORE_FRAUD

This transaction has been blocked based on our fraud policies and procedures.

Please contact us if you have any questions or concerns.

SUSPECTED_FRAUD_SANCTIONS

This transaction has been flagged as high-risk based on our sanctions list analysis.

Please be aware that we cannot disclose this reason to your end customers.

Unfortunately, this classification is final and cannot be appealed.

SUSPECTED_FRAUD_SOFT_DECLINE

We cannot process this e-commerce transaction due to high risk factors. We require the merchant to complete 3DS before we can approve the transaction.

TRANSACTION_TYPE_NOT_SUPPORTED

We have restrictions on this type of transaction, and sometimes the scheme won't allow it.

Please check if spend control is set up to block this transaction.

UNEXPECTED_ERROR

There may have been a communication error between the merchant's system and our server, but the POS system may have already notified the user of this issue.

The Card Transaction resource
Fields
idtext

ID of the transaction

cardTokenstring

Unique identifier of the card

typestring

Type of the transaction

declineReasonDeclineReason

Code of the decline reason if applicable

createdDatedatetime

When transaction or transaction state change occurred

statestring

The current state of the transaction

cardLastDigitsstring

Last 4 digits of the card

transactionAmount.valuedecimal

Transaction amount

transactionAmount.currencystring

Currency code

fees[n].amountdecimal

Fee amount

fees[n].currencystring

Currency code

fees[n].fee_typestring

Fee type

transactionAmountWithFees.valuedecimal

Transaction amount including fees

transactionAmountWithFees.currencystring

Currency code

merchant.namestring

Name of the merchant

merchant.location.countrystring

Country where merchant is located

merchant.location.citystring

City where merchant is located

merchant.location.zipCodestring

Zip code where merchant is located

merchant.location.regionstring

Region where merchant is located

merchant.location.statestring

State where merchant is located

merchant.category.namestring

Category of the merchant

merchant.category.codestring

MCC code of the merchant

merchant.category.descriptionstring

Description of the merchant

authorisationMethodstring

Authorisation method

balanceTransactionIdinteger

Associated balance transaction ID if applicable

debits[n].balanceIdinteger

Balance ID

debits[n].debitedAmount.amountdecimal

Amount taken from the balance

debits[n].debitedAmount.currencystring

Currency code

debits[n].forAmount.amountdecimal

Amount converted to

debits[n].forAmount.currencystring

Currency code

debits[n].ratedecimal

Exchange rate

debits[n].fee.amountdecimal

Conversion fee amount

debits[n].fee.currencystring

Currency code

credit.balanceIdinteger

Balance ID

credit.creditedAmount.amountdecimal

Amount credited to the balance

credit.creditedAmount.currencystring

Currency code

Example
{
  "id": "342671",
  "cardToken": "59123122-223d-45f9-b840-0ad4a4f80937",
  "type": "ECOM_PURCHASE",
  "declineReason": null,
  "createdDate": "2022-11-28T08:17:54.241236Z",
  "state": "IN_PROGRESS",
  "cardLastDigits": "3086",
  "transactionAmount": {
    "amount": 1.5,
    "currency": "SGD"
  },
  "fees": [],
  "transactionAmountWithFees": {
    "amount": 1.5,
    "currency": "SGD"
  },
  "merchant": {
    "name": "Test Payment",
    "location": {
      "country": "France",
      "city": "Rouen",
      "zipCode": "00000",
      "region": null,
      "state": null
    },
    "category": {
      "name": "RMiscellaneousAndSpecial",
      "code": "5999",
      "description": "5999 R Miscellaneous and Special"
    }
  },
  "authorisationMethod": "MANUAL_ENTRY",
  "balanceTransactionId": 2598366,
  "debits": [
    {
      "balanceId": 52832,
      "debitedAmount": {
        "amount": 1.06,
        "currency": "EUR"
      },
      "forAmount": {
        "amount": 1.5,
        "currency": "SGD"
      },
      "rate": 1.43073,
      "fee": {
        "amount": 0.01,
        "currency": "EUR"
      }
    }
  ],
  "credit": null
}
Get card transaction by ID

GET /v3/spend/profiles/{{profileId}}/cards/transactions/{{transactionId}}

Retrieve a card transaction based on its transaction ID.

Response

The possible type values are:

ACCOUNT_CREDIT - Receiving money on the card, excluding Visa OCT or Mastercard MoneySend
ACCOUNT_FUNDING - Sending money to another card or e-wallet
CASH_ADVANCE - Cash disbursement
CASH_WITHDRAWAL - ATM withdrawal
CHARGEBACK - Currently unused. Reserved for future use.
CREDIT_TRANSACTION - Visa OCT and Mastercard MoneySend
ECOM_PURCHASE - Online purchase
POS_PURCHASE - Purchase via a POS Terminal
REFUND - Partial or full refund of an existing card transaction

The possible state values are listed here.

When a refund happens, a separate transaction will be added with a REFUND transaction type.

Example Request
curl -X GET \
  https://api.wise-sandbox.com/v3/spend/profiles/{{profileId}}/cards/transactions/{{transactionId}} \
  -H 'Authorization: Bearer <your api token>'
Example Response
{
  "id": "342671",
  "cardToken": "59123122-223d-45f9-b840-0ad4a4f80937",
  "type": "REFUND",
  "declineReason": null,
  "createdDate": "2022-11-28T08:17:54.241236Z",
  "state": "IN_PROGRESS",
  "cardLastDigits": "3086",
  "transactionAmount": {
    "amount": -1.5,
    "currency": "SGD"
  },
  "fees": [],
  "transactionAmountWithFees": {
    "amount": -1.5,
    "currency": "SGD"
  },
  "merchant": {
    "name": "Test Payment",
    "location": {
      "country": "France",
      "city": "Rouen",
      "zipCode": "00000",
      "region": null,
      "state": null
    },
    "category": {
      "name": "RMiscellaneousAndSpecial",
      "code": "5999",
      "description": "5999 R Miscellaneous and Special"
    }
  },
  "authorisationMethod": "MANUAL_ENTRY",
  "balanceTransactionId": 2598366,
  "debits": [],
  "credit": {
    "balanceId": 52832,
    "creditedAmount": {
      "amount": 1.5,
      "currency": "SGD"
    }
  }
}