# ProfileCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/api-reference/profile

---

Profile
Copy

Profiles are connected to a User account and are either personal or business. The requests described here refer to a {{profileId}} or "profile id" - this value can be the business profile ID, or the personal profile ID.

Operations

POST
/v2/profiles/personal-profile
POST
/v2/profiles/business-profile
PUT
/v2/profiles/{{profileId}}/personal-profile
PUT
/v2/profiles/{{profileId}}/business-profile
GET
/v2/profiles/{{profileId}}
GET
/v2/profiles
GET
/v3/profiles/{{profileId}}/business-profile/business-representative
PUT
/v3/profiles/{{profileId}}/business-profile/business-representative
POST
/v3/profiles/{{profileId}}/business-profile/business-representative
POST
/v1/profiles/{{profileId}}/verification-documents
POST
/v1/profiles/{{profileId}}/directors
GET
/v1/profiles/{{profileId}}/directors
PUT
/v1/profiles/{{profileId}}/directors
POST
/v1/profiles/{{profileId}}/ubos
GET
/v1/profiles/{{profileId}}/ubos
PUT
/v1/profiles/{{profileId}}/ubos
DELETE
/v3/profiles/{{profileId}}/trusted-verification
POST
/v1/profiles/{{profileId}}/update-window
DELETE
/v1/profiles/{{profileId}}/update-window
GET
/v1/profiles/{{profileId}}/extension-requirements
POST
/v1/profiles/{{profileId}}/extension-requirements
POST
/v3/profiles/{{profileId}}/verification-status/bank-transfer?source_currencies={{currency_array}}
Profile object
v2
Fields
idinteger

Unique identifier for the profile.

publicIdtext

Publicly accessible identifier for the profile.

userIdinteger

The ID of the user associated with this profile.

typetext

Type of profile, either "PERSONAL" or "BUSINESS".

addressobject

Main registered address of the profile.

address.idinteger

ID of the address.

address.addressFirstLinetext

First line of the address.

address.citytext

City of the address.

address.countryIso2Codetext

Two-letter ISO country code (e.g., "GB").

address.countryIso3Codetext

Three-letter ISO country code (e.g., "gbr").

address.postCodetext

Postal code of the address.

address.stateCodetext (can be null)

State code of the address (can be null for some countries).

emailtext

Primary email address for the profile.

createdAtdate-time

Timestamp when the profile was created (ISO 8601 format).

updatedAtdate-time

Timestamp when the profile was last updated (ISO 8601 format).

avatartext

Link to person avatar image.

currentStatetext

Current status of this profile, one of: "HIDDEN", "VISIBLE", "DEACTIVATED".

contactDetailsobject

Contact information for the profile.

contactDetails.emailtext

Contact email address.

contactDetails.phoneNumbertext

Contact phone number.

firstNametext

First name of the profile holder (for PERSONAL profiles).

lastNametext

Last name of the profile holder (for PERSONAL profiles).

preferredNametext

Preferred name of the profile holder (for PERSONAL profiles).

dateOfBirthyyyy-mm-dd

Date of birth of the profile holder (for PERSONAL profiles).

phoneNumbertext

Phone number of the profile holder (for PERSONAL profiles).

secondaryAddressesarray

An array of secondary addresses associated with the profile.

fullNametext

Full name of the profile holder or business.

businessNametext

Registered business name (for BUSINESS profiles).

registrationNumbertext

Business registration number (for BUSINESS profiles).

descriptionOfBusinesstext

Brief description of the business (for BUSINESS profiles).

webpagetext

Business website URL (for BUSINESS profiles).

companyTypetext

Type of company, e.g., "SOLE_TRADER" (for BUSINESS profiles).

businessFreeFormDescriptiontext

Free-form description of the business activities (for BUSINESS profiles).

firstLevelCategorytext

Primary business category (for BUSINESS profiles).

secondLevelCategorytext

Secondary business category (for BUSINESS profiles).

operationalAddressesarray

An array of operational addresses for the business.

operationalAddresses[n].idinteger

ID of the operational address.

operationalAddresses[n].addressFirstLinetext

First line of the operational address.

operationalAddresses[n].citytext

City of the operational address.

operationalAddresses[n].countryIso2Codetext

Two-letter ISO country code for the operational address.

operationalAddresses[n].countryIso3Codetext

Three-letter ISO country code for the operational address.

operationalAddresses[n].postCodetext

Postal code for the operational address.

operationalAddresses[n].stateCodetext (can be null)

State code for the operational address (can be null).

Personal Profile Object
{
  "type": "PERSONAL",
  "id": 14575282,
  "publicId": "a1b2c3d4-e5f6-7890-1234-567890abcdef",
  "userId": 9889627,
  "address": {
    "id": 36086782,
    "addressFirstLine": "24 Willow Creek Lane",
    "city": "Bristol",
    "countryIso2Code": "GB",
    "countryIso3Code": "gbr",
    "postCode": "BS1 6AE",
    "stateCode": null
  },
  "email": "sarah.jenkins@example.com",
  "createdAt": "2023-01-15T10:30:00",
  "updatedAt": "2025-06-18T14:20:00",
  "avatar": "https://example.com/avatars/sarah_jenkins.png",
  "currentState": "VISIBLE",
  "contactDetails": {
    "email": "sarah.contact@example.com",
    "phoneNumber": "+447700900123"
  },
  "firstName": "Sarah",
  "lastName": "Jenkins",
  "preferredName": "Sal",
  "dateOfBirth": "1985-05-20",
  "phoneNumber": "+447700900456",
  "secondaryAddresses": [],
  "fullName": "Sarah Jenkins"
}
Business Profile Object
{
  "type": "BUSINESS",
  "id": 14599371,
  "publicId": "f0e9d8c7-b6a5-4321-fedc-ba9876543210",
  "userId": 9889627,
  "address": {
    "id": 36152772,
    "addressFirstLine": "15 Tech Hub Studios",
    "city": "Manchester",
    "countryIso2Code": "GB",
    "countryIso3Code": "gbr",
    "postCode": "M1 7JA",
    "stateCode": null
  },
  "email": "info@innovate-solutions.co.uk",
  "createdAt": "2024-03-10T09:00:00",
  "updatedAt": "2025-06-18T14:22:00",
  "contactDetails": {
    "email": "contact@innovate-solutions.co.uk",
    "phoneNumber": "+441617891234"
  },
  "businessName": "Innovate Solutions Ltd",
  "registrationNumber": "SC1234567890ABCD",
  "descriptionOfBusiness": "SOFTWARE_DEVELOPMENT",
  "webpage": "https://www.innovate-solutions.co.uk",
  "companyType": "LIMITED_COMPANY",
  "businessFreeFormDescription": "We create cutting-edge software for businesses.",
  "firstLevelCategory": "TECHNOLOGY",
  "secondLevelCategory": "SOFTWARE_SERVICES",
  "operationalAddresses": [
    {
      "id": 36152773,
      "addressFirstLine": "Unit 5, Innovation Park",
      "city": "Leeds",
      "countryIso2Code": "GB",
      "countryIso3Code": "gbr",
      "postCode": "LS1 4YZ",
      "stateCode": null
    }
  ],
  "fullName": "Innovate Solutions Ltd"
}
Create a Personal Profile
v2

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
Create a Business Profile
v3 (beta)

POST /v3/profiles/business-profile (beta)

Create a business profile and its authorized representative in a single request.

This request accepts an optional field in the header, `X-idempotence-uuid`. This should be unique for each Profile you create. In the event that the request fails, you should use the same value again when retrying. If the `X-idempotence-uuid` header is not provided and a Profile already exists, then you will receive a response with an HTTP status code `409`.
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

businessRepresentative.businessRepresentativeIdtext

ID of a Business Representative. This can be obtained from a previous call to create a business profile, in which case the same bBusiness Representative is linked to the current business effectively sharing it across multiple businesses. When the Business Representative ID is provided in this way it is the only field required for the Business Representative and none of the other fields should be provided.

businessRepresentative.firstNametext

First name of the person representating the business (including middle names). Required (unless Business Representative ID is provided)

businessRepresentative.lastNametext

Last name of the person representating the business. Required (unless Business Representative ID is provided)

businessRepresentative.preferredNametext

Preferred first name of the person representing the business, if different to the legal first name.

businessRepresentative.address.addressFirstLinetext

First line of address of the person representing the business. Required (unless Business Representative ID is provided)

businessRepresentative.address.citytext

City of the person representing the business. Required (unless Business Representative ID is provided)

businessRepresentative.address.countryIso3Codetext

3 Letter country code (lower case) of the person representing the business. Required (unless Business Representative ID is provided)

businessRepresentative.address.postCodetext

Postal code of the person representing the business

businessRepresentative.address.stateCode (max 5 chars)text

State code of the person representing the business. Required for US, CA, BR and AU addresses (unless Business Representative ID is provided)

businessRepresentative.contactDetails.emailemail

Contact email address of the person representing the business. Please speak with your integration account manager for details on how customer communication is handled for your integration. Required (unless Business Representative ID is provided)

businessRepresentative.contactDetails.phoneNumbertext

Contact phone number in international phone number format, example: "+1408XXXXXXX" of the person representing the business. Please speak with your integration account manager for details on how customer communication is handled for your integration. Required (unless Business Representative ID is provided)

businessRepresentative.dateOfBirthyyyy-mm-dd

Date of birth of the person representing the business. Required (unless Business Representative ID is provided)

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
Example Request with Business Representative details
curl -X POST \
  https://api.wise-sandbox.com/v3/profiles/business-profile \
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
    "webpage": "https://abc-logistics.com",
    "businessRepresentative": {
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
      "dateOfBirth": "1977-07-01",
      "contactDetails": {
        "email": "o.wilson@example.com",
        "phoneNumber": "+3725064992"
      }
    }
  }'
Example Request with Business Representative ID
curl -X POST \
  https://api.wise-sandbox.com/v3/profiles/business-profile \
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
    "webpage": "https://abc-logistics.com",
    "businessRepresentative": {
      "businessRepresentativeId": 123
    }
  }'
Update a Personal Profile
v2

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
Update a Business Profile
v2

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
Retrieve a profile by ID

GET /v2/profiles/{{profileId}}

Get profile info by ID.

Example Request
curl -X GET \
  https://api.wise-sandbox.com/v2/profiles/{{profileId}} \
  -H 'Authorization: Bearer <your api token>'
List profiles for a user account

GET /v2/profiles

List of all profiles belonging to user.

Response

An array of profile objects will be returned. Note that there might be more than one business profile returned in the response.

Example Request
curl -X GET \
  https://api.wise-sandbox.com/v2/profiles \
  -H 'Authorization: Bearer <your api token>'
Retrieve the Business Representative of a Business Profile

GET /v3/profiles/{{business-profile-id}}/business-profile/business-representative (beta)

Get Business Representative info by Business Profile ID.

Example Request
curl -X GET \
  https://api.wise-sandbox.com/v3/profiles/{{business-profile-id}}/business-profile/business-representative \
  -H 'Authorization: Bearer <your api token>'
Update the Business Representative of a Business Profile

PUT /v3/profiles/{{business-profile-id}}/business-profile/business-representative (beta)

Update the existing Business Representative details for the specified Business Profile.

If the Business Representative has been verified then there are restrictions on what information is allowed to change.
If this Business Representative is shared across multiple Business Profiles, then changes will be reflected for all of the Business Profiles.
Request Fields
updateContexttext

UPDATE_DETAILS

firstNametext

First name of the person representating the business (including middle names).

lastNametext

Last name of the person representating the business.

preferredNametext

Preferred first name of the person representing the business, if different to the legal first name.

address.addressFirstLinetext

First line of address of the person representing the business.

address.citytext

City of the person representing the business.

address.countryIso3Codetext

3 Letter country code (lower case) of the person representing the business.

address.postCodetext

Postal code of the person representing the business

address.stateCode (max 5 chars)text

State code of the person representing the business.

contactDetails.emailemail

Contact email address of the person representing the business. Please speak with your integration account manager for details on how customer communication is handled for your integration.

contactDetails.phoneNumbertext

Contact phone number in international phone number format, example: "+1408XXXXXXX" of the person representing the business. Please speak with your integration account manager for details on how customer communication is handled for your integration.

dateOfBirthyyyy-mm-dd

Date of birth of the person representing the business.

actorEmailtext

Email of the actor

Example Request - Update Details
curl -X PUT \
  https://api.wise-sandbox.com/v3/profiles/123/business-profile/business-representative \
  -H 'Authorization: Bearer <your api token>' \
  -H 'Content-Type: application/json' \
  -d '{
    "updateContext": "UPDATE_DETAILS",
    "contactDetails": {
      "email": "o.wilson@example.com",
      "phoneNumber": "+3725064992"
    }
  }'
Submit a new Business Representative for a Business Profile

PUT /v3/profiles/{{business-profile-id}}/business-profile/business-representative (beta)

Transfer authority of the specified Business Profile to a new Business Representative. This will overwrite the existing Business Representative. The new Business Representative will need to be verified for KYC purposes.

Request Fields
updateContexttext

TRANSFER_OF_AUTHORISATION

firstNametext

First name of the person representating the business (including middle names). Required

lastNametext

Last name of the person representating the business. Required

preferredNametext

Preferred first name of the person representing the business, if different to the legal first name. Required

address.addressFirstLinetext

First line of address of the person representing the business. Required

address.citytext

City of the person representing the business. Required

address.countryIso3Codetext

3 Letter country code (lower case) of the person representing the business. Required

address.postCodetext

Postal code of the person representing the business

address.stateCode (max 5 chars)text

State code of the person representing the business. Required for US, CA, BR and AU addresses

contactDetails.emailemail

Contact email address of the person representing the business. Please speak with your integration account manager for details on how customer communication is handled for your integration. Required

contactDetails.phoneNumbertext

Contact phone number in international phone number format, example: "+1408XXXXXXX" of the person representing the business. Please speak with your integration account manager for details on how customer communication is handled for your integration. Required

dateOfBirthyyyy-mm-dd

Date of birth of the person representing the business. Required

Example Request - Transfer of Authorisation
curl -X PUT \
  https://api.wise-sandbox.com/v3/profiles/123/business-profile/business-representative \
  -H 'Authorization: Bearer <your api token>' \
  -H 'Content-Type: application/json' \
  -d '{
    "updateContext": "TRANSFER_OF_AUTHORISATION",
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
    "dateOfBirth": "1977-07-01",
    "contactDetails": {
      "email": "o.wilson@example.com",
      "phoneNumber": "+3725064992"
    }
  }'
Create an identification document for a profile

POST /v1/profiles/{{profileId}}/verification-documents

Add identification document details to user profile. Applicable to personal profiles (not business) only.
Returns empty result if successful.

When sending a social security number (SSN) only type and uniqueIdentifier (only 9 digits no letters or symbols) are required.

Request Fields
firstNametext

Person first name in document.

lastNametext

Person last name in document.

type (conditional)text

Document type. Allowed Values:

DRIVERS_LICENCE
IDENTITY_CARD
GREEN_CARD
MY_NUMBER
PASSPORT
SSN
EMIRATES_EMPLOYER
EMIRATES_PLACE_OF_BIRTH
CPF_CNPJ
FINANCIAL_CAPACITY_BR
OTHER
uniqueIdentifier (required)text

Document number or value. Must be digits only when SSN or FINANCIAL_CAPACITY_BR. Max 30 characters.

issueDateyyyy-mm-dd

Document issue date.

issuerCountrytext

Issued by country code. For example "US".

issuerStatetext

Issued by state code. For example "NY".

expiryDateyyyy-mm-dd

Document expiry date.

nationalitytext

2 characters ISO country code.

employerNametext

The name of the employer. Type must be EMIRATES_EMPLOYER.

employerCitytext

The city of the employer. Type must be EMIRATES_EMPLOYER.

employerCountrytext

2 characters ISO country code. Type must be EMIRATES_EMPLOYER.

birthCitytext

The city of birth of the customer. Type must be EMIRATES_PLACE_OF_BIRTH

birthCountrytext

2 characters ISO country code. Type must be EMIRATES_PLACE_OF_BIRTH

Example Request
curl -X POST \
  https://api.wise-sandbox.com/v1/profiles/{{profileId}}/verification-documents \
  -H 'Authorization: Bearer <your api token>' \
  -H 'Content-Type: application/json' \
  -d '{ 
    "firstName": "Oliver",
    "lastName": "Wilson",
    "type": "IDENTITY_CARD",
    "uniqueIdentifier": "AA299822313",
    "issueDate": "2017-12-31",
    "issuerCountry": "EE",
    "issuerState": "",
    "expiryDate": "2027-12-31"
  }'
Request Payload - CPF or CNPJ
{
  "type": "CPF_CNPJ",
  "uniqueIdentifier": "938.936.652-69"
}
Request Payload - Financial Capacity Brazil
{
  "type": "FINANCIAL_CAPACITY_BR",
  "uniqueIdentifier": "150000.00"
}
Example Response
{
  "errorMessage": null,
  "success": true
}
Create a business director for a profile

POST /v1/profiles/{{business-profile-id}}/directors

Adds new directors to the business profile.

Returns the list of all directors associated with the business profile.

Director Object Fields
idinteger

ID of the director. Automatically set.

firstNametext

Director first name

lastNametext

Director last name

dateOfBirthyyyy-mm-dd

Date of birth

countryOfResidenceIso3Codetext

3 character country code

Example Request
curl -X POST \
  https://api.wise-sandbox.com/v1/profiles/{{business-profile-id}}/directors \
  -H "Authorization: Bearer <your api token>" \
  -H "Content-Type: application/json" \
  -d '[
    {
      "firstName": "John",
      "lastName": "Doe",
      "dateOfBirth": "1982-05-20",
      "countryOfResidenceIso3Code": "usa"
    },
    {
      "firstName": "Jane",
      "lastName": "Doe",
      "dateOfBirth": "1981-12-07",
      "countryOfResidenceIso3Code": "usa"
    }
  ]'
List business directors for a profile

GET /v1/profiles/{{business-profile-id}}/directors

Returns the list of director objects associated with the business profile.

Example Request
curl -X GET \
  https://api.wise-sandbox.com/v1/profiles/{{business-profile-id}}/directors \
  -H 'Authorization: Bearer <your api token>'
Example Response
[
  {
    "id": 10,
    "firstName": "John",
    "lastName": "Doe",
    "dateOfBirth": "1982-05-20",
    "countryOfResidenceIso3Code": "usa"
  },
  {
    "id": 11,
    "firstName": "Jane",
    "lastName": "Doe",
    "dateOfBirth": "1981-12-07",
    "countryOfResidenceIso3Code": "usa"
  }
]
Update business directors for a profile

PUT /v1/profiles/{{business-profile-id}}/directors

Overrides directors in the business profile. Returns the list of all directors associated with the business profile.

Example Request
curl -X PUT \
  https://api.wise-sandbox.com/v1/profiles/{{business-profile-id}}/directors \
  -H 'Authorization: Bearer <your api token>' \
  -H 'Content-Type: application/json' \
  -d '[
    {
      "firstName": "John",
      "lastName": "Doe",
      "dateOfBirth": "1982-05-20",
      "countryOfResidenceIso3Code": "usa"
    },
    {
      "firstName": "Jane",
      "lastName": "Doe",
      "dateOfBirth": "1981-12-07",
      "countryOfResidenceIso3Code": "usa"
  }
  ]'
Example Response
[
  {
    "id": 14,
    "firstName": "John",
    "lastName": "Doe",
    "dateOfBirth": "1982-05-20",
    "countryOfResidenceIso3Code": "usa"
  },
  {
    "id": 15,
    "firstName": "Jane",
    "lastName": "Doe",
    "dateOfBirth": "1981-12-07",
    "countryOfResidenceIso3Code": "usa"
  }
]
Create a business ultimate owner for a profile

POST /v1/profiles/{{business-profile-id}}/ubos

Adds new ultimate beneficial owners to the business profile. Returns the list of all ultimate beneficial owners associated with the business profile.

Note that in some cases, we do not require the ownershipPercentage. In these cases, null should be passed as the value.

Ultimate Business Owner Object
nametext

Owner full name

dateOfBirthyyyy-mm-dd

Date of birth

countryOfResidenceIso3Codetext

3 character country code

addressFirstLinetext

First line of address

postCodetext

Address post code

ownershipPercentageinteger

Percentage of ownership

Example Request
curl -X POST \
  https://api.wise-sandbox.com/v1/profiles/{{business-profile-id}}/ubos \
  -H 'Authorization: Bearer <your api token>' \
  -H 'Content-Type: application/json' \
  -d '[
    {
      "name": "John Doe",
      "dateOfBirth": "1982-05-20",
      "countryOfResidenceIso3Code": "usa",
      "addressFirstLine": "123 Fake St",
      "postCode": "FK 12345",
      "ownershipPercentage": 30
    }
  ]'
Example Response
[
  {
    "id": "013ab1c2688d0185b582ee7e0bcb28b2",
    "name": "John Doe",
    "dateOfBirth": "1982-05-20",
    "countryOfResidenceIso3Code": "usa",
    "addressFirstLine": "123 Fake St",
    "postCode": "FK 12345",
    "ownershipPercentage": 30
  }
]
List business ultimate owners for a profile

GET /v1/profiles/{{business-profile-id}}/ubos

Returns the list of all ultimate beneficial owners associated with the business profile.

Example Request
curl -X GET \
  https://api.wise-sandbox.com/v1/profiles/{{business-profile-id}}/ubos \
  -H 'Authorization: Bearer <your api token>' 
Example Response
[
  {
    "id": "013ab1c2688d0185b582ee7e0bcb28b2",
    "name": "John Doe",
    "dateOfBirth": "1982-05-20",
    "countryOfResidenceIso3Code": "usa",
    "addressFirstLine": "123 Fake St",
    "postCode": "FK 12345",
    "ownershipPercentage": 30
  },
  {
    "id": "912ce3f31c8b3a10572137e78417caa3",
    "name": "Jane Doe",
    "dateOfBirth": "1981-12-07",
    "countryOfResidenceIso3Code": "usa",
    "addressFirstLine": "125 Fake St",
    "postCode": "FK 12545",
    "ownershipPercentage": 70
  }
]
Update business ultimate owners for a profile

PUT /v1/profiles/{{business-profile-id}}/ubos

Overrides ultimate beneficial owners in the business profile.

Returns the list of all ultimate beneficial owners associated with the business profile.

Example Request
curl -X PUT \
  https://api.wise-sandbox.com/v1/profiles/{{business-profile-id}}/ubos \
  -H 'Authorization: Bearer <your api token>' \
  -H 'Content-Type: application/json' \
  -d '[
    {
      "name": "John Doe",
      "dateOfBirth": "1982-05-20",
      "countryOfResidenceIso3Code": "usa",
      "addressFirstLine": "123 Fake St",
      "postCode": "FK 12345",
      "ownershipPercentage": 30
    },
    {
      "name": "Jane Doe",
      "dateOfBirth": "1981-12-07",
      "countryOfResidenceIso3Code": "usa",
      "addressFirstLine": "125 Fake St",
      "postCode": "FK 12545",
      "ownershipPercentage": 70
    }
  ]'
Example Response
[
  {
    "id": "ff01cf3f206b40c090a14a1e51163e9e",
    "name": "John Doe",
    "dateOfBirth": "1982-05-20",
    "countryOfResidenceIso3Code": "usa",
    "addressFirstLine": "123 Fake St",
    "postCode": "FK 12545",
    "ownershipPercentage": 30
  },
  {
    "id": "c36b687d28ad44ad8c3864411f5f2612",
    "name": "Jane Doe",
    "dateOfBirth": "1981-12-07",
    "countryOfResidenceIso3Code": "usa",
    "addressFirstLine": "125 Fake St",
    "postCode": "FK 12545",
    "ownershipPercentage": 70
  }
]
Remove trusted verification from a profile

DELETE /v3/profiles/{{profileId}}/trusted-verification

This endpoint allows partners to remove the verification that was given to the profile through them creating the profile. This does not delete a profile nor archive it, it simply removes the trusted verification from that partner.

Note that this uses a client-credentials-token and not a user access_token for authentication.

Example Request
curl -X DELETE \
  https://api.wise-sandbox.com/v3/profiles/{{profileId}}/trusted-verification \
  -H 'Authorization: Bearer <client-credentials-token>' 
Example Response - 204
No Content
Open an update window for a profile

POST /v1/profiles/{{profileId}}/update-window

Opens the update window for updating the profile information: details, addresses, directors, owners, others.

Example Request
curl -X POST \
  https://api.wise-sandbox.com/v1/profiles/{{profileId}}/update-window \
  -H 'Authorization: Bearer <your api token>' 
Close an update window for a profile

DELETE /v1/profiles/{{profileId}}/update-window

Deletes the update window for updating the profile.

Example Request
curl -X DELETE \
  https://api.wise-sandbox.com/v1/profiles/{{profileId}}/update-window \
  -H 'Authorization: Bearer <your api token>' 
Retrieve profile extension requirements for a profile

GET /v1/profiles/{{profileId}}/extension-requirements

This endpoint is deprecated. Please check the Additional Verification endpoints for providing additional verification details for a profile.

After having a profile created, in some situations we can need more specific information about it. In order to know which fields are required for a given profile, and to send the information over, we expose a few endpoints:

GET /v1/profiles/{{profileId}}/extension-requirements
POST /v1/profiles/{{profileId}}/extension-requirements

and

POST /v1/profiles/{{profileId}}/extensions
GET /v1/profiles/{{profileId}}/extensions

The GET and POST profile extension-requirements endpoints help you to figure out which fields are required to create a valid profile for different regions. You can use this data to build a dynamic user interface on top of these endpoints.

The POST and GET profile extensions endpoints allow you to send the extra profile information and retrieve it, respectively.

This format for dynamic forms is the same as the one used for recipient creation. See Recipient.Requirements

This is a step-by-step guide on how these endpoints work.

Using profile extension requirements

First create a profile. See User Profiles Create (Personal) and User Profiles Create (Business)

Call GET /v1/profiles/{{profileId}}/extension-requirements to get the list of fields you need to fill with values in the "details" section for adding information that will make a profile valid.

Some fields require multiple levels of fields in the details request. This should be handled by the client based on the refreshRequirementsOnChange field. A top level field can have this field set to true, indicating that there are additional fields required depending on the selected value. To manage this you should create a request with all of the initially requested data and call the POST extension-requirements endpoint. You will be returned a response similar the previously returned data from GET extension-requirements but with additional fields.

Once you have built your full profile extension details object you can add it to add information to the profile.

Building an user interface

When requesting the form data from the extension-requirements endpoint, the response defines different types of extensions that can be added. Each extension type then has multiple fields describing the form elements required to be shown to collect information from the user. Each field will have a type value, these tell you the field type that your front end needs to render to be able to collect the data. A number of field types are permitted, these are:

type	UI element
text	A free text box
select	A selection box/dialog
radio	A radio button choice between options
date	A text box with a date picker

Example data is also included in each field which should be shown to the user, along with a regex or min and max length constraints that should be applied as field level validations. You can optionally implement the dynamic validation using the validationAsync field, however these checks will also be done when a completed profile extension is submitted to POST /v1/profiles/{{profileId}}/extensions.

Response
typetext

"profile-extensions-requirements"

fields[n].nametext

Field description

fields[n].group[n].keytext

Key is name of the field you should include in the JSON

fields[n].group[n].typetext

Display type of field (e.g. text, select, etc)

fields[n].group[n].refreshRequirementsOnChangeboolean

Tells you whether you should call POST extension-requirements once the field value is set to discover required lower level fields.

fields[n].group[n].requiredboolean

Indicates if the field is mandatory or not

fields[n].group[n].displayFormattext

Display format pattern.

fields[n].group[n].exampletext

Example value.

fields[n].group[n].minLengthinteger

Min valid length of field value.

fields[n].group[n].maxLengthinteger

Max valid length of field value.

fields[n].group[n].validationRegexptext

Regexp validation pattern.

fields[n].group[n].validationAsynctext

Deprecated. This validation will instead be performed when submitting the request.

fields[n].group[n].valuesAllowed[n].keytext

List of allowed values. Value key

fields[n].group[n].valuesAllowed[n].nametext

List of allowed values. Value name.

Example Request - Get Extension Requirements
curl -X GET \
  https://api.wise-sandbox.com/v1/profiles/{{profileId}}/extension-requirements \
  -H 'Authorization: Bearer <your api token>'
Example Response
[
  {
    "type": "profile-extensions-requirements",
    "usageInfo": null,
    "fields": [
      {
        "name": "Tell us what you're using TransferWise for",
        "group": [
          {
            "key": "accountPurpose",
            "name": "Account Purpose",
            "type": "select",
            "refreshRequirementsOnChange": false,
            "required": true,
            "displayFormat": null,
            "example": null,
            "minLength": null,
            "maxLength": null,
            "validationRegexp": null,
            "validationAsync": null,
            "valuesAllowed": [
              {
                "key": "CONTRIBUTING_TO_PERSONAL_SAVINGS",
                "name": "Contributing to personal savings"
              },
              {
                "key": "GENERAL_MONTHLY_LIVING_EXPENSES",
                "name": "General monthly living expenses"
              },
              {
                "key": "INVESTING_IN_FUNDS_STOCKS_BONDS_OPTIONS_FUTURES_OR_OTHER",
                "name": "Investing in funds stocks bonds options futures or other"
              },
              {
                "key": "PAYING_FOR_GOODS_OR_SERVICES_ABROAD",
                "name": "Paying for goods or services abroad"
              },
              {
                "key": "PAYING_RENT_MORTGAGE_BANK_LOAN_INSURANCE_CREDIT",
                "name": "Paying rent mortgage bank loan insurance credit"
              },
              {
                "key": "PAYING_RENT_UTILITIES_OR_PROPERTY_CHARGES",
                "name": "Paying rent utilities or property charges"
              },
              {
                "key": "RECEIVE_SALARY_IN_DIFFERENT_CURRENCY",
                "name": "Receive salary in different currency"
              },
              {
                "key": "RECEIVE_PENSION_IN_DIFFERENT_CURRENCY",
                "name": "Receive pension in different currency"
              },
              {
                "key": "SENDING_MONEY_REGULARLY_TO_FAMILY",
                "name": "Sending money regularly to family"
              },
              {
                "key": "SENDING_MONEY_TO_MY_OWN_ACCOUNT_TO_BENEFIT_FROM_EXHCANGE_RATE",
                "name": "Sending money to my own account to benefit from exchange rate"
              }
            ]
          }
        ]
      }
    ]
  }
]
Example Request - Post Extension Details
curl -X POST \
  https://api.wise-sandbox.com/v1/profiles/{{profileId}}/extensions \
  -H 'Authorization: Bearer <your api token>' \
  -H 'Content-Type: application/json' \
  -d '{
    "details": {
      "accountPurpose": "SENDING_MONEY_REGULARLY_TO_FAMILY"
    }
  }'
Check profile verification status

You can check the verification status of a profile using the following API. This is a profile-specific API resource which should be accessed using an access token acquired for the profile.

Response
source_currencytext

ISO 4217 currency code.

current_statusverification status code (see below)

Current verification status

maximum_entitled_amountdecimal

Maximum entitled amount that can be transferred with the user's given verification state.

Guide on how to process the API response
current_status	maximum_entitled_amount	How to process
verified	any value	Profile is verified. maximum_entitled_amount field can be ignored (note that some regional limits may still apply).
not_verified	0	Profile is not verified. Any payments from the user will be delayed by verification.
not_verified	> 0	Profile is not verified yet but the user is allowed to make payments up to a certain limit. Once the limit is reached, following payments might be delayed because of additional verification requirements. Please note that the limit is cumulative and not per transfer. This state might allow some users to experience the product immediately even if the verification process isn't complete yet.
Verification status code	Description
verified	The profile is sufficiently verified to start making payments (note that some regional limits may still apply)
not_verified	The profile is currently awaiting verification or there are pending issues with the verification process

Please note that we do not expose any finer details of customer verification.

Example Request
curl -X POST \
  'https://api.wise-sandbox.com/v3/profiles/{{profileId}}/verification-status/bank-transfer?source_currencies={{currency_array}}' \
  -H "Authorization: Bearer <your API token>"
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