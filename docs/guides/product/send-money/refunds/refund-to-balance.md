# Direct Refund to Balance Copy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/send-money/refunds/refund-to-balance

---

Direct Refund to Balance
Copy

If the customer has a Wise account balance, this balance can serve as a refund recipient. To use the Wise account balance as a refund recipient, we will need to get the Recipient ID of the balance.

To get the Balance Recipient ID, please see the Retrieve multi currency account for a profile API reference.

From the Multi Currency Account object response, there will be a recipientId field. This will be the Recipient ID that can be used as the refund recipient.

This section is only applicable for `Direct Refund to Balance`. If you are refunding to sender bank account or using the Net Settlement model, you may skip this section.