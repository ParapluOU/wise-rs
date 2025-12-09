# Accessing Customer AccountsCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/kyc/partner-kyc/accessing-accounts

---

Accessing Customer Accounts
Copy

In order to perform actions on behalf of a customer you need to create a new or link an existing Wise account.

Create a new account

If a user doesn't already have a Wise account then you can create one for them. The signup with registration code feature lets you create new users directly via an API call. You will send via API all of the data Wise needs to serve these users in your region, meaning users have their accounts created without ever leaving your banking app, making a very streamlined flow.

You will define a registration_code for the user that act similarly to a password, although is limited in scope to be used only by your integration over the API. You exchange this value for user tokens, as described in the detailed documentation above. This code can be used to regenerate tokens should they become invalid, so you should save it in your database to allow this. Due to the password-like nature of this data we recommend to store it encrypted at rest for security.

Below is a sequence diagram showing this flow.

If you attempt to create a user that already has a Wise account they will always need to be redirected to the account linking flow, you can detect this at the point you attempt to create the user based on the API response of 409 conflict. See the detailed guide under the endpoint documentation for more details.

Verification Documents

In certain integrations, we may require additional documentation regardless of our ability to rely on the partners KYC information. For example, in Brazil, we require the CPF value for a profile to allow us to report to the central bank in Brazil.

If you are creating accounts and a verification document is required, please see the Create an identification document for a profile API reference to pass the correct type and uniqueIdentifier.

We can provide this option to banks where we can create a trusted reliance or outsourcing model on your KYC processes.
New users have to accept the Wise Terms and Conditions as part of their sign-up process, which should be available for them to read as a link to our website.
Linking an existing account

In order to perform actions on behalf of an existing Wise customer the user has to login and give consent. We then redirect them back to your app.

The standard website redirection flow is as follows:

Your app redirects the user to Wise authorization web page.
The user logs in to Wise.
The user agrees to give your application access to one of their Wise profiles.
Wise redirects the user back to your back to your app.
Your app exchanges the authorization code for API tokens.

These steps are explained in more detail below.

You can also have a look at the code sample of how to implement that flow.

Example of the redirect flow. Source code.

1. Your app redirects user to Wise authorization page

Your website or app opens Wise authorization page in the user's browser.

On mobiles apps you should not use legacy WebView components to show the authorization page to the users because they are not secure and will not allow users to log in to Wise with Google, which is an option used by many of our users. Your app should instead open the device's full browser app or use the newer iOS and Android features to show isolated web pages within apps in a secure way, namely a SafariViewController or a ChromeCustomTab, respectively.

List of available parameters
client_id (required)string

The client ID you received from us.

redirect_uri (required)string

The pre-configured url in your application where users will be sent after authorization.

statestring

An opaque value, used for security purposes. If this parameter is set in the request, then it is returned to the application as part of the redirect url. More about state parameter.

Sandbox authorization page URL

https://wise-sandbox.com/oauth/authorize/?client_id=yourapp&redirect_uri=https://www.yourapp.com

Production authorization page URL

https://wise.com/oauth/authorize/?client_id=yourapp&redirect_uri=https://www.yourapp.com

The `redirect_uri` has to be preconfigured before it can be used. The value used should be equal to the value preconfigured, including the query parameters.
2. The user logs in to Wise

Our usual log in screens are presented to the user if they are not already logged in on the browser being used. If enabled for a user they will also be prompted to go through our two-factor authentication procedure.

3. The user creates or selects a profile

The Wise user will create or select a profile that will be linked with your application. Wise profiles are the personal or business entities which will be involved in money transfers and other Wise activities.

Once the authorization process is complete, your app will be authorized to perform actions on the selected profile. Access to other profiles associated with the Wise user must be separately authorized.

4. The user agrees to grant access and we forward them to your
redirect_url

Once a user gives your application authorization to connect to Wise and access their data, they will be redirected back to your redirect URL with some additional query parameters.

List of query parameters
codestring

Authorization code. Your website or service can use it to generate a User Token.

statestring

Any state parameter you initially passed to us when initiating the flow.

profileIdstring

The profile ID that the Wise user granted you access to.

If you are building your Wise integration as a native mobile phone app then the redirect URL should be able to handle returning the user to the correct place in the app, using a "deep link" based on a custom URL scheme defined by your mobile app.

Wise redirects back to

https://www.yourapp.com/?code=ABCDEF&profileId=30000000

5. Your app exchanges the authorization code for API tokens

Your website or service can use the authorization code to obtain API tokens to act on behalf of the user account as described in the generate a User Token section.

The auth code provided in the parameter is one time use only expires within 30 minutes.
Error handling

When errors occur during authorization request handling, we display error details on our web pages. The user may also see a link back to your application, with error and error_description parameters in the URL instead of code. Your callback URL handler should be prepared to accept such "error" requests.

Actions after linking

Upon linking to an existing Wise account you need to ensure that you have connected to an account that represents the same natural person or business. To do this for personal profiles please check the date of birth of the connected wise profile matched the date of birth you hold for that customer in your platform. For businesses the comparison required changes with the region you are servicing, please discuss the best approach with your implementation team.

Email flow

We also offer an alternative way to to retrieve an authorization code – by us sending an email to the end user with a link to login to Wise and the user then manually typing a code in to your app.

The redirection flow is always preferred as it requires no manual user actions. In some cases it is not possible to securely open the Wise website within your app, in which case the email flow can be used. Before using the email flow please discuss the implications with your implementation teams.

In the event of a 409 response(the user already has a Wise account) the flow should be as follows:

Wise emails the customer a link to log in and generate an authorization code.
The user opens their email app and clicks the link in the email. They login to their Wise account, grant access, and are presented the code.
The user can then enter the authorization code in to your app.
Your app can then use this code to generate a User Token.
The flow is more cumbersome for a user, this should only be used as a last resort. To enable this flow, you must send the **sendAuthorizationCodeAsEmail** parameter as true when creating a user