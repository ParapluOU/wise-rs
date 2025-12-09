# Payouts for banks and financial institutionsCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/send-money/use-cases/correspondent

---

Payouts for banks and financial institutions
Copy

Make cross-border payments on behalf of your customers. Fund payments with a Nostro account from Wise.

Use this implementation only if you have a license to be in the flow of funds for your customers - for example, you have a banking, e-money or payment institution license.

Core concepts
Use this implementation to give your customers (as ultimate senders) the ability to send cross-border payments, funded from your Wise nostro account or another agreed funding source.
No contact between Wise and your customers. Your customers get access to our network of payment currencies without having to onboard to Wise.
No need for your customers to undergo KYC (Know Your Customer) from Wise.
Overview of API steps in this implementation

Using the Wise Platform API, you can think of your implementation as three distinct stages.

Complete the Wise developer onboarding process:

You only need to do these things once, and then occasionally manage when needed. You can find guidance for setting up as a developer on Wise platform in the Getting started guide.

You will get help from your implementation managers throughout this process.

Send money:

Every time you send money using Wise Platform, you follow these key steps:

Create a quote so your customer can see how much the transaction will cost, and the exchange rate. You can present an authenticated or unauthenticated quote.
An unauthenticated quote is an optional first step. The quote is based only an indicative rate, and may change when more information is added. This is helpful to illustrate the costs of a payment without submitting a request with a token or profile details. You cannot use this quote to create the transfer.
An authenticated quote includes profile data and your request must include your profile ID and user acces token.
Add a recipient's account details - the recipient is the ultimate beneficiary - the party receiving the money.
Update the quote - update your quote with the recipient information to create a more precise, authenticated quote.
Create the transfer with all required information about:
The recipient (ultimate beneficiary).
The originator (ultimate sender).
The quote.
Fund the transfer by specifying the source of funding the money is to be taken from. This can be your Wise nostro account, or another agreed funding source. Once this step is complete the money is sent.
Track payments:
Subscribe to the Transfer state change webhook for asynchronous updates.
Get the status of a transfer using the API.
Next steps
Review the Getting Started Guide.
Learn how to send money on behalf of a third party.
Learn how to use webhooks to track payments.