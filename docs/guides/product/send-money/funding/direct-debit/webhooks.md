# WebhooksCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/send-money/funding/direct-debit/webhooks

---

Webhooks
Copy

Webhooks provide real-time updates on status changes for your Payment Instruments and Payins. You can subscribe to these events to avoid constantly polling the API. See here for how to create a webhook with Wise.

There is a distinct webhook for Payment Instrument and Payin status changes:

Payment Instrument Status Change - The payment-instruments#status-change webhook notifies you when an instrument’s status changes (e.g., from PENDING to CONNECTED or DISCONNECTED).
Payin Status Change - The payment-instruments-payins#status-change webhook informs you of every status change for a Payin, including PROCESSING, CONFIRMED, SETTLED, or FAILED.