# Multi Currency AccountsCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/accounts

---

Multi Currency Accounts
Copy

This document provides guidelines for the implementation of multi-currency accounts for all partners.




It's broken down into key areas and describes the logic to:

Determine a user's eligibility for multi-currency accounts.
Determine which balance account currencies are supported.
Create balance accounts for personal and business profiles.
Add bank account details to eligible balance accounts.
Top up a balance account from an external source.
Send money from a balance account.
Convert funds between two balances in different currencies.
View the transaction history for a balance account.
Multi Currency Accounts Postman Collection

We've created a Postman collection to help you test your integrations easily! It includes:

Our sandbox environment (https://api.wise-sandbox.com)
Common API calls for creating users and profiles
Automated tests that copy response data between calls
Example responses for reference

Fork the collection to get updates automatically. This lets you test the complete flow with minimal setup!

Fork in Postman
Multi-currency account structure

The structure of a multi-currency account is as follows:

A User has a profile (personal or business).
A Profile has the ability to open a Multi-Currency Account.
A Multi-Currency Account has the ability to have multiple Balance Accounts in different currencies.
A Balance Account has a specific currency and can contain a set of Bank Account Details.
Bank Account Details are a set of local account details.

The diagram below illustrates the relationships between each of the components highlighted above.

Authentication

For details on authenticating against our API, please see Authenticating with the Wise API.

Profiles

Most endpoints included in this guide require a profile ID and/or a balance ID as part of the path. These are denoted with {{profileId}} and {{balanceId}} throughout the guide, and are obtained from previous requests to endpoints as outlined.

Additional headers
Accept-Language

Each endpoint accepts an additional header of Accept-Language. This should include an ISO 639-1 language code. This will default to en if it is not passed as part of the request. The examples in this guide include the header for those calls that return translated strings.

Content-Type

Endpoints that include a body require a Content-Type: application/json header to be included. This is included in the examples where required.