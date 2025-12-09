# Strong Customer Authentication & 2FACopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/developer/auth-and-security/sca-and-2fa

---

Strong Customer Authentication & 2FA
Copy

Strong customer authentication (SCA) is a security measure that requires customers to provide multiple forms of identification to verify their identity before accessing sensitive information or performing high-risk transactions. The term SCA is essentially a form of MFA designed for the financial industry. The term MFA, which stands for Multi-Factor Authentication, is a broader technical concept whereas SCA defines the business feature.

The goal of SCA is to prevent fraud and protect sensitive information by ensuring that only authorized users can access it.

European Regulatory Requirement

Strong customer authentication (SCA) is a European regulatory requirement as part of the second Payment Services Directive (PSD2) for authenticating online payments and making them more secure. There are some actions such as funding a transfer from your multi-currency account or retrieving a statement that require SCA in the UK and EEA.

Please note that Wise may enforce SCA on endpoints based on our risk assessment as part of our best efforts for consumer protection.

SCA Protected Endpoints

In Wise, when an endpoint performs an action that requires strong customer authentication, the initial request to that endpoint will be rejected with a response status of 403 (Forbidden) to inform that stronger authentication is needed. The endpoint may be retried again with a stronger authentication.

To know if an endpoint requires a stronger authentication, please look out for a notice box when checking out our API Reference doc.

For example, the notice box would like this:

This endpoint is SCA protected when it applies. If your profile is registered within the UK and/or EEA, SCA most likely applies to you. Please read more about implementing SCA below.

For more information on how to build a stronger authentication request call, please refer to our One Time Token section.

Connection Guide

Depending on your connection type, a different type of integration is required. The first step is to identify the kind of payment flow you are building so you can select an appropriate integration path.

Connection Type	Recommended Guide
Customer Accounts with Partner KYC	SCA over API
Customer Accounts with Wise KYC	Embedded SCA component
Partner Account	Contact our sales team
Open Banking	Open Banking Guide

If in doubt, please contact our support team to advise on the most appropriate solution for your use case.