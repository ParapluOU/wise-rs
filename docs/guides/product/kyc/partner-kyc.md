# Customer Accounts with Partner KYCCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/kyc/partner-kyc

---

Customer Accounts with Partner KYC
Copy

This guide is intended for Wise Platform partners who are qualified and would prefer to perform their own know-your-customer (KYC) checks. Under this model, Wise Platform will outsource the customer verification process to you. This helps provide a more seamless experience as customers don’t have to go through the onboarding process and submit information they have already provided.

To allow for this, the below requirements and preconditions must be met. If any of the preconditions change, Wise Platform must be notified in a timely manner.

You are a regulated entity.
You perform customer due diligence (“CDD” and “KYC”) and CDD record keeping in compliance with your verification procedure.
You have regular internal quality assurance processes regarding your AML/KYC to check procedures are compliant with the AML/CTF Program and have been adhered to. This needs to happen at least annually.
You have an appointed AML/CTF Compliance Officer.
You perform on-going ML/TF risk awareness training and training on CDD and record keeping for employees.
You don’t allow customers who are onboarded through simplified due diligence or a lower KYC standard than the Wise AML/CTF Program to access Wise through your app.

You need to let us know if you rely on a 3rd party or other Financial Institutions to perform customer identification or verification. This is known as a reliance model and would mean our Compliance would need to audit both your business and that of the 3rd party conducting the checks.

Postman Collection

We've created a Postman collection to help you test your integrations easily! It includes:

Our sandbox environment (https://api.wise-sandbox.com)
Common API calls for creating users and profiles
Automated tests that copy response data between calls
Example responses for reference

Fork the collection to get updates automatically. This lets you test the complete flow with minimal setup!

Authentication & Access

Wise supports two ways to authenticate to the Wise Platform API.

Use a personal API token if you’re a small business user automating your own Wise account.
Use OAuth 2.0 if you’re a partner building for end customers or a large enterprise
Personal API token (Small Business Users)

A personal token authenticates requests for a single Wise.com user and has limited API access compared to OAuth credentials.

To generate an API token go to Wise.com select your business profile then go to Your Account > Integrations and Tools > API tokens > Add new Token (2‑step login is required).

You can then pass this token with Authorization: Bearer token in your API calls.

Important personal token limitations

Some endpoints and actions aren’t available with personal tokens. EU/UK: Due to PSD2, you cannot fund transfers or view balance statements via API with a personal token.

If in doubt, contact your CSM to confirm the correct auth type.

OAuth 2.0 (Partners & Enterprises)

Wise uses standard OAuth 2.0 protocol for partner authentication and authorisation.

Once our partnership begins, we’ll send you API access credentials for the sandbox environment consisting of a Client ID and a Client Secret. The credentials are needed to either create users over API or complete the authorization_code OAuth 2.0 grant type through which the customer will allow your application access to their account.

Keep your Client Secret Safe

The Client Secret is highly sensitive data that can impersonate your institution on the Wise Platform API. Handle it with utmost care, limit access, and store it securely.

Need a refresher on the protocol? See: https://auth0.com/docs/protocols/oauth2 Not sure which method to use? Contact your CSM.

We also need redirect_url from your technical team which is used to forward users to after successfully granting your application access to their Wise account. Specifying this explicitly makes the integration more secure. This article about OAuth 2.0 framework is a great way to refresh your knowledge about the protocol itself

Environments

We provide two distinct environments, Production and Sandbox, to allow developers to develop, test, and deploy their applications.

For each environment we offer an endpoint that requires mutual TLS (mTLS) and a separate endpoint for non-mTLS. For most situations we recommend to use mutual TLS, which requires some additional setup.

Production Environment is the live environment that developers can use to send real-time requests to the Wise Platform API. This environment is intended to be used in the final stages of development when the application is ready for deployment.

Sandbox Environment is the testing environment that developers can use to build and test the application. This environment allows developers to make API requests without affecting the production environment. It is ideal for testing API requests, because it is similar (but not identical) to the production environment.

The Sandbox environment is designed for testing and does not support real money transfers, actual financial controls, or all currency routes available in our Production environment. Endpoints will also have more latency in Sandbox than in Production.

We provide a Simulation API to test money movement flows and other features, supporting the testing of webhooks and their payloads, which otherwise would not be possible.

Sandbox is a test environment. Therefore, only test data should be included in this environment. Do not include real data in sandbox when building your integration, as the data is not subject to the same access controls as our production environment.

Sandbox API endpoint

https://api-mtls.wise-sandbox.com (mTLS enabled)

https://api.wise-sandbox.com (TLS only)

Production API endpoint

https://api-mtls.transferwise.com (mTLS enabled)

https://api.wise.com (TLS only)

Sandbox UI

https://wise-sandbox.com

What you can test in Sandbox

There are some differences between the Production and Sandbox environments. If you are a partner and you run into issues testing in Sandbox, please fall back on the following features, countries, and currencies, as these are most stable. If there is an issue with one of the listed flows that is reported by a partner, we will open an internal incident and prioritise fixing it.

Please note that your implementation team will give you a test plan during your implementation if you are a partner, and you will need to adhere to that. This is simply a list of what is and isn't available, for reference.

Sandbox does not support load or performance testing.

Supported Regions and Currencies in Sandbox

If you are testing a different currency or region in Sandbox and it does not work, please fall back on the following currencies and regions:

-	Personal	Business
Regions	UK	UK (Sole Trader)
Currencies	GBP, USD, EUR	GBP, USD, EUR

The currencies and regions apply to all the following features. Please reach out to us if there is a problem in these regions or currencies, and fall back on these if another region or currency you are testing does not work.

The following sections outline the functionality you can test in Sandbox if you are a partner.

Access the API
Receive API client credentials from our integration success team.
Obtain a client credentials token for your application.
Obtain access/refresh tokens from one of our onboarding flows.
Refresh an access token from a refresh token.
Update keys and certificates for testing mTLS, JWS, and JWE.
Onboard Customers to Sandbox
Create personal and business profiles.
Create multiple business profiles.
Check that a user exists over API.
Create a new user over API.
Create a new Wise account in the UI and link it via OAuth UI.
Disconnect and reconnect Wise accounts.
Create a new Wise account via API and claim it with a registration code.
Simulate the verification of one of the supported profile types.
Receive a verification status completed webhook event.
Send Money
Create fixed target and fixed source quotes.
Get realistic but not live rates.
Create a recipient over the API.
Create a quote over the API.
Request transfer requirements for the above.
Create a transfer from the above recipient and quote.
Create a third-party transfer from the above recipient and quote.
Create a partner license transfer from the above recipient and quote.
Create a batch payment from the above recipients and quotes.
Initiate the fund of a transfer with balance or bulk settlement.
Subscribe to and receive transfer state change webhook events.
Simulate transfer state changes.
Hold Money
Create a balance for a supported currency.
Convert balance amounts between supported currencies.
Receive Money into a Balance
Simulate a top up of a balance for a supported currency.
Subscribe to and receive a balance update webhook event.
Retrieve a statement with a simulated top-up present.
See sent and received payments in a statement.
See balances updated with received transfers.
Receive Money via the SWIFT network
Simulate an MT103 message.
Subscribe to and receive a balance update webhook event.
Retreive a statement with a simulated top-up present.
Order Cards
Check card order availability in supported regions.
Create a card order.
Set a card PIN.
Get card order requirements.
Retreive a list of card orders.
Subscribe to and receive card order state change webhook events.
Manage Cards
Retrieve a list of cards.
Change the status of a card.
Retrieve sensitive details of a card (PAN, PIN, CVV).
Subscribe to and receive card state change webhook events.
Manage Spending Controls
Add and delete spending limits of the card per card.
Manage spending limits of the card per profile.
Add spending controls (MCC and currency) to all cards of the application.
Test Card Transactions
Retreve a card transaction.
Subscribe to and receive card transaction state change webhook events.
Simulate a card transaction authorization and clearing for Visa.
Simulate a card transaction authorization for Mastercard.
Test Card Transaction Disputes
Submit a dispute.
Retreive a dispute status.
Withdraw a dispute.
Subscribe to and receive dispute state change webhook events.
Test Strong Customer Authentication
Add PIN as a factor.
Add Facetec biometric as a factor.
Add device fingerprinting as a factor.
Verify an SCA protexted action through PIN, Facetec, or device fingerprinting challenges.
What you can't test in Sandbox

The Sandbox environment has some limitations, specifically that it does not support real money transfers, actual financial controls, or all currency routes available in our Production environment.

If a feature isn't specified in the above lists, it can be assumed that we don't officially support it in Sandbox.

We cannot guarantee all currencies will work the same in Sandbox as Production. If you run into issues, please fall back to the currencies and regions shared above.

In addition, we do not support:

Any other UI screen not explicitly stated above.
Movement of real money.
Notifications by email or phone (2FA code is always 111111).
Verification and payment issue flows (including cases and disputes), other than what is required in the supported onboarding flows.
Real verification or onboarding rules/fincrime checks (including what evidences we may ask for in different regions) due to compliance constraints.
Any deprecated APIs/versions for features covered in above sections.
Strong customer authentication with SMS, phone call, and Whatsapp (SCA can be disabled via your client credentials).
Direct Debit/ACH funding for USD.
Mobile tokenization flows (ex. Google Pay, Apple Pay).
3DS transactions.
Simulations of credit transactions for cards.
Card manufacturing flows.
Live rates.
Delivery of refund webhooks.
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

We have a dedicated team focusing on bank partnerships who will help you along the way, sharing knowledge and experience from previous integrations to help you build a robust and highly available system.

Refer to versioning for information on how we version and deprecate our APIs.

Edge Cases

This section discusses some edge cases that you should test and handle before going live with your integration.

Email Address Considerations

Due to how getting access to user accounts works the Wise platform relies on user email addresses matching between the bank and ourselves. At the point the bank attempts to create a user we check and see if an account already exists for that email address, if so we return a 409 response and the client application forwards the user to log in to Wise to do the OAuth grant flow.

This works well when the email addresses match in the first place and aren't updated on either side after the link is established. Of course, this is not always going to be the case, so we must consider what happens in either eventuality.

Non-matching email addresses

If a user already has a Wise account, and you create a user for the same person under a different email address they could end up with a duplicate user account under the second email address. Currently, we monitor this behaviour for abuse, but we are working on a more robust user creation solution to prevent this from occurring.

Email Change

It is possible to change a user’s email address both at Wise and potentially also on the bank platform. These flows can cause complications with the integration.

Email changed at Wise

If a user changes their email address, all tokens to the user account are revoked. In this case the bank will receive a 400 when attempting to generate an access_token and as such should follow the same process as described in the "Token Expiry" section below and start the sign-up flow from the beginning.

In this case, if the user has changed their email address at Wise, it is possible the user will end up with a new Wise account using their old email address still held by the bank, or they might link their bank account to a different already existing Wise account under the old email address.

Email changed at the bank

In this case the tokens will remain valid for the Wise account, however, depending on how the user originally linked the account, different things can happen when/if that token expires.

If the bank created the account originally, they will be unable to generate tokens using the registration_code they have, as the endpoint requires the email address which will now no longer match. To mitigate this it is recommended that the bank store the email that was originally used for signup alongside the registration code and use this rather than the most up-to-date email address they store for the user.

If the token expires for a user not created by the bank and the user has a new email address at the bank then they can be pushed through the signup flow with this new email address and either have a new account created or link an existing against the new email, as described in the Token Expiry section below.

The result of many of these flows is that the user may end up with more than one Wise account, which is undesirable. Currently, we monitor this behaviour for abuse, but we are working on a more robust user creation scenario to prevent this from occurring.

Email change mitigation

The result of these eventualities are that over time a user of the bank could be linked to more than one Wise account, and therefore you will need to be defensive when requesting older user data as the request may fail because we forbid one user to access other user's data. We recommend to keep a local copy of your user's transfer data and update it asynchronously such that older transfers remain accessible to the user in the case where it can no longer be accessed. You should also make sure to handle these failing calls gracefully and continue to process transfers that can be accessed over the API.

In the event a user is not happy at losing access to their older data or having two accounts is confusing then we can manually update the email addresses to match for the two accounts they want.

Token Expiry

It is also possible that a user's refresh token will become invalid. This could happen for a number of reasons, for example:

The refresh token's validity period expires (usually set at three months or more)
The user revokes the access of your application to their account.
The user enables enhanced security on their Wise account.
Wise revoke a token due to a suspected security breach of the token or your client secret.

Due to this possibility your application should handle the scenario where tou fail to generate a new access token from the refresh token. Correctly handling this depends on how you originally gained access to the user.

1. An existing user granted your application access to the account

If you were granted access by an existing user then you should send the user through the same flow as you initially did to generate tokens described in "Connecting to an Existing Wise account" below. You will then have new access and refresh tokens generated which you can now store and use as before.

2. Your application created the user

In the case you created the user using the Profile Creation API flow then the mechanism for regenerating tokens is dependent on whether the user you created has "reclaimed" their Wise account and used our website or apps directly, which they may have done by following a secure process on our website or apps.

If the user has not reclaimed their account then the original registration_code you generated should still be able to generate new tokens for the user. Because of this you should store this code alongside the created user ID in your database at the point of user generation, encrypted for security purposes.

If the previously stored token fails with an error code 400 and error:

{
  "error": "invalid_grant",
  "error_description": "Invalid user credentials."
}

Then you can assume the user has reclaimed the account and push them through the "Connecting Existing Accounts" flow.
Connecting existing accounts

When you are submitting an email which already exists amongst our users then an attempt to create a new one will fail. In that case you should follow the standard website redirection flow.

Going live checklist
1. Make your integration bulletproof
Implement basic retry mechanism to handle potential failures or network interruptions
Implement duplicate prevention mechanism to avoid duplicate payments. Verify that UUID is uniquely generated for each individual payment and its value is kept same in case of retrying.
Implement basic logging to help out in debugging and problem-solving, if needed.
Check that you can handle all possible transfer states during polling of transfer info.
Handle the potential issues described in Edge Case Handling above
Required data fields for recipients and transfers vary for different currencies. Please explore Recipient Account Requirements and Transfer Requirements
Some good recipient currencies to test are:
CAD - has several fields in a field group.-0
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
4. Monitor API status
You can always track our API status here.
User Experience

There are different ways to build your user experience, especially when it comes to the sequence of steps in the payment flow. We do have a recommended order that has been tuned by the Wise team, is the simplest to understand for the customer, and the easiest to build. We are happy to assist you to design and build a great experience for your customers.

Your Wise user experience

There are two main user flows that must be built in order to integrate with Wise.

The user onboarding flow

There are two ways to access the Wise Platform API depending on if you customer already has a Wise account or not. You should handle both cases in your integration.

Flowchart

This flowchart describes the different scenarios you will encounter and how you should handle them.

You need to go through this flow only once for each customer before they can set up their first transfer.

Gain access to a Wise user:
by creating a new account over API(see Accessing Customer Accounts - New Account)
OR
by linking to an existing account(see Accessing Customer Accounts - Linking Accounts)
Create personal user profile (See Profile Creation - Personal Profile)
Create business user profile (See Profile Creation - Business Profile) - this is an optional step only to be used if your bank is providing business customers access to Wise.

Upon linking to an existing Wise account you need to ensure that you have connected to an account that represents the same natural person or business of the bank account in your platform. To do this for personal profiles please check the date of birth of the connected Wise profile matched the date of birth you hold for that customer in your platform. For businesses the comparison required changes with the region you are servicing, please discuss the best approach with your implementation team.

The transfer flow

To create transfers on behalf of users you need these building blocks:

TODO: link to API ref sections
...