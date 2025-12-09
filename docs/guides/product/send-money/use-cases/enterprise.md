# Payouts for enterpriseCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/send-money/use-cases/enterprise

---

Payouts for enterprise
Copy

Use the Wise Platform API to send money from your Wise account and create regular payments. This guide is for integrations where you are a direct customer of Wise, using your own profile to manage transfers.

In this implementation, you are not in the flow of funds on behalf of any other party.

Core concepts

As a Wise partner, you are a direct customer of Wise and use the API to make payouts from your Wise account.

As a Wise partner, you are a direct customer of Wise and use the API to make payouts from your Wise account.
No contact required between Wise and your recipients.
No need for your recipients to onboard or undergo KYC (Know Your Customer) on Wise.
Requirements

Before sending money, need to have your own profile to manage all transactions.

Overview of API requests
Complete the Wise developer onboarding process:

You only need to do these things once, and then occasionally manage when needed. You can find guidance for setting up as a developer on Wise platform in the Getting started guide.

You will get help from your implementation managers throughout this process.

Send money

Follow these steps to a transfer using the Wise Platform API:

Establish the quote and the recipient. You can do these steps in whichever order makes the most sense for your use case:
Create a Quote: Send a request to POST /v3/profiles/{{profileId}}/quotes to get an authnticated quote for a transfer.
Create a Recipient: Send a request to POST /accounts to add the recipient's bank details.
Create a Transfer: Send a request to POST /profiles/{{profileId}}/transfers to initiate the transfer.
Fund the Transfer: Send a request to POST /profiles/{{profileId}}/transfers/{{transferId}}/payments to complete the transfer.