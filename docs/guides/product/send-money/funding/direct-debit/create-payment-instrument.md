# Direct Debit: Create a Payment InstrumentCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/send-money/funding/direct-debit/create-payment-instrument

---

Direct Debit: Create a Payment Instrument
Copy

A Payment Instrument is a saved funding source, like a customer's bank account, which can be used to request direct debit Payins to fund transfers.

In order to set up a Payment Instrument (to allow submissions for Payins), you need to:

Create the Payment Instrument using POST /v3/profiles/{{profileId}}/payment-instruments
Connect the Payment Instrument using POST /v3/profiles/{{profileId}}/payment-instruments/{{paymentInstrumentId}}/manual-confirmation.
Create a new Payment Instrument

To create a new Payment Instrument, send a request to POST /v3/profiles/{{profileId}/payment-instruments. The API returns a PENDING status, indicating it is not yet ready for use.

In your request, you must provide:

Payment Instrument type
BACS for the Direct Debit scheme for GBP accounts
SEPA for the Direct Debit scheme for EUR accounts
Bank Account Details
For Bacs the accountNumber should be the customers bank account number and the branchCode should be their sort code
For SEPA only the accountNumber field is required as it represents the customers IBAN number
Email
For Bacs and SEPA the email used for communication with the Payment Instrument owner must be provided
This is the email that is used for sending mandate creation notification and advanced payment notices

In your request, you may optionally provide:

Account Holder Name
If not provided, the associated business profile's company name is used when creating the Payment Instrument
Reference
This is the reference that is displayed in the debtor's bank statement (alongside the reference provided during Payin creation)
If not provided, this is set to a random alphanumeric text field
BACS Example

Sample request:

{
    "type": "BACS",
    "accountHolderName": "John Smith",
    "accountNumber": "12345678",
    "branchCode": "00-00-00",
    "reference": "REFERENCE01",
    "accountOwnerDetails" : {
        "email": "test@email.com"
    }

}

Sample response:

{
    "id": "6713050b-e64f-4da7-0770-5b0583f81739",
    "type": "BACS",
    "bankName": "BARCLAYS BANK PLC",
    "bankCode": null,
    "branchCode": "00-00-00",
    "accountNumber": "XXXX5678",
    "status": "PENDING",
    "accountOwnerDetails": {
        "companyName": "ABC Logistics Ltd",
        "country": "GB",
        "state": null,
        "city": "London",
        "postCode": "EC2A 4JE",
        "firstLine": "65 Clifton St"
    },
    "reference": "REFERENCE01"
}
SEPA Example

Sample request:

{
    "type": "SEPA",
    "accountHolderName": "John Smith",
    "accountNumber": "EE382200221020145685",
    "reference": "REFERENCE01",
    "accountOwnerDetails" : {
        "email": "test@email.com"
    }
}

Sample response:

{
    "id": "c1dca180-2939-461b-913c-d9638e6a93ee",
    "type": "SEPA",
    "bankName": "SWEDBANK AS",
    "bankCode": null,
    "branchCode": null,
    "accountNumber": "XXXXXXXXXXXXXXXX5685",
    "status": "PENDING",
    "accountOwnerDetails": {
        "companyName": "ABC Logistics Ltd",
        "country": "EE",
        "state": null,
        "city": "Tallinn",
        "postCode": "10135",
        "firstLine": "Veerenni tn 24"
    },
    "reference": "REFERENCE01"
}
Connect the Payment Instrument

After you complete the mandate setup with your customer, send a request to POST /v3/profiles/{{profileId}}/payment-instruments/{{paymentInstrumentId}}/manual-confirmation.

This action confirms to Wise that you have followed all scheme rules. The Payment Instrument's status changes to CONNECTED, indicating you can now use it to initiate Payins.

Sample response:

{
    "id": "6713050b-e64f-4da7-0770-5b0583f81739",
    "type": "BACS",
    "bankName": "BARCLAYS BANK PLC",
    "bankCode": null,
    "branchCode": "00-00-00",
    "accountNumber": "XXXX5678",
    "status": "CONNECTED",
    "accountOwnerDetails": {
        "companyName": "ABC Logistics Ltd",
        "country": "GB",
        "state": null,
        "city": "London",
        "postCode": "EC2A 4JE",
        "firstLine": "65 Clifton St"
    },
    "reference": "REFERENCE01"
}
Manage the Payment Instrument
Track the Payment Instrument

You can retrieve the status of a Payment Instrument at any time by sending a request to GET /v3/profiles/{{profileId}}/payment-instruments/{{paymentInstrumentId}}.

We also expose updates to the Payment Instrument via webhooks, see the Webhooks section for more details.

Payment Instrument Status Breakdown
PENDING - The Payment Instrument requires user confirmation before becoming CONNECTED.
CONNECTED - The Payment Instrument is authorised and ready to be used for Payin creation.
DISCONNECTED - The Payment Instrument is no longer authorised (due to failure or manual disconnection) and cannot be used for Payin creation.
Disconnect the Payment Instrument

You can disconnect a Payment Instrument at any time by sending a request to POST /v3/profiles/{{profileId}}/payment-instruments/{{paymentInstrumentId}}/disconnection.

Sample response:

{
    "id": "6713050b-e64f-4da7-0770-5b0583f81739",
    "type": "BACS",
    "bankName": "BARCLAYS BANK PLC",
    "bankCode": null,
    "branchCode": "00-00-00",
    "accountNumber": "XXXX5678",
    "status": "DISCONNECTED",
    "accountOwnerDetails": {
        "companyName": "ABC Logistics Ltd",
        "country": "GB",
        "state": null,
        "city": "London",
        "postCode": "EC2A 4JE",
        "firstLine": "65 Clifton St"
    },
    "reference": "REFERENCE01",
    "disconnectReason": "MANUALLY_DISCONNECTED"
}

On disconnecting via API, we would return back the disconnect reason as MANUALLY_DISCONNECTED. See the section below for more details.

Handling a Disconnected Payment Instrument

A Payment Instrument may become disconnected for various reasons (e.g. customer bank account is not active, customer cancels their authorisation via their bank, Payment Instrument cancelled via the API request above). These cases all lead to the Payment Instrument being marked as DISCONNECTED - it can no longer be used for initiating Payins.

We provide a disconnectReason field to allow you to handle these scenarios gracefully, which may be one of the following values:

AUTHORISATION_REMOVED - The account has been disconnected by the customer or their bank.
ACCOUNT_NO_LONGER_VALID - The bank account for this account has been closed or transferred.
REFER_TO_PAYER - The customers bank was unable to authorise the Payment Instrument. This could be due to the instrument type not being supported or the customer disputing authorisation with their bank. In these cases, please contact the customer.
INCORRECT_ACCOUNT_DETAILS - The specified bank account does not exist or was closed.
PAYMENT_INSTRUMENT_TYPE_NOT_ENABLED - The provided bank account does not support the selected instrument type.
MANUALLY_DISCONNECTED - The Payment Instrument was disconnected via the supplied disconnect API.
OTHER - A catch-all for other disconnection reasons not covered by the specific categories above. This may include technical issues or less common bank-specific failure reasons.