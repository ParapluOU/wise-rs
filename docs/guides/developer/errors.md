# Error HandlingCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/developer/errors

---

Error Handling
Copy

Catch and respond to invalid data, transfer limits and more.

This guide is intended for Wise Platform partners to understand how Wise API error codes function and how best to build your integration around them.

Error Categories

Wise error responses fall into two main categories: system errors and client errors. Your integration should accommodate both situations by either passing Wise error messages through to your UI or mapping codes to your own application messages.

All Wise error responses can be structured as a simple response with relevant fields, or an array containing one or more objects related to the error flow.

Error Code Structure

Currently, Wise error codes are not unique and must be used in conjunction with the provided payload body to determine the specific error.

System Errors

System errors are typically not caused by end users and often require developer intervention. These errors should be handled by mapping the error or path field to a user-friendly UI response.



Example
{
  "error": "invalid_token",
  "error_description": "945fdc16-56cb-4039-90b4-998d583740ce"
}
Client Errors

Client errors consist of issues that can be resolved by the end user or application design improvements. These include missing data input, poorly formatted inputs, or missing required field validation.



Example "End User Issue"
{
  "errors": [
    {
      "code": "error.greaterThanZero",
      "message": "Please type in a number that's larger than 0.",
      "path": "sourceAmount",
      "arguments": []
    }
  ]
}


Example "Application Error"
{
  "errors": [
    {
      "code": "ERROR",
      "message": "Unable to parse number [null]",
      "path": "targetRecipientId",
      "arguments": [
        "targetRecipientId"
      ]
    }
  ]
}
HTTP Status Codes

We use standard HTTP status codes in response headers to indicate success or failure:

Code	Description
200	OK. Successful request
201	OK. Resource created
400	Bad request. Request data failed validation
401	Unauthorized. Not authorized to access requested data
403	Forbidden. Access to requested data is forbidden
404	Not Found. Requested resource does not exist
408	Timeout. Operation timed out
422	Unprocessable entity. Request data failed validation
429	Too Many Requests
500	Server error
Rate Limiting

An HTTP 429 response indicates too many requests in a given timeframe. Default thresholds are:

100 requests per second
1000 requests per minute
Handling 429 Responses
Implement retry mechanism with exponential backoff
Check for Retry-After header indicating minimum retry time
Reduce request rate (limit concurrent requests for batch operations)
Contact Wise to request increased thresholds if needed
Passthrough Errors

Passthrough errors should be passed to users as they contain end-user-friendly messages. These are typically related to UI input or form data that users can resolve.

Please make sure to only passthrough end-user friendly messages. The message field is not always suitable for UI exposure for partners.



Example "Passthrough Error"
{
  "errors": [
    {
      "code": "MISSING",
      "message": "Please enter your first name",
      "path": "firstName",
      "arguments": ["firstName"]
    },
    {
      "code": "ERROR",
      "message": "Unparseable date: \"1234\"",
      "path": "dateOfBirth",
      "arguments": ["dateOfBirth"]
    }
  ]
}
Where to use passthrough errors

Build passthrough error handling for these flows:

Dynamic Forms
Recipient Requirements
Transfer Requirements
Quotes
Quote Creation
Account Management
Recipient Account creation

Depending on your integration type their may be other errors structured for passthrough.

Partner Mapped Errors

Partner mapped errors are responses that should not be passed directly to the UI, and should be used for error logging and mapping to your own UI error messages. This should typically be done for non-friendly error responses, and errors that occur in flows where the end user is unable to resolve error themselves.

Please make sure you build to accommodate both Key/Value and Array responses.



Example "Key/Value Response"
{
  "error": "invalid_token",
  "error_description": "945fdc16-56cb-4039-90b4-998d583740ce"
}
Where to use partner mapped errors

Mapping Wise error responses or building your system to avoid those errors should be done for the majority of errors if they are not listed specifically in the Passthrough Errors section.



Example "Array Response"
{
  "errors": [
    {
      "code": "CurrencyCode",
      "message": "That wasn't a valid ISO-4217 currency code (like GBP, USD or EUR).",
      "path": "targetCurrency",
      "arguments": []
    },
    {
      "code": "CurrencyCode",
      "message": "That wasn't a valid ISO-4217 currency code (like GBP, USD or EUR).",
      "path": "sourceCurrency",
      "arguments": []
    }
  ]
}
Error Response Fields
Field	Type	Description
timestamp	string	Timestamp when the error occurred
code	string	Non-unique error code (error category)
path	string	Unique error name for mapping
message	string	User-friendly message for UI display
arguments	array	Additional error-specific information
error	string	Unique error name for mapping
error_description	string	Detailed error description
status	string	Service-specific status
Common API Errors
Create User

Endpoint: /v1/user/signup/registration_code

API Reference: Create a user with a registration code

409 - Conflict (Email Already Exists)
Failure "System Error"

The email you are trying to use to create an account has already been used. To access this account, you would need to perform an OAuth to that account, meaning you would also need to know the password.

{
  "errors": [
    {
      "code": "NOT_UNIQUE",
      "message": "You're already a member. Please login",
      "path": "email",
      "arguments": ["email", "ma********************@wise.com"]
    }
  ]
}
400 - Missing Required Fields
Failure "Client Error"

Missing required fields. The email and registrationCode fields must be supplied in all calls.

{
  "errors": [
    {
      "code": "NotNull",
      "message": "must not be null",
      "path": "email",
      "arguments": []
    },
    {
      "code": "NotNull",
      "message": "must not be null",
      "path": "registrationCode",
      "arguments": []
    }
  ]
}
400 - Registration Code Size Error
Failure "Client Error"

The registration code is too small. Increase its size to fit the restrictions.

{
  "errors": [
    {
      "code": "Size",
      "message": "size must be between 32 and 2147483647",
      "path": "registrationCode",
      "arguments": [
        "2147483647",
        "32"
      ]
    }
  ]
}
422 - Invalid Email Format
Failure "Client Error"

Email is poorly formatted. Only send proper emails.

{
  "errors": [
    {
      "code": "ERROR",
      "message": "Please provide a valid email",
      "path": "email",
      "arguments": [
        "email",
        "ma********************@w"
      ]
    }
  ]
}
Create Personal Profile

Endpoint: /v1/profiles

API Reference: Profile

400 - Bad Profile Type
Failure "Client Error"

The type of profile was incorrect. It must be either 'personal' or 'business'.

{
  "errors": [
    {
      "code": "BAD_REQUEST",
      "message": "Could not resolve type id",
      "arguments": []
    }
  ]
}
400 - Missing Details
Failure "Client Error"

You must pass a details object.

{
  "errors": [
    {
      "code": "BAD_REQUEST",
      "message": "Missing details property",
      "arguments": []
    }
  ]
}
422 - Validation Errors
Failure "Client Error"

For type 'personal', you must include 4 fields, and they cannot be blank.



Info "Field Requirements"
firstName (cannot be blank)
lastName (cannot be blank)
dateOfBirth (YYYY-MM-DD format)
phoneNumber (with international prefix)
{
  "errors": [
    {
      "code": "MISSING",
      "message": "Please enter your first name",
      "path": "firstName",
      "arguments": ["firstName"]
    },
    {
      "code": "MISSING",
      "message": "Property [lastName] cannot be blank",
      "path": "lastName",
      "arguments": ["lastName"]
    },
    {
      "code": "ERROR",
      "message": "Unparseable date",
      "path": "dateOfBirth",
      "arguments": ["dateOfBirth"]
    },
    {
      "code": "TOO_SMALL",
      "message": "Value provided is too short. Please enter a valid phone number",
      "path": "phoneNumber",
      "arguments": ["phoneNumber", "+123", "5", "25"]
    },
    {
      "code": "UNEXPECTED",
      "message": "Please enter a valid phone number",
      "path": "phoneNumber",
      "arguments": ["phoneNumber", "+123"]
    }
  ]
}
Create Business Profile

Endpoint: /v2/profiles/business-profile

API Reference: Create Business Profile

400 - Missing Required Fields
Failure "Client Error"

Certain fields are required. Please make sure these are provided. Address must be provided, and include the first line, city, and ISO 3 Country code. For businesses that must have a registration number, it is required. For SOLE_PROPRIETORSHIP, this field is optional.

{
  "errors": [
    {
      "code": "api.businessName.missing",
      "message": "Please specify the name of your business",
      "path": "businessName",
      "arguments": null
    },
    {
      "code": "api.registrationNumber.missing",
      "message": "Please specify the registration number of your business ",
      "path": "registrationNumber",
      "arguments": null
    },
    {
      "code": "api.firstLevelCategory.missing",
      "message": "Please select a category",
      "path": "firstLevelCategory",
      "arguments": null
    },
    {
      "code": "api.address.missing",
      "message": "Please specify the address of your business",
      "path": "address",
      "arguments": null
    },
    {
      "code": "api.companyType.missing",
      "message": "Please select a company type",
      "path": "companyType",
      "arguments": null
    }
  ]
}
400 - Field Validation Errors
Failure "Client Error"

Field validation will be completed on supplied fields that are expected, including those that are optional.

{
  "errors": [
    {
      "code": "api.businessName.missing",
      "message": "Please specify the name of your business",
      "path": "businessName",
      "arguments": null
    },
    {
      "code": "api.businessName.invalid",
      "message": "Business name contains invalid characters",
      "path": "businessName",
      "arguments": null
    },
    {
      "code": "api.businessName.size.too.short",
      "message": "Business name must be at least 1 character long",
      "path": "businessName",
      "arguments": null
    },
    {
      "code": "api.companyType.invalid",
      "message": "Invalid company type selected",
      "path": "companyType",
      "arguments": null
    },
    {
      "code": "api.secondLevelCategory.invalid",
      "message": "Invalid category selected",
      "path": "secondLevelCategory",
      "arguments": null
    }
  ]
}
Create Address

Endpoint: /v1/addresses

API Reference: Addresses

403 - Unauthorized
Failure "Client Error"

If no body or a profileID that is not owned by the user is sent (including blank and null), you will be given a 403. Ensure that a profileID is provided.

{
  "timestamp": "YYYY-MM-DDThh:mm:ss.sTZD",
  "status": 403,
  "error": "unauthorized",
  "message": "Unauthorized"
}
422 - Missing Required Fields
Failure "Client Error"

Required fields, must not be blank.

{
  "errors": [
    {
      "code": "MISSING",
      "message": "Property [addressCountryCode] cannot be blank",
      "path": "addressCountryCode",
      "arguments": ["addressCountryCode"]
    },
    {
      "code": "MISSING",
      "message": "Please specify Address",
      "path": "addressFirstLine",
      "arguments": ["addressFirstLine"]
    },
    {
      "code": "MISSING",
      "message": "Property [addressCity] cannot be blank",
      "path": "addressCity",
      "arguments": ["addressCity"]
    }
  ]
}
500 - Missing Body Details
Failure "Client Error"

If you send a body without detail, it will tell you that those fields are needed.

{
  "errors": [
    {
      "code": "UNEXPECTED_ERROR",
      "message": "Validation failed for argument...",
      "arguments": []
    }
  ]
}
500 - Missing Post Code
Failure "Client Error"

Certain countries require a postCode. If you try to create an address without a postCode for a country that requires it, you will get the following error.

{
  "errors": [
    {
      "code": "UNEXPECTED_ERROR",
      "message": "Validation failed for argument...",
      "arguments": []
    }
  ]
}
Create UBOs

Endpoint: /v1/profiles/{{business-profile-id}}/ubos

API Reference: Create UBOs

400 - Bad Request
Failure "Client Error"

UBOs must include a full object. Submitting a blank UBO object will result in the 400 error.

!!! note Unlike other APIs, this is the format for missing details or those that are otherwise errored. Please review this message for the details as to why the call failed.

{
  "errors": [
    {
      "code": "BAD_REQUEST",
      "message": "ApiError...",
      "arguments": []
    }
  ]
}
Create Quote

Endpoint: /v3/profiles/{{business-profile-id}}/quotes

API Reference: Create an authenticated quote

400 - Invalid Currency Codes
Failure "Client Error"

Invalid source or target currency codes.

{
  "errors": [
    {
      "code": "CurrencyCode",
      "message": "That wasn't a valid ISO-4217 currency code (like GBP, USD or EUR).",
      "path": "targetCurrency",
      "arguments": []
    },
    {
      "code": "CurrencyCode",
      "message": "That wasn't a valid ISO-4217 currency code (like GBP, USD or EUR).",
      "path": "sourceCurrency",
      "arguments": []
    }
  ]
}
400 - Negative Amount
Failure "Client Error"

Negative amount is not supported.

{
  "errors": [
    {
      "code": "error.greaterThanZero",
      "message": "Please type in a number that's larger than 0.",
      "path": "sourceAmount",
      "arguments": []
    }
  ]
}
400 - Ambiguous Amount Specification
Failure "Client Error"

Can't determine if source or target route because source and target amount is specified. Please pick either sourceAmount or targetAmount, not both.

{
  "errors": [
    {
      "code": "error.quote.amount.onlySingle",
      "message": "Please only specify a source or target amount.",
      "path": "sourceAmount",
      "arguments": []
    }
  ]
}
Create Recipient

Endpoint: /v1/accounts

API Reference: Create a recipient account

422 - Unprocessable Entity (Missing name)
Error Type: Client
Description: First and last name is required for personal recipients, with at least 2 characters.
Example Response:
{
  "timestamp": "YYYY-MM-DDThh:mm:ss.sTZD",
  "errors": [
    {
      "code": "NOT_VALID",
      "message": "Please enter the recipients first and last name.",
      "path": "accountHolderName",
      "arguments": [
        "accountHolderName",
        "John S"
      ]
    }
  ]
}
422 - Unprocessable Entity (Bank information)
Error Type: Client
Description: Bank account information is invalid. (iban, swift, bank_code, account_number, etc)
Example Response:
{
  "timestamp": "YYYY-MM-DDThh:mm:ss.sTZD",
  "errors": [
    {
      "code": "NOT_VALID",
      "message": "Please specify an IBAN.",
      "path": "IBAN",
      "arguments": [
        "IBAN"
      ]
    }
  ]
}
Create Transfer

Endpoint: /v1/transfers
API Reference: Create transfer

422 - Unprocessable Entity (Recipient address)
Error Type: Client
Description: Recipient address is required for this transfer because the source or the target currency route requires it.
Example Response:
{
  "errors": [
    {
      "code": "INVALID_RECIPIENT",
      "message": "Recipient address is mandatory for this payment, please update or create a new recipient.",
      "arguments": []
    }
  ]
}
Fund Transfer

Endpoint: /v3/profiles/{{profileId}}/transfers/{{transferId}}/payments
API Reference: Fund a Transfer

422 - Unprocessable Entity
Error Type: Server
Description: Not enough funds in balance.
Example Response:
{
  "type": "BALANCE",
  "status": "REJECTED",
  "errorCode": "balance.payment-option-unavailable",
  "errorMessage": null
}
403 - Unauthorized
Error Type: Server
Description: SCA Required. See Strong Customer Authentication & 2FA for more details.
xample Response:
{
  "error": "unauthorized",
  "message": "Unauthorized",
  "path": null,
  "status": 403,
  "timestamp": "YYYY-MM-DDThh:mm:ss.sTZD"
}
Cancel Transfer

Endpoint: /v1/transfers/{{transferId}}/cancel

API Reference: Cancel a Transfer

409 - Conflict
Error Type: Client
Description: Attempting to cancel a completed transfer.
xample Response:
{
  "errors": [
    {
      "code": "transfer.cancellation.not.allowed",
      "message": "Transfer cannot be cancelled",
      "arguments": []
    }
  ]
}