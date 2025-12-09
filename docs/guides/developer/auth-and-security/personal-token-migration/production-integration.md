# Integrating in ProductionCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/developer/auth-and-security/personal-token-migration/production-integration

---

Integrating in Production
Copy
Switching From a Personal Token

If you already use client credentials, please follow the steps in already using client credentials.

Please ensure you have requested your client credentials for Production, and have shared the membership numbers (starting with a P, accessible via your Wise.com settings) with us.

We recommend keeping the logic you have that uses personal tokens (with mTLS disabled) in case of unforeseen issues.

Please follow the directions in Generating & Uploading a CSR.
Switch to the new flow in Production. If you encounter any issues, please turn on the personal token logic once again and reach out to us immediately. As a reminder, the personal token logic will be deprecated by the end of next year, once your migration is successful.

Some profile webhook subscriptions are not available as application subscriptions. To see the full list of available options, please refer to our Webhooks & Notifications documentation. If you're currently using personal tokens for any webhooks, you can continue to do so without any changes.

Already Using Client Credentials

If you are using personal tokens, please follow the steps in switching from a personal token.

Please ensure you have shared the membership numbers (starting with a P, accessible via your Wise.com settings) with us.

Once you've tested thoroughly in Sandbox, please follow the directions in Generating & Uploading a CSR.
Set up your integration to use mTLS in Production.
Switch the subdomain from api.wise.com to api-mtls.transferwise.com. If you encounter any issues, please roll back to api.wise.com and let us know immediately.
Once you've told us it's working, we will enforce mTLS, and all of your endpoints will need to use the new subdomain.
In order to remain compliant, we will enforce mTLS for all endpoints by the end of next year, so please ensure you have tested, integrated, and notified us before then.