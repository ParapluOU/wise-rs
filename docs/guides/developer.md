# Getting Started with Wise APICopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/developer

---

Getting Started with Wise API
Copy

This guide provides the essential information you need to begin integrating with the Wise API. Take a look at each section to make sure you know how to get up and running quickly.

API access

Once our partnership begins, we'll send you API access credentials for the sandbox environment, consisting of a Client ID and a Client Secret. These credentials are required for making authenticated calls to the Wise API.

Keep your Client Secret Safe

The Client Secret is highly sensitive data that can impersonate your institution on the Wise Platform API. Handle it with utmost care, limit access, and store it securely.

Personal tokens are not meant to be used for partner integrations and cannot be used to access certain endpoints. If in doubt, please contact your CSM to ensure you are using the correct token type.

Environments

Wise provides different environments for development, testing, and production. Understanding these environments is crucial for a smooth integration process.




Sandbox: This is your initial playground. Use the sandbox environment for development and testing your integration without affecting live data or customer funds.




Production: Once your integration is thoroughly tested and approved, you'll move to the production environment for live operations.

Environment	URL
Sandbox	https://api.wise-sandbox.com
Production	https://api.wise.com

Learn more about Environments

Security best practices

Security is critical when working with financial APIs. Beyond protecting your Client Secret, consider these fundamental best practices:





Mutual TLS (mTLS): Implement mTLS for enhanced security, ensuring both client and server authenticate each other. This provides a strong layer of trust for your API communications.




Secure Credential Storage: Always store your API credentials, especially the Client Secret, in a secure, encrypted manner. Avoid hardcoding them directly into your application code.

Next steps

Now you have the foundational knowledge to begin your integration. Here's what to do next:

Dive straight into the specific integration guide for your use case. These guides provide step-by-step instructions and code examples for our most popular integration models.
Explore the API Reference: Look closer at the Wise Platform API with our comprehensive API Reference.
Webhooks: Understand how to use webhooks to receive real-time notifications about events, which is crucial for handling asynchronous processes.
Error Handling: Design error handling into your application to manage API responses and ensure a smooth user experience.
Versioning: Familiarize yourself with the API versioning strategy to ensure your integration remains compatible with future updates.