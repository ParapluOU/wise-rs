# Configure authorisation rulesCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/issue-cards/card-spend-controls

---

Configure authorisation rules
Copy

Using our spend control APIs, you can block or allow transactions based on the MCC (merchant category code) or the currency of the transaction.

Currency codes follow the ISO-4217 standard, please refer to iban.com.

Note that the MCC list is based on Visa or Mastercard schemes.

Spend controls are scoped at the application level. Meaning that the client credentials token will be required to call these APIs

Blocking a type of transaction
Identify the MCC or currency to block. For example, block gambling transactions - MCC 7995.
Create an authorization rule with the following:
type as MCC
operation as BLOCK
values as [7995]
description as block gambling MCC 7995 (optional)
Retrieve the authorization rules to see that the rule has been created.
Apply the rule to start blocking transactions

An ALLOW rule permits only the transactions that match the specified criteria and blocks all others. For instance, a rule allowing SGD transactions will block all transactions that are not in SGD"