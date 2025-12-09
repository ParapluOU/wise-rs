# Quote CreationCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/send-money/quotes

---

Quote Creation
Copy

Get FX transfer rates, fees and delivery estimates

Learn how to create, update and use quotes to check FX rates, fees, and delivery estimates, enabling seamless money transfers with Wise Platform.

Overview

The Send Money flow begins with the creation of a quote to transfer a FX payment.

A quote defines:	And will return:
which currencies you’re converting	the rate and any fees
how much to send or receive	estimated delivery speed
who is sending and receiving the money	available methods to fund the transfer
Quote Types

There are two types of quotes that can be generated:

Unauthenticated Quotes

POST /v3/quotes (optional)

Purpose:

Optional to offer fast estimates for illustration
No Auth or ProfileID needed
But can’t be used to create real transfers
Authenticated Quotes

POST /v3/profiles/{profileId}/quotes

Purpose:

Required to create a real transfer
Must be assoicated with profile
Needs updating with recipient for accurate fees

Providing payin and payout details improves the accuraty of the quote.

Authenticated vs Unauthenticated Quotes
Feature	Authenticated	Unauthenticated
Authentication	Required (OAuth 2.0)	Optional
Profile ID	Required	Not applicable
Quote ID	Generated	Not generated
Can create transfer	✅ Yes	❌ No
Rate locking	Available	Not available
Use case	Real transactions	Display/marketing
Rate limits	More restrictive	More permissive
The Quote flow

Typically there's 2 or 3 steps involved in creating a quote that can be used for a transfer.

Configure pricing

This is an optional step to allow partners to override pricing per quote via pricing configuration. Most partners can skip this step.

Create a quote

Create an authenticated quote. You need to provivde a profileID (the account sending the money), currencies and either the money to send or the amount to be received (never both).

Add the recipient

Before creating a transfer, the quote must be updated to state the recipient of the money. This will ensure you have accurate fees, delivery estimates and the payout network.

The mid‑market exchange rate is locked when the Authenticated quote is created and typically remains valid for 30 minutes, giving the user time to complete the flow.

Fees and Pricing

Included in a quote are the fee and price objects for each payin option. These include the summary and detailed views of the fees to be charged on the transfer.

The fee object gives a summary of the fees. The transferwise fee includes any taxes or fees that Wise will charge. Additional fields, such as payIn, discount and partner will include any amounts as applicable. total sums all fees and discounts for that payment option.

The price object further breaks down the fees and discounts as line items. This will include any discounts, fees or taxes and give the appropriate name for each.

Any transfer to or from BRL will always include an IOF Tax fee and be collected during the transfer funding process. This should be shown to users as a separate line item, separate from all other fees.
Testing Quotes in Sandbox

Sandbox provides realistic but not live rates, so the quote you receive when you test may differ from what is available in Production.

In Sandbox, you cannot test a rateType of FLOATING.

Speak with the Wise team if you have further questions about testing quotes in Sandbox.