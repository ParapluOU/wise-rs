# Getting StartedCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/send-money/use-cases/embedded/getting-started

---

Getting Started
Copy
Environments

It is a strict requirement to use only the mTLS endpoints for this type of integration. The TLS only endpoints will not be available in production

Please note regarding sandbox testing: we only support our Send Money product with Embedded Finance at this time.

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

You should expose an API internally for your web and mobile clients to call to provide the required Wise features. Your backend system will manage both communication to the Wise Platform API and internal operations such as querying user KYC data to send to Wise, checking a user has sufficient funds to make the requested transfer and triggering the payment of funds to Wise when a user confirms a transfer.

You should also store a copy of certain data relating to Wise to decrease latency and increase resiliency when users review previous transfers they have made or recipients they sent funds to. The extent of what you store will depend on your integration, but we recommend to store at a minimum:

Quotes that have been used to create transfers
Transfer records including ID and status
Recipient IDs, names and account summary data

The goal is to store locally in your platform all the data you need to drive your UI, such that calling our API is not required when reviewing historic data.

You should subscribe to our webhooks to keep this data up to date.

Refer to versioning for information on how we version and deprecate our APIs.

Going live checklist
1. Make your integration bulletproof
Implement basic retry mechanism to handle potential failures or network interruptions
Implement duplicate prevention mechanism to avoid duplicate payments. Verify that UUID is uniquely generated for each individual payment and its value is kept same in case of retrying.
Implement basic logging to help out in debugging and problem-solving, if needed.
Check that you can handle all possible transfer states during polling of transfer info.
Required data fields for recipients and transfers vary for different currencies. Please explore Recipient Account Requirements and Transfer Requirements
Some good recipient currencies to test are:
CAD - has several fields in a field group.-0
USD - the country field has refreshRequirementsOnChange.
JPY - the bank field has refreshRequirementsOnChange.
KRW - has a field using a date component type.
2. Set up security for the production environment
For integrations using client credentials tokens it is a strict requirement to set up and use mutual TLS (mTLS).
Make sure you have stored and encrypted your client credentials securely in an credential management system. Wise secrets must never exposed to frontend systems.
Ensure access tokens are also stored securely and never exposed to frontend systems.
Never store or log client credentials or tokens in plain text.
Make sure your server has TLS version 1.2 or higher.
Your access tokens have a pre-configured expiry, for longer running jobs implement a mechanism to retrieve a new client credentials access token upon expiration.
Do not persist the access tokens, use them and discard them. Do not presume the size and format of the access token as we reserve the right to change these in future.
3. Testing in the production environment
Launch to a limited set of your customers and test all currency routes you will offer end-to-end.
We recommend launching a limited set of currencies initially to limit the scope of potential issues, please seek guidance from the Wise team.
Test successful flow and bounce back flow (where funds cannot be delivered).
4. Monitor API status
You can track our service availability here.