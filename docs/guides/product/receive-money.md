# International ReceiveCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/receive-money

---

International Receive
Copy

Use International Receive to allow your customers to receive international payments via Swift. Payments are made through the Swift network to your Wise account. You can then move the money to your customers' bank account.

How it works

Before you receive payments through Swift, you must make sure your BIC is connected to the currencies you plan to receive from. This step can be completed with help from your implementation team at Wise.

Once you have set up your currencies, the process of handling incoming Swift payments with Wise Platform looks like this:

A customer uses your Swift BIC and IBAN to direct a payment over Swift into your Wise account. As the money hits your balance, you receive a swift-in#credit webhook notification from Wise Platform to tell you:
That a payment has been made to your account via Swift.
The amount and origin of the payment.
Any fees incurred for the Swift payment.
The details of your customer - including bank account information - so you can move this money to your customer's account.
Get a balance statement for reconcilliation. In addition to webhook notifications, you can check for inbound Swift payments with a request to get your balance-statement.
Once the funds are in your account, you can send the money to your customer using Wise Platform's API, or proceed according to your business arrangements.

Wise does not track this payment once it has been added to your balance. You can use the Wise API to send money directly to your customer, or perform a sweep, or hold the money. This is part of your business process, and you should track on your side.

As you work with us to build your implementation, and set up your BICs for International Receive, you should consider these timelines and manage your release of this feature accordingly.

Next steps
If you are new to Wise Platform, use the Getting started resources for developers to get ready to create and test any API integration.
Learn how to handle the business process for International Receive, including how to connect your BIC and share your IBAN details.
Subscribe to the webhook notifications required for International Receive.
Learn how to request a balance-statement via API.
Learn how to manage returns.
Test your build in the Wise Platform sandbox environment.