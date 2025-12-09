# Connect your BIC detailsCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/receive-money/connect-bic-details

---

Connect your BIC details
Copy

To enable International Receive Swift payments to your Wise account, you must ensure that your Swift BIC codes are connected to the correct currencies through Wise.

Connect BIC through our implementation team

You cannot connect your BIC via our API, but your Wise implementation partners can help ensure this is set up correctly for you. All you need to do is provide us with the BICs you would like us to match and use to deposit funds into your Wise account.

This can be a list of specific BICs or can be based on a "starts with" match.

For example, if we configure "BANKDE12", any branch code will also be matched, such as "BANKDE12XXX". If you would like for us to match specific branch codes, those should be provided.

Provide an unused BIC for testing

Due to the length of time it can take for the Swift network to update the standing settlement instructions for BICs, you should provide a BIC to be used for testing.

This should never be a BIC that is in use by your customers or in production. Often this is a separate BIC that is unused or reserved for testing.

Speak with your implementation manager further about this and when it is applicable to make this change.