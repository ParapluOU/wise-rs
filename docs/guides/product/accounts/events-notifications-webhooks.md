# Events Notifications & WebhooksCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/accounts/events-notifications-webhooks

---

Events Notifications & Webhooks
Copy

Certain webhooks should be subscribed to and listened for with relation to multi-currency accounts and related transactions on balance accounts.

Profile based subscriptions

The following two event types will update you on any changes or issues related with transfers and balance accounts for the profile. These are profile specific, and therefore need to be subscribed to on a profile basis.

transfers#active-cases - Monitors and alerts for any issues with a transfer.
balances#credit - Monitors and alerts if any credits are applied to a balance. Useful for when pay in may take time to complete or if being paid from another source.

It's recommended that subscriptions for these endpoints be included in your integration to monitor for transactions on the balance and if there are any issues with transfers.

Integration based subscriptions

The following two event types are available for the verification of profiles and for monitoring transfer state changes. These are supported at an integration level (subscribe once for the entire integration).

transfers#state-change - Monitor state change of a transfer and trigger updates in your UI or other user notifications.
profiles#verification-state-change - Monitors and notifies if the user/profile is not verified initially and becomes verified.
Full webhook reference

Please see the full Webhook & Notifications section to learn more.