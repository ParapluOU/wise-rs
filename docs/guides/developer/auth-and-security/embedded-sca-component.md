# Embedded SCA componentCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/developer/auth-and-security/embedded-sca-component

---

Embedded SCA component
Copy

By default, your customers will now be expected to authenticate themselves by entering their Wise password as well as an OTP code sent to their phone.

If you prefer your customers to use a different authentication method, please reach out to your implementation manager.

JavaScript Library

We have created a library (available on npmjs.com that allows you to embed the Wise SCA flow into your application as a UI component, making it easier to integrate strong customer authentication into your application.

We have also provided also a sample app on GitHub showcasing how to use that library.

Below is a sequence diagram showing this flow.

Detailed workflow

Here is a step-by-step workflow with example commands to process a payment (which may vary slightly depending on the exact versions of utilities used).

Similar example is also available in our code samples repository.

Your customer makes a request which requires strong authentication. The call is made using our SCA helper library from your frontend.

SCA helper library
import { create, Mode } from '@transferwise/approve-api-action-helpers';
const request = create({ mode: Mode.PRODUCTION });
const res = await request('/tw-fund-payment', { method: 'POST', data: { ... } });

Your server forwards the request to Wise Platform API.

Request
  $ curl -i -X POST 'https://api.wise-sandbox.com/v3/profiles/{profileId}/transfers/{transferId}/payments' \
         -H 'Content-Type: application/json' \
         -H 'Authorization: Bearer <your api token>' \
         -d '{"type": "BALANCE"}'

The request is declined with HTTP status 403 / Forbidden with the following response headers

X-2FA-Approval-Result: REJECTED
X-2FA-Approval containing the one-time token (OTT) value which is what needs to be signed
Response
HTTP/1.1 403 Forbidden
Date: Fri, 03 Jan 2020 12:34:56 GMT
Content-Type: application/json;charset=UTF-8
x-2fa-approval-result: REJECTED
x-2fa-approval: be2f6579-9426-480b-9cb7-d8f1116cc8b9
...
{
    "timestamp": "2020-01-03T12:34:56.789+0000",
    "status": 403,
    "error": "Forbidden",
    "message": "You are forbidden to send this request",
    "path": "/v3/profiles/{profileId}/transfers/{transferId}/payments"
}

Your server reflects the response status and the header x-2fa-approval back to your frontend.

Our frontend library now understands that SCA is required and asks the customer to pass challenges (such as password).

When customer has completed challenges, our frontend library automatically triggers the initial call (1) again, but this time it includes the OTT (x-2fa-approval header) and sends it to your backend.

Request after implementing SCA
$ curl -i -X POST 'https://api.wise-sandbox.com/v3/profiles/{profileId}/transfers/{transferId}/payments' \
       -H 'Content-Type: application/json' \
       -H 'Authorization: Bearer <your api token>' \
       -H 'x-2fa-approval: be2f6579-9426-480b-9cb7-d8f1116cc8b9' \
       -d '{"type": "BALANCE"}'

Your server repeats the initial request above with the addition of the one-time token.

Response after implementing SCA
HTTP/1.1 200 OK
Date: Fri, 03 Jan 2020 12:34:56 GMT
Content-Type: application/json;charset=UTF-8
x-2fa-approval-result: APPROVED

{
  "type": "BALANCE",
  "status": "COMPLETED",
  "errorCode": null
}

Customer has authenticated and the action is completed. You will receive a response with x-2fa-approval-result status APPROVED in headers.

Using our API

In case you would love to provide a customized experience to your user. You can explore our SCA over API feature.

Do take note that not all verifications are available on the API.