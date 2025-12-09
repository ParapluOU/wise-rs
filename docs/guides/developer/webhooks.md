# Webhooks & NotificationsCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/developer/webhooks

---

Webhooks & Notifications
Copy

Listen for events to trigger actions

To keep your application informed about events within Wise, you can utilize webhooks. Webhooks are user-defined callbacks triggered by business events in Wise's system. For instance, a webhook callback can notify you about a change in the status of a transfer you've made.

In Wise, all webhook callbacks use HTTP POST requests. A webhook subscription allows a web application to receive notifications about specific business events.

Events will not contain any personally identifiable information.

We support the following subscription types:




Application webhook subscriptions enable you to receive notifications when events occur for resources created directly by your application.

Profile webhook subscriptions enable you to receive notifications when events occur for resources created under a specific Wise profile.

Application and profile webhooks will contain the same information, however creating, managing, and using them is done with different endpoints. Please make sure you are using the right endpoints to subscribe and manage webhooks.
Webhook Delivery

When a business event occurs and a webhook is created an HTTP POST request is prepared. The webhook is considered successfully processed only when a 200 response is received.

In Wise we have implemented HTTP request retries with exponential backoff with the following policy:

first retry with a 1-min delay
second retry with a 2-min delay
third retry with a 4-min delay
...
after the eleventh retry, there are 14 more retries once a day
Webhook URLs

Before proceeding, make sure the endpoint where you intend to receive webhooks satisfies the following requirements:

Has a valid domain name (IPs are disallowed)
Listens to HTTPS requests on port 443
Has a valid HTTPS certificate signed by a trusted Certificate Authority - CA (self-signed or expired certificates are not accepted)
Does not include any query arguments in the URL

https://webhooks.example.com/balance-change is a valid URL; http://webhooks.example.com:8080/hook.php?type=balance is not.

You can have multiple subscriptions per event type though be mindful you will receive duplicate callbacks, one for each subscription.

Creating Webhook Subscriptions

You can manage webhooks in two ways, depending on the scope:

Application-level webhooks: These subscriptions are managed through our API and allow you to receive notifications for events related to resources created by your application. For detailed instructions on creating, managing, and viewing these subscriptions, please refer to the full API reference on Webhooks.

Profile-level webhooks: You can manage these subscriptions through the developer tools page in your Wise account settings and also via our API, as detailed in the full API reference on Webhooks.