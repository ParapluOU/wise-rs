# UserCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/api-reference/user

---

User
Copy

In our API, a User serves as the primary entity and can possess multiple Profiles to represent different contexts or settings. Specifically, a User can have one personal Profile and multiple business Profiles. Each Profile - whether personal or business - can have its own multi-currency account, enabling transactions across various currencies. This hierarchical structure allows for flexible management of user settings and financial operations, accommodating both personal and business needs.

User
Personal Profile
Business Profile
Multi Currency Account
Multi Currency Account

Operations

GET
/v1/me
GET
/v1/users/{{userId}}
POST
/v1/user/signup/registration_code
POST
/v1/users/exists
PUT
/v1/users/{{userId}}/contact-email
GET
/v1/users/{{userId}}/contact-email
The User resource
Fields
idinteger

userId

nametext

User's full name

emailtext

User's email

activeboolean

If user is active or not

details.firstNametext

User's first name

details.lastNametext

User's lastname

details.phoneNumbertext

Phone number

details.dateOfBirthYYYY-MM-DD

Date of birth

details.occupationtext

Person's occupation

details.avatartext

Link to person avatar image.

details.primaryAddressinteger

Address object ID to use in addresses endpoints

details.address.countryCodetext

Address country code in 2 digits. "US" for example

details.address.firstLinetext

Address first line

details.address.postCodetext

Address post code

details.address.citytext

Address city name

details.address.statetext

Address state code State code. Required if country is US, CA, AU, BR.

details.address.occupationtext

User occupation. Required for US, CA, JP

User Object
{
  "id": 101,
  "name": "Example Person",
  "email": "person@example.com",
  "active": true,
  "details": {
    "firstName": "Example",
    "lastName": "Person",
    "phoneNumber": "+37111111111",
    "occupation": "",
    "address": {
      "city": "Tallinn",
      "countryCode": "EE",
      "postCode": "11111",
      "state": "",
      "firstLine": "Road 123"
    },
    "dateOfBirth": "1977-01-01",
    "avatar": "https://lh6.googleusercontent.com/photo.jpg",
    "primaryAddress": 111
  }
}
Retrieve current user by token

GET /v1/me

Get authenticated user details for the user's token submitted in the Authorization header. Response includes also personal user's profile info.

Response

Returns a user object

Example Request
curl -X GET \
  https://api.wise-sandbox.com/v1/me \
  -H 'Authorization: Bearer <your api token>' 
Retrieve a user by Id

GET /v1/users/{{userId}}

Get authenticated user details by user ID. Response includes also personal user's profile info.

Response

Returns a user object

Example Request
curl -X GET \
  https://api.wise-sandbox.com/v1/users/{{userId}} \
  -H 'Authorization: Bearer <your api token>' 
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
Check User Exists

POST /v1/users/exists

Wise uses email address as unique identifier for users. If email has already been used by a user, it cannot be reused to create a new user.

Note that this uses a client-credentials-token and not a user access_token for authentication.

Request
emailemail

User's email address

Response
existsboolean

Email has already exist

Example Request
curl -X POST \
  https://api.wise-sandbox.com/v1/users/exists \
  -H 'Authorization: Bearer <client credentials token>' \
  -H 'Content-Type: application/json' \
  -d '{
    "email": "test@wise.com"
  }'
Example Response (Success: 200 - User is created successfully)
{
  "exists": true
}
Set a contact email address

PUT /v1/users/{{userId}}/contact-email

Sets a contact email address. The contact email address is used to send notifications to users who have been registered with a dummy email address.

Request
emailemail

Contact email address

Response
emailemail

Contact email address

Example Request
curl -X PUT \
  https://api.wise-sandbox.com/v1/users/{{userId}}/contact-email \
  -H 'Authorization: Bearer <your api token>' \
  -H 'Content-Type: application/json' \
  -d '{
    "email": "new-user@email.com"
  }'
Example Response
{
  "email": "new-user@email.com"
}
Retrieve a contact email address

GET /v1/users/{{userId}}/contact-email

Retrieves a contact email address.

Response

Returns a contact email object.

Response
emailemail

Contact email address

Example Request
curl -X GET \
  https://api.wise-sandbox.com/v1/users/{{userId}}/contact-email \
  -H 'Authorization: Bearer <your api token>' 
Example Response
{
  "email": "new-user@email.com"
}