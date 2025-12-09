# Transfer From Balance AccountCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/accounts/transfer-from-balance-account

---

Transfer From Balance Account
Copy

Transferring from a balance follows the exact same flow as a standard transfer, with the exception being that it is funded from balance.

The steps below illustrate the flow required to create a quote, recipient, and transfer. Once the transfer is created, it can then be funded from the balance account

Generate transfer

Follow the guides below to create a quote, recipient, and transfer as per the usual flow.

Quote API
Recipient API
Transfer API
Fund a transfer

POST /v3/profiles/{{profileId}}/transfers/{{transferId}}/payments

This endpoint is SCA protected when it applies. If your profile is registered within the UK and/or EEA, SCA most likely applies to you. Please read more about implementing SCA. If you use mTLS and client credentials to make API requests, SCA does not apply.

This API call is the final step for executing payouts when using a balance with Wise. Upon calling the endpoint, Wise will begin the processing of the transfer, depending on the status of funds.

When using the transfer by transfer settlement model, the following funding type(s) must be used:

BALANCE - Funds are pulled from a multi-currency account held with Wise.
BANK_TRANSFER - Manually send funds from your business bank account to pay for any transfers. Only applicable when using the Batch Group API.

When funding through the Bulk Settlement model, the following funding type(s) must be used:

TRUSTED_PRE_FUND_BULK - Funds for the transfer will be settled through a bulk payment at a later date. This method is not applicable to first party Payouts.

If funding from BALANCE, and your multi-currency account does not have the required funds to complete the action, then this call will fail with an "insufficient funds" error. Once funds are added and available, you must call this endpoint again.

{{profileId}} refers to the profile that created the transfer. It can be either your personal profile ID, or your business profile ID.

Request
typetext

"BALANCE"
This indicates the type of funding you would like to apply to the transfer.

partnerReference (conditionally required)string

The transaction/payment identifier in your system, uniquely identifies the transfer in your platform. This is required for the Cross Currency Bulk Settlement model.

Response
typetext

"BALANCE"
This indicates the type of funding you would like to apply to the transfer.

statustext

"COMPLETED" or "REJECTED"

errorCodetext

Failure reason. For example "balance.payment-option-unavailable".

Example Request
curl -X POST \
  https://api.wise-sandbox.com/v3/profiles/{{profileId}}/transfers/{{transferId}}/payments \
  -H 'Authorization: Bearer <your api token>' \
  -H 'Content-Type: application/json' \
  -d '{
    "type": "BALANCE"
  }'
Example Request - Balance - Completed
{
  "type": "BALANCE",
  "status": "COMPLETED",
  "errorCode": null
}
Example Request - Balance - Insufficient Funds
{
  "type": "BALANCE",
  "status": "REJECTED",
  "errorCode": "transfer.insufficient_funds"
}