# Updating ProfilesCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/send-money/use-cases/embedded/updating-profiles

---

Updating Profiles
Copy

As part of our compliance processes, we require annual updates of the details stored in the personal and business profiles.

If any customer details have changed (such as a change of address, name change due to marriage, etc) then the profile information at Wise must be updated. In order to fulfil the profile update you must always open and close a profile "update window" to inform us that a profile update is underway and then completed.

If the customer details remain the same then you must still open and close an update window to inform us that nothing has changed.

How to implement

The process requires a valid API access token.

A common strategy is to collect and send this data when a customer next interacts with the Wise integration in your product.

We recommend recording the most recent date each personal/business profile was last updated, you can use this date to check whether it's been more than one year since the profile was last updated. If it has been over a year, you must perform a profile update using the steps described below.

Below describes each journey:

No updates required for profile

Open update window: Open the update window for profile updates.

Close update window: Close the update window for profile updates.

Do keep in mind that we require the update window to be opened and closed even if there were no changes to the profiles. This will essentially let us know that nothing has changed for the profile and that the information is up to date.

Updates to Personal profile

Open update window: Open the update window for profile updates.

Update personal profile: Update the personal details including the address.

Close update window: Close the update window for profile updates.

Updates to Business profile

Open update window: Open the update window for profile updates.

Update business profile - general data: Update the business details.

Update business profile - address data: Update the address of the business.

Update business profile - directors data: Update directors of the business.

Update business profile - UBO data: Update ultimate business owners of the business.

Close update window: Close the update window for profile updates.