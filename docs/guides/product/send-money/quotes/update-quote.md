# Updating a Quote Copy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/send-money/quotes/update-quote

---

Updating a Quote
Copy

It is important to update the quote before creating a transfer as fees and expected delivery times can change based on the recipient, payout method, or payment metadata.

For example, the fees and delivery estimate are different in the case of sending USD to a country other than the USA - we call this Global USD. Multiple factors can play into this, so it is important to complete this step.

Updating the quote ensures you can show the customer the correct final price and delivery estimate time before they commit to creating and funding the transfer.

Updating a quote with a recipient ID is not a required step, and in certain integrations is not necessary. Our team will help determine if this step can be removed from your application.

To update a quote you use the PATCH call to add a recipient. This will update the saved quote's data based on who and where the money will be sent to.

Updating the quote with a recipient may cause the available payment options, prices and estimated delivery times to change from the original quoted amounts. This is due to the fact that sending some currencies to some destinations costs a different amount based on the payment networks we use, for example sending USD to a country outside the USA uses international rather than domestic payment networks and as such costs more, or sending CAD over the Interac network is a more expensive operation.

Transfer Nature is a required field that should always be patched to a quote for transfers to and from BRL. This ensures the correct IOF tax is applied.