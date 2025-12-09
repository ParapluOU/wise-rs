# Additional Customer VerificationCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/api-reference/verification

---

Additional Customer Verification
Copy

In certain situations, additional evidence is required to verify customers and ensure we’re compliant with the KYC regulations.

Additional Verification APIs support a list of evidences that can be found here.

If you use the Customer Account with Partner KYC model and your customers are primarily based in the EU, refer to this Onboarding EU customers guide for instructions on how to use these APIs.

If you use the Customer Account with Partner KYC model and you are onboarding high risk business customers based primarily based in the US, refer to this Onboarding High Risk US Businesses guide for instructions on how to use these APIs.

Operations

POST
/v5/profiles/{{profileId}}/additional-verification/upload-evidences
POST
/v3/profiles/{{profileId}}/verification-status/upload-document
GET
/v3/profiles/{{profileId}}/verification-status/required-evidences
Upload Evidences
v5

POST /v5/profiles/{{profileId}}/additional-verification/upload-evidences

Use upload-evidences endpoint to submit evidence for review.

Submitting an evidence that was already uploaded will result in an attempt to update the evidence.

Note that the set of supported fields are different for business and consumer profiles.

Supported fields for personal profiles
accountPurposetext

Allowed values:


MOVING_SAVINGS

SENDING_MONEY_TO_FRIENDS_OR_FAMILY

GENERAL_LIVING_EXPENSES

BUYING_GOODS_OR_SERVICES_ABROAD

PAYING_FOR_MORTGAGE_OR_LOAN

PAYING_BILLS

RECEIVING_SALARY_OR_PENSION

INVESTING

accountPurposeExplanationtext

Additional reasons and specific examples for using Wise account. This is a free-text field.

intendedCountrieslist of text

List of iso3 country codes.

Check the lists of countries you can send money to with Wise.

yearlyAnticipatedVolumetext

Values are in Euros. Allowed values:


0_2350

2351_6000

6001_11500

11501_60000

60001_175000

175001_+

mainSourceOfIncometext

Must be submitted with annualIncome. Allowed values:


SALARY

INVESTMENTS

PENSION

INHERITANCE

LOAN

OTHER

annualIncometext

Must be submitted with mainSourceOfIncome. Values are in Euros. Allowed values:


0_11500

11501_19000

19001_28500

28501_47500

47501_75000

75001_+

incomeExplanationtext

Additional ways you earn money that were not identified in the provided documents. This is a free-text field.

Supported fields for business profiles
businessUseCaselist of text

Allowed values:


INVESTING_IN_FUNDS_STOCKS_BONDS_OR_SIMILAR

DISTRIBUTING_COMPANY_PROFITS_OR_PAYING_DIVIDENDS

PAYING_MORTGAGE_BANK_LOAN_INSURANCE_OR_CREDIT

PAYING_FOR_GOODS_PROPERTIES_OR_SERVICES_ABROAD

PAYING_RENT_OR_UTILITIES

PAYING_SUPPLIERS_CONTRACTORS_OR_EMPLOYEES

PAYING_TAX_ON_PROFIT_OR_PROPERTY

TRANSFER_WITHIN_COMPANY_OR_GROUP

RECEIVE_INVESTMENTS_OR_FUNDS

RECEIVE_PAYMENTS_FROM_CLIENTS

DONATION

OTHER

intendedCountrieslist of text

List of iso3 country codes. Check the lists of countries you can send money to with Wise.

mainSourceOfFundingtext

Allowed values:


REVENUE

BUSINESS_LOAN

FUNDING_AND_SHAREHOLDER_INVESTMENTS

INVESTMENT_INCOME

DONATIONS

GRANTS

OTHER

monthlyAnticipatedVolumetext

Values are in Euros. Allowed values:


0_1200

1201_6000

6001_12000

12001_60000

60001_120000

120001_235000

235001_600000

600001_1200000

1200001_6000000

6000001_12000000

12000001_+

Response
overallStatustext

Status of the entire request. Possible values:


SUCCESS

PARTIAL_SUCCESS

FAIL
resultsarray

Results for each evidence.

results[n].evidenceRequirementKeytext

Name of the evidence.

results[n].resulttext

Result for the evidence. Possible values:


SUCCESS

FAIL
results[n].messagetext

Explanation of result for the evidence.

HTTP status codes
207 - Multi-Status

Evidence(s) successfully uploaded, fully or partially.

400 - Bad Request

Invalid request payload (e.g. no evidence, misspelled evidence, unexpected value).

403 - Forbidden

The client is not authorized to perform this request.

Example payload for Consumer
{
  "accountPurpose" : "MOVING_SAVINGS",
  "intendedCountries" : ["deu", "esp"],
  "yearlyAnticipatedVolume" : "0_2350",
  "mainSourceOfIncome" : "SALARY",
  "annualIncome": "0_11500"
}
Example Payload for Business
{
  "businessUseCase" : ["PAYING_SUPPLIERS_CONTRACTORS_OR_EMPLOYEES", "PAYING_RENT_OR_UTILITIES"],
  "intendedCountries" : ["deu", "esp"],
  "monthlyAnticipatedVolume" : "0_1200",
  "mainSourceOfFunding": "BUSINESS_LOAN"
}
Example request for Consumer
curl -L -X POST \
'https://api.wise-sandbox.com/v5/profiles/{{profileId}}/additional-verification/upload-evidences' \
-H 'Authorization: Bearer <user access token>' \
-H 'Content-Type: application/json' \
-d '{
    "accountPurpose": "MOVING_SAVINGS",
    "intendedCountries": [
        "deu",
        "esp"
    ],
    "yearlyAnticipatedVolume": "0_1200",
    "mainSourceOfIncome": "SALARY",
    "annualIncome": "0_11500"
}'
Example success response
{
  "overallStatus": "SUCCESS",
  "results": [
    {
      "evidenceRequirementKey": "accountPurpose",
      "result": "SUCCESS",
      "message": "SUCCESS"
    },
    {
      "evidenceRequirementKey": "annualIncome",
      "result": "SUCCESS",
      "message": "SUCCESS"
    },
    {
      "evidenceRequirementKey": "intendedCountries",
      "result": "SUCCESS",
      "message": "SUCCESS"
    },
    {
      "evidenceRequirementKey": "mainSourceOfIncome",
      "result": "SUCCESS",
      "message": "SUCCESS"
    },
    {
      "evidenceRequirementKey": "yearlyAnticipatedVolume",
      "result": "SUCCESS",
      "message": "SUCCESS"
    }
  ]
}
Example partial success response
{
  "overallStatus": "PARTIAL_SUCCESS",
  "results": [
    {
      "evidenceRequirementKey": "accountPurpose",
      "result": "SUCCESS",
      "message": "SUCCESS"
    },
    {
      "evidenceRequirementKey": "annualIncome",
      "result": "SUCCESS",
      "message": "SUCCESS"
    },
    {
      "evidenceRequirementKey": "intendedCountries",
      "result": "FAIL",
      "message": "Could not update verification requirement at the moment"
    },
    {
      "evidenceRequirementKey": "mainSourceOfIncome",
      "result": "SUCCESS",
      "message": "SUCCESS"
    },
    {
      "evidenceRequirementKey": "yearlyAnticipatedVolume",
      "result": "SUCCESS",
      "message": "SUCCESS"
    }
  ]
}
Upload Document

POST /v3/profiles/{{profileId}}/verification-status/upload-document

Use upload-document to provide verification documents for review. You can upload multiple files at once.

A valid document must fulfil these requirements:

The document must be clear.
The document needs to be a .jpg, .png., or .pdf file type up to 10MB in size.
Request
filestext

Path of the files to be uploaded.

documentTypetext

Allowed values:


SOURCE_OF_WEALTH

ID_PROOF

ADDRESS_PROOF

TRADING_ADDRESS_PROOF

BUSINESS_AUTH_REP_PROOF_BY_AUTH_LETTER


Read our guides on acceptable documents for each of the following:


Source of wealth - guide

Id Proof - guide

Address Proof - guide

Trading Address Proof - guide

Business Authorisation Letter - guide

Response

Possible HTTP status codes

204 - No Content

Document successfully uploaded.

400 - Bad Request

Document cannot be uploaded in the system in the current format. Check file size and extension.

403 - Forbidden

The client is not authorized to perform this request.

409 - Conflict

Document is already uploaded and can't be re-submitted before a review.

413 - Content Too Large

Requested file size is too large.

415 - Unsupported Media Type

Request payload is in an unsupported format. This endpoint expects content-type to be multipart/form-data with correctly populated boundary value.

Example request
curl -L -X POST \
  'https://api.wise-sandbox.com/v3/profiles/{{profileId}}/verification-status/upload-document' \
  -H 'Authorization: Bearer <user access token>' \
  -H 'Content-Type: application/json' \
  -F 'files=@"/Users/test/file.png"' \
  -F 'documentType="ID_PROOF"'
Required Evidences

GET /v3/profiles/{{profileId}}/verification-status/required-evidences

Fetches the required evidences for a profile to complete additional customer verification.

If one or more evidences are returned, then the customer should submit those evidences with the help of the section below.

Example request
curl -X GET \
  'https://api.wise-sandbox.com/v3/profiles/{{profileId}}/verification-status/required-evidences' \
  -H 'Authorization: Bearer <user access token>'
Example Response
{
  "required_evidences": ["SOURCE_OF_WEALTH", "INCOME", "USE_CASE_COUNTRIES"]
}
List of Supported Evidences

Evidences are pieces of data required by Wise to review and verify customers.

Additional Verification APIs supports the following list of evidences.

Supported evidences for personal profile
ACCOUNT_PURPOSE

Primary purpose of using Wise account.

Submit this using the accountPurpose field in POST /upload-evidences.

ACCOUNT_PURPOSE_EXPLANATION

Additional reasons and specific examples for using Wise account. This is a free-text field.

Submit this using the field accountPurposeExplanation in POST /upload-evidences.

ANNUAL_VOLUME

Submit your yearly anticipated volume using the field yearlyAnticipatedVolume in POST /upload-evidences.

INCOME

Provide your income using the fields annualIncome and mainSourceOfIncome in POST /upload-evidences.

INCOME_EXPLANATION

Additional ways you earn money that were not identified in the provided documents. Provide this explanation using the field incomeExplanation in POST /upload-evidences.

USE_CASE_COUNTRIES

List of iso3 country codes that your customer intend to send and receive money from.

Check the lists of countries you can send money to with Wise.

Submit this using the field intendedCountries in POST /upload-evidences.

SOURCE_OF_WEALTH

A document detailing source of wealth according to specified main source of income.

See the table here for a list of acceptable documents.

Upload the document using POST /upload-document.

ID_PROOF

A document detailing proof of identification for the profile.

Check here for a list of acceptable documents.

Upload the document using POST /upload-document.

ADDRESS_PROOF

A document detailing proof of residential address of the profile owner.

Check here for a list of acceptable documents.

Upload the document using POST /upload-document.

Supported evidences for business profile
SOURCE_OF_FUNDING

Main source of wealth for the business.

Provide source of funding of your business using the field mainSourceOfFunding in POST /upload-evidences.

BUSINESS_MONTHLY_VOLUME

Monthly anticipated volume of your business.

Submit this using the field monthlyAnticipatedVolume in POST /upload-evidences.

BUSINESS_USE_CASE

The main use case for which your business wants to use Wise.

Submit this using the field businessUseCase in POST /upload-evidences.

USE_CASE_COUNTRIES

List of iso3 country codes that your customer intend to send and receive money from.

Check the lists of countries you can send money to with Wise.

Submit this using the field intendedCountries in POST /upload-evidences.

SOURCE_OF_WEALTH

Upload a source of wealth document according on the source of funding provided.

See the table here for a list of acceptable documents.

Upload the document using POST /upload-document.

TRADING_ADDRESS_PROOF

A document detailing proof of operational address.

Check here for a list of acceptable documents.

Upload the document using POST /upload-document.