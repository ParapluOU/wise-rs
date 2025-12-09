# One Time TokenCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/developer/auth-and-security/one-time-token

---

One Time Token
Copy

Wise employs the One Time Token Framework, a unified verification framework designed to manage Strong Customer Authentication sessions and provide secure access to protected endpoints.

The One Time Token is a state holder for Strong Customer Authentication sessions, tied to a user's one-time session when performing an action. To validate the SCA session, multiple authentications are required.

The One Time Token stores information regarding the type of actions available, challenges that can be presented to the user, successfully verified challenges, validity duration of the token, and its status as ready for use.

Response Headers

In all Wise endpoints, whenever there is a need to enhance security, the endpoint will always return a response status of 403 and two custom headers specific to Wise.

Header	Description
x-2fa-approval-result	This header will only appear in response, and there are 2 values:
 APPROVED - The endpoint call is now authorised and you should be able to access the protected resource.
 REJECTED - Your request requires additional verification, please use the provided value in x-2fa-approval header.
x-2fa-approval	This header will appear in response when x-2fa-approval-result has the value of REJECTED.
Value of the header (OTT) will then be used to list the required challenges, identify and clear the challenge.
Attach this header to a protected endpoint to get the desired response.
Best practices
Understanding the One Time Token on how to determine a token is ready to be used.
Only trigger a phone based challenge (SMS/WhatsApp/Voice) when your end user selected the challenge.
Automate the SCA flow by always intercepting the known response headers.