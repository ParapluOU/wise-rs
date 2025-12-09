# Onboarding CustomersCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/send-money/use-cases/embedded/onboarding-customers

---

Onboarding Customers
Copy

In order to onboard a customer, you need to create one profile for a person or business.

Once the profile is created you may need to supply additional verification documents (see Additional Customer Verification).

There are some notable fields in the Profile creation APIs that are important for this type of integration:

The contactDetails fields are mandatory, we require this information for KYC and other mandatory contact scenarios. We do not use these details for marketing.
The externalCustomerId is an optional field but is recommended so we can offer ways to look up profiles for a customer in future.
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