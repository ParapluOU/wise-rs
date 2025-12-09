# View card transactionsCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/issue-cards/card-transaction

---

View card transactions
Copy

Track and manage customer card transaction data

Card Transactions

It is recommended to subscribe to card transaction state change webhook events to receive card transaction data to build your card transaction page.

To retrieve more information related to the card transaction (such as merchant data) you need to use the card transaction API.

The possible state values are:

IN_PROGRESS - The transaction has been authorized but not captured.
COMPLETED - The transaction has been captured and/or settled.
DECLINED - The transaction has been declined.
CANCELLED - The transaction has been cancelled.
UNKNOWN - Default fallback status if the state can't be confirmed.

The transition from CANCELLED to COMPLETED is an edge case. Wise will release user funds after 7 days (30 days for pre-authorization) if the merchant has not captured the transaction and the state is moved to CANCELLED. After this, the merchant can decide to capture the transaction after the release of funds, the state will then be move to COMPLETED.

For a detailed list of decline reasons see here.

To receive webhook events on changes in a user's balance see balance update events

Card transaction state transitions
Activities API

For a simple solution to display all actions that have been performed on a user account, see the activity API.