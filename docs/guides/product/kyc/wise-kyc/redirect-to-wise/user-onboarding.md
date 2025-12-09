# User OnboardingCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/kyc/wise-kyc/redirect-to-wise/user-onboarding

---

User Onboarding
Copy

When having Wise complete the KYC/KYB for your customers, we require the account to be created in a similar fashion to wise.com. This generally means that we require your users to redirect to wise.com, complete the sign up process, and then redirect back to your application.

During that flow, the user will additionally authorize your application to access their newly created or linked account. The redirect will then include a code you can use to exchange via the API for access and refresh tokens.

At a high level, there are two flows that should be considered when relying on Wise KYC/KYB:

Linking an existing account
Creating a new account and authorizing

This portion of the guide explains these flows in detail.

Checking Account Existence

Accounts with Wise are unique on the email of the user. We expose an API that allows you to check if a user exists via the email they plan to use for their connection. Note that this email must be valid and verifiable, so please ensure it is a trusted and verified email.

Once you have checked the email against the User Existence API, you will be able to direct the user into the correct flow as outlined below.

Your application does not need to use the User Exists endpoint. Instead, you can have a single redirection to the combined sign up/login page instead. This can reduce complexity of your integration but isn't recommended if using our Onboarding API.

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

Creating a new account

Similar to linking to an existing account, there are five steps to creating and authorizing a new Wise account:

Your application redirects the user to the Wise authorization page, which prompts them to create a new account.
The user enters their email, password, and completes additional steps to create the user account and profiles for personal and business (if applicable).
The user agrees to give your application access to the Wise profile they created.
Wise redirects the user back to your back to your app.
Your app exchanges the authorization code for API tokens.
Note that step 4 and 5 are identical to linking an existing account and result in the same redirect, auth code, and exchange process.
Verification of New Profiles

Once a Wise user has connected their profile to your application, your system can start interacting with that profile's Wise Platform API resources. The Wise Platform API can be used to generate quotes for money transfers, set up recipients for payments, set up money transfers, and so on as described in our Product Guides and API Reference.

Once you acquire an API access token for a profile you can immediately start using the API on the customer's behalf. However, there are some considerations that are important to avoid problems when working with Wise profiles. This section specifically relates to the verification of profiles and the limitations when they are not verified.

In both of the flows outlined above, profiles could be either created or still unverified by Wise. Your application should check to ensure the profile is verified, and if not, ensure webhooks are subscribed so that notification of verification is received.
There may be a case when a Customer Account without a Profile is linked. When this happens, reject linking the account and show an error message to the user.
Verification status

Wise is obliged to verify the identity of personal and business profiles as part of our Know Your Customer policy. We must be satisfied that we know our customers well enough before conducting financial transactions with them.

Verification involves the Wise customer submitting various forms of evidence to us for our consideration. Evidence can include official documents, for example. We may accept or reject such evidence, or consider some evidence as being superior to other forms of evidence.

What evidence we have verified for a profile can affect how successful or how quickly money transfers can be made. For certain values of transfer, or certain currency routes, we may require a certain level of verification. If we do not consider that a profile has been sufficient verified for a particular money transfer, we might delay the processing of the transfer while we seek further information from the customer.

For more information on verification: How does Wise verify my identity?

If you intend to use the Wise Platform API to set up money transfers, then it is desirable for you to know whether such verification issues might delay or prevent transfers from succeeding. This is particularly relevant for profiles that have just been created as part of linking your application to Wise: new profiles may not yet be verified, and money transfers may be delayed while this process happens, leading to a bad experience for our customers.

Wise exposes the verification status of a profile to help connected applications understand when money transfers are able to be created without such verification issues (but see below for an important caveat).

Verification status is a simple indicator of the current level of verification that has been made for a profile to some particular level of confidence: a level that is likely to allow money transfers to occur without being delayed by verification-related issues.

Verification status is exposed as a property of a profile: i.e. a profile is verified or not verified. It is useful to have such a property as a "gate" in the connected application's system that prevents transfers from being created before they are likely to be processed without delays.

Connected applications can query the verification status of a profile using the Wise Platform API. Wise can also asynchronously notify connected applications about the change in a profile's verification status using a webhook notification.

Although we expose verification status as a profile-level property, there are implicit limits to which any "verified" status applies. A "verified" status applies to some reasonable limits that customers should not normally reach. However, it is possible for individual transfers to require additional verification to be performed, even after we have given the profile a verification status of "verified". Therefore, connected applications should be prepared for verification issues to affect money transfers regardless of the reported "verification status", although this is not generally expected.
Verification status integration strategy

The following strategy may be useful for your integration with respect to profile verification:

At some point before going live to your customers, you set up an "application" webhook subscription for verification status change events. This is a single subscription that will allow you to receive verification status events related to any profile that links to your application in the future. (See here for more information.)
After going live, a user connects their profile to your application using the OAuth authorization flow. (Note that the profile might be an existing, verified profile, or a brand new, unverified profile).
During the authorization flow, your system receives an authorization code and Wise profile ID in a request to your callback endpoint (see User authorization). Once API tokens have been acquired using the authorization code, you now have API tokens and the related profile ID: you can now access the profile's API resources.
The verification status of the profile is queried using the Wise Platform API. If the response indicates the profile is "verified", you can proceed with transfer operations. If the response indicates "not verified", then transfers are likely to be delayed: you should wait to receive a webhook notification for the customer. However in some cases, users might be allowed to make payments up to a certain limit even though their profile is "not_verified" (see "Bank Transfer Verification Check Result" below).
If a profile is unverified, the verification process takes place. Later, when verification completes successfully, you will receive a webhook notification informing you that the profile has become verified: you may then allow transfer creation. (Alternatively, you can continue to poll the API again, but this is not recommended.)
Checking verification status using the API

You can check the verification status of a profile using the Check Profile Verification API. This is a profile-specific API resource which should be accessed using an access token acquired for the profile.

Verification State Change webhook

You can subscribe to profiles#verification-state-change event to be notified when the user is ready to use Wise.