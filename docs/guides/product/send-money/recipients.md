# RecipientsCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/send-money/recipients

---

Recipients
Copy

After creating a quote, your application next needs to allow users to choose who will receive the funds, often referred to as the recipient or beneficiary.

This can either be completed by selecting an existing recipient the user previously created or by the user entering data to create a new one. What data that needs to be entered is dependent on a number of factors, which is why it can be requested as part of our dynamic forms solution.

Once the recipientID is known, your application is ready to create a transfer.

Listing Current Recipients

Each profile contains a list of active recipients that the user has previously sent to. Your application should use the list recipients by currency API to present a list of recipients to the user along with the ability to create a new recipient.

Once a user selects a particular recipient, your application should then update the quote with the recipient.

Always load a fresh set of recipient details from the Wise API when displaying a list or specific information about the recipient.
Create a new recipient

When creating a new recipient, there are a number of factors to consider, such as the currency pair, where the account is based, the amount of the transfer, and many more items. These factors additionally change on a regular basis, meaning creating and maintaining recipient account can be difficult.

With Wise, we have prepared a set of dynamic forms that work in a predictable and functional way. This allows you to rely on a single API endpoint to retrieve the details required for a particular quote and display that to users.

In order to do this, please see the Account Requirements Dynamic Forms API reference. This explains how to retrieve the account detail requirements, how to display and validate the data, and how to collect additional details.

Once the user has provided the details, your application will need to create the account. This can be done with the Create a New Recipient API.

Account Requirements Dynamic Forms are an absolute must for all integrations. This ensures that your application always has the most up-to-date information for creating recipients and that the flow will have fewer errors.

Not including dynamic forms for account requirements will result in your application breaking upon a change to those requirements, with users no longer able to create or update that type of recipient. If you are concerned about including dynamic forms in your application, please speak with the Wise team.

Recipient verification

You can use the recipient accounts resource to check if your recipient's details, such as their account number and name, match those held by their bank. This helps reduce errors and failed transfers.

Verification checks over The Wise Platform API provides a confirmations field in the response that tells you the outcome of this check.

Currencies with recipient verification
KRW (South Korean Won)
INR (Indian Rupee)
IDR (Indonesian Rupiah)
EUR (Euro)
Check if a recipient has been verified

When you create a recipient using the POST /v1/accounts API endpoint, the response includes a confirmations field. This field contains an outcomes array.

The outcomes[0].outcome field tells you the verification status. The outcomes[0].requiresCustomerAcceptance field is crucial, as it indicates whether the customer needs to take action.

Handle verification by status
Match: The details are correct. The recipient is created and ready for use. No action is needed.
Non-blocking Mismatch: There's a minor discrepancy, but the recipient is created and usable. You can choose to inform the customer, but no further API call is required.
Blocking Mismatch: A significant mismatch is found. The recipient is created but cannot be used for a transfer until the customer explicitly confirms the details. You must present the mismatch to the customer and get their approval.
Validation Error: A basic check fails (e.g., an invalid account number). The recipient is not created. You must show the error to the customer and ask them to correct the details before trying again.
Confirmations contract
{
    "id": 123,
    ...
    "confirmations": {
      "acceptedOutcomes": true | false,
      "outcomes": [
        {
          "type": "ACCOUNT_EXISTENCE" | "NAME_MATCHING" | "NAME_RESOLUTION",
          "timestamp": "2024-11-11T23:58:11.105916743Z",
          "outcome": "SUCCESS" | "PARTIAL_FAILURE" | "FAILURE" | "COULD_NOT_CHECK",
          "requiresCustomerAcceptance": false | true,
          "fieldsChecked": [
            "accountNumber",
            "accountHolderName",
            "details/legalType"
          ],
          "providedName": "Mickey Mouse", // only populated for NAME_MATCHING & NAME_RESOLUTION types for certain outcomes
          "resolvedName": "Actual Name", // only populated for NAME_MATCHING & NAME_RESOLUTION types for certain outcomes
          "message": "Message giving some details about the check",
          "recommendedUpdates": [
            {
              "path": "accountHolderName",
              "value": "Actual Name"
            },
            {
              "path": "details/legalType",
              "value": "PRIVATE"
            }
          ]
        }
      ] 
  }
}

At the moment it is safe to assume that there is only one element in the outcomes list.

Field descriptions
confirmations.acceptedOutcomes: whether we've received an explicit customer acceptance
confirmations.outcomes[0].type: the type of confirmation we're doing
confirmations.outcomes[0].outcome: the actual outcome of the confirmation
confirmations.outcomes[0].requiresCustomerAcceptance: whether we require customer acceptance. Whether this value is true or false is dependent on the currency and the nature of the confirmation.
confirmations.outcomes[0].fieldsChecked: fields we used to confirm the account
confirmations.outcomes[0].providedName: The name that the customer provided when creating the recipient account.
confirmations.outcomes[0].resolvedName: The name that we resolved during name matching or name resolution. Whether we always can return the resolved name depends on the outcome of the check and the currency specific configuration.
confirmations.outcomes[0].message: Customer facing message about the outcome of the check.
confirmations.outcomes[0].recommendedUpdates: Shown what are the correct values for some of the fields we've checked. Whether they're populated depends on the outcome of the check and the currency specific configuration.
Samples and handling
Recipient created with a blocking mismatch

Here is a sample response for a blocking mismatch. Notice that requiresCustomerAcceptance is true.

{
  "id": 12345,
  ...
  "confirmations": {
    "acceptedOutcomes": false,
    "outcomes": [
      {
        "type": "NAME_MATCHING",
        "outcome": "PARTIAL_FAILURE",
        "requiresCustomerAcceptance": true,
        "fieldsChecked": ["accountHolderName", "accountNumber"],
        "message": "The name entered does not match the name on the account.",
        "providedName": "Jan Doe",
        "resolvedName": "Jane M. Doe",
        "recommendedUpdates": [
          {
            "path": "accountHolderName",
            "value": "Jane M. Doe"
          }
        ]
      }
    ]
  }
}

This response indicates that we need customer's explicit action to accept the outcome before they can proceed with using the recipient. To supply the customer's acceptance call POST /v1/accounts with the response body from creation but change the confirmations path object to:

{
  "id": 12345, 
  ...
  "confirmations": {
    "acceptedOutcomes": true
  }
}
Testing Recipients
Testing in Sandbox

Because Sandbox is a test environment, there are some differences between Sandbox and Production. Please keep these in mind as you test.

While we validate some recipient account details in Sandbox, full bank detail validation can only be tested in Production.

Speak with the Wise team if you have further questions about testing recipients in Sandbox.