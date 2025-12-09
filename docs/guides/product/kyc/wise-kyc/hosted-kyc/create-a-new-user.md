# Create a new userCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/kyc/wise-kyc/hosted-kyc/create-a-new-user

---

Create a new user
Copy

Creating a new user means adding your customer's details to Wise, and then making a profile for them with which they can authorise transfers and access their balance. These steps are a quick guide to support the implementation of Hosted KYC.

To create a new user:

Retrieve your client credentials token by calling the Retrieve Client Credentials endpoint.
Create a user for your customer using Create User endpoint.
Retrieve access tokens for the user by using Retrieve User Tokens endpoint.
Create a personal profile for the user by using Create Personal Profile endpoint. If you are onboarding a business, this personal profile would be the Business Representative.
If you are onboarding a business, create a business profile by using Create Business Profile endpoint.
If you are onboarding a business, you may optionally add:
Business directors Create business directors
Ultimate beneficial owners (UBOs) of the business Create UBOs
Retrieve a client credentials token

POST /oauth/token

Obtain access_token based on your API client credentials.

Request

Use Basic Authentication with your api-client-id/api-client-secret as the username/password. The body of the request must be sent as x-www-form-urlencoded.

grant_typetext

"client_credentials"

Response

Returns a client credentials token object

Example Request
curl \
  https://api.wise-sandbox.com/oauth/token \
  -u '<client id>:<client secret>' \
  -d 'grant_type=client_credentials'
Create a user with a registration code

POST /v1/user/signup/registration_code

Wise uses email address as unique identifier for users. If email is new (there is no active user already) then new user will be created.

When you are submitting an email which already exists amongst our users then you will get a warning that "You’re already a member. Please login". If user already exists then you need to redirect to "Get user authorization" webpage.

Request
emailemail

New user's email address

registrationCodetext, min length is 32 chars

Randomly generated registration code that is unique to this user and request. At least 32 characters long. You need to store registration code to obtain access token on behalf of this newly created user in next step. Please apply the same security standards to handling registration code as if it was a password.

language (Optional)text, 2 chars

User default language for UI and email communication. Allowed values EN, US, PT, ES, FR, DE, IT, JA, RU, PL, HU, TR, RO, NL, HK. Default value EN.

Response
idinteger

userId

nametext

User full name. Empty.

emailtext

Customer email

activeboolean

true

detailsgroup

User details. Empty.

Example Request
curl -X POST \
  https://api.wise-sandbox.com/v1/user/signup/registration_code \
  -H 'Authorization: Bearer <client credentials token>' \
  -H 'Content-Type: application/json' \
  -d '{
    "email": "user@email.com",
    "registrationCode": <unique guid for the user>,
    "language": "EN"
  }'
Example Response (Success: 200 - User is created successfully)
{
  "id": 12345,
  "name": null,
  "email": "new.user@domain.com",
  "active": true,
  "details": null
}
Example Response (Failure: 409 - User already exists)
{
  "errors": [
    {
      "code": "NOT_UNIQUE",
      "message": "You’re already a member. Please login",
      "path": "email",
      "arguments": [
        "email",
        "class com.transferwise.fx.api.ApiRegisterCommand",
        "existing.user@domain.com"
      ]
    }
  ]
}
Retrieve user tokens with registration code

POST /oauth/token

You can now use registration code to obtain user access token and refresh token.

Request
grant_typetext

"registration_code"

emailemail

New user's email address

client_idtext

Your API client_id

registration_codetext

registrationCode

Response

Returns a user tokens object

Example Request
curl \
  https://api.wise-sandbox.com/oauth/token \
  -u '<client id>:<client secret>' \
  -d 'grant_type=registration_code' \
  -d 'client_id=<client id>' \
  -d 'email=<user email>' \
  -d 'registration_code=<registration code used to create user>'
Example Response (Failure: 401 - User reclaimed the account or invalid registration code used)
{
  "error": "invalid_grant",
  "error_description": "Invalid user credentials."
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