# Funding a TransferCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/send-money/funding

---

Funding a Transfer
Copy

Once a transfer has been created, it needs to be funded. There are several mechanisms offered by Wise to fund a transfer and settle the funds to us.

Once you have successfully created a transfer, the correct amount should be funded to the transfer as soon as possible. This ensures that any FX rates do not expire, which can result in the target amount changing. The correct amount to fund is always the amount provided in the final quote object after any recipient updates. Certain funding options take care of this for you automatically.

Upon funding completion (funds received), the transfer will move from the incoming_payment_waiting status to the processing status.

While some funding options are instant, not all are. The status change only occurs once fund are received.
Funding Options

There are multiple options that are available for funding transfers.

Direct Bank Transfer (Push)

The most simple is to trigger a bank transfer or wire directly from the customer's account to our bank account in your currency. Below describes the direct transfer by transfer funding mechanism.

In order for us to link this in coming domestic payment with a corresponding transfer order, the individual creating the external push of funds need to include specific text in the "payment reference" field. Get the transfer id for the transfer you're trying to fund and append the letter T as a prefix. So for a hypothetical transfer with ID 80106743, the correct "payment reference" would be T80106743. Include this string in the reference when sending the funds to Wise's bank account.

For a full reference, please see th retrieve bank transfer deposit details for a transfer API reference.

Direct Debit Bank Transfer (Pull)

In certain countries and currencies, we offer the ability to fund transfers via direct debit. This requires the ability to create a direct debit account with Wise on a profile and then instruct us to pull funds from that account.

This option is only available to select partners. If you are interested in this model please get in touch with the Wise team to discuss the option and for technical documentation on how it works.

Fund From Wise Balance

Wise accounts are able to hold funds in a balance, and these funds can be used to fund a transfer directly. To do so, a transfer must be created in the currency of the balance that will be used to fund it. We currently do not support cross balance funding.

For a full reference, please see the fund a transfer from balance API reference.

Bulk Settlement

We offer a "prefunded" model for those partners that want to speed up transfers for their customers in regions where the domestic payment networks are slow or otherwise do not allow for instant funding. If you are interested in this model please get in touch with the Wise team to discuss the option and for technical documentation on how it works.

The funding options are dependent on the integration type, the partner experience, source and target currencies, and money movement rails used to transfer funds. The goal is always to have transfers move as instantly as possible, so we will work with you to use the best solution to make transfers as instant as possible.
Testing Funding
Testing in Sandbox

Because Sandbox is a test environment, there are some differences between Sandbox and Production. Please keep these in mind as you test.

We provide a balance top-up simulation endpoint to top up your balance if you want to test funding a transfer from balance.

It is possible to test funding via bulk settlement, but you'll need a configuration. Please reach out to the Wise team for assistance if you'd like to test this.

Speak with the Wise team if you have further questions about testing funding in Sandbox