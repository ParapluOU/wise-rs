# Additional Customer VerificationCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/kyc/partner-kyc/additional-verification

---

Additional Customer Verification
Copy

In certain situations, additional evidence is required to verify customers and ensure we’re compliant with the KYC regulations.

If your customers are primarily based in the EU, refer to the section Onboarding EU Customers below. Note: this functionality is only available as a closed BETA.

If you are onboarding high riåsk businesses primarily based in the US, refer to the section Onboarding High Risk US Businesses below. Note: this functionality is only available as a closed BETA.

Onboarding EU Customers

If you have customers that meet all the criteria below, then you must implement the steps in this guide to onboard all your new customers.

You use the customer account with Partner KYC model (we onboard and create a Wise account for your customers)
Your customer’s address is in an EEA country
Your customer creates a quote with source currency for an EEA currency

During onboarding, customers need to provide additional evidence using the upload-evidences endpoint. We will use this to perform an initial automated check before either verifying the customer, or requesting more information. Successful verification will be returned within a few seconds. Unsuccessful verification will create a case for our team to review, so it can take longer to receive an update.

Subscribe to additional verificaiton state change event webhook to receive updates about your customers' additional verification status. This webhook will also inform you if any further evidence or documents are required to complete the review.

Provide those additional evidence by using upload-document endpoint.

Based on Wise.com average customer base, about 95%* of customers do not need to provide documents or additional information after the initial upload-evidences information is submitted.

Sequence Diagram

Endpoints
POST
/v3/profiles/{{profileId}}/additional-verification/upload-evidences
POST
/v3/profiles/{{profileId}}/verification-status/upload-document
Repapering EU Customers

This functionality is in a closed BETA.

Repapering is the process of performing additional verification for existing customers.

You only need to use this if you began onboarding customers before 1/1/2023 and if you have customers that meet all the criteria below.

You use the customer account with Partner KYC model (we onboard and create a Wise account for your customers)
Your customer’s address is in an EEA country
Your customer creates a quote with source currency for an EEA currency

The process starts by calling required-evidences endpoint before a transfer is created. This returns a list of required evidence for your customer. If no evidence is required, then proceed to transfer as usual. Otherwise, if one or more evidences are required, submit those via upload-evidences.

The process from that point will follow the flow Onboarding EU customers. Once verified, the customer may proceed with their transfer.

Sequence Diagram

Endpoints
POST
/v3/profiles/{{profileId}}/additional-verification/upload-evidences
POST
/v3/profiles/{{profileId}}/verification-status/upload-document
GET
/v3/profiles/{{profileId}}/verification-status/required-evidences
Understanding evidences

Evidences are pieces of data that are required by Wise to verify customers.

This is the list of evidences that are supported by the APIs.

Source of Wealth document

Source of wealth is a document that proves the customer's main source of income.

If SOURCE_OF_WEALTH is required (e.g. via cdd-check-state-change webhook), your customer must provide a verification document depending on the table below.

Acceptable source of wealth documents for a personal profile

For personal profile, the acceptable documents depends on the source_of_income your customers provided in the upload-evidences endpoint. Here is a list of acceptable documents for each source_of_income for personal profiles.

Source of Income	Acceptable documents
SALARY	PaySlip (3 months worth)
3 months of Bank statements/transaction details confirming salary deposits
Statements from Payoneer/Paysera/PayAlly/PayPal etc
Tax return
Employment contract
Proof of employment
Unemployment allocation
Maternity/Paternity allowance/benefit
W2 Earnings Summary
Bolt courier report / Wolt, Uber or Bolt driver report or other similar
UpWork statement
E-commerce platform statement
Invoice (contractors only)
INVESTMENTS	Employee stock plan exercise confirmation
Investment/portfolio account statement
Dividend voucher
Share sale confirmation
Crypto wallet statement
Bank statement confirming income from investments
PENSION	Pension statement
Bank statement confirming pension income
INHERITANCE	Grant of Probate
Will
Trust Deed
Death certificate (as supporting doc)
Bank statement, confirming income from inheritance
LOAN	Loan Agreement with supporting bank statement showing the amount being received
OTHER	For example documents that proves:
Gift
Sold property
Receive money from family
Scholarship
Dividends
Renting out property
Lottery winnings
Divorce
Child support
Savings
Freelancing/Self-employed/Contractor
Acceptable source of wealth documents for a business profile

For business profile, the document depends on source_of_funding.

Source of funding	Acceptable documents
REVENUE	Banks statements or invoices with 3-6 months of most recent transactions
Audited financial statements or annual reports
Business tax return or declaration
BUSINESS_LOAN	Loan agreements or contracts within the last 12 months
INVESTMENT_INCOME	Share sale certificates or asset sale agreements from the last 12 months
FUNDING_AND_SHAREHOLDER_INVESTMENTS	Equity or subscription agreements
DONATIONS	Donation receipts, tax receipts, or similar documents from the last 12 months
GRANTS	Grant award letter from the last 12 months
OTHER	Any official document showing where your funding comes from
Proof of Trading address document

Proof of trading address is a document that proves the business operates from the provided address in their profile.

If TRADING_ADDRESS_PROOF is required (e.g. via cdd-check-state-change webhook), your customer must provide a verification document with business name on it and issued to the business.

These are the acceptable sources

Bank/card statement
Any document from a regulated financial services firm
Gas/water/electricity/waste bill/rental agreement or valid contract
Any document issued by an institution of the government
Business Authorisation Letter document

Business Authorisation letter is a document that authorises a person to act financially on behalf of the company while using Wise Business account.

Templates for the letter based on where the business is registered .

API references

Use these API references to implement these flows for your customers.

Upload Evidences

Use upload-evidences endpoint to submit customer information for review

Upload Document

Use upload-document to provide verification documents for review

Required Evidences

Use required-evidences to get the evidence types required to complete additional customer verification for your customer