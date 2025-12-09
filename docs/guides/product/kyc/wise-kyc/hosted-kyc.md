# Wise Hosted KYC and KYBCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/kyc/wise-kyc/hosted-kyc

---

Wise Hosted KYC and KYB
Copy

Learn how to verify your customers identity

To offer your customers a Wise account, you must complete identity verification steps, known as Know Your Customer (KYC). This guide will teach you how to set-up these checks using our hosted interface.

Core concepts

To offer your customers a Wise account, you must complete identity verification steps, known as Know Your Customer (KYC). Wise Platform's Hosted KYC APIs allow you to integrate directly with Wise's KYC experience.

You provide your customers with a link to a Wise-hosted UI. This UI guides them through various forms and might include liveness checks or instructions to upload documents, such as IDs. The specific requirements depend on factors such as:

Your customer's country of residence
Source currency
Target currency
Transfer amount.

During implementation, you can configure the UI's color theme to match your brand.

This approach means you don't need to build the verification screens yourself, as Wise handles the collection of all KYC requirements.

Choose when to perform KYC

You can choose when to initiate the hosted verification process:

When your customer signs up to get an account. This is called proactive KYC because you trigger the KYC process before a payment needs to be made.
When your customer makes their first payment. This means your customers can register for an account with minimal information, but we don't collect verification details until the first time they send money.

Whichever you choose, the process is the same in terms of API requests and webhook notifications.

How it works

To trigger a KYC flow:

Create a user and a profile for your customer.
Subscribe to the KYC review state change webhook.
Start the KYC process:
For proactive KYC, start a KYC review before your customer tries to send money. Use the Create KYC Review endpoint, including PROACTIVE_SEND_MONEY type in the request body. Currently, proactive KYC is only supported in EU, UK, SG, JP and AU regions for business Send Money.
For KYC at the point of sending money the first request to create a transfer automatically triggers the hosted KYC process.
Check your webhook notifications for the status of your customer's KYC.
When the status is WAITING_CUSTOMER_INPUT, use Update KYC Review to receive a redirect link for your customer to follow.
Direct your customer to follow the link provided and completes the KYC process through the hosted experience.
Check your webhook notifications. When the status is PASSED, this means your customer has completed KYC.

Once the process is complete, your customer can continue to send money and access their account.

Next Steps

Now that you have an overview of how to implement hosted KYC, take a look at suggested next steps:

Create User and Profile: Learn how to set up customer accounts
Manage Hosted KYC Process: Understand the verification workflow
KYC Review Status Notifications: Handle webhook notifications effectively