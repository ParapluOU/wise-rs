# Transfer To Balance AccountCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/accounts/transfer-to-balance-account

---

Transfer To Balance Account
Copy

There are two different ways you can transfer money to a multi-currency balance accounts:

Simply send domestic bank transfer from a bank account to the balance account's bank account details (e.g. IBAN, sort code & account number, etc.). There is no need to set up a quote/transfer in this case.

Create a quote/transfer and then fund that transfer as per requirements.

Get multi-currency account recipientId

Next, call Check account balance endpoint and get the recipientId field value from the response.

Note that recipientId does not change so can be stored and reused.

Create transfer

Next, create a transfer. This is the same call as described in the Transfer Create API Reference, however the value for targetAccount should be the recipientId as determined from the previous step.

As per the standard create transfer flow, you also need to create and pass a customerTransactionId for idempotency.

Fund transfer

Once the transfer is created, a method will be required to fund the transfer. This will differ based on your specific integration.

In order to simulate transfers in Sandbox, use the provided sandbox simulation API. Simulate Transfers in Sandbox