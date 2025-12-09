# Bulk SettlementCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/api-reference/bulk-settlement

---

Bulk Settlement
Copy

Operations

POST
/v1/settlements
The Settlement Journal resource
Request Fields
typestring

Type of settlement. Always TRUSTED_BULK_SETTLEMENT.

settlementReferencestring

Reference used when paying the end of day settlement monies to TW to cover all transfers. This should match such that we can link the payment. It can have up to 10 characters and must start with TPFB four letter prefix.

settlementDatestring

The date you send the settlement funds, ISO8601 format.

settlementCurrencystring

The currency in which the transfers in this journal will be settled. If included, the exchangeRate must be set on every transfer in the transfers array. Note: Only include if you will settle in a currency different to the source currency of transfers.

transfersarray of objects

Array of transfers that happened on the settlementDate.

transfers.idlong

ID of the transfer to be settled.

transfers.datestring

The date the transfer was created, ISO8601 format.

transfers.sourceAmountdecimal

Transfer source amount with fee, always positive.

transfers.sourceCurrencystring

Transfer source currency.

transfers.customerNamestring

Name of the customer. Helps to identify customer when manual operation is required.

transfers.partnerReferencestring

The transaction/payment identifier in your system, uniquely identifies the transfer in your platform.

transfers.commentstring

Other data about the transfer or sender. For USA, you should send the account refund information for the sending customer (account number, routing number, street address, etc.)

transfers.exchangeRatedecimal

The exchange rate you used to calculate the settlement amount for this transfer. This is only required if the source currency is different from the settlement currency. This is a required field if the settlementCurrency field has been set for the journal, and it must be set for all transfers. You can set this value to be unique per transfer or the same for every transfer, depending on your settlement approach and agreements with Wise.

refundedTransfersarray if object

Array of transfers which refunds have been processed. Only for Net Settlement.

refundedTransfers.idlong

ID of the refunded transfer.

refundedTransfers.exchangeRatedecimal

The exchange rate you used to calculate the settlement amount for this transfer. This should be the same value as the one used for the original transfer settlement.

refundedTransfers.partnerReferencestring

The transaction/payment identifier in your system, uniquely identifies the transfer in you platform.

balanceTransferdecimal

The amount of money to be deducted from the expected settlement amount based on what is owed to you for previous days where the total settlement from you to Wise was negative. This value must be 0 or negative.

Settlement Journal Object
{
  "type": "TRUSTED_BULK_SETTLEMENT",
  "settlementReference": "TPFB190322",
  "settlementDate": "2019-03-22T23:59:59-05:00",
  "settlementCurrency": "USD",
  "transfers": [
    {
      "id": 125678,
      "date": "2019-03-22T10:00:12-05:00",
      "sourceAmount": 23.24,
      "sourceCurrency": "PHP",
      "customerName": "Joe Bloggs",
      "partnerReference": "11111",
      "comment": "Extra Data",
      "exchangeRate": 0.875469
    },
    {
      "id": 178889,
      "date": "2019-03-23T12:40:05-05:00",
      "sourceAmount": 125.67,
      "sourceCurrency": "PHP",
      "customerName": "Mat Newman",
      "partnerReference": "11112",
      "comment": "Extra Data",
      "exchangeRate": 0.875469
    }
  ],
  "refundedTransfers": [
    {
      "id": 178889,
      "partnerReference": "11112"
    },
    {
      "id": 124345,
      "partnerReference": "11102"
    }
  ],
  "balanceTransfer": 0
}
Send a settlement journal

POST /v1/settlements

Sends the settlement journal. There are two types of settlement:

Same Currency Settlement - If you settle in the same currency. e.g. settle USD account with USD
Cross Currency Settlement - If you settle in a different currency. e.g. settle PHP account with USD. In this case, you will need to specify settlementCurrency and transfers.exchangeRate fields. Settlement Currency will be the currency you plan to settle in or pay, which is USD for this example.
You must use a client credentials token to authenticate this call
Request Fields
typestring

Type of settlement. Always TRUSTED_BULK_SETTLEMENT.

settlementReferencestring

Reference used when paying the end of day settlement monies to Wise to cover all transfers. This should match such that we can link the payment. It can have up to 10 characters and must start with TPFB four letter prefix.

settlementDatestring

The date you send the settlement funds, ISO8601 format.

settlementCurrency (optional)string

The currency in which the transfers in this journal will be settled. If included, the exchangeRate must be set on every transfer in the transfers array. Note: Only include if you will settle in a currency different to the source currency of transfers.

transfersarray of objects

Array of transfers that happened on the settlementDate.

transfers.idlong

ID of the transfer to be settled.

transfers.datestring

The date the transfer was created, ISO8601 format.

transfers.sourceAmountdecimal

Transfer source amount with fee, always positive.

transfers.sourceCurrencystring

Transfer source currency.

transfers.customerNamestring

Name of the customer. Helps to identify customer when manual operation is required.

transfers.partnerReferencestring

The transaction/payment identifier in your system, uniquely identifies the transfer in your platform.

transfers.comment (optional)string

Other data about the transfer or sender. For USA, you should send the account refund information for the sending customer (account number, routing number, street address, etc.)

transfers.exchangeRate (optional)decimal

The exchange rate you used to calculate the settlement amount for this transfer. This is only required if the source currency is different from the settlement currency. This is a required field if the settlementCurrency field has been set for the journal, and it must be set for all transfers. You can set this value to be unique per transfer or the same for every transfer, depending on your settlement approach and agreements with Wise.

refundedTransfersarray if object

Array of transfers which refunds have been processed. Only for Net Settlement.

refundedTransfers.idlong

ID of the refunded transfer.

refundedTransfers.exchangeRatedecimal

The exchange rate you used to calculate the settlement amount for this transfer. This should be the same value as the one used for the original transfer settlement.

refundedTransfers.partnerReferencestring

The transaction/payment identifier in your system, uniquely identifies the transfer in you platform.

balanceTransferdecimal

The amount of money to be deducted from the expected settlement amount based on what is owed to you for previous days where the total settlement from you to Wise was negative. This value must be 0 or negative.

Example Request - Same Currency Settlement
curl -X POST \
  https://api.wise-sandbox.com/v1/settlements \
  -H "Authorization: Bearer <your client credentials token>" \
  -H "Content-Type: application/json" \
  -d '{
    "type": "TRUSTED_BULK_SETTLEMENT",
    "settlementReference": "TPFB190322",
    "settlementDate": "2019-03-22T23:59:59-05:00",
    "transfers": [
      {
        "id": 125678,
        "date": "2019-03-22T10:00:12-05:00",
        "sourceAmount": 23.24,
        "sourceCurrency": "USD",
        "customerName": "Joe Bloggs",
        "partnerReference": "11111",
        "comment": "Extra Data"
      },
      {
        "id": 178889,
        "date": "2019-03-23T12:40:05-05:00",
        "sourceAmount": 125.67,
        "sourceCurrency": "USD",
        "customerName": "Mat Newman",
        "partnerReference": "11112",
        "comment": "Extra Data"
      }
    ],
    "refundedTransfers": [
      {
        "id": 178880,
        "partnerReference": "11108"
      }
    ],
    "balanceTransfer": 0
  }'
Example Request - Cross Currency Settlement
 curl -X POST \
  https://api.wise-sandbox.com/v1/settlements \
  -H "Authorization: Bearer <your client credentials token>" \
  -H "Content-Type: application/json" \
  -d '{
    "type": "TRUSTED_BULK_SETTLEMENT",
    "settlementReference": "TPFB190322",
    "settlementDate": "2019-03-22T23:59:59-05:00",
    "settlementCurrency": "USD",
    "transfers": [
      {
        "id": 125678,
        "date": "2019-03-22T10:00:12-05:00",
        "sourceAmount": 23.24,
        "sourceCurrency": "PHP",
        "customerName": "Joe Bloggs",
        "partnerReference": "11111",
        "comment": "Extra Data",
        "exchangeRate": 0.875469
      },
      {
        "id": 178889,
        "date": "2019-03-23T12:40:05-05:00",
        "sourceAmount": 125.67,
        "sourceCurrency": "PHP",
        "customerName": "Mat Newman",
        "partnerReference": "11112",
        "comment": "Extra Data",
        "exchangeRate": 0.875469
      }
    ],
    "refundedTransfers": [
      {
        "id": 178880,
        "partnerReference": "11108"
      },
      {
        "id": 178881,
        "partnerReference": "11109"
      }
    ],
    "balanceTransfer": 0
  }'
Example Response

HTTP 200

Empty body