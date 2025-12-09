# Create a Custom ReceiptCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/send-money/receipts

---

Create a Custom Receipt
Copy

This guide will walk you through how to create a custom transfer receipt that can be shared with your customers. Generally, this includes the following steps:

Make a request to GET /v1/transfers/{{transferId}} to fetch the transfer details.
Make a request to GET /v1/transfers/{{transferID}}/invoices/bankingpartner to fetch the banking partner details.
Combine the information from both responses to create a custom transfer receipt.
Get a transfer by ID

GET /v1/transfers/{{transferId}}

Get transfer info by ID. To receive dynamic updates as the state of the transfer changes, please see our documentation on webhooks.

Response

Returns a transfer object, with or without an originator block depending on the type of transfer.

Sample Request
curl -X GET \
  https://api.wise-sandbox.com/v1/transfers/{{transferId}} \
  -H 'Authorization: Bearer <your api token>'
Get payout information

GET /v2/transfers/{{transferId}}/invoices/bankingpartner

To utilize this endpoint, you will need to replace transferID with the specific transfer's unique identifier. The transfer endpoint will return the details of the transfer, including the processorName, deliveryMode, bankingPartnerReference, bankingPartnerName, and mt103. This information will enable your recipients to track the transfer with their bank. It may take up to 3 days to get the correct information through this endpoint, as some partners don't share the information until 3 days later.

Fetch banking reference information for transfers that are in outgoing_payment_sent status, enabling you to track transfers with the transfer recipient’s bank.

Request
transferIDtext

The unique identifier of the transfer.

Response
processorNamestring

The legal entity that processed the transfer on behalf of the customer.

deliveryModestring

The delivery mode for the payment (e.g., Swift).

bankingPartnerReferencestring

The reference used by the partner bank to identify and track the transfer.

bankingPartnerNamestring

The name of the sending bank to the recipient's bank.

mt103string

The MT103 of the transfer, if available.

Request Example
curl -X GET \
  https://api.wise-sandbox.com/v2/transfers/{{transferId}}/invoices/bankingpartner \
  -H 'Authorization: Bearer <your api token>' 
Response Example
{
  "processorName": "Acme Bank Ltd.",
  "deliveryMode": "SWIFT",
  "bankingPartnerReference": "ABCD1234",
  "bankingPartnerName": "Global Bank Corp."
  "mt103": "{1:F01XXXXGBXXAXXX0000000000}{2:I103XXXXGBXXXXXXN}{3:{108:1234567}{111:001}{121:00000000-0000-0000-0000-000000000000}}{4:\n:20:1234567\n:23B:CRED\n:32A:221212USD12345,\n:33B:USD12345,\n:50K:/11111111\nSOME COMPANY INC.\n1 SOME STREET MIAMI 33132 US\n:59:/GB00000000000000\nCOMPANY NAME LTD\nUK LONDON 1234 GB\n:70:REFERENCE\n:71A:OUR\n:71G:USD11,\n-}\n"
}
Creating the Custom Transfer Receipt

Once you have fetched the transfer details and banking partner details, you can combine the information to create a custom transfer receipt. The custom transfer receipt should include the following information:

Sender details (name and account number)
Recipient details (name and account number)
Amount transferred
Currency
Status of the transfer
Date and time of transfer creation
Processor name
Delivery mode
Banking partner reference
Banking partner name
MT103

With this information, you can format the transfer receipt in a manner that suits your needs and preferences. You can create a PDF, HTML, or plain text receipt, and share it with your customers via email or other communication channels.