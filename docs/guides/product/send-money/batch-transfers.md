# Batch TransfersCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/send-money/batch-transfers

---

Batch Transfers
Copy

Many of our partner’s customers need to make multiple payments at once. For instance, they may want to pay all their monthly bills or they may already be used to doing monthly or quarterly payment runs.

Batch Groups enable partners to group up to 1000 transfers under a single reference, this means that all the payments within the batch group can be funded at once.

Follow the guide below to understand more about how batch groups work, how to create a group, add transfers, and funding the batch once closed.



Postman Collection

We've created a Postman collection to help you test your integrations easily! It includes:

Our sandbox environment (https://api.wise-sandbox.com)
Common API calls for creating users and profiles
Automated tests that copy response data between calls
Example responses for reference

Fork the collection to get updates automatically. This lets you test the complete flow with minimal setup!

Batch Transfers Postman Collection

We've created a Postman collection to help you test your integrations easily! It includes:

Our sandbox environment (https://api.wise-sandbox.com)
Common API calls for creating users and profiles
Automated tests that copy response data between calls
Example responses for reference

Fork the collection to get updates automatically. This lets you test the complete flow with minimal setup!

Fork in Postman
The Batch Group resource

Many Batch Group API endpoints return a batch group resource. Batch group resources have the following properties:

Fields
idtext

Batch group ID

versioninteger

Batch version, used for concurrency control. This number is updated whenever there is a change to the batch group state (its status, the identity of the transfers in the batch, etc).

Some API operations will require this version in requests, and operations may be rejected when the requested version does not match the server’s version.

The version will be a signed integer and is not ordered with respect to any previous version.

nametext

Descriptive name

sourceCurrencytext

Source currency code (ISO 4217 Alphabetic Code) - This currency is expected to be used for the funding the batch group

statustext

Current Batch Group Status (see below)

transferIdsarray

The IDs of all transfers successfully added to the group

payInDetailsarray

List of pay in details (see Pay In Details below). Provided only when the batch-group is in the COMPLETED state

statustext
NEW: New batch group with zero or more transfers. Able to have more transfers added to it. Any transfers in a NEW group cannot be yet be funded and paid out.
COMPLETED: The batch group has had all the desired transfers added to it and is now closed to further changes. The transfers in the group are now able to be funded and paid out.

Note: COMPLETED does not imply that payouts have been successfully completed. It means that all required transfers have been created and associated with the batch group.
MARKED_FOR_CANCELLATION: Cancellation of the transfers in the batch group was requested.
PROCESSING_CANCEL: Transfers in the group are being cancelled. This takes time in Wise's system.
CANCELLED: Transfers in the group have been cancelled.
Pay In Details

Pay In Details describe how the batch group can be funded. They are only populated when a batch group is in the COMPLETED state.

The following fields are always populated:

Always Populated
typetext

Method of payment. Currently supported types: bank_transfer.

referencetext

The reference that should be used when funding the transfers in the batch group. This reference should be treated as an opaque value and there should be no attempt to decode or decompose it.

amountdecimal

The total pay in amount for all transactions in the batch when paying-in with this reference and method.

currencytext

Three-character ISO 4217 currency code.

These fields are populated when type is bank_transfer:

Optionally Populated
nametext

Name of the bank account holder.

branchNametext

Name of the bank branch, provided only when the currency route requires it (such as JPY).

accountNumbertext

Bank account number.

accountTypetext

The Bank Account Type, provided only when the currency route requires it.

bankCodetext

Bank identifier or routing number, depending on pay in type and currency.

bankAddressobject

The Pay In Details Address for the receiving bank, provided only when the currency route requires it.

transferWiseAddressobject

Wise's Pay In Details Address, provided only when the currency route requires it.

ibantext

ISO 13616 International Bank Account Number (when available).

bbantext

Basic Bank Account Number (BBAN). Provided only when the currency route requires it (such as NOK).

institutionNumbertext

Financial Institution number (3 digits). Provided only when the currency route requires it (such as CAD).

transitNumbertext

Branch Transit Number (5 digits). Provided only when the currency route requires it (such as CAD).

beneficiaryBankBICtext

Beneficiary Bank Business Identifier Code (BIC). Provided only when the currency route requires it (such as CAD).

intermediaryBankBICtext

Intermediary Bank Business Identifier Code (BIC). Provided only when the currency route requires it (such as CAD).

fpsIdentifiertext

Faster Payment System identifier. Provided only when the currency route requires it (such as HKD).

clearingNumbertext

Clearing number. Provided only when the currency route requires it (such as SEK).

Pay In Details Address

Bank transfer pay in details may contain bank and Wise's addresses for some source currencies.

These currencies include: USD, NOK.

Title
nametext

Bank name / Wise's company name

firstLinetext

Street address

postCodetext

Postcode / ZIP code

citytext

City

stateCodetext (format depends on country)

State, province or region code

country2 character iso 3316 country code

Country code

Batch Group Object
{
  "id": "54a6bc09-cef9-49a8-9041-f1f0c654cd89",
  "version": 123,
  "name": "My batch group",
  "sourceCurrency": "NOK",
  "status": "COMPLETED",
  "transferIds": [
    234,
    456
  ],
  "payInDetails": [
    {
      "type": "bank_transfer",
      "reference": "B5323",
      "amount": 12504.54,
      "currency": "NOK",
      "name": "TransferWise Ltd",
      "bankCode": "8301",
      "bankAddress": {
        "name": "CitiBank Europe Plc",
        "firstLine": "Bolette brygge 1",
        "postCode": "0252",
        "city": "Oslo",
        "country": "Norway",
        "stateCode": null
      },
      "transferWiseAddress": {
        "name": "TransferWise Ltd",
        "firstLine": "6th Floor, The Tea Building, 56 Shoreditch High Street",
        "postCode": "E1 6JJ",
        "city": "London",
        "country": "United Kingdom",
        "stateCode": null
      },
      "accountNumber": "9910728",
      "iban": null,
      "bban": "83019910728"
    }
  ]
}
Create a batch group

POST /v3/profiles/{{profileId}}/batch-groups

Create a new batch group.

Request
sourceCurrencytext

Source currency code (ISO 4217 Alphabetic Code) - This currency is expected to be used for the funding the batch group

nametext

Descriptive name for display purposes, recommended to use a name that uniquely represents this batch. Maximum length of 100 characters.

Response

Returns a batch group object without payInDetails.

Example Request
curl -X POST \
  https://api.wise-sandbox.com/v3/profiles/{{profileId}}/batch-groups \
  -H 'Authorization: Bearer <your api token>' \
  -H 'Content-Type: application/json' \
  -d '{
    "sourceCurrency": "GBP",
    "name": "my-batch-group"
  }'
Example Response
{
  "id": "54a6bc09-cef9-49a8-9041-f1f0c654cd88",
  "version": 0,
  "name": "my-batch-group",
  "sourceCurrency": "GBP",
  "status": "NEW",
  "transferIds": [],
  "payInDetails": []
}
Complete a batch group

PATCH /v3/profiles/{{profileId}}/batch-groups/{{batchGroupId}}

Completes the batch group and allows funding to proceed. Note: this action prevents any further modification.

Request
profileIdnumber

The profile that the batch group is associated with

batchGroupIdtext

Batch group ID

statustext

Status to be updated to. Use COMPLETED as the value.

versioninteger

The expected batch group version. This is a concurrency control mechanism. For the change to be accepted by the server, the expected version of the group must match the current version held by the server.

Versions are discovered by requesting batch group resources.

Response

Returns a batch group object with payInDetails.

Example Request
curl -X PATCH \
  https://api.wise-sandbox.com/v3/profiles/{{profileId}}/batch-groups/{{batchGroupId}} \
  -H 'Authorization: Bearer <your api token>' \
  -H 'Content-Type: application/json' \
  -d '{
    "status": "COMPLETED",
    "version": 1234
  }'
Cancel a batch group

PATCH /v3/profiles/{{profileId}}/batch-groups/{{batchGroupId}}

Cancel a batch group. Cancelling closes the group to further modification and initiates the cancellation of all transfers in the batch group. Only batches that are not funded can be cancelled. Cancellation is final it cannot be undone.

Request
profileIdnumber

The profile that the batch group is associated with

batchGroupIdtext

Batch group ID

statustext

Status to be updated to. Use CANCELLED as the value.

versioninteger

The expected batch group version. This is a concurrency control mechanism. For the change to be accepted by the server, the expected version of the group must match the current version held by the server.

Versions are discovered by requesting batch group resources.

Response

Returns a batch group object without payInDetails.

Example Request
curl -X PATCH \
  https://api.wise-sandbox.com/v3/profiles/{{profileId}}/batch-groups/{{batchGroupId}} \
  -H 'Authorization: Bearer <your api token>' \
  -H 'Content-Type: application/json' \
  -d '{
    "status": "CANCELLED",
    "version": 1234
  }'
Example Request
{
    "id": "54a6bc09-cef9-49a8-9041-f1f0c654cd88",
    "version": 12345,
    "name": "my-batch-group",
    "sourceCurrency": "GBP",
    "status": "MARKED_FOR_CANCELLATION",
    "transferIds": [
      100001,
      100002,
      100003
    ],
    "payInReferences": []
}
Retrieving a batch group by ID

GET /v3/profiles/{{profileId}}/batch-groups/{{batchGroupId}}

Get an existing batch group.

Request
profileIdnumber

The profile that the batch group is associated with

batchGroupIdtext

Batch group ID

Response

Returns a batch group object.

Example Request
curl -X GET \
  https://api.wise-sandbox.com/v3/profiles/{{profileId}}/batch-groups/{{batchGroupId}} \
  -H 'Authorization: Bearer <your api token>'
Create a batch group transfer

POST /v3/profiles/{{profileId}}/batch-groups/{{batchGroupId}}/transfers

Create a transfer in the batch group, using a previously created recipient account and quote. Please see quote creation and recipient account creation documentation.

For the request body format please see transfer creation.

Response

Returns a transfer object.

Example Request
curl -X \
  POST https://api.wise-sandbox.com/v3/profiles/{{profileId}}/batch-groups/{{batchGroupId}}/transfers \
  -H 'Authorization: Bearer <your api token>' \
  -H 'Content-Type: application/json' \
  -d '{
    "sourceAccount": {{refund recipient account ID}},
    "targetAccount": {{recipient account ID}},
    "quoteUuid": {{quote ID}},
    "customerTransactionId": "{{the unique identifier you generated for the transfer attempt}}",
    "details" : {
      "reference" : "to my friend",
      "transferPurpose": "verification.transfers.purpose.pay.bills",
      "transferPurposeSubTransferPurpose": "verification.sub.transfers.purpose.pay.interpretation.service",
      "sourceOfFunds": "verification.source.of.funds.other"
    }
  }'
Fund a batch group

POST /v3/profiles/{{profileId}}/batch-payments/{{batchGroupId}}/payments

Funds all the transfers in a batch, they are paid out immediately. Applicable when funding from a multi-currency account. The Batch Group must first be completed, and there needs to be enough funds in the account for the whole batch otherwise an insufficient funds error will be returned.

Request
typetext

The method of payment to use (must always be BALANCE)

Response

You need to save transfer ID for tracking its status later.

idtext

Unique batch group ID.

nametext

Descriptive name.

fileNametext

If this batch was submitted as a file, this is the given file name

alreadyPaidboolean

This field is not applicable to this use case

shortIdnumber

This field is not applicable to this use case

userIdnumber

The ID of the user who initiated this payment

profileIdnumber

The ID of the profile this payment belongs to

sourceCurrencytext

The source currency of the batch (note: we will prefer this currency but if there are insufficient funds then an automatic conversion from another currency can occur)

statustext

Current Batch Group Status

groupTypetext

Whether this batch was submitted over the "API" or as a "FILE"

transferIdsarray

The IDs of all transfers in the group.

This endpoint is SCA protected when it applies. If your profile is registered within the UK and/or EEA, SCA most likely applies to you. For more information please read more implementing SCA.

Example Request
curl -X \
  POST https://api.wise-sandbox.com/v3/profiles/{{profileId}}/batch-payments/{{batchGroupId}}/payments \
  -H 'Authorization: Bearer <your api token>' \
  -H 'Content-Type: application/json' \
  -d '{
    "type": "BALANCE"
  }'
Example Response
{
    "id": "54a6bc09-cef9-49a8-9041-f1f0c654cd88",
    "name": "my-batch-group",
    "fileName": null,
    "alreadyPaid": true,
    "shortId": 12345,
    "userId": 33333333,
    "profileId": 44444444,
    "sourceCurrency": "GBP",
    "status": "COMPLETED",
    "groupType": "API",
    "transferIds": [
      100001,
      100002,
      100003
    ]
}
Fund a batch group via direct debit

POST /v1/profiles/{{profileId}}/batch-groups/{{batchGroupId}}/payment-initiations

Funds all the transfers in a batch via direct debit. The Batch Group must be in the completed state. To use this funding method, you need to link an external bank account first. More info on how to create a direct debit account can be found here.

Request
typetext

The method of payment to use (must always be DIRECT_DEBIT)

accountIdnumber

Direct debit account ID direct debit creation

referencetext

Optional. Payment initiation reference. The current limit is 10 characters. If not provided, the reference will be generated automatically and returned in the response

Response

You need to save payment initiation ID for tracking its status later.

idnumber

Payment initiation ID

batchGroupIdtext

Batch group ID

referencetext

This reference will be passed to the network and can be used for reconciliation

userIdnumber

The ID of the user who initiated this payment

profileIdnumber

The ID of the profile this payment belongs to

typetext

Type (DIRECT_DEBIT)

statustext

Status (NEW, PROCESSING, COMPLETED, FAILED, CHARGED_BACK)

accountIdnumber

External account ID associated with the payment

createdTimetimestamp

Date of payment initiated creation

This endpoint is SCA protected when it applies. If your profile is registered within the UK and/or EEA, SCA most likely applies to you. For more information please read more implementing SCA.

Example Request
curl -X POST \
  https://api.wise-sandbox.com/v1/profiles/{{profileId}}/batch-groups/{{batchGroupId}}/payment-initiations \
  -H 'Authorization: Bearer <your api token>' \
  -H 'Content-Type: application/json' \
  -d '{
    "type": "DIRECT_DEBIT",
    "accountId": 1
  }'
Example Response
{
  "id": 12345,
  "batchGroupId": "068e186d-9632-4937-b753-af3e53f4d0b0",
  "reference": "B1234567",
  "userId": 33333333,
  "profileId": 44444444,
  "type": "DIRECT_DEBIT",
  "status": "NEW",
  "accountId": 1,
  "createdTime": "2022-01-01T19:51:41.423404Z"
}
Retrieve a batch group initiated payment

GET /v1/profiles/{{profileId}}/batch-groups/{{batchGroupId}}/payment-initiations/{{paymentInitiationId}}

Get payment initiation info by ID. In addition to webhooks, this endpoint can be used for polling the status of payment initiation.

Request
profileIdnumber

The profile that the batch group is associated with

batchGroupIdtext

Batch group ID

paymentInitiationIdnumber

The payment initiation ID

Response

You need to save payment initiation ID for tracking its status later.

idnumber

Payment initiation ID

batchGroupIdtext

Batch group ID

referencetext

This reference will be passed to the network and can be used for reconciliation

userIdnumber

The ID of the user who initiated this payment

profileIdnumber

The ID of the profile this payment belongs to

typetext

Type (DIRECT_DEBIT)

statustext

Status (NEW, PROCESSING, COMPLETED, FAILED, CHARGED_BACK)

accountIdnumber

External account ID associated with the payment

transferIdnumber

Transfer ID of a direct debit payment. Present only after a direct debit is initiated

createdTimetimestamp

Date of payment initiated creation

Example Request
curl -X GET \
  https://api.wise-sandbox.com/v1/profiles/{{profileId}}/batch-groups/{{batchGroupId}}/payment-initiations/{{paymentInitiationId}} \
  -H 'Authorization: Bearer <your api token>' \
  -H 'Content-Type: application/json'
Example Response
{
  "id": 12345,
  "batchGroupId": "068e186d-9632-4937-b753-af3e53f4d0b0",
  "reference": "B1234567",
  "userId": 33333333,
  "profileId": 44444444,
  "type": "DIRECT_DEBIT",
  "status": "PROCESSING",
  "accountId": 1,
  "transferId": 100004,
  "createdTime": "2022-01-01T19:51:41.423404Z"
}