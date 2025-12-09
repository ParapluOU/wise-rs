# ContactCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/api-reference/contact

---

Contact
Copy

A contact is a representation of a recipient or beneficiary.

Operations

POST
/v2/profiles/{{profileId}}/contacts?isDirectIdentifierCreation=true
Create a contact from an identifier

POST /v2/profiles/{{profileId}}/contacts?isDirectIdentifierCreation=true

This endpoint finds an existing discoverable Wise profile and adds it to your recipient list. The recipient is found just by an identifier, without having to know their bank details. The contactId from the response can be used to create a transfer.

Request
identifiertext

A Wise profile’s identifier. It can be a Wisetag, email or phone number.

targetCurrencytext

3 character currency code

Response
contactIdtext

contactId in the format of UUID

nametext

full name of the contact

Example Request - GBP Recipient
curl -X POST \
  https://api.wise-sandbox.com/v2/profiles/{{profileId}}/contacts?isDirectIdentifierCreation=true \
  -H 'Authorization: Bearer <client credential token>' \
  -H 'Content-type: application/json' \
  -d '{
    "identifier":"@JonathanP",
    "targetCurrency":"EUR"
  }'

Creating a transfer with a contact
Create a Quote where the targetAccount field is replaced with "contactId": <contactId>
Extract the targetAccount value from the Quote
Create a Transfer