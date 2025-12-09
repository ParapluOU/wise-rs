# mTLS for webhook notificationsCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/developer/auth-and-security/mtls-for-webhooks

---

mTLS for webhook notifications
Copy

Receiving events in the mTLS webhook endpoint.

For additional security, you can add mTLS certification to webhook notifications you receive from Wise Platform.

To do this, you need to import a Certificate Authority chain, provided by Wise, to your webhook server.

The setup process depends on your server infrastructure. Typically, the CA chain can be imported to the API gateway.

Core concepts and overview of steps

To secure your webhook notifications with mTLS, you must first import a Certificate Authority (CA) chain from Wise into your webhook server. Once the setup is complete, you can use the API to manage your mTLS-enabled webhook subscriptions.

To add mTLS certification to webhooks:

Create Subscription: Send a POST request to the subscriptions endpoint with mtls_enabled set to true.
name (string): A name for the subscription.
trigger_on (string): The event you want to be notified about (e.g., transfers#state-change).
delivery (object): Contains details about the notification delivery.
url (string): Your webhook endpoint URL.
mtls_enabled (boolean): Must be set to true to enable mTLS.
Verify Setup: After creating the subscription, you can test your mTLS configuration by sending a test webhook notification to your server. This is done with a POST request to the subscriptions/{{subscriptionId}}/test-notifications endpoint.
Create a new subscription with mTLS enabled

Use the following sample curl request in the sandbox environment to create a new subscription with mTLS enabled.

curl -X POST https://api.wise-sandbox.com/v3/applications/{{clientId}}/subscriptions \ 
     -H 'Authorization: Bearer <your api token>' \
     -d '{ 
         "name": "Webhook Subscription #1", 
         "trigger_on": "transfers#state-change", 
         "delivery": { 
             "version": "2.0.0", 
             "url": "{{yourWebhookUrl}}", 
             "mtls_enabled": true 
         }
     }'
Verify the mTLS by sending a test webhook event to your webhook server

Use the following sample curl request in the sandbox environment to verify the mTLS setup by sending a test webhook event to your webhook server. Specify the subscriptionId from POST /v3/applications/{{clientKey}}/subscriptions.

curl -X POST https://api.wise-sandbox.com/v3/applications/{{clientId}}/subscriptions/{{subscriptionId}}/test-notifications \ 
     -H 'Authorization: Bearer <your api token>'