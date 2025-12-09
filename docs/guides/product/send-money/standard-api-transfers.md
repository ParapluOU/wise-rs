# Standard Transfers: Creating Direct Payments with the Transfer API | Wise Platform

Source: https://docs.wise.com/guides/product/send-money/standard-api-transfers

---

Create a transfer
Copy

POST /v1/transfers

Request
sourceAccount (optional)integer

Refund recipient account ID

targetAccountinteger

Recipient account ID. You can create multiple transfers to same recipient account.

quoteUuidtext

V2 quote ID. You can only create one transfer per one quote. You cannot use same quote ID to create multiple transfers.

customerTransactionIduuid

This is required to perform idempotency check to avoid duplicate transfers in case of network failures or timeouts.

details.referencetext

Recipient will see this reference text in their bank statement. Maximum allowed characters depends on the currency route. Business Payments Tips article has a full list.

details.transferPurpose (conditionally required)text

For example when target currency is THB. See more about conditions at Transfers.Requirements

details.transferPurposeSubTransferPurpose (conditionally required)text

For example when target currency is CNY. See more about conditions at Transfers.Requirements

details.transferPurposeInvoiceNumber (conditionally required)text

For example when target currency is INR. See more about conditions at Transfers.Requirements

details.sourceOfFunds (conditionally required)text

For example when target currency is USD and transfer amount exceeds 80k. See more about conditions at Transfers.Requirements

There are options to deal with conditionally required fields:

Always call transfer-requirements endpoint and submit values only if indicated so.
Always provide values for these fields based on a dynamically retrieved list (transfer-requirements endpoint). It is possible these fields change over time, so take this into account if hard coding the options.

Contact api@wise.com if you have questions about this property.

Response

Returns a standard transfer object.

Avoiding duplicate transfers

We use customerTransactionId field to avoid duplicate transfer requests. If your initial call to create a transfer fails (error or timeout) then you should retry the call using the same value in the customerTransactionId field that you used in the original call. This way we can treat subsequent retry messages as repeat messages and will not create duplicate transfers to your account should one have succeeded before. You should not retry indefinitely but use a sensible limit, perhaps with a back-off approach.

Payment Approvals

Business Payment Approvals created on your wise.com settings page are not compatible with creating transfers over the API.

If you use personal tokens and do not use client credentials, and if your business account has payment approvals, your application will run in to this error when attempting to create a transfer

Quote cannot be accepted with this request due to missing approval.

Consider removing the payment rule if you are going to use the API to create transfers.

Example Request
curl -X POST \
  https://api.wise-sandbox.com/v1/transfers \
  -H 'Authorization: Bearer <your api token>' \
  -H 'Content-Type: application/json' \
  -d '{
    "sourceAccount": <refund recipient account ID>,
    "targetAccount": <recipient account ID>,
    "quoteUuid": <quote ID>,
    "customerTransactionId": "<the unique identifier you generated for the transfer attempt>",
    "details" : {
      "reference" : "to my friend",
      "transferPurpose": "verification.transfers.purpose.pay.bills",
      "transferPurposeSubTransferPurpose": "verification.sub.transfers.purpose.pay.interpretation.service",
      "sourceOfFunds": "verification.source.of.funds.other"
    }
  }'