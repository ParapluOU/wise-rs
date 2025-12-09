# Fund a Transfer from Your Wise BalanceCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/send-money/funding/fund-by-transfer

---

Fund a Transfer from Your Wise Balance
Copy

Learn how to fund transfers using your Wise multi-currency account balance.

Overview

The fund transfer endpoint is the final step in executing a payout when using a Wise balance. Once called, Wise immediately begins processing the transfer if sufficient funds are available.

Endpoint: POST /v3/profiles/{profileId}/transfers/{transferId}/payments

Before you begin
Create your transfer: You must have already created a transfer.
Check your balance: Ensure you have sufficient funds in the appropriate currency balance
Fund a transfer

To fund a transfer from your balance, make a POST request with type set to BALANCE:

Request body
curl -X POST \
  https://api.wise-sandbox.com/v3/profiles/{profileId}/transfers/{transferId}/payments \
  -H 'Authorization: Bearer YOUR_API_TOKEN' \
  -H 'Content-Type: application/json' \
  -d '{
    "type": "BALANCE"
  }'
Parameters
Parameter	Type	Required	Description
profileId	integer	Yes	The ID of the profile that created the transfer (personal or business)
transferId	integer	Yes	The ID of the transfer to fund
type	string	Yes	Must be "BALANCE" to fund from your multi-currency account
Successful funding

When your balance has sufficient funds, you'll receive a COMPLETED status:

Response
{
  "type": "BALANCE",
  "status": "COMPLETED",
  "errorCode": null
}
Parameters
Field	Type	Description
type	string	The funding type used (BALANCE)
status	string	COMPLETED when successful, REJECTED when failed
errorCode	string	Error code if funding failed, null on success
Insufficient funds

If your balance doesn't have enough funds, the request will fail:

Response
{
  "type": "BALANCE",
  "status": "REJECTED",
  "errorCode": "transfer.insufficient_funds"
}

When this occurs:

Add funds to your balance
Retry this endpoint once funds are available
The transfer will remain in incoming_payment_waiting status until funded
What happens next

After successful funding:

The transfer status changes from incoming_payment_waiting to processing
Wise begins processing the payout to the recipient
You'll receive webhook notifications as the transfer progresses
You can track the transfer status using the Get Transfer API
Testing in sandbox

In the sandbox environment, you can:

Create test balances
Add test funds to practice the funding flow
Test insufficient funds scenarios

Learn more about testing