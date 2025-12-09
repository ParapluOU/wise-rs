# ChangelogCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/changelog

---

Changelog
Copy
24 Nov 2025
Updated all sandbox URLs to point to the new sandbox environment.
18 Nov 2025
Added creationTime and modificationTime fields in Payment Token list.
5 Nov 2025
Added missing fields in Payin Deposit details.
29 Oct 2025
Updating the description of the Open Banking Transactions endpoint.
Add details for submitting Account Intent Kyc Requirement kyc-review#kyc-submit
21 Oct 2025
Add new webhook event type transfers#transfer-suspension
10 Oct 2025
Add new strong customer authentication endpoints.
Deprecated SCA enrolment factors endpoints.
Deprecated SCA verification endpoints.
03 Oct 2025
Update recipient confirmation API guide.
11 Sep 2025
Add new simulation endpoint to simulate verify profile by id.
09 Sep 2025
Add new simulation endpoint to simulate bank transaction import.
08 Sep 2025
Add originatorLegalEntityType field for transfer-requirements to include sender type transfer-requirements.
Updated the Transfer resource documentation, to include the payinSessionId field
02 Sep 2025
Update documentation for OpenBanking from v3.1 to v3.1.11 OpenBanking.
26 Aug 2025
Add replacementDetails field in Card Order creation.
25 Aug 2025
Add cardHolderProfileId field in Card Order creation.
8 Aug 2025
Updated Card Issuance guide to reflect changes in card activation.
Deprecated complete card order.
Deprecated unlockSpendingPermissions in Card resource.
28 Jul 2025
Restructured the hosted KYC guide with clearer content organization.
Updated Hosted KYC guide to include Proactive trigger.
24 Jul 2025
Add new List KYC Review Items endpoint reference documentation.
Add new Simulate KYC Review Add requirement endpoint reference documentation.
Add new Simulate KYC Review Submit requirement endpoint reference documentation.
Add new Simulate KYC Review Verify requirements endpoint reference documentation.
18 Jul 2025
Updated Get Delivery Estimate to include the new formattedEstimatedDeliveryDate field
16 Jul 2025
Updated Card Transactions Decline Reasons to include new decline reason codes
9 Jul 2025
Updated Simulate Top-Up to include new field (Channel) in request body.
3 Jul 2025
Updating verification-documents to state the characters limitation in the unique identifier field.
30 Jun 2025
Updating Language Support with Danish, Norwegian and Swedish
Add new payment-returns guide.
21 May 2025
Updating the swift-in#credit webhook supported subscriptions
19 May 2025
Updating the regions where occupations is required in Create or update an address and Create a Personal Profile
13 May 2025
Add new Create Business Profile V3 (beta) endpoint reference documentation.
Add new Get Business Representative API endpoint reference documentation.
Add new Transfer Business Representative API endpoint reference documentation.
Add new Update Business Representative API endpoint reference documentation.
8 May 2025
Updating hosted KYC webhook trigger references information with card, proactive and bank details information
6 May 2025
Merchant name and location added to the card transaction state change webhook
29 Apr 2025
Adding new endpoint for US combined receipt for partners servicing US retail consumers (instead of existing receipt endpoint) which is a regulatory requirement.
28 Apr 2025
Add new Create SCA session endpoint reference documentation.
Updated SCA over API guide
25 Apr 2025
Updated swift-in test message in sandbox to accept more custom fields
17 Apr 2025
Updated Refund Recipient endpoint reference documentation and sample
10 Apr 2025
Added new expiration related fields to User Token Resource
7 Apr 2025
Updated dispute sub status and transition diagram
27 Mar 2025

The v3 upload dispute file endpoint will be deprecated in favor of v4 upload dispute file endpoint that does not require additional permission setup for the endpoint to be accessible.

The billingAddress field is now changed to the address field. You'll still be able to send billingAddress in the /card-order request, but we recommend switching to address as soon as possible. We'll deprecate the billingAddress field at a later date.

21 Mar 2025
Added guide on card kiosk collection
Added a new section for card kiosk collection APIs
Added card production simulation endpoint to simulate different card production statuses
Added cards#card-production-status-change webhook
21 Mar 2025
Move '#update-phone-number' of 'card' to 'deprecated-api'
Updated the field description of phone-number in the card object to indicate that it is deprecated and refer them to profile endpoints.
20 Mar 2025
Renamed triggerReference field in KYC Review to triggerReferences. Updated the field in KYC Review webhook accordingly.
18 Mar 2025
Removed /features page. Migrated content into api-reference pages.
17 Mar 2025
Added new includeWise parameter to comparison v4
12 Mar 2025
Remove purge_timestamp from cards#transaction-state-change webhook. Please use data.purge_time instead
10 Mar 2025
Added Business profile creation step to Hosted KYC guide
10 Mar 2025
Added new Strongly Authenticated Session Section.
6 Mar 2025
Deprecate purge_timestamp and add purge_time in the cards#transaction-state-change webhook.
4 Mar 2025
Added a new unlockSpendingPermissions field to the card object.
Added new fields pin_validation_result, approval_code and purge_timestamp to the cards#transaction-state-change webhook.
Updated the guide for partner control to enable contactless and magstripe card payments, detailing suggested methods of checking that the end user has received their card.
Updated the API for update card order status.
3 Mar 2025
Updated the Quote API Reference - Corrected the price explanation id by adding plainText, so it reads option.price.items[n].explanation.plainText
26 Feb 2025
Added accountDetails field to Partner License Transfer.
25 Feb 2025
Updated Upload Document endpoint by adding BUSINESS_AUTH_REP_PROOF_BY_AUTH_LETTER to the list of allowed values for documentType field.
24 Feb 2025
Updated acceptable list of transfer funding types for Fund a Transfer endpoint.
18 Feb 2025
Moved information about duplicate account handling from the reference to the appropriate integration guides.
Added the mTLS API endpoints into the Environment section.
Updated the Updating Profiles guide to more clearly describe how to handle profiles that are due annual review but no updates are required.
14 Feb 2025
Added the optional externalCustomerId field to Personal and Business profile creation. This allows integrators to map a profile entity to a customer in their own system.
13 Feb 2025
Added accountDetails field to Third Party Transfer.
10 Feb 2025
Updated the comparison docs with a new response field isConsideredMidMarketRate to show whether a provider's rate can be classified as the mid-market rate or not.
05 Feb 2025
Added a new currentState field to the Profile object resource.
29 Jan 2025
Updated the comparison docs with the new /v4/comparisons endpoint.
21 Jan 2025
Published a new version (v5) of the Upload Evidences endpoint.
08 Jan 2025
The personal token SCA documentation is no longer publicly available. Partners need to contact their CSM if they are still require access to this guide.
06 Jan 2025
Added changed_by to the cards#card-status-change webhook.
11 Dec 2024
Updated the International Receive webhook subscription guide.
Removed the V2.0.0 swift-in-credit webhook because it's no longer used.
10 Dec 2024
Added a new endpoint to get a Third Party Transfer by ID.
5 Dec 2024
Added a new V2 endpoint to update business profiles.
Added a new version V3.0.0 of the bulk settlement payment received webhook.
3 Dec 2024
Added a new V2 endpoint to update personal profiles.
2 Dec 2024
Added a new balance_movements field to the cards#transaction-state-change webhook.
28 Nov 2024
Added an update card order resource to add Delivery Option.
24 Nov 2024
Added new guide for 3D Secure authentication.
14 Nov 2024
Updated the Card Issuance guides with partner control information to set a card order as COMPLETED.
Updated the cancel card order endpoint to an update card order status endpoint (a new COMPLETED status can be set for certain integrations).
Updated the permissions resource to include the isLocked field.
Added a transaction decline reason ATM_PIN_CHANGE_NOT_ALLOWED.
13 Nov 2024
Updated the Card Issuance guides.
7 Nov 2024
Added a new API to allow the cancellation of a card order.
12 Aug 2024
Updated the APIs for card transaction simulations (authorisation including refund, reversal, and clearing).
15 Jul 2024
Added a new credit field to the cards#transaction-state-change webhook.
Added a new credit field to the card transaction resource.
27 May 2024
Added new fields to the cards#transaction-state-change webhook.
2 May 2024
Added a new webhook event to allow subscribing to dispute updates.
28 Mar 2024
Added new APIs to allow managing disputes.
18 Mar 2024
Exposed delivery tracking details if available.
28 Feb 2024
Added a new API to allow deleting card spend limits.
21 Feb 2024
Added explanation for dispute handling via API reasons.
24 Jan 2024
Added explanation for card transaction decline reason.
29 Nov 2023
Added a new API to allow listing payment tokens.
22 Nov 2023
Added guide on dispute handling via API.
22 Nov 2023
Added deliveryEstimate field to Card Order.
20 Nov 2023
Added a new API to add more spend-controls on your card transactions.
16 Nov 2023
Added API guide for push provisioning.
27 Oct 2023
Added an API to get a list of card transactions marked for deprecation.
17 Oct 2023
Updated card spend limits to support transaction limit.
9 Oct 2023
Published a new version (v4) of the /{{cardToken}}/spend-limits endpoint, which has a new request/response format.
2 Oct 23
The billingAddress field is now changed to the address field. You'll still be able to send billingAddress in the /card-order request, but we recommend switching to address as soon as possible. We'll deprecate the billingAddress field at a later date.
Support for BR addresses is now available.
30 Aug 2023
Added a new API, /address/validate. If you currently don't retrieve the address via another endpoint, please be sure to validate all addresses with the above endpoint before you send them in the /card-order request.
11 Aug 2023
Return all the possible card statuses in both the /status API and cards#card-order-status-change webhook.
8 Aug 2023
Ordering a physical card over the API is now available.
27 Jul 2023
Added new fields to the cards#transaction-state-change webhook.