# Language SupportCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/developer/language-support

---

Language Support
Copy

See our translation and language support

Internationalisation support for translation of strings returned by the API is supported for endpoints on the right.

When calling these endpoints, you can include an Accept-Language header or a query string parameter of statementLocale with a supported language code as the value then strings will be returned in the requested language. The languages supported by Wise are:

Language	Code
American English	en_US
Arabic	ar
British English	en
Chinese	zh
Czech	cs
Danish	da
Dutch	nl
French	fr
German	de
Hebrew	he
Hungarian	hu
Italian	it
Japanese	ja
Korean	ko
Norwegian	nb
Polish	pl
Portuguese	pt
Romanian	ro
Russian	ru
Spanish	es
Swedish	sv
Turkish	tr
Ukrainian	uk

If you request an unsupported language then British English will be returned by default.

Language Supported Endpoints - Header
/v1/currencies
/v3/quotes
/v1/quotes/<quoteId/quoteUuid>/account-requirements
/v1/accounts
/v1/transfers
/v1/transfer-requirements
/v1/profiles/<profileId>/transfers/<transferId>/deposit-details/bank-transfer
Language Supported Endpoints - Query String
/v1/profiles/<profileId>/balance-statements/<balanceId>/statement