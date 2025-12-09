# Unauthenticated QuotesCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/send-money/quotes/unauthenticated-quote

---

Unauthenticated Quotes
Copy

Get exchange rates and fees from Wise without authentication, perfect for showcasing example exchange rates, fees and trasnfer speeds on your website or app.

What Are Unauthenticated Quotes?

Unauthenticated quotes let you fetch example exchange rates and fees while only requiring basic transfer details. These quotes show potential costs and exchange rates, making them ideal for marketing pages and product displays without user sign-up.

Important Limitations

Unauthenticated quotes:

Cannot be used to create actual transfers
Are for display and estimation purposes only
Should not be stored long-term or relied upon for transactions
Rates expire and should be refreshed regularly

To create actual transfers, you must use the authenticated quote endpoint.

Creating an Unauthenticated Quote

To create a unauthenticated quote, you'll send an HTTP POST request to /v3/quotes.

Request Body

The body of your request is where you'll specify all the information about the FX payment quote you want to create.

You will need to always provide the currencies to exchange:

Parameter	Description	Example
sourceCurrency	Currency for sending currency (ISO 4217)	"USD"
targetCurrency	Currency for receiving currency (ISO 4217)	"EUR"

Then EITHER sourceAmount OR targetAmount (never both):

Parameter	Description	Example
sourceAmount	Amount to send	100.00
targetAmount	Amount recipient should receive	85.00

Here's some example for both scenerios.



Option A: Fixed Send Amount

Use when you know how much the user wants to send.

Example Request:

{
  "sourceCurrency": "USD",
  "targetCurrency": "EUR",
  "sourceAmount": 100
}


Option B: Fixed Receive Amount

Use when you know how much the recipient should receive.

Example Request:

{
  "sourceCurrency": "USD",
  "targetCurrency": "EUR",
  "targetAmount": 85
}
Understanding the Response
Complete Response Example
{
  "sourceCurrency": "USD",
  "targetCurrency": "EUR",
  "sourceAmount": 100.00,
  "targetAmount": 92.15,
  "rate": 0.9215,
  "createdTime": "2024-01-15T10:30:00Z",
  "rateType": "FIXED",
  "rateExpirationTime": "2024-01-15T11:00:00Z",
  "paymentOptions": [{
    "formattedEstimatedDelivery": "by Tuesday, January 16",
    "estimatedDeliveryDelays": [],
    "fee": {
      "transferwise": 2.58,
      "payIn": 0.00,
      "discount": 0.00,
      "total": 2.58
    }
  }]
}
Key Response Fields
Field	Description	Example Use
rate	Current exchange rate (targetCurrency per 1 sourceCurrency)	Display "1 USD = 0.9215 EUR"
sourceAmount	Total amount user needs to send	Show "You send $100.00"
targetAmount	Amount recipient receives after fees	Show "Recipient gets €92.15"
createdTime	When this quote was created (ISO 8601)	Log for debugging
rateExpirationTime	When this rate expires (ISO 8601)	Show "Rate valid for 30 minutes"
paymentOptions[0].fee.total	Total fee charged	Show "Fee: $2.58"
paymentOptions[0].formattedEstimatedDelivery	Human-readable delivery estimate	Show "Arrives by Tuesday"