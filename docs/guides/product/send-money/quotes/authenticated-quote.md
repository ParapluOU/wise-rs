# Authenticated QuotesCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/send-money/quotes/authenticated-quote

---

Authenticated Quotes
Copy

Create quotes tied to user profiles to enable actual money transfers, the essential step for the Send Money flow.

What are Authenticated Quotes?

Authenticated quotes are profile-specific exchange rate quotes that serve as the foundation for creating actual money transfers. Unlike unauthenticated quotes (used for display purposes), authenticated quotes can be used to create real transfers.

They include a unique quote ID and lock in the rate for a limited time.

When to use this endpoint

Use authenticated quotes when:

Users are ready to create an actual transfer
You need to lock in an exchange rate
You're building the transfer creation flow
You need to show exact fees for a real transaction
Creating an authenticated quote

To create a unauthenticated quote, you'll send an HTTP POST request to /v3/profiles/{profileId}/quotes.

You will need to pass in the header:

Parameter	Description	Example
profileId	The user or account's profile ID	87654321
Request Body

The body of your request is where you'll specify all the information about the FX payment quote you want to create.

You will need to always provide the currencies to exchange:

Parameter	Description	Example
sourceCurrency	Currency for sending currency (ISO 4217)	"USD"
targetCurrency	Currency for receiving currency (ISO 4217)	"EUR"

If the account ID is not yet known, you can pass null. You can then PATCH to add the recipient information once you have it.

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
  "sourceAmount": 1000,
  "targetAccount": null
}


Option B: Fixed Receive Amount

Use when you know how much the recipient should receive.

Example Request:

{
  "sourceCurrency": "USD",
  "targetCurrency": "EUR",
  "targetAmount": 850,
  "targetAccount": null
}

*Choose one amount parameter, not both.

Optional Parameters
Payment Method Selection
Parameter	Type	Description	Example
payOut	string	Payout method (if multiple available)	"BANK_TRANSFER", "SWIFT", "INTERAC"

Example:

{
  "sourceCurrency": "CAD",
  "targetCurrency": "USD",
  "sourceAmount": 1000,
  "targetAccount": null,
  "payOut": "INTERAC"
}
Pre-linking to Recipient

Instead of null, you can provide a recipient account ID to get fees specific to that recipient:

{
  "sourceCurrency": "USD",
  "targetCurrency": "EUR",
  "sourceAmount": 1000,
  "targetAccount": 98765432
}

Benefits:

More accurate fees (some recipients may have different fee structures)
Faster transfer creation (one less step)
Better user experience
Understanding the Response
Complete Response Example
{
  "id": "a0123456-b789-c012-d345-e67890123456",
  "sourceCurrency": "USD",
  "targetCurrency": "EUR",
  "sourceAmount": 1000.00,
  "targetAmount": 921.50,
  "rate": 0.9215,
  "createdTime": "2024-01-15T10:30:00Z",
  "createdByUserId": 12345678,
  "user": 12345678,
  "profile": 87654321,
  "rateType": "FIXED",
  "rateExpirationTime": "2024-01-15T11:00:00Z",
  "providedAmountType": "SOURCE",
  "paymentOptions": [
    {
      "payIn": "BALANCE",
      "fee": {
        "transferwise": 5.15,
        "payIn": 0.00,
        "discount": 0.00,
        "partner": 0.00,
        "total": 5.15
      },
      "price": {
        "priceSetId": 189,
        "total": {
          "type": "TOTAL",
          "label": "Total fees",
          "value": {
            "amount": 5.15,
            "currency": "USD",
            "label": "5.15 USD"
          }
        }
      },
      "sourceAmount": 1000.00,
      "targetAmount": 921.50,
      "estimatedDelivery": "2024-01-17T17:00:00Z",
      "formattedEstimatedDelivery": "by Wednesday, January 17",
      "estimatedDeliveryDelays": [],
      "allowedProfileTypes": ["PERSONAL", "BUSINESS"],
      "payInProduct": "BALANCE",
      "feePercentage": 0.51
    }
  ],
  "payOut": "BANK_TRANSFER",
  "status": "PENDING",
  "expirationTime": "2024-01-16T10:30:00Z",
  "notices": []
}
Key Response Fields
Field	Description	Example Use
id	Unique quote identifier - save this!	Use to create transfer
rate	Exchange rate locked for this quote	Display "1 USD = 0.9215 EUR"
sourceAmount	Total amount user needs to send	Show "You send $1,000.00"
targetAmount	Amount recipient receives	Show "Recipient gets €921.50"
rateType	"FIXED" or "FLOATING"	Inform user if rate is guaranteed
rateExpirationTime	When the locked rate expires	Show countdown timer
expirationTime	When the entire quote expires	Must create transfer before this
paymentOptions[0].fee.total	Total fee in source currency	Show "Fee: $5.15"
paymentOptions[0].formattedEstimatedDelivery	Human-readable delivery time	Show "Arrives by Wednesday"
paymentOptions[0].estimatedDelivery	Exact estimated delivery timestamp	Calculate delivery time
paymentOptions[0].payIn	Payment method	"BALANCE", "BANK_TRANSFER", etc.
status	Quote status	"PENDING", "ACCEPTED", "EXPIRED", CANCELLED
profile	Profile ID this quote belongs to	Verify correct profile
providedAmountType	Which amount was provided	"SOURCE" or "TARGET"
Quote Status Values
Status	Description	Can Create Transfer?
PENDING	Quote is active and usable	✅ Yes
ACCEPTED	Quote was used to create a transfer	❌ No (already used)
EXPIRED	Quote has expired	❌ No
CANCELLED	Quote was cancelled	❌ No