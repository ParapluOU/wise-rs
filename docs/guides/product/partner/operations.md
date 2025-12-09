# Operations GuideCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/partner/operations

---

Operations Guide
Copy

This operational guide pulls together all of the webhooks and API endpoints we have available for your operational excellence. Implementing these will help you make your operational processes bulletproof – reduce customer contacts and allow your own operational teams to help the customer with any remaining queries.

The guide walks you through the most common operational workflows and aims to help you understand which resources you can implement to tackle each of them. We cover the following flows:

Customer has a question about payment status / Money hasn’t reached the recipient
Customer needs a receipt or proof of payout
Customer asks why the funds were returned to them
Customer is unable to create transfers because their account is no longer active.

In all of these cases, there are a number of solutions available. In most cases, you have two ways to implement these:

Implement in your customer-facing UI so customers can help themselves and will not need to reach out to you.
Implement in your back office UI so your operational teams can help your customers when they reach out.

For convenience, we’ve pulled all helpful resources into a table. The links will redirect you to the endpoint in our API reference or to the event type in the webhooks section of our API docs.

Necessary Webhooks and Endpoints
Webhook or Endpoint	Link	Description
webhook	balance-update	Notification whenever a multi-currency account is credited or debited
webhook	transfer-state-change	Notification whenever transfer state changes
webhook	transfer-active-cases	Notification whenever a transfer’s list of active cases is updated (active cases indicate potential problems with transfer processing)
webhook	transfer-payout-failure	Notification of payout failure reason whenever a payout fails and funds bounce back
webhook	users-state-change	Notification that triggers when a user state changes.
endpoint	delivery estimate	Allows to get the time at which we currently expect the transfer to arrive
endpoint	receipt-pdf	Allows to get the payout receipt PDF detailing sender, recipient, and some additional information about the transfer
endpoint	banking partner payout information	Allows to get payout information based on which you can generate your own payout receipt (in-app, PDF or some other format)
endpoint	cancel transfer	Allows to cancel the transfer before it has been sent
Payment Status/Money Hasn't Reached Recipient

A common customer query is getting an update on the payment status when it’s taking longer than they expected or when the beneficiary claims they haven’t yet received the funds. Some customers will simply want to know the status or ETA, others might ask to track, trace or recall. There are several ways to help the customer in these situations.

The transfer-state-change webhook allows you to see the latest state the payment is in (you can see the different statuses here: transfer-statuses).
The transfer-active-cases webhook notifies you of any issues with the transfer – even if there is no action you can take to resolve the issue at this point (we simply might be doing additional checks on our side), it gives an indication of delays in processing the payment.
The delivery estimate endpoint will also give you the current estimate of when the payment should hit the recipient’s account.

You can also read more about tracking transfers in our Send Money API guide.

Receipt or Proof of Payout

Sometimes the customer needs to provide the recipient with proof of the payment being made, or they simply request this for their own records or peace of mind.

The receipt-pdf endpoint will allow you to download a Wise-branded PDF.
The banking partner payout information endpoint will allow you to fetch specific payout information so that you can generate your own transfer receipt.

You can also read more about custom receipts in our Custom Receipt guide.

Reason for Funds being Returned

Sometimes we cannot pay the funds out or the funds are returned to us (e.g. the recipient’s details were incorrect or their account is closed).

The transfer-state-change webhook helps you understand the current status of the payment (was it already returned to us but we haven’t yet refunded the customer or the refund has already been made).
The transfer-payout-failure webhook allows you to see the specific reason for the bounce back.
User state change

In the event we need to deactivate or reactivate a mutual customer's Wise account, we will trigger a users#state-change notification via webhook.

Wise account deactivation invalidates any access tokens, refresh tokens or registration codes you possess for the customer's Wise account.

A symptom of this change can be 401 response errors when you try to generate new access tokens for the customer via POST /oauth/token.

The users-state-change webhook provides updates about Wise account deactivation or reactivation along with the reason for deactivation if this can be shared with the end customer.