# Customer Accounts with Wise KYCCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/kyc/wise-kyc/redirect-to-wise

---

Customer Accounts with Wise KYC
Copy

This guide is intended for Wise Platform partners who are not licensed and able to perform know-your-customer or know-your-business (KYC or KYB) checks in-house or where a reliance model (see note) is not possible. If that is the case then Wise will have to conduct the KYC/KYB checks for you.

The following pages will walk you through the steps to build a journey which redirects your end users to our standard wise.com onboarding flow. Users should have the ability to create a profile, get verified and then be redirected back to your own app.

Alternatively, if you already store some customer KYC/KYB data, we also provide a UI flow with API pre-upload. This means we can allow you to create user profiles and submit data on behalf of your customers to Wise over our API. Users will still need to be redirected to Wise.com to validate and/or supplement the pre-filled data, as well as to complete their verification and onboarding. However, having some of the details pre-populated for them can lead to an improved user experience and also means that more of the user journey can happen within your own UI. This feature is particularly useful for business customers where more data points are required (e.g. UBO and director information).

Wise KYC Postman Collection

We've created a Postman collection to help you test your integrations easily! It includes:

Our sandbox environment (https://api.wise-sandbox.com)
Common API calls for creating users and profiles
Automated tests that copy response data between calls
Example responses for reference

Fork the collection to get updates automatically. This lets you test the complete flow with minimal setup!

Fork in Postman
Mobile Device Rendering

On iOS devices, you might need to set some WKWebViewConfiguration configurations to initialize the user's device camera. There are points in the Wise KYC onboarding which will require the user to upload or take a photo.

Enable (set to true) inline media playback allowsInlineMediaPlayback | Apple Developer Documentation
Ensure the user does not have to do any action to start playback by setting this value mediaTypesRequiringUserAction to none or an empty array in swift.

These properties are on the configuration WKWebViewConfiguration | Apple Developer Documentation you set on the WKWebView.

Another note, there seems to be a bug on the WKWebView which means for these settings to apply you must pass them into the WKWebView initialiser. Simply modifying the configuration by accessing the web views property doesn’t seem to apply the configuration properly and won’t work.

On Android devices, you might need to enable camera permissions for the web-view even if the app has the user-permissions. To do this, add the following block of code where the web view is initialised.

webView.webChromeClient = object : WebChromeClient() {
    override fun onPermissionRequest(request: PermissionRequest) {
        request.grant(request.resources)
    }
}

Note, that request.grant() would provide any available permission for the app to the web view. In case the camera permission is denied by the user, they would see an error page. You do have the option to initiate the permission sequence in this callback and go back to the flow if required.

Building Your Backend

You will build your Wise user experience directly into your mobile and desktop applications, and will build a backend service to support the features it offers. Your user interface should never directly call any authenticated Wise endpoint which requires an API token, this should always be done by your backend system.

Depending on the features and settlement model you will build, there are some different components you will need to build. Please ask your Wise implementation team for advice based on the requirements of your integration.

You should expose an API internally for your web and mobile clients to call to provide the required Wise features. Your backend system will manage both communication to the Wise Platform API and internal operations such as querying user KYC data to send to Wise, checking a user has sufficient funds to make the requested transfer and triggering the payment of funds to Wise when a user confirms a transfer.

You should also store a copy of certain data relating to Wise to decrease latency and increase resiliency when users review previous transfers they have made or recipients they sent funds to. The extent of what you store will depend on your integration, but we recommend to store at a minimum:

Quotes that have been used to create transfers
Transfer records including ID and status
Recipient IDs, names and account summary data

The goal is to store locally in your platform all the data you need to drive your UI, such that calling our API is not required when reviewing historic data.

You should subscribe to our webhooks to keep this data up to date.

Edge Cases

This section discusses some edge cases that you should test and handle before going live with your integration.

Email changed at Wise

If a user changes their email address, all tokens to the user account are revoked. In this case the bank will receive a 403 when attempting to generate an access_token and as such should follow the same process as described in the "Token Expiry" section below and start the sign-up flow from the beginning.

In this case, if the user has changed their email address at Wise, it is possible the user will end up with a new Wise account using their old email address still held by the bank, or they might link their bank account to a different already existing Wise account under the old email address.

Token Expiry

It is also possible that a user's refresh token will become invalid. This could happen for a number of reasons, for example:

The refresh token's validity period expires (usually set at 10 years or more)
The user revokes the access of your application to their account.
The user enables enhanced security on their Wise account.
Wise revokes a token due to a suspected security breach of the token or your client secret.

Due to this possibility your application should handle the scenario where you fail to generate a new access token from the refresh token. Correctly handling this depends on how you originally gained access to the user.

An existing user granted your application access to the account

If you were granted access by an existing user then you should send the user through the same flow as you initially did to generate tokens described in "Connecting to an Existing Wise account" below. You will then have new access and refresh tokens generated which you can now store and use as before.

Connecting existing accounts

When you are submitting an email which already exists amongst our users then an attempt to create a new one will fail. In that case you should follow the standard website redirection flow.

Going live checklist
1. Make your integration bulletproof
Implement basic retry mechanism to handle potential failures or network interruptions
Implement duplicate prevention mechanism to avoid duplicate payments. Verify that UUID is uniquely generated for each individual payment and its value is kept same in case of retrying.
Implement basic logging to help out in debugging and problem solving, if needed.
Check that you can handle all possible transfer states during polling of transfer info.
Handle the potential issues described in Edge Case Handling above
Required data fields for recipients and transfers vary for different currencies. Please explore Recipient Account Requirements and Transfer Requirements
Some good recipient currencies to test are:
CAD - has several fields in a field group.
USD - the country field has refreshRequirementsOnChange.
JPY - the bank field has refreshRequirementsOnChange.
KRW - has a field using a date component type.
2. Set up security for LIVE environment
Make sure you have received and successfully decrypted Live API credentials, storing them securely.
Ensure access tokens and refresh tokens are also stored securely and only exposed to authorized persons.
Make sure your server has TLS version 1.2 or higher.
Implement a mechanism to obtain new access token upon expiration.
3. Do some testing in LIVE
Launch LIVE integration to a limited set of your customers and test all currency routes you will offer end-to-end.
We recommend launching a limited set of currencies initially to limit the scope of potential issues, please seek guidance from the Wise team.
Test successful flow and bounce back flow (where funds cannot be delivered).
All set. Switch it on.
4. Monitor API Status
You can always track our API status here.
User Experience

There are different ways to build your user experience, especially when it comes to the sequence of steps in the payment flow, but we have a recommended order. We strongly recommend you follow this flow as it has been tuned by the Wise team to be the simplest to understand for the customer and the easiest to build using the APIs defined below.

We are happy to help you to design and build a great experience for your customers using our experience so please don't hesitate to get in touch if you need advice, especially if you want to deviate from the recommended flow.

Your Wise user experience

There are two main user flows that must be built in order to integrate with Wise.

The user onboarding flow

There are two ways to access the Wise Platform API depending on if you customer already has a Wise account or not. You should handle both cases in your integration.

Flowchart

This flowchart describes the different scenarios you will encounter and how you should handle them.

You need to go through this flow only once for each customer before they can set up their first transfer.

Upon linking to an existing Wise account you need to ensure that you have connected to an account that represents the same natural person or business of the bank account in your platform. To do this for personal profiles please check the date of birth of the connected Wise profile matched the date of birth you hold for that customer in your platform. For businesses the comparison required changes with the region you are servicing, please discuss the best approach with your implementation team.