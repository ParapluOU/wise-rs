# Partner SupportCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/partner

---

Partner Support
Copy

Interacting with our support service

During the normal operation of integrations, either our partners or Wise will require additional information or have questions. To facilitate this in an efficient way, we offer multiple ways to interact with our partner support system.

Partner support is provided via a standard case system, similar to common support ticketing software like Zendesk, JIRA Service or Twillio. When requesting additional information, a new case will be opened and actioned by the appropriate stakeholders until it is considered resolved.

It's important to understand the benefits and costs of each of the below methods. Wise currently offers partner support and operations via email and API.

Depending on the type of your integration and requirements, your implementation manager will recommend the best option for your needs.

Contact Us

If you are thinking about or want to learn more about a direct integration partnership with Wise, head to Wise Platform to find out more.

For technical questions about the Wise API, contact api@wise.com. To help speed up resolution, please provide an "x-trace-id" response header value, along with other relevant request details like environment, request url. Please note the x-trace-id is only useful for 7 days in the Sandbox environment, and 10 days in Production.

For support with our Open Banking APIs, head to our Open Banking Guide.

Wise API Status
Keep your Client Secret Safe

If you're experiencing issues calling our API endpoints, the first place to check is status.wise.com.

You can also integrate with the Wise Status Page over API. Learn more.

Partner Support via Email

Creating cases via email is a standard method of integrating your support processes with Wise. We will issue you a specific email address to use and then all support cases should be sent there.

Responses will be sent to the sender email, but we can also designate a cc email for all cases and updates as well.

As all responses are delivered via email, we cannot guarantee the structure or how it will be represented by email clients. We recommend that if you plan to automate or otherwise rely on the structure of the support emails, that you use the API method to integrate instead.

Partner Support via API

Creating and managing cases via the Partner Support APIs allow for partners to connect directly in a structured and more secure way to our partner support system.

The Partner Support APIs are currently in a closed Beta and subject to change. Please speak with your implementation manager if you would like to integrate with these APIs

Frequently Asked Questions
How do I get a Client ID and Client Secret?

We only provide a Client ID and Client Secret for Wise Platform partner integrations. You can still connect with your own Wise account over API though, personal API tokens will help you achieve this.

Can I use Wise API to receive money in my online checkout?

The main use for the Wise API is to automate payouts. We don't offer the option to build Wise into your checkout flow as a payment option to receive money.

The good news is that you can use the local bank details that come with your Wise account (USD, EUR, GBP, AUD, NZD and PLN) and reconcile the incoming payments via the API.

How do I create a payment link on the API?

Unfortunately this isn't a feature that is supported on our Public API. Please continue to create these links on the Wise.com interface.

How can I start testing the API?

You'll need to create a new account in Wise Sandbox to get started testing the API - your actual Wise account won't work. Also, logging in or registering with your Facebook or Google account is not supported.

Does it cost anything to use the Wise API?

There aren't any additional charges for using Wise API with a personal token - the regular pricing system adopted by Wise for customers is applied. You can find out more about Wise account features and pricing here.

What currencies can I send to and from?

You can find more info on which currencies you can send to and from here; and which currencies you can add, keep, and receive into your Wise account here.

I have a question about my Wise account, can you help?

Head to the Wise Help Centre to get your question to the right team.