# Test and prepare to go liveCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/receive-money/test-and-go-live

---

Test and prepare to go live
Copy

Testing directly over the Swift Network is complex and can be slow. Instead, you should test your integration by mocking a Swift message with the Wise Platform sandbox.

Mock Swift payments with sandbox

Using sandbox, you can test the initial tech build for webhook subscriptions, balance statements, and sweeping funds.

To test your listener service, you should use the following simulation endpoint to send a mock Swift payment to sandbox. If the request is successful the transfer is logged in our back office, which triggers a swift-in#credit event and results in a balance update.

Please check with your dedicated Implementation Manager if you don't receive the event.

Schema
currencyCodestring

Currency received by Wise.
Mandatory
Length: Exactly 3 Characters
Format: 3-letter ISO currency code

amountdouble

Amount received by Wise.
Mandatory

beneficiaryNamestring

Name of the ultimate beneficiary. Optional

beneficiaryAccountstring

Account number of the ultimate beneficiary. Optional

beneficiaryAddressstring

Address of the ultimate beneficiary. Optional

senderNamestring

Name of the sender. Optional

senderBicstring

BIC of the sender's bank. Optional

senderAccountstring

Account number of the sender. Optional

senderAddressstring

Address of the sender. Optional

senderAddressstring

Custom payment reference. Optional

Example swift-in simulation request
curl -X POST https://api.wise-sandbox.com/v1/simulation/profiles/{profileId}/swift-in \
     -H 'Authorization: Bearer <your api token>' \
     -H 'Content-Type: application/json' \
     -d '{
          "currencyCode": "EUR",
          "amount": 100,
          "beneficiaryName": "John Doe",
          "beneficiaryAccount": "DE89370400440532013000",
          "beneficiaryAddress": "1234 Elm St, Springfield, IL 62704",
          "senderName": "Jane Doe",
          "senderBic": "DEUTDEDBFRA",
          "senderAccount": "DE89370400440532013000",
          "senderAddress": "5678 Oak St, Springfield, IL 62704",
          "paymentReference": "Test123"
        }'
Going Live

Once configuration and production testing is completed, you'll need to update SSI to point your production BIC to our BIC.

It's also recommended you notify other members of the Swift network that the SSI has been updated. Otherwise it might take a while before they start routing payments correctly. This can be done through an MT670 message sent directly to Swift. Wise will need to send this message on your behalf.