# Profile CreationCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/kyc/partner-kyc/profile-creation

---

Profile Creation
Copy
Getting started with Wise profiles

Once a Wise user has connected their profile to your application, your system can start interacting with that profile's Wise Platform API resources. The Wise Platform API can be used to generate quotes for money transfers, set up recipients for payments, set up money transfers, and so on as described in our full API documentation.

Once you acquire an API access token for a profile you can immediately start using the API on the customer's behalf. However, there are some considerations that are important to avoid problems when working with Wise profiles.

There may be a case when a Customer Account without a Profile is linked. When this happens, create a Profile for the Customer by following the guide below.

Handling duplicate Wise accounts

One person cannot have multiple active duplicate user profiles, creating multiple profiles with the same details will fail with a 409 error. When this happens, you should show an error message to the user informing them that they may have an existing Wise account. The customer should then be allowed to link to an existing Wise account.

Profile extensions

Some regions demand more profile data due to local regulations, in order to be compliant the extra data has to be provided. This is required based on jurisdiction of operation and customer demographics of your bank or other financial institution, please discuss with your implementation team to see if it is necessary to include the following behaviour. To collect this data please use our User Extensions dynamic form endpoint.

Verification status

Wise is obliged to verify the identity of personal and business profiles as part of our Know Your Customer policy. We must be satisfied that we know our customers well enough before conducting financial transactions with them.

Verification involves the Wise customer submitting various forms of evidence to us for our consideration. Evidence can include official documents, for example. We may accept or reject such evidence, or consider some evidence as being superior to other forms of evidence.

What evidence we have verified for a profile can affect how successful or how quickly money transfers can be made. For certain values of transfer, or certain currency routes, we may require a certain level of verification. If we do not consider that a profile has been sufficient verified for a particular money transfer, we might delay the processing of the transfer while we seek further information from the customer.

For more information on verification: How does Wise verify my identity

If you intend to use the Wise Platform API to set up money transfers, then it is desirable for you to know whether such verification issues might delay or prevent transfers from succeeding. This is particularly relevant for profiles that have just been created as part of linking your application to Wise: new profiles may not yet be verified, and money transfers may be delayed while this process happens, leading to a bad experience for our customers.

Wise exposes the verification status of a profile to help connected applications understand when money transfers are able to be created without such verification issues (but see below for an important caveat).

Verification status is a simple indicator of the current level of verification that has been made for a profile to some particular level of confidence: a level that is likely to allow money transfers to occur without being delayed by verification-related issues.

Verification status is exposed as a property of a profile: i.e. a profile is verified or not verified. It is useful to have such a property as a "gate" in the connected application's system that prevents transfers from being created before they are likely to be processed without delays.

Connected applications can query the verification status of a profile using the Wise Platform API. Wise can also asynchronously notify connected applications about the change in a profile's verification status using a webhook notification.

Although we expose verification status as a profile-level property, there are implicit limits to which any VERIFIED status applies. A VERIFIED status applies to some reasonable limits that customer should not normally reach. However, it is possible for individual transfers to require additional verification to be performed, even after we have give the profile a verification status of VERIFIED. Therefore, connected applications should be prepared for verification issues to affect money transfers regardless of the reported VERIFICATION STATUS, althouogh this is not generally expected

Verification status integration strategy

The following strategy may be useful for your integration with respect to profile verification:

At some point before going live to your customers, you set up an "application" webhook subscription for verification status change events. This is a single subscription that will allow you to receive verification status events related to any profile that links to your application in the future. (See webhooks for more information.)
After going live, a user connects their profile to your application using the OAuth authorization flow. (Note that the profile might be an existing, verified profile, or a brand new, unverified profile).
During the authorization flow, your system receives an authorization code and Wise profile ID in a request to your callback endpoint (see User Authorization). Once API tokens have been acquired using the authorization code, you now have API tokens and the related profile ID: you can now access the profile's API resources.
The verification status of the profile is queried using the Wise Platform API. If the response indicates the profile is "verified", you can proceed with transfer operations. If the response indicates "not verified", then transfers are likely to be delayed: you should wait to receive a webhook notification for the customer. However in some cases, users might be allowed to make payments up to a certain limit even though their profile is "not_verified" (see "Bank Transfer Verification Check Result" below).
If a profile is unverified, the verification process takes place. Later, when verification completes successfully, you will receive a webhook notification informing you that the profile has become verified: you may then allow transfer creation. (Alternatively, you can continue to poll the API again, but this is not recommended.)
Checking verification status using the API

You can check the verification status of a profile using the following API. This is a profile-specific API resource which should be accessed using an access token acquired for the profile.

Request

POST https://api.wise-sandbox.com/v3/profiles/{profileId}/verification-status/bank-transfer?source_currencies=GBP,USD,EUR

Field	Description	Format
profileId	Profile ID	Integer
source_currencies	List of source currency codes (ISO 4217)	Text
Example Request
    curl -X POST 'https://api.wise-sandbox.com/v3/profiles/{{profileId}}/verification-status/bank-transfer?source_currencies=GBP,USD,EUR' \
         -H 'Authorization: Bearer <your api token>'
Response
Field	Description	Format
request_id	Unique identifier of the request.	Text
routes	List of verification check results for each provided source currency.	Bank Transfer Verification Check Result (see below)
Example Response
{
  "routes": [
    {
      "source_currency": "GBP",
      "maximum_entitled_amount": 100000,
      "current_status": "verified"
    },
    {
      "source_currency": "USD",
      "maximum_entitled_amount": 0,
      "current_status": "not_verified"
    },
    {
      "source_currency": "EUR",
      "maximum_entitled_amount": 10000,
      "current_status": "not_verified"
    }
  ],
  "request_id": "e66da5f6-2456-403c-bfcb-908885ee1a61"
}
Bank Transfer Verification Check Result

Represents the verification status of a profile when funding with a bank transfer.

Schema
Field	Description	Format
source_currency	ISO 4217 currency code.	Text
current_status	Current verification status	Verification status code (see below)
maximum_entitled_amount	Maximum entitled amount that can be transferred with the user's given verification state.	Decimal
Guide on how to process the API response:
current_status	maximum_entitled_amount	How to process
verified	any value	Profile is verified. maximum_entitled_amount field can be ignored (note that some regional limits may still apply).
not_verified	0	Profile is not verified. Any payments from the user will be delayed by verification.
not_verified	> 0	Profile is not verified yet but the user is allowed to make payments up to a certain limit. Once the limit is reached, following payments might be delayed because of additional verification requirements. Please note that the limit is cumulative and not per transfer. This state might allow some users to experience the product immediately even if the verification process isn't complete yet.
Verification status code	Description
verified	The profile is sufficiently verified to start making payments (note that some regional limits may still apply)
not_verified	The profile is currently awaiting verification or there are pending issues with the verification process

Please note that we do not expose any finer details of customer verification.

Verification State Change webhook

You can subscribe to Webhooks - Verification State Change event type to be notified when the user is ready to use Wise.

Create a personal profile

POST /v2/profiles/personal-profile

If a customer you are creating a profile for has first or last names that exceed 30 characters (e.g. they have many middle names) then you should truncate the names at length 30 characters and submit that value.

Occupations is required for CA, IN, JP, ID, IL, MX and within the US for the state NM.

Contact Details are used only for mandatory customer notifications and to help identify your customer if you contact Wise's Customer Support team.

This request accepts an optional field in the header, `X-idempotence-uuid`. This should be unique for each Profile you create. In the event that the request fails, you should use the same value again when retrying. If the `X-idempotence-uuid` header is not provided and a Profile already exists, then you will receive a response with an HTTP status code `409`. If this happens, you can retrieve the profiles with the 'List profiles for a user account' API `GET /v2/profiles`.
Request Fields
firstNametext (max 30 chars)

First name (including middle names). Required

lastNametext (max 30 chars)

Last name. Required

preferredNametext (max 30 chars)

Preferred first name, if different to the legal first name.

details.firstNameInKanatext (katakana)

First name in Katakana. Required for from-JPY personal transfers

details.lastNameInKanatext (katakana)

Last name in Katakana. Required for from-JPY personal transfers

address.addressFirstLinetext

First line of address. Required

address.citytext

City. Required

address.countryIso3Codetext

3 Letter country code (lower case). Required

address.postCodetext

Postal code

address.stateCode (max 5 chars)text

State code. Required for US, CA, BR and AU addresses

nationalitytext

3 Letter country code (lower case)

dateOfBirthyyyy-mm-dd

Date of birth. Required

externalCustomerIdtext

An external reference identifier mapping the customer of this profile to your system.

contactDetails.emailemail

Contact email address. Please speak with your integration account manager for details on how customer communication is handled for your integration. Required

contactDetails.phoneNumbertext

Contact phone number in international phone number format, example: "+1408XXXXXXX". Please speak with your integration account manager for details on how customer communication is handled for your integration. Required

occupations[n].codetext

Occupation field code - this field accepts any job or occupation that the customer does, as the only occupation format currently accepted is "FREE_FORM".

occupations[n].formattext

Occupation field format - the format of the occupation code being submitted. As of now, it only accepts the value "FREE_FORM".

Example Request
curl -X POST \
  https://api.wise-sandbox.com/v2/profiles/personal-profile \
  -H 'Authorization: Bearer <your api token>' \
  -H 'Content-Type: application/json' \
  -H 'X-idempotence-uuid: 054064c9-e01e-49fb-8fd9-b0990b9442f4' \
  -d '{
    "firstName": "Oliver",
    "lastName": "Wilson",
    "preferredName": "Olivia",
    "firstNameInKana": null,
    "lastNameInKana": null,
    "address": {
      "addressFirstLine": "50 Sunflower Ave",
      "city": "Phoenix",
      "countryIso3Code": "usa",
      "postCode": "10025",
      "stateCode": "AZ"
    },
    "nationality": "usa",
    "dateOfBirth": "1977-07-01",
    "externalCustomerId": "12345-oliver-wilson",
    "contactDetails": {
      "email": "o.wilson@example.com",
      "phoneNumber": "+3725064992"
    },
    "occupations": [
      {
        "code": "Software Engineer",
        "format": "FREE_FORM"
      }
    ]
  }'
Example Response
{
  "id": 30000001,
  "type": "personal",
  "details": {
    "firstName": "Oliver",
    "lastName": "Wilson",
    "preferredName": "Olivia",
    "address": {
      "addressFirstLine": "50 Sunflower Ave",
      "city": "Phoenix",
      "countryIso3Code": "usa",
      "postCode": "10025",
      "stateCode": "AZ"
    },
    "nationality": "usa",
    "dateOfBirth": "1977-07-01",
    "externalCustomerId": "12345-oliver-wilson",
    "contactDetails": {
      "email": "o.wilson@example.com",
      "phoneNumber": "+3725064992"
    },
    "occupations":  [
       {
         "code": "Software Engineer",
         "format": "FREE_FORM"
       }
     ],
    "localizedInformation": []
  }
}
Create a business profile

POST /v2/profiles/business-profile

It is required that you first create a personal profile with details of the authorized representative of the business. It is currently not possible to create a business profile without first creating a personal profile.

This request accepts an optional field in the header, `X-idempotence-uuid`. This should be unique for each Profile you create. In the event that the request fails, you should use the same value again when retrying. If the `X-idempotence-uuid` header is not provided and a Profile already exists, then you will receive a response with an HTTP status code `409`. If this happens, you can retrieve the profiles with the 'List profiles for a user account' API `GET /v2/profiles`.
Request Fields
businessNametext

Business name

businessNameInKatakanatext (Katakana)

Business name in Katakana (only for Japanese businesses)

businessFreeFormDescriptiontext

Business free form description. *Required if companyType is OTHER. If this is not provided for an OTHER companyType, the profile should not be allowed to create a transfer. For the rest of the companyType(s), it is highly recommended to always provide the business' description, to avoid payment issues such as suspensions. Wise will send a request for information (RFI) if this detail is not provided.

registrationNumbertext

Business registration number

acntext

Australian Company Number (only for Australian businesses)

abntext

Australian Business Number (only for Australian businesses)

arbntext

Australian Registered Body Number (only for Australian businesses)

companyTypetext

Company legal form. Allowed values:

LIMITED
PARTNERSHIP
SOLE_TRADER
LIMITED_BY_GUARANTEE
LIMITED_LIABILITY_COMPANY
FOR_PROFIT_CORPORATION
NON_PROFIT_CORPORATION
LIMITED_PARTNERSHIP
LIMITED_LIABILITY_PARTNERSHIP
GENERAL_PARTNERSHIP
SOLE_PROPRIETORSHIP
PRIVATE_LIMITED_COMPANY
PUBLIC_LIMITED_COMPANY
TRUST
OTHER
companyRoletext

Role of person. Allowed Values:

OWNER
DIRECTOR
OTHER
address.addressFirstLinetext

First line of address

address.citytext

City

address.countryIso2Codetext

2 letter country code.

address.countryIso3Codetext

3 Letter country code. Must be lowercase

address.postCodetext

Postal code

address.stateCodetext

State code

externalCustomerIdtext

An external reference identifier mapping the customer of this profile to your system.

actorEmailtext

Email of the actor

firstLevelCategorytext

Category of the business

secondLevelCategorytext

Secondary category of the business

operationalAddressesarray

List of operational addresses

operationalAddresses[0].addressFirstLinetext

First line of address

operationalAddresses[0].citytext

City

operationalAddresses[0].countryIso2Codetext

2 letter country code

operationalAddresses[0].countryIso3Codetext

3 Letter country code. Must be lowercase

operationalAddresses[0].postCodetext

Postal code

operationalAddresses[0].stateCodetext

State code

webpage (conditional)text

Business webpage. *Required if companyType is OTHER. If this is not provided for an OTHER companyType, the profile should not be allowed to create a transfer. For the rest of the companyTypes, it is highly recommended to always provide the business' website,to avoid payment issues such as suspensions. Wise will send a request for information (RFI) if this detail is not provided.

Business Category

Ensure when submitting a business profile that you submit a category and associated sub-category from the list below. You should map from the information you have about the business to one of our categories and sub-categories. If this is problematic please get in touch with us to discuss alternate solutions.

The categories and their sub-categories are as follows:

CHARITY_NON_PROFIT
CHARITY_ALL_ACTIVITIES
CONSULTING_IT_BUSINESS_SERVICES
ADVERTISING_AND_MARKETING
ARCHITECTURE
COMPANY_ESTABLISHMENT_FORMATION_SERVICES
DESIGN
FINANCIAL_CONSULTING_ACCOUNTING_TAXATION_AUDITING
IT_DEVELOPMENT
IT_HOSTING_SERVICES
IT_CONSULTING_AND_SERVICES
LEGAL_SERVICES
MANAGEMENT_CONSULTING
SCIENTIFIC_AND_TECHNICAL_CONSULTING
SOFTWARE_AS_A_SERVICE
TRANSLATION_AND_LANGUAGE_SERVICES
CONSULTING_OTHER
SERVICES_OTHER
FREELANCE_PLATFORMS
RECRUITMENT_SERVICES
MAINTENANCE_SERVICES
FREELANCE_PLATFORMS
DESIGN_MARKETING_COMMUNICATIONS
ADVERTISING_AND_MARKETING
ARCHITECTURE
AUDIO_AND_VIDEO
DESIGN
PHOTOGRAPHY
PRINT_AND_ONLINE_MEDIA
TELECOMMUNICATIONS_SERVICES
TRANSLATION_AND_LANGUAGE_SERVICES
MEDIA_COMMUNICATION_ENTERTAINMENT
ADULT_CONTENT
AUDIO_AND_VIDEO
FINE_ARTS
ARTS_OTHER
EVENTS_AND_ENTERTAINMENT
GAMBLING_BETTING_AND_ONLINE_GAMING
NEWSPAPERS_MAGAZINES_AND_BOOKS
PERFORMING_ARTS
PHOTOGRAPHY
TELECOMMUNICATIONS_SERVICES
VIDEO_GAMING
EDUCATION_LEARNING
SCHOOLS_AND_UNIVERSITIES,
TEACHING_AND_TUTORING
ONLINE_LEARNING
FINANCIAL_SERVICES_PRODUCTS_HOLDING_COMPANIES
CROWDFUNDING
CRYPTOCURRENCY_FINANCIAL_SERVICES
FINANCIAL_CONSULTING_ACCOUNTING_TAXATION_AUDITING
HOLDING_COMPANIES
INSURANCE
INVESTMENTS
MONEY_SERVICE_BUSINESSES
FINANCIAL_SERVICES_OTHER
FOOD_BEVERAGES_TOBACCO
ALCOHOL
FOOD_MANUFACTURING_RETAIL
RESTAURANTS_AND_CATERING
SOFT_DRINKS
TOBACCO
VITAMINS_AND_DIETARY_SUPPLEMENTS
HEALTH_PHARMACEUTICALS_PERSONAL_CARE
HEALTH_AND_BEAUTY_PRODUCTS_AND_SERVICES
DENTAL_SERVICES
DOCTORS_AND_MEDICAL_SERVICES
ELDERLY_OR_OTHER_CARE_HOME
FITNESS_SPORTS_SERVICES
MEDICAL_EQUIPMENT
NURSING_AND_OTHER_CARE_SERVICES
PHARMACEUTICALS
PHARMACY
VITAMINS_AND_DIETARY_SUPPLEMENTS
PUBLIC_GOVERNMENT_SERVICES
PUBLIC_ALL_SERVICES
MAINTENANCE_SERVICES
GOVERNMENT_SERVICES
TELECOMMUNICATIONS_SERVICES
UTILITY_SERVICES
REAL_ESTATE_CONSTRUCTION
ARCHITECTURE
CONSTRUCTION
REAL_ESTATE_DEVELOPMENT
REAL_ESTATE_SALE_PURCHASE_AND_MANAGEMENT
RETAIL_WHOLESALE_MANUFACTURING
AGRICULTURE_SEEDS_PLANTS
FINE_ARTS
ARTS_OTHER
AUTOMOTIVE_SALES_SPARE_PARTS_TRADE
AUTOMOTIVE_MANUFACTURING
CHEMICALS
CLOTHING
ELECTRICAL_PRODUCTS
FIREARMS_WEAPONS_AND_MILITARY_GOODS_SERVICES
HOME_ACCESSORIES_FURNITURE
FINE_JEWELLERY_WATCHES
FASHION_JEWELLERY
HEALTH_AND_BEAUTY_PRODUCTS_AND_SERVICES
LEGAL_HIGHS_AND_RELATED_ACCESSORIES
MACHINERY
PETS
PRECIOUS_STONES_DIAMONDS_AND_METALS
SPORTING_EQUIPMENT
MANUFACTURING_OTHER
RETAIL_WHOLESALE_MARKETPLACE_AUCTION
RETAIL_WHOLESALE_OTHER
TOYS_AND_GAMES
TRAVEL_TRANSPORT_TOUR_AGENCIES
ACCOMMODATION_HOTELS
PASSENGER_TRANSPORT
FREIGHT_TRANSPORT
RIDESHARING_TRANSPORT_SHARING_SERVICES
TRANSPORT
TRAVEL_AGENCIES
TOUR_OPERATORS
TRAVEL_OR_TOUR_ACTIVITIES_OTHER
OTHER
OTHER_NOT_LISTED_ABOVE
Example Request
curl -X POST https://api.wise-sandbox.com/v2/profiles/business-profile \
  -H 'Authorization: Bearer <your api token>' \
  -H 'Content-Type: application/json' \
  -H 'X-idempotence-uuid: 054064c9-e01e-49fb-8fd9-b0990b9442f4' \
  -d '{
    "businessName": "ABC Logistics Ltd",
    "businessNameInKatakana": null,
    "businessFreeFormDescription": "Biz free form desc",
    "registrationNumber": "12144939",
    "acn": null,
    "abn": null,
    "arbn": null,
    "companyType": "LIMITED",
    "companyRole": "OWNER",
    "address": {
      "addressFirstLine": "1 A road",
      "city": "London",
      "countryIso2Code": "gb",
      "countryIso3Code": "gbr",
      "postCode": "11111"
    },
    "externalCustomerId": "67890-biz-acct",
    "actorEmail": "biz-acct@abcl.com",
    "firstLevelCategory": "CONSULTING_IT_BUSINESS_SERVICES",
    "secondLevelCategory": "DESIGN",
    "operationalAddresses": [
      {
        "addressFirstLine": "1 A road",
        "city": "London",
        "countryIso2Code": "gb",
        "countryIso3Code": "gbr",
        "postCode": "11111"
      }
    ],
    "webpage": "https://abc-logistics.com"
  }'
Update a personal profile

PUT /v2/profiles/{{personal-profile-id}}/personal-profile

Update user profile information for a personal profile.

If user profile has been verified then there are restrictions on what information is allowed to change. Where permitted, use the update window functionality by opening the update window, submitting the updated information using this endpoint, and finally closing the update window.
Request Fields
firstNametext (max 30 chars)

First name (including middle names). Required

lastNametext (max 30 chars)

Last name. Required

preferredNametext (max 30 chars)

Preferred first name, if different to the legal first name.

firstNameInKanatext (katakana)

First name in Katakana. Required for from-JPY personal transfers

lastNameInKanatext (katakana)

Last name in Katakana. Required for from-JPY personal transfers

address.addressFirstLinetext

First line of address. Required

address.citytext

City. Required

address.countryIso3Codetext

3 Letter country code (ISO3, lower case). Required

address.postCodetext

Postal code or ZIP code

address.stateCode (max 5 chars)text

State code. Required for US, CA, BR and AU addresses

nationalitytext

3 Letter country code (ISO3, lower case)

dateOfBirthyyyy-mm-dd

Date of birth. Required

contactDetails.emailemail

Contact email address. Please speak with your integration account manager for details on how customer communication is handled for your integration. Required

contactDetails.phoneNumbertext

Contact phone number in international phone number format, example: "+1408XXXXXXX". Please speak with your integration account manager for details on how customer communication is handled for your integration. Required

occupations[n].codetext

Occupation field code - this field accepts any job or occupation that the customer does, as the only occupation format currently accepted is "FREE_FORM".

occupations[n].formattext

Occupation field format - the format of the occupation code being submitted. As of now, it only accepts the value "FREE_FORM".

Example Request
curl -X PUT \
  https://api.wise-sandbox.com/v2/profiles/{{personal-profile-id}}/personal-profile \
  -H 'Authorization: Bearer <your api token>' \
  -H 'Content-Type: application/json' \
  -d '{
    "firstName": "Oliver",
    "lastName": "Wilson",
    "preferredName": "Olivia",
    "firstNameInKana": null,
    "lastNameInKana": null,
    "address": {
      "addressFirstLine": "50 Sunflower Ave",
      "city": "Phoenix",
      "countryIso3Code": "usa",
      "postCode": "10025",
      "stateCode": "AZ"
    },
    "nationality": "usa",
    "dateOfBirth": "1977-07-01",
    "contactDetails": {
      "email": "o.wilson@example.com",
      "phoneNumber": "+3725064992"
    },
    "occupations": [
      {
        "code": "Software Engineer",
        "format": "FREE_FORM"
      }
    ]
  }'
Update a business profile

PUT /v2/profiles/{{business-profile-id}}/business-profile

Update user profile information for a business profile.

If user profile has been verified then there are restrictions on what information is allowed to change.

Where permitted, use the update window functionality by opening the update window, submitting the updated information using this endpoint, and finally closing the update window.

Request Fields
idtext

Business profile ID. This must match the business profile ID supplied in the URL. Required

businessNametext

Business name

businessNameInKatakanatext (Katakana)

Business name in Katakana (only for Japanese businesses)

businessFreeFormDescriptiontext

Business free form description

registrationNumbertext

Business registration number

acntext

Australian Company Number (only for Australian businesses)

abntext

Australian Business Number (only for Australian businesses)

arbntext

Australian Registered Body Number (only for Australian businesses)

companyTypetext

Company legal form. Allowed values:

LIMITED
PARTNERSHIP
SOLE_TRADER
LIMITED_BY_GUARANTEE
LIMITED_LIABILITY_COMPANY
FOR_PROFIT_CORPORATION
NON_PROFIT_CORPORATION
LIMITED_PARTNERSHIP
LIMITED_LIABILITY_PARTNERSHIP
GENERAL_PARTNERSHIP
SOLE_PROPRIETORSHIP
PRIVATE_LIMITED_COMPANY
PUBLIC_LIMITED_COMPANY
TRUST
OTHER
address.addressFirstLinetext

First line of address

address.citytext

City

address.countryIso2Codetext

2 letter country code

address.countryIso3Codetext

3 letter country code. Must be lowercase

address.postCodetext

Postal code

address.stateCodetext

State code

firstLevelCategorytext

Category of the business

secondLevelCategorytext

Secondary category of the business

operationalAddressesarray

List of operational addresses

operationalAddresses[0].addressFirstLinetext

First line of address

operationalAddresses[0].citytext

City

operationalAddresses[0].countryIso2Codetext

2 letter country code

operationalAddresses[0].countryIso3Codetext

3 letter country code. Must be lowercase

operationalAddresses[0].postCodetext

Postal code

operationalAddresses[0].stateCodetext

State code

webpage (conditional)text

Business webpage. Required if companyType is "OTHER". If this is not provided for an "OTHER" companyType, the profile should not be allowed to create a transfer. For the rest of the companyTypes, it is highly recommended to always provide the business' website, to avoid payment issues such as suspensions. Wise will send a request for information (RFI) if this information is not provided.

Business Category

Ensure if you are updating the business categories of a business profile that you submit a category and associated sub-category from the list below. You should map from the information you have about the business to one of our categories and sub-categories. If this is problematic please get in touch with us to discuss alternate solutions.

The categories and their sub-categories are as follows:

CHARITY_NON_PROFIT
CHARITY_ALL_ACTIVITIES
CONSULTING_IT_BUSINESS_SERVICES
ADVERTISING_AND_MARKETING
ARCHITECTURE
COMPANY_ESTABLISHMENT_FORMATION_SERVICES
DESIGN
FINANCIAL_CONSULTING_ACCOUNTING_TAXATION_AUDITING
IT_DEVELOPMENT
IT_HOSTING_SERVICES
IT_CONSULTING_AND_SERVICES
LEGAL_SERVICES
MANAGEMENT_CONSULTING
SCIENTIFIC_AND_TECHNICAL_CONSULTING
SOFTWARE_AS_A_SERVICE
TRANSLATION_AND_LANGUAGE_SERVICES
CONSULTING_OTHER
SERVICES_OTHER
FREELANCE_PLATFORMS
RECRUITMENT_SERVICES
MAINTENANCE_SERVICES
FREELANCE_PLATFORMS
DESIGN_MARKETING_COMMUNICATIONS
ADVERTISING_AND_MARKETING
ARCHITECTURE
AUDIO_AND_VIDEO
DESIGN
PHOTOGRAPHY
PRINT_AND_ONLINE_MEDIA
TELECOMMUNICATIONS_SERVICES
TRANSLATION_AND_LANGUAGE_SERVICES
MEDIA_COMMUNICATION_ENTERTAINMENT
ADULT_CONTENT
AUDIO_AND_VIDEO
FINE_ARTS
ARTS_OTHER
EVENTS_AND_ENTERTAINMENT
GAMBLING_BETTING_AND_ONLINE_GAMING
NEWSPAPERS_MAGAZINES_AND_BOOKS
PERFORMING_ARTS
PHOTOGRAPHY
TELECOMMUNICATIONS_SERVICES
VIDEO_GAMING
EDUCATION_LEARNING
SCHOOLS_AND_UNIVERSITIES,
TEACHING_AND_TUTORING
ONLINE_LEARNING
FINANCIAL_SERVICES_PRODUCTS_HOLDING_COMPANIES
CROWDFUNDING
CRYPTOCURRENCY_FINANCIAL_SERVICES
FINANCIAL_CONSULTING_ACCOUNTING_TAXATION_AUDITING
HOLDING_COMPANIES
INSURANCE
INVESTMENTS
MONEY_SERVICE_BUSINESSES
FINANCIAL_SERVICES_OTHER
FOOD_BEVERAGES_TOBACCO
ALCOHOL
FOOD_MANUFACTURING_RETAIL
RESTAURANTS_AND_CATERING
SOFT_DRINKS
TOBACCO
VITAMINS_AND_DIETARY_SUPPLEMENTS
HEALTH_PHARMACEUTICALS_PERSONAL_CARE
HEALTH_AND_BEAUTY_PRODUCTS_AND_SERVICES
DENTAL_SERVICES
DOCTORS_AND_MEDICAL_SERVICES
ELDERLY_OR_OTHER_CARE_HOME
FITNESS_SPORTS_SERVICES
MEDICAL_EQUIPMENT
NURSING_AND_OTHER_CARE_SERVICES
PHARMACEUTICALS
PHARMACY
VITAMINS_AND_DIETARY_SUPPLEMENTS
PUBLIC_GOVERNMENT_SERVICES
PUBLIC_ALL_SERVICES
MAINTENANCE_SERVICES
GOVERNMENT_SERVICES
TELECOMMUNICATIONS_SERVICES
UTILITY_SERVICES
REAL_ESTATE_CONSTRUCTION
ARCHITECTURE
CONSTRUCTION
REAL_ESTATE_DEVELOPMENT
REAL_ESTATE_SALE_PURCHASE_AND_MANAGEMENT
RETAIL_WHOLESALE_MANUFACTURING
AGRICULTURE_SEEDS_PLANTS
FINE_ARTS
ARTS_OTHER
AUTOMOTIVE_SALES_SPARE_PARTS_TRADE
AUTOMOTIVE_MANUFACTURING
CHEMICALS
CLOTHING
ELECTRICAL_PRODUCTS
FIREARMS_WEAPONS_AND_MILITARY_GOODS_SERVICES
HOME_ACCESSORIES_FURNITURE
FINE_JEWELLERY_WATCHES
FASHION_JEWELLERY
HEALTH_AND_BEAUTY_PRODUCTS_AND_SERVICES
LEGAL_HIGHS_AND_RELATED_ACCESSORIES
MACHINERY
PETS
PRECIOUS_STONES_DIAMONDS_AND_METALS
SPORTING_EQUIPMENT
MANUFACTURING_OTHER
RETAIL_WHOLESALE_MARKETPLACE_AUCTION
RETAIL_WHOLESALE_OTHER
TOYS_AND_GAMES
TRAVEL_TRANSPORT_TOUR_AGENCIES
ACCOMMODATION_HOTELS
PASSENGER_TRANSPORT
FREIGHT_TRANSPORT
RIDESHARING_TRANSPORT_SHARING_SERVICES
TRANSPORT
TRAVEL_AGENCIES
TOUR_OPERATORS
TRAVEL_OR_TOUR_ACTIVITIES_OTHER
OTHER
OTHER_NOT_LISTED_ABOVE
Example Request
curl -X PUT \
  https://api.wise-sandbox.com/v2/profiles/123/business-profile \
  -H 'Authorization: Bearer <your api token>' \
  -H 'Content-Type: application/json' \
  -d '{
    "id": "123",
    "businessName": "ABC Logistics Ltd",
    "businessNameInKatakana": null,
    "businessFreeFormDescription": "Biz free form desc",
    "registrationNumber": "12144939",
    "acn": null,
    "abn": null,
    "arbn": null,
    "companyType": "LIMITED",
    "address": {
      "addressFirstLine": "1 A road",
      "city": "London",
      "countryIso2Code": "gb",
      "countryIso3Code": "gbr",
      "postCode": "11111"
    },
    "firstLevelCategory": "CONSULTING_IT_BUSINESS_SERVICES",
    "secondLevelCategory": "DESIGN",
    "operationalAddresses": [
      {
        "addressFirstLine": "1 A road",
        "city": "London",
        "countryIso2Code": "gb",
        "countryIso3Code": "gbr",
        "postCode": "11111"
      }
    ],
    "webpage": "https://abc-logistics.com"
  }'