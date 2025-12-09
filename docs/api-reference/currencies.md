# CurrenciesCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/api-reference/currencies

---

Currencies
Copy

Operations

GET
/v1/currencies
The Currencies resource
Fields
codetext

Currency code (ISO 4217 Alphabetic Code)

symboltext

The symbol of this currency

nametext

Display name of this currency

countryKeywordslist of strings

Keywords associated with this currency

supportsDecimalsboolean

Whether this currency supports decimal values or not

Currencies Object
[
  {
    "code": "AUD",
    "symbol": "A$",
    "name": "Australian dollar",
    "countryKeywords": [
      "AUD",
      "AU",
      "Australia",
      "aus"
    ],
    "supportsDecimals": true
  },
  {
    "code": "JPY",
    "symbol": "¥",
    "name": "Japanese yen",
    "countryKeywords": [
      "JPY",
      "JP",
      "Japan",
      "jpn"
    ],
    "supportsDecimals": false
  },
  ...
]
Get all currencies allowed for transfers

GET /v1/currencies

Get the list of allowed currencies that you can use when setting up your transfers.

Response

Returns a currencies object.

Example Request
curl -X GET \
  https://api.wise-sandbox.com/v1/currencies \
  -H 'Authorization: Bearer <your api token>'