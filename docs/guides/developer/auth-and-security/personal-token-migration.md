# API Migration Guide: Moving to OAuth 2.0 and mTLSCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/developer/auth-and-security/personal-token-migration

---

API Migration Guide: Moving to OAuth 2.0 and mTLS
Copy

The purpose of this guide is to help you seamlessly transition from personal token authentication to the more secure OAuth 2.0 with mTLS.

This guide is intended for Wise partners who are the direct customer of Wise by transacting on their own account (First Party).

Key Differences between Personal token and OAuth+mTLS Authentication:

Personal token is replaced with a set of Refresh and Access Tokens
Token expiration and refresh mechanism

To understand the basics of OAuth 2.0, have a look at our Security & Access guide.

To understand the basics mTLS, have a look at our Enhanced Security guide.

In depth resources:

OAuth 2.0
Mutual TLS
Migration Steps
Please execute the steps in the order they are listed in this guide and make sure to verify a step is successfully completed before moving to the next one.

We recommend you do the migration in our Sandbox test environment first, and only move to Production once you have verified that all business flows are working.

Integrate and test in Sandbox
Integrate and test in Production