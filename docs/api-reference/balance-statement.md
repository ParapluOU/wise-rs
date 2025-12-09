# Balance StatementCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/api-reference/balance-statement

---

Balance Statement
Copy

Balance statements contains transactional activities on a Wise Multi-Currency Account.

Operations

GET
/v1/profiles/{{profileId}}/balance‑statements/{{balanceId}}/statement.json
The Balance Statement resource
Fields
accountHolder.typetext

Account holder type (PERSONAL, BUSINESS)

accountHolder.address.addressFirstLinetext

Account holder address street

accountHolder.address.citytext

Account holder address city

accountHolder.address.postCodetext

Account holder address zipcode

accountHolder.address.stateCodetext

Account holder address state

accountHolder.address.countryNametext

Account holder address country

accountHolder.firstNametext

Account holder first name

accountHolder.lastNametext

Account holder last name

issuer.nametext

Account issuer name

issuer.firstLinetext

Account issuer address street

issuer.citytext

Account issuer address city

issuer.postCodetext

Account issuer address zipcode

issuer.stateCodetext

Account issuer address state

issuer.countrytext

Account issuer address country

bankDetailsgroup

Your local bank details

transactions[n].typetext

Type (DEBIT, CREDIT)

transactions[n].datetimestamp

Date of when transaction was created

transactions[n].amount.valuedecimal

Transaction amount

transactions[n].amount.currencytext

Transaction currency code

transactions[n].totalFees.valuedecimal

Transaction fee amount

transactions[n].totalFees.currencytext

Transaction fee currency code

transactions[n].details.typetext

Type (CARD, CONVERSION, DEPOSIT, TRANSFER, MONEY_ADDED, INCOMING_CROSS_BALANCE , OUTGOING_CROSS_BALANCE, DIRECT_DEBIT, BALANCE_INTEREST, BALANCE_ADJUSTMENT, UNKNOWN, ACCRUAL_CHARGE, INVESTMENT_TRADE_ORDER, ACQUIRING_PAYMENT, CARD_CASHBACK, CARD_ORDER_CHECKOUT)

transactions[n].details.descriptiontext

Human readable explanation about the transaction

transactions[n].details.amount.valuedecimal

Amount in original currency (card transactions abroad)

transactions[n].details.amount.currencytext

Original currency code (ISO 4217 Alphabetic Code)

transactions[n].details.senderNametext

Deposit sender name

transactions[n].details.senderAccounttext

Deposit sender bank account details

transactions[n].details.paymentReferencetext

Deposit payment reference text

transactions[n].details.categorytext

Card transaction category

transactions[n].details.merchant.nametext

Card transaction merchant name

transactions[n].details.merchant.firstLinetext

Merchant address street

transactions[n].details.merchant.postCodetext

Merchant address zipcode

transactions[n].details.merchant.citytext

Merchant address city

transactions[n].details.merchant.statetext

Merchant address state

transactions[n].details.merchant.countrytext

Merchant address country

transactions[n].details.merchant.categorytext

Merchant category

transactions[n].exchangeDetails.toAmount.valuedecimal

Exchange target amount

transactions[n].exchangeDetails.toAmount.currencytext

Exchange currency code (ISO 4217 Alphabetic Code)

transactions[n].exchangeDetails.fromAmount.valuedecimal

Exchange source amount

transactions[n].exchangeDetails.fromAmount.currencytext

Exchange currency code (ISO 4217 Alphabetic Code)

transactions[n].exchangeDetails.ratedecimal

Exchange rate

transactions[n].runningBalance.valuedecimal

Running balance after the transaction

transactions[n].runningBalance.currencytext

Running balance currency code (ISO 4217 Alphabetic Code)

transactions[n].referenceNumbertext

Wise assigned unique transaction reference number, this number can be used to map the refunds to the transfer that was refunded.

endOfStatementBalance.valuedecimal

Closing balance for specified time period

endOfStatementBalance.currencytext

Closing balance currency code (ISO 4217 Alphabetic Code)

query.intervalStarttimestamp

Query parameter repeated

query.intervalEndtimestamp

Query parameter repeated

query.currencytext

Query parameter repeated

query.accountIdinteger

Query parameter repeated

Balance Statement Object
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
  "issuer": {
    "name": "TransferWise Ltd.",
    "firstLine": "56 Shoreditch High Street",
    "city": "London",
    "postCode": "E1 6JJ",
    "stateCode": "",
    "country": "United Kingdom"
  },
  "bankDetails": null,
  "transactions": [
    {
      "type": "DEBIT",
      "date": "2018-04-30T08:47:05.832Z",
      "amount": {
        "value": -7.76,
        "currency": "EUR"
      },
      "totalFees": {
        "value": 0.04,
        "currency": "EUR"
      },
      "details": {
        "type": "CARD",
        "description": "Card transaction of 6.80 GBP issued by Tfl.gov.uk/cp TFL TRAVEL CH",
        "amount": {
          "value": 6.8,
          "currency": "GBP"
        },
        "category": "Transportation Suburban and Loca",
        "merchant": {
          "name": "Tfl.gov.uk/cp",
          "firstLine": null,
          "postCode": "SW1H 0TL  ",
          "city": "TFL TRAVEL CH",
          "state": "   ",
          "country": "GB",
          "category": "Transportation Suburban and Loca"
        }
      },
      "exchangeDetails": {
        "forAmount": {
          "value": 6.8,
          "currency": "GBP"
        },
        "rate": null
      },
      "runningBalance": {
        "value": 16.01,
        "currency": "EUR"
      },
      "referenceNumber": "CARD-249281"
    },
    {
      "type": "CREDIT",
      "date": "2018-04-17T07:47:00.227Z",
      "amount": {
        "value": 200,
        "currency": "EUR"
      },
      "totalFees": {
        "value": 0,
        "currency": "EUR"
      },
      "details": {
        "type": "DEPOSIT",
        "description": "Received money from HEIN LAURI with reference SVWZ+topup card",
        "senderName": "HEIN LAURI",
        "senderAccount": "EE76 1700 0170 0049 6704 ",
        "paymentReference": "SVWZ+topup card"
      },
      "exchangeDetails": null,
      "runningBalance": {
        "value": 207.69,
        "currency": "EUR"
      },
      "referenceNumber": "TRANSFER-34188888"
    },
    {
      "type": "CREDIT",
      "date": "2018-04-10T05:58:34.681Z",
      "amount": {
        "value": 9.94,
        "currency": "EUR"
      },
      "totalFees": {
        "value": 0,
        "currency": "EUR"
      },
      "details": {
        "type": "CONVERSION",
        "description": "Converted 8.69 GBP to 9.94 EUR",
        "sourceAmount": {
          "value": 8.69,
          "currency": "GBP"
        },
        "targetAmount": {
          "value": 9.94,
          "currency": "EUR"
        },
        "fee": {
          "value": 0.03,
          "currency": "GBP"
        },
        "rate": 1.147806
      },
      "exchangeDetails": null,
      "runningBalance": {
        "value": 9.94,
        "currency": "EUR"
      },
      "referenceNumber": "CONVERSION-1511237"
    }
  ],
  "endOfStatementBalance": {
    "value": 9.94,
    "currency": "EUR"
  },
  "query": {
    "intervalStart": "2018-03-01T00:00:00Z",
    "intervalEnd": "2018-04-30T23:59:59.999Z",
    "currency": "EUR",
    "accountId": 64
  }
}
Retrieving a balance account statement

GET /v1/profiles/{{profileId}}/balance-statements/{{balanceId}}/statement.json?currency=EUR&intervalStart=2018-03-01T00:00:00.000Z&intervalEnd=2018-03-15T23:59:59.999Z&type=COMPACT

This endpoint allows for statements to be generated for the provided balanceId, with the response in JSON. To generate in CSV, PDF, XLSX, CAMT.053, MT940 or QIF, replace statement.json with statement.csv, statement.pdf, statement.xlsx, statement.xml, statement.mt940 or statement.qif respectively in the above URL. Note that the PDF includes Wise branding.

The period between intervalStart and intervalEnd cannot exceed 469 days (around 1 year 3 months).

This endpoint is SCA protected when it applies. If your profile is registered within the UK and/or EEA, SCA most likely applies to you. The additional authentication is only required once every 90 days, viewing the statement on the website or in the mobile app counts towards that as well.
Learn more

Request
currencytext

Currency of the balance statement requested

intervalStarttimestamp

Statement start time in UTC time

intervalEndtimestamp

Statement end time in UTC time

typetext
COMPACT for a single statement line per transaction
FLAT for accounting statements where transaction fees are on a separate line
statementLocaletext

Language that you wish the statement to be in. Supports 2 character language codes

Response

Returns a balance statement object.

Example Request
curl -X GET \
  https://api.wise-sandbox.com/v1/profiles/{{profileId}}/balance-statements/{{balanceId}}/statement.json
    ?currency=EUR
    &intervalStart=2018-03-01T00:00:00.000Z
    &intervalEnd=2018-03-15T23:59:59.999Z
    &type=COMPACT \
  -H 'Authorization: Bearer <your api token>'