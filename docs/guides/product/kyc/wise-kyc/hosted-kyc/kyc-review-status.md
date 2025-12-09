# KYC Review StatusCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/kyc/wise-kyc/hosted-kyc/kyc-review-status

---

KYC Review Status
Copy

Subscribe to the kyc review state change webhook to keep track of status of KYC Reviews of your customers.

KYC review statuses and what they mean:

NEW - KYC Review has been newly created and KYC requirements calculation is being processed. KYC Review will be in this state at most few seconds.
PASSED - No customer action is required, underlying action (for example, transfer) can proceed.
PASSED_WITH_REQUIREMENTS - No customer action is required for now and the underlying action can proceed, but there are requirements that need to be fulfilled by the requiredBy date or future actions (for example, transfers after the date) will be blocked.
WAITING_CUSTOMER_INPUT - Customer needs to provide some KYC information to unblock the underlying action.
PROCESSING - KYC information has been collected from customer and is being processed. KYC Review will be in this state at most few seconds when processing is done automatically. It might take longer when manual review is required. For example, when provided document needs to be reviewed manually.

KYC Review status flow map