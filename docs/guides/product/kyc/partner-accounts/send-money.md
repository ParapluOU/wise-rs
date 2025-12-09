# Send MoneyCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/kyc/partner-accounts/send-money

---

Send Money
Copy

Wise Send Money partner account API documentation is focused on integrations where the partner is the end customer and possess a profile ID of their own. First Party Partners would follow the standard Send Money flow. However, Third Party Partners would need to follow the steps highlighted below for creating transfers using the API that will include originator information.

Please refer to the flowchart below for the overview of the process.

First Party Transfers

Please follow the standard Send Money API flow.

Third Party Transfers

The standard Send Money API flow is applicable to Third Party transfers, with exception of the Create Transfer endpoint detailed below.

This endpoint is only applicable for Third Party Transfers

Create a third party transfer

POST /v2/profiles/{{profileId}}/third-party-transfers

When creating a transfer on behalf of a third party, you must take note that:

The originator datablock is required. This details the ultimate sender of funds in the transfer.
Depending on the legal entity type of the originator (PRIVATE or BUSINESS), the required fields vary. Please refer the two sample request examples on the right.
OriginalTransferId field must be used. This is your own ID for the transfer.
Request
targetAccountinteger

Recipient account ID. You can create multiple transfers to same recipient account.

quotetext

V2 quote ID. You can only create one transfer per one quote.
You cannot use same quote ID to create multiple transfers.

originalTransferIdtext

Unique transfer ID in your system. We use this field also to perform idempotency check to avoid duplicate transfers in case of network failures or timeouts. You can only submit one transfer with same originalTransferId.

details.reference (optional)text

Recipient will see this reference text in their bank statement. Maximum allowed characters depends on the currency route. Business Payments Tips article has a full list.

originatorgroup

Data block to capture payment originator details.

originator.legalEntityTypetext

PRIVATE or BUSINESS. Payment originator legal type.

originator.referencetext

Unique customer ID in your system. This allows us to uniquely identify each originator. Required.

originator.nametext

Data block to capture the originator name details.
Depends on the type of legal entity (PRIVATE or BUSINESS), the required fields and inputs are different.

originator.name.givenNametext

Payment originator first name. Required if legalEntityType = PRIVATE.

originator.name.middleNames (optional)text array

Payment originator middle name(s). Used only if legalEntityType = PRIVATE.

originator.name.familyNametext

Payment originator family name. Required if legalEntityType = PRIVATE.

originator.name.patronymicName (optional)text

Payment originator patronymic name. Used only if legalEntityType = PRIVATE.

originator.name.fullNametext

Payment originator full legal name. Required if legalEntityType = BUSINESS.

originator.dateOfBirthyyyy-mm-dd

Payment originator date of birth. Required if legalEntityType = PRIVATE.

originator.businessRegistrationCodetext

Payment originator business registry number / incorporation number. Required if legalEntityType = BUSINESS.

originator.address.firstLinetext

Payment originator address first line. Required

originator.address.citytext

Payment originator address city. Required

originator.address.stateCodetext

Payment originator address state code. Required if address country code in (US, CA, BR, AU).

originator.address.countryCodetext

Payment originator address first line. Required

originator.address.postCode (optional)text

Originator address zip code.

originator.accountDetails (optional)text

Originator account number.

Response

Returns an originator transfer object.

You need to save the transfer ID for tracking its status later via webhooks.

Avoiding duplicate transfers

We use originalTransferId field to avoid duplicate transfer requests. When your first call fails (error or timeout) then you should use the same value in originalTransferId field that you used in the original call when you are submitting a retry message. This way we can treat subsequent retry messages as repeat messages and will not create duplicate transfers to your account.

Example Request - Personal
curl -X POST \
  https://api.wise-sandbox.com/v2/profiles/{{profileId}}/third-party-transfers \
  -H 'Authorization: Bearer <your api token>' \
  -H 'Content-Type: application/json' \
  -d '{
    "targetAccount": <recipient account ID>,
    "quote": "<quote ID>",
    "originalTransferId": "<unique transfer ID in your system>",
    "details" : {
      "reference" : "Ski trip"
    },
    "originator" : {
      "legalEntityType" : "PRIVATE",
      "reference" : "<unique customer ID in your system>",
      "name" : {
        "givenName": "John",
        "middleNames": ["Ryan"],
        "familyName": "Godspeed"
      },
      "dateOfBirth": "1977-07-01",
      "address" : {
        "firstLine": "Salu tee 100, Apt 4B",
        "city": "Tallinn",
        "countryCode": "EE",
        "postCode": "12112"
      }
    }
  }'
Example Request - Business
curl -X POST \
  https://api.wise-sandbox.com/v2/profiles/{{profileId}}/third-party-transfers \
  -H 'Authorization: Bearer <your api token>' \
  -H 'Content-Type: application/json' \
  -d '{
    "targetAccount": <recipient account ID>,
    "quote": "<quote ID>",
    "originalTransferId": "<unique transfer ID in your system>",
    "details" : {
        "reference" : "Payment for invoice 22092"
    },
    "originator" : {
      "legalEntityType" : "BUSINESS",
      "reference" : "<originator customer ID in your system>",
      "name" : {
        "fullName": "Hot Air Balloon Services Ltd"
      },
      "businessRegistrationCode": "1999212",
      "address" : {
        "firstLine": "Aiandi tee 1431",
        "city": "Tallinn",
        "countryCode": "EE",
        "postCode": "12112"
      }
    }
  }'
Example Request - Canadian Personal
curl -X POST \
  https://api.wise-sandbox.com/v2/profiles/{{profileId}}/third-party-transfers \
  -H 'Authorization: Bearer <your api token>' \
  -H 'Content-Type: application/json' \
  -d '{
    "targetAccount": <recipient account ID>,
    "quote": "<quote ID>",
    "originalTransferId": "<unique transfer ID in your system>",
    "details" : {
      "reference" : "Ski trip"
    },
    "originator" : {
      "legalEntityType" : "PRIVATE",
      "reference" : "<unique customer ID in your system>",
      "name" : {
        "givenName": "John",
        "middleNames": ["Ryan"],
        "familyName": "Godspeed"
      },
      "dateOfBirth": "1977-07-01",
      "address" : {
        "firstLine": "102-3393 Capilano Road",
        "city": "North Vancouver",
        "countryCode": "CA",
        "postCode": "V7R 4W7"
      },
      "accountDetails": "<the unique account number of the customer>"
    }
  }'
Fund

Please refer to Send Money - Funding to fund the transaction.