# Transfer FundingCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/send-money/funding/bulk-settlement/transfer-funding

---

Transfer Funding
Copy

Once a transfer has been created, it needs to be funded. For Bulk Settlement, we can mark a transfer as funded using API.

For a full reference, please see the fund a transfer API reference.

Use TRUSTED_PRE_FUND_BULK as the type when calling the API.

If you are using the Cross Currency Bulk Settlement model, you need to include the `partnerReference` field with the same partner reference that you would put in the settlement journal.

Upon funding completion, the transfer will move from the incoming_payment_waiting status to the processing status.

The funding API acts as a guarantee that you will send the funds to Wise at the end of the agreed settlement period and this is legally binding. Only call the API when you are certain you will send the funds to us.
Example Request Body
{
  "type": "TRUSTED_PRE_FUND_BULK",
  "partnerReference": "ref_12345"
}
Delayed Funding

In the case you want to lock an FX rate and fund a transfer later, you can use delayed funding. Delayed funding works by setting status parameter in the request body in the two following steps:

Step	Status	Description
1	INITIATE	The FX rate is locked for the transfer
2	COMPLETE	The transfer is completed and paid out
Delayed funding is only available to partners who have an active agreement for this feature with Wise.
Example Request Body
{
  "type":"TRUSTED_PRE_FUND_BULK",
  "status": "INITIATE"
}
Error Handling

When calling the funding API for Bulk settlement, you may encounter the following error responses.

Description	Reason	HTTP Code	Error Code
Unauthorized access	You can only fund a transfer with a user token that owns the transfer being funded.	403	transfer.not-accessible-for-user
Transfer does not exist	You can only attempt to fund a transfer that exists	404	transfer.not-found
Payment does not exist	You called status = COMPLETE before status = INITIATE when making delayed funding	404	payment.not-found
Invalid transfer state	If this transfer is already funded (by calling this endpoint or by another payment) then the attempt to mark the transfer as funded will fail as you can’t fund a transfer twice. Additionally if the transfer is canceled or already being processed it cannot be funded.	422	transfer.invalid-state
Using a non-permitted type parameter	Bulk settlement is not enable.	422	trustedprefundbulk.payment-option-unavailable

Of these errors, the only one you are likely to see after going live should be transfer.invalid-state. The others should never happen if your integration works and passes the testing phase.

This error will normally only occur if you attempt to retry calling the funding API for a transfer that you already called it for, as it is unlikely the customer will fund the transfer directly, although this is possible. In the case of a failed retry, you can generally assume that the first call would have been successful and the transfer will be processed.

Regardless of whether this API returns CREATED or REJECTED you should still send the funds to Wise .

In the unlikely event a transfer has had no successful payment attempts, the transfer will still be processed on receipt of the funds and/or settlement file containing the transfer. The difference is the customer will have to wait until this time rather than having an instant payment experience.

Example Response
{
  "type": "TRUSTED_PRE_FUND_BULK",
  "status": "REJECTED",
  "errorCode": "transfer.invalid-state",
  "errorMessage": null
}
Settlement Limits

In order to limit exposure and risk, Wise might prevent payments from being paid out instantly in case the total amount owed to Wise for the current settlement period is above the collateral we are currently holding for your institution. In this case you will still receive a successful response with an HTTP status code 200.

The payment will still be created but the payout to the recipient will be delayed until we confirm that the funds arrive to Wise upon the settlement funds arriving after the end of the settlement period.

You should not cause the user process to fail or error at this point, and should consider the API call successful (hence the 200). However, you may wish to trigger some logic based on this response to run your settlement process earlier or alert a member of staff to do so, in order to prevent delays in processing of transfers.

Please discuss and agree on a suitable process with our implementation team on how to proceed once the limits are reached.

Example Response
{
  "type": "TRUSTED_PRE_FUND_BULK",
  "status": "CREATED",
  "errorCode": "trustedprefundbulk.limit-reached",
  "errorMessage": null
}