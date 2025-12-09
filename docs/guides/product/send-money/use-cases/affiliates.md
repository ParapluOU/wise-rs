# Affiliates Integration GuideCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/send-money/use-cases/affiliates

---

Affiliates Integration Guide
Copy
API access

Once you become our affiliate partner, please get in touch with our team at partnerwise@wise.com. We will review your request and send over instructions on how to get the API access credentials.

There are two endpoints Exchange Rates.List and Get Temporary Quote which you can call with this authentication method.

Postman Collection

Please visit our Postman collection for more details and examples of how to use our API, including how to get comparison, rates as well as pricing and speed data.

TEST and LIVE environments
Sandbox API is located at https://api.wise-sandbox.com
LIVE API is located at https://api.transferwise.com
Retrieve current rates

GET /v1/rates

Fetch latest exchange rates of all currencies.


GET /v1/rates?source=EUR&target=USD

Fetch latest exchange rate of one currency pair.


This endpoint only supports Basic authentication (client_id/client_secret) for Affiliate partners.

Please see the rate API reference for full details.

Example Request (Basic authentication)
curl -X GET https://api.wise-sandbox.com/v1/rates?source=EUR&target=USD \
     --user <your api client_id>:<your api client_secret> 
Example Response
[
  {
    "rate": 1.166,
    "source": "EUR",
    "target": "USD",
    "time": "2018-08-31T10:43:31+0000"
  }
]
Get pricing and speed

POST https://api.wise-sandbox.com/v2/quotes

Is currency route supported?

If we don't support a route then this endpoint will respond with an error code "error.route.not.supported".

How much does a transfer cost?

The Wise fee is included in the response.

How long does my transfer take?

Estimated delivery time is included in the response. This can vary quite a lot for different currency routes. For example transfers often only take a few hours from EUR to GBP, while sending money from USD can take 1-2 business days. This endpoint allows you to find out the estimated delivery time for each currency route.

See more at Creating Temporary Quote

Example Request
curl -X POST https://api.wise-sandbox.com/v2/quotes \
    --header 'Content-Type: application/json' \
    --data-raw '{"targetCurrency":"GBP","targetAmount":600,"sourceCurrency":"EUR"}' \
    --user <your api client_id>:<your api client_secret>
Example Response
{
  "sourceCurrency": "EUR",
  "targetCurrency": "GBP",
  "targetAmount": 600,
  "clientId": "unknown",
  "createdTime": "2022-10-25T10:03:22Z",
  "expirationTime": "2022-10-25T10:33:22Z",
  "funding": "POST",
  "guaranteedTargetAmount": false,
  "guaranteedTargetAmountAllowed": true,
  "notices": [],
  "payOut": "BANK_TRANSFER",
  "paymentOptions": [
    {
      "allowedProfileTypes": ["PERSONAL", "BUSINESS"],
      "disabled": false,
      "estimatedDelivery": "2022-10-25T10:03:22Z",
      "estimatedDeliveryDelays": [],
      "feePercentage": 0.0048,
      "formattedEstimatedDelivery": "in seconds",
      "payIn": "BANK_TRANSFER",
      "payInProduct": "CHEAP",
      "payOut": "BANK_TRANSFER",
      "price": {
        "items": [
          {
            "type": "TRANSFERWISE",
            "label": "Our fee",
            "value": {
              "amount": 3.32,
              "currency": "EUR",
              "label": "3.32 EUR"
            },
            "priceSetId": 132,
            "total": {
              "type": "TOTAL",
              "label": "Total fees",
              "value": {
                "amount": 3.32,
                "currency": "EUR",
                "label": "3.32 EUR"
              }
            }
          }
        ]
      },
      "sourceAmount": 691.85,
      "sourceCurrency": "EUR",
      "targetAmount": 600,
      "targetCurrency": "GBP"
    }
  ],
  "providedAmountType": "TARGET",
  "rate": 0.871426,
  "rateExpirationTime": "2022-10-27T14:59:59Z",
  "rateTimestamp": "2022-10-25T10:02:42Z",
  "rateType": "FIXED",
  "status": "PENDING",
  "targetAmountAllowed": true,
  "type": "REGULAR"
}