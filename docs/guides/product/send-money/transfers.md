# Create TransfersCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/send-money/transfers

---

Create Transfers
Copy

Once a quote and recipient have been created, there's just two more step before you can create the transfer:

Checking the Transfer Requirements - Each currency, region and markets may require different mandorary information. This check ensures you have everything needed to create the transfer.
Updating the Quote - This allow you to include additional payment or recipient information to ensure final cost is as accurate as possible.
Transfer Requirements

Each transfer can have additional requirements that are needed in order to process the transfer. To determine the transfer requirements needed, your application should use the transfer requirements endpoints to request the requirements for the transfer.

Requirements are returned in a dynamic form, with form types, restrictions, and validation included in a consistent method. This allows your application to present these to users in a consistent way.

Once all transfer requirements have been gathered, they should be added to the details section of the create transfer API call.

For a full reference, please see the request transfer requirements API reference.

Including transfer requirements that are not optional is important, as it helps ensure that transfers are able to process as quickly as possible with further requests for information reduced. Please ensure that this process is part of your integration.
Transfer Creation

Different integration types will require different transfer types to be created. All transfers may require additional transfer requirements, which should be requested and captured before creation of the transfer.

Once ready to create a transfer, your application should use one of the following transfer creation types:

Standard API Transfers

For transfers via API where you do are not have financial license for the currency routes or you are the customer of the transfer.

Third Party API Transfers

For transfers via API where you have a financial license for the currency routes and are acting on behalf of your customers.

Swift Network Transfers

For transfers via the Swift Network typically by banks and financial institutions with a Swift or BIC code.

Only one type of transfer creation will be made available for your integration, with others resulting in a 403 - Unauthorized response. If you feel this is incorrect, please speak with the Wise team.