# Partner AccountCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/kyc/partner-accounts

---

Partner Account
Copy

This guide is intended for Wise partners who are the direct customer of Wise either by transacting on their own partner account (First Party), OR by transacting on their partner account as their customer, providing originator data (Third Party).

First Party

In this model, the end customer is the partner we are working with, and the partner will onboard to Wise and have their own profile. They may be doing payouts or internal treasury operations over Wise’s API.

Third Party

In this model, Wise onboards the partner directly. Wise does not onboard the partner’s customers. Instead, the partner’s customers access Wise services through the partner app as a “third party” customer of Wise. Wise still collects the data of the end user as an “originator” but they do not have their own profiles ( 3rd Party Infrastructure Partner). These are often white-labeled solutions and the customer is not made aware that Wise is being used for their payments.

To use the first party model, the partner MUST be a direct customer of Wise and are only moving funds they own on the Wise platform, such as payouts to employees or moving a large number of payments internally.

To allow for the third party model, Third Party Partners must have a sponsor bank or their own license under which they can move funds to Wise.

Partner Account Postman Collection

We've created a Postman collection to help you test your integrations easily! It includes:

Our sandbox environment (https://api.wise-sandbox.com)
Common API calls for creating users and profiles
Automated tests that copy response data between calls
Example responses for reference

Fork the collection to get updates automatically. This lets you test the complete flow with minimal setup!

Fork in Postman
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
4. Monitor API status
You can always track our API status here.
User Experience

There are different ways to build your user experience, especially when it comes to the sequence of steps in the payment flow. We do have a recommended order that has been tuned by the Wise team, and we are happy to assist you to design and build a great experience for your customers.

The transfer flow

To create transfers you need these building blocks:

Refresh Access Token
Create Quote
Create Recipient Account
Create Transfer
Fund Transfer