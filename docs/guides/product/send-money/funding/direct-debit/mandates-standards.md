# Direct Debits in the UK and EuropeCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/send-money/funding/direct-debit/mandates-standards

---

Direct Debits in the UK and Europe
Copy

To help you integrate Direct Debits into your platform, there are universal requirements you need to follow.

Mandate requirements

You must obtain a mandate, which is the customer's legal authorization for you to debit their account. This consent can be in either paper or electronic form. Your mandate must include:

The customer's full name and bank account details (e.g. IBAN).
Your company's name and Creditor ID.
Specific legal wording that authorizes you to pull funds from the account.
Required mandate information

Beyond the legal requirement to create a mandate, any customer-facing pages you create must display specific, information to be compliant. This includes:

Mandatory legal wording: The mandate page must contain specific legal text as defined by the European Payments Council (EPC). The exact wording differs between the Core and B2B schemes. This text outlines the customer's rights, including their "no-questions-asked" refund period. Explore the mandatory wording on the European Payments Council website.
Unique Mandate Reference (UMR): A unique reference number that identifies this specific mandate. This number must be assigned by you and remain consistent for the lifetime of the mandate.
Creditor Identifier: Your company’s official SEPA Creditor ID (CI) must be clearly displayed. This ID allows banks to recognize you as the authorized entity collecting the funds.
Company Information: Your company's legal name, and often your full address, must be visible on the page.
Visual and functional guidance
Logo: While not explicitly a mandate requirement, the use of a Direct Debit logo is a best practice to build trust and recognition. You'll often need to obtain approval from your sponsoring payment service provider to use this logo in your documentation and on your forms.
Confirmation: The customer must take an explicit action, such as clicking a button or signing, to give their authorization. This "electronic signature" must be logged and stored.
Payment confirmation page: After a successful mandate setup, the payment confirmation page should reiterate the key details of the agreement and provide a clear, positive message confirming that the mandate has been created.
Record keeping: You are legally required to securely store all mandates (both paper and electronic) for the duration of their validity and for a minimum of 36 months after the final collection. This is required for verification or legal purposes if a dispute arises.
Pre-notification rules

Before each payment, you are required to send a pre-notification to your customer to inform them of the upcoming debit. This ensures transparency and gives them time to prepare.

Notice Period: You must send this notification at least 14 calendar days before the collection date. This period can be shorter if you have agreed to this with your customer in the mandate.
Required Information: The pre-notification must clearly state the payment amount, the collection date, and the unique mandate reference.
Funding transfers

Once a payment is collected, the funds will be used to directly fund a transfer. This process typically takes a few business days for the funds to settle.