# Fund transfers with Direct DebitCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/send-money/funding/direct-debit

---

Fund transfers with Direct Debit
Copy

Use the Wise direct debits API to pull funds from a UK or European bank account to fund your transfers.

This guide provides step-by-step instructions for integrating with the direct debit API, from creating a payment instrument to managing transfers and webhooks.

Core concepts

To fund a transfer using direct debit, you need to work with two primary resources: Payment Instruments and Payins.

A Payment Instrument is a saved pre-authorised funding source, such as a bank account, from which you can pull funds.
A Payin is the request you initiate to pull a specific payment from that instrument.
How Direct Debit payin can fund a transfer

The Direct Debit API uses the paymentInstrumentId and the payinSessionId from a new transfer to create a new Payin. Once the direct debit payin is received, the associated transfer is funded. The funds are never shown as a credit on your balance and are used immediately.

Overview of API requests

Follow these steps to implement direct debits via the API.

Create the Payment Instrument: Send a request to POST /v3/profiles/{{profileId}}/payment-instruments to initiate the authorization flow for a new funding source.
Connect the Payment Instrument: Send a request to POST /v3/profiles/{{profileId}}/payment-instruments/{{paymentInstrumentId}}/manual-confirmation to confirm the payment instrument's details are correct.
This leads to a CONNECTED Payment Instrument, which then supports the submission of new Payins
Create a transfer: As with any process to send money on Wise Platform - you must create a quote, and then create a transfer using POST /v1/transfers.
Capture the payinSessionId from the response you receive.
Create a Payin: Send a request, including the payinSessionId, to POST /v3/profiles/{{profileId}}/payment-instruments/{{paymentInstrumentId}}/payins.
Once this payin is complete, the transfer is funded and your money is sent to the transfer recipient.
Check status: Send a GET request for either a payment instrument or a payin to retrieve its status at any time.
Next steps
Create and connect a payment instrument.
Create and track payins.
Learn more about sending money.
Learn more about Direct Debit mandates and standards.