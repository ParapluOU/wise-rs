# Tracking TransfersCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/send-money/tracking-transfers

---

Tracking Transfers
Copy

You should use a subscription to the transfer state change webhook to keep the statuses of your local transfer data storage up to date. The transfers will move through the following statuses in the happy path.

Incoming Payment Waiting ⇒ Processing ⇒ Funds Converted ⇒ Outgoing Payment Sent

Outgoing Payment Sent is the final state of the normal flow. If the payment fails, the following problematic flow will occur. An example would be if the recipient bank account does not exist or the data is entered incorrectly and the payment is returned to Wise. The problematic state flow of transfers is:

Outgoing Payment Sent ⇒ Bounced Back ⇒ Processing ⇒ Cancelled ⇒ Funds Refunded

Most bounce backs occur within 2-3 business days, however this could happen up to several weeks after a transfer is sent.

Transfer Statuses

NB: Transfers support rollback transitions, it allows return transfer back to one of previous states.

See below for the full list of transfer statuses and what they mean in the order of occurrence:

incoming_payment_waiting – The transfer has been submitted, and it’s waiting for the funds to arrive with Wise.

incoming_payment_initiated - The funding has been initiated but the money has not yet arrived to Wise's account.

processing – We have received the funds and are processing the transfer. Processing is a generic term and means we’re doing behind-the-scene activities before the money gets sent to the recipient, such as AML, compliance and fraud checks.

funds_converted – All compliance checks have been completed for the transfer and funds have been converted from the source currency to the target currency.

outgoing_payment_sent – Wise has paid out funds to the recipient. This is the final state of the transfer, assuming funds will not be returned. When a transfer has this state, it does not mean the money has arrived in the recipient’s bank account, just that we have sent it from ours. Note: Payment systems in different countries operate in different speeds and frequency. For example, in the UK, the payment will reach the recipient bank account within few minutes after we send the outgoing payment. However, in the US it usually takes a day until funds are received.

cancelled – This status is used when the transfer was never funded and therefore never processed. This is a final state of the transfer.

funds_refunded – Transfer has been refunded. This is a final state of the transfer.

bounced_back – Transfer has bounced back but has not been cancelled nor refunded yet. This is not a final transfer state, it means the transfer will either be delivered with delay or it will turn to funds_refunded state.

charged_back - This status is used when we have problem to debit payer's account or payer requested money back. Chargeback can happen from any other state.

unknown - This status is used when we don’t have enough information to move the transfer into a final state. We send out an email for more information. e.g. Sender account details to refund money back.

Keep in mind the transfer statuses in our API have different names than what you’ll see on our website or app. That’s because we use more consumer friendly language in the front end of our products. For example "Completed" on our website means outgoing_payment_sent in the API.

You should use the following descriptions in your website or app for the potential statuses we return:

Status	Description
incoming_payment_waiting	"On its way to Wise"
incoming_payment_initiated	"On its way to Wise"
processing	"Processing"
funds_converted	"Processing"
outgoing_payment_sent	"Sent"
charged_back	"Charged back"
cancelled	"Cancelled"
funds_refunded	"Refunded"
bounced_back	"Bounced back"
unknown	"Unknown"

Transfer Status Flow Map

Get the delivery estimate for a transfer

GET /v1/delivery-estimates/{{transferId}}?timezone=Asia/Singapore

Get the live delivery estimate for a transfer by the transfer ID.

The delivery estimate is the time at which we currently expect the transfer to arrive in the beneficiary's bank account.

This is not a guaranteed time but we are working hard to make these estimates as accurate as possible.

Request
timezonetext

Timezone ID for the formatted text. Example: UTC, Asia/Singapore. Defaults to UTC if not provided.

Response
estimatedDeliveryDatetimestamp

Estimated time when funds will arrive to recipient's bank account

formattedEstimatedDeliveryDatetext

A string to display to users for the estimated delivery date.

Example Request
curl -X GET \
  https://api.wise-sandbox.com/v1/delivery-estimates/{{transferId}}?timezone=Asia/Singapore \
  -H 'Authorization: Bearer <your api token>' 
Example Response
{
  "estimatedDeliveryDate" : "2018-01-10T12:15:00.000+0000",
  "formattedEstimatedDeliveryDate" : "in seconds"
}
Testing Transfer Statuses
Testing in Sandbox

Because Sandbox is a test environment, there are some differences between Sandbox and Production. Please keep these in mind as you test.

Since Sandbox does not have the full processing capabilities of a live environment, we provide transfer state change simulation endpoints to allow moving transfers to various states, including triggering the Transfer State Change webhook event.

Available Statuses:

processing
funds_converted
outgoing_payment_sent
bounced_back
funds_refunded

Note: These statuses have to be called in order.