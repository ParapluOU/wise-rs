# Direct Debit: Create a PayinCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/send-money/funding/direct-debit/create-payin

---

Direct Debit: Create a Payin
Copy

The Payment Instrument acts as a pre-authorised source of funds allowing for the funding of Transfers via a pull without the owner of the funds being present at the moment of initiation.

Before creating a Payin you need to generate the Transfer that will be funded, see this guide for more details.

Create a new Payin

To pull funds from a connected payment instrument, send a request to POST /v3/profiles/{{profileId}}/payment-instruments/{{paymentInstrumentId}}/payins.

You must provide the payinSessionId, which links the Payin to a specific transfer you are funding.

The API returns a Payin object with a status of PROCESSING.

Sample request:

{
  "payinSessionId": "8d63a219-eb86-42c7-a785-02875c281507",
  "reference": "PAY0001"
}

Sample response:

{
    "id": "67136333-0b4d-4f72-49bf-24088f687416",
    "amount": 123.45,
    "currency": "GBP",
    "status": "PROCESSING",
    "paymentInstrumentId": "67133810-d4f6-47b4-a948-09b2d8438b43",
    "payinSessionId": "8d63a219-eb86-42c7-a785-02875c281507",
    "reference": "PAY0001",
    "failureReason": null
}
Manage the Payin
Track the Payin

You can monitor the status of a Payin to see its progression. We expose two endpoints that can track the Payins:

Get a Payin: GET /v3/profiles/{{profileId}}/payment-instruments/{{paymentInstrumentId}}/payins/{{paymentId}}

Get all Payins for a Payment Instrument: GET /v3/profiles/{{profileId}}/payment-instruments/{{paymentInstrumentId}}/payins

We also expose updates to the Payin via webhooks, see the Webhooks section for more details.

Payin Status Breakdown
PROCESSING - Wise in the process of pulling the funds from the account.
CONFIRMED - The source institution has confirmed that the source instrument has been debited.
SETTLED - The funds are now available for use and will fund the associated transfer.
FAILED - The payment has failed due to cancellation or issues encountered while processing the payment. Refer to failureReason for more granular reasoning.
Handling a Failed Payin

Payins can fail for various reasons, when this occurs the status is set to FAILED.

We provide a failureReason field to allow you to handle these scenarios gracefully, which may be one of the following values:

REFER_TO_PAYER - The customer's bank was unable to pay the Direct Debit.
AUTHORISATION_DISPUTED - The customer has disputed authorising this mandate or the specific Payin amount.
PAYMENT_INSTRUMENT_DISCONNECTED - The payin failed because the associated payment instrument is no longer able to accept Payins
OTHER - A catch-all for other failure reasons not covered by the specific categories above. This may include technical issues or less common bank-specific failure reasons.