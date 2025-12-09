# Spend ControlsCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/api-reference/spend-controls

---

Spend Controls
Copy

An authorization rule dictates whether transactions should be declined or approved based on a pre-determined set of rules. A transaction can only pass if it satisfies all the applied rules.

Creating a rule has no practical implication until it is applied. Applying a rule results in the authorization rule being applied to all cards transactions.

A rule is scoped at the application level, it means that an API token needs to be used to call our authorization rule endpoints. Follow the client_credentials oauth flow to retrieve an API token.

Operations

POST
/v3/spend/applications/{{clientId}}/spend-controls/rules
POST
/v3/spend/applications/{{clientId}}/spend-controls/rules/apply
POST
/v3/spend/applications/{{clientId}}/spend-controls/rules/unapply
GET
/v3/spend/applications/{{clientId}}/spend-controls/rules/applied
GET
/v3/spend/applications/{{clientId}}/spend-controls/rules
DELETE
/v3/spend/applications/{{clientId}}/spend-controls/rules/{{ruleId}}
GET
/v3/spend/profiles/{{profileId}}/spending-limits
The Rule resource

The Rule resource returns information on the existing rules that have been created/applied.

An ALLOW rule permits only the transactions that match the specified criteria and blocks all others. For instance, a rule allowing SGD transactions will block all transactions that are not in SGD

Object
idinteger

The unique ID for the authorisation rule

typestring

The type of authorization rule. One of MCC or CURRENCY

operationstring

Determines whether the transactions should be allowed or blocked. One of ALLOW or BLOCK

descriptionstring

The description of the authorization rule

valueslist of values

A list of values based on the type of rule configured

Rule Resource
{
  "id": 123,
  "description": "my authorization rule",
  "type": "MCC",
  "operation": "BLOCK",
  "values": [
    "1234",
    "5678"
  ]
}
Creating an authorization rule

Creates an authorization rule. It won't be enabled unless it is applied.

Request
typetext

The type of authorization rule. One of MCC or CURRENCY

operationtext

Determines whether the transactions should be allowed or blocked. One of ALLOW or BLOCK

description (optional)text

The description of the authorization rule

valueslist of text

A list of values based on the type of rule. For example, setting MCC as type will requires values to be set as ['1234', '5678']

Response

Returns a Rule

Example Request
curl -X POST \
  'https://api.wise-sandbox.com/v3/spend/applications/{{clientId}}/spend-controls/rules' \
  -H 'Authorization: Bearer <your API token>' \
  -H 'Content-Type: application/json' \
  -d '{
    "description": "Blocking all transactions from MCC 1234 and 5678",
    "type": "MCC",
    "operation": "ALLOW",
    "values": ["1234", "5678"]
  }'
Applying an authorization rule

Apply an authorization rule. This will result in a rule being evaluated against every incoming card authorisation requests.

Request
ruleIdtext

The ID of the authorization rule

Response

Returns a 200 - OK

Example Request
curl -X POST \
  'https://api.wise-sandbox.com/v3/spend/applications/{{clientId}}/spend-controls/rules/apply' \
  -H 'Authorization: Bearer <your API token>' \
  -H 'Content-Type: application/json' \
  -d '{
    "ruleId": "123",
  }'
Unapply an authorization rule

This endpoint deactivates an authorization rule. This will result in all card transactions NOT being evaluated against this rule.

The rule still exists and can be applied again.

Request
ruleIdtext

The ID of the authorization rule

Response

Returns a 200 - OK

Example Request
curl -X POST \
  'https://api.wise-sandbox.com/v3/spend/applications/{{clientId}}/spend-controls/rules/unapply' \
  -H 'Authorization: Bearer <your API token>' \
  -H 'Content-Type: application/json' \
  -d '{
    "ruleId": "123",
  }'
Retrieve all applied authorization rules

Returns the list of all the active authorisation rules that have been applied.

Response

Returns a list of all authorisation rules that are applied.

Example Request
curl -X GET \
  'https://api.wise-sandbox.com/v3/spend/applications/{{clientId}}/spend-controls/rules/applied' \
  -H 'Authorization: Bearer <your API token>'
Example Response
[
  {
    "ruleId": 123,
  },
  {
    "ruleId": 456,
  }
]
Retrieve all authorization rules

Retrieves all the existing authorization rules, regardless of whether or not they were applied.

Response

Returns a collection of Rules.

Example Request
curl -X GET \
  'https://api.wise-sandbox.com/v3/spend/applications/{{clientId}}/spend-controls/rules' \
  -H 'Authorization: Bearer <your API token>'
Example Response
[
  {
    "id": 1,
    "description": "Blacklist gambling MCCs",
    "type": "MCC",
    "operation": "BLOCK",
    "values": [
      "7801",
      "7802",
      "7995",
      "9754"
    ]
  }
]
Deleting an authorization rule

Deletes an authorization rule that is currently not applied. If a rule is applied, you should unapply the rule before deleting it.

Response

Returns a 200 - OK

Example Request
curl -X DELETE \
  'https://api.wise-sandbox.com/v3/spend/applications/{{clientId}}/spend-controls/rules/{{ruleId}}' \
  -H 'Authorization: Bearer <your API token>' \
  -H 'Content-Type: application/json'
Retrieve spending limits for a profile

This endpoint has been deprecated and you can access the redesigned profile limits here.

Retrieves the spending limits that are configured for a profileId.

Request
typeString

The type of limit that is configured for the card. One of TRANSACTION, DAILY, MONTHLY, LIFETIME.

usageFloat

The amount which has been captured for the specific type and limit up till now

thresholdFloat

The transaction limit configured for the spending limit

currencyString

The 3-digit currency code assigned to the spending limit

expiresAtString

The timestamp at which the spending limit will expire, IS0-8601 timestamp with timezone (Z)

Response
spendingsList<Spending>

A collection of Spending resources

Example Request
curl -X GET \
  'https://api.wise-sandbox.com/v3/spend/profiles/{{profileId}}/spending-limits' \
  -H 'Authorization: Bearer <your api token>'
Example Response
{
  "spendings": [
    {
      "type": "ATM_WITHDRAWAL",
      "limits": [
        {
          "type": "TRANSACTION",
          "usage": 0,
          "threshold": 1750,
          "currency": "SGD",
          "expiresAt": null
        },
        {
          "type": "DAILY",
          "usage": 0,
          "threshold": 2700,
          "currency": "SGD",
          "expiresAt": "2022-12-15T16:00:00Z"
        },
        {
          "type": "MONTHLY",
          "usage": 0,
          "threshold": 5250,
          "currency": "SGD",
          "expiresAt": "2022-12-31T16:00:00Z"
        }
      ]
    },
    {
      "type": "ECOM_PURCHASE",
      "limits": [
        {
          "type": "TRANSACTION",
          "usage": 0,
          "threshold": 17500,
          "currency": "SGD",
          "expiresAt": null
        },
        {
          "type": "DAILY",
          "usage": 0,
          "threshold": 17500,
          "currency": "SGD",
          "expiresAt": "2022-12-15T16:00:00Z"
        },
        {
          "type": "MONTHLY",
          "usage": 0,
          "threshold": 35000,
          "currency": "SGD",
          "expiresAt": "2022-12-31T16:00:00Z"
        }
      ]
    },
    {
      "type": "CHIP_WALLET_PURCHASE",
      "limits": [
        {
          "type": "TRANSACTION",
          "usage": 0,
          "threshold": 4300,
          "currency": "SGD",
          "expiresAt": null
        },
        {
          "type": "DAILY",
          "usage": 0,
          "threshold": 5300,
          "currency": "SGD",
          "expiresAt": "2022-12-15T16:00:00Z"
        },
        {
          "type": "MONTHLY",
          "usage": 0,
          "threshold": 17500,
          "currency": "SGD",
          "expiresAt": "2022-12-31T16:00:00Z"
        }
      ]
    },
    {
      "type": "CONTACTLESS_PURCHASE",
      "limits": [
        {
          "type": "TRANSACTION",
          "usage": 0,
          "threshold": 900,
          "currency": "SGD",
          "expiresAt": null
        },
        {
          "type": "DAILY",
          "usage": 0,
          "threshold": 900,
          "currency": "SGD",
          "expiresAt": "2022-12-15T16:00:00Z"
        },
        {
          "type": "MONTHLY",
          "usage": 0,
          "threshold": 7000,
          "currency": "SGD",
          "expiresAt": "2022-12-31T16:00:00Z"
        }
      ]
    },
    {
      "type": "MAGSTRIPE_PURCHASE",
      "limits": [
        {
          "type": "TRANSACTION",
          "usage": 0,
          "threshold": 550,
          "currency": "SGD",
          "expiresAt": null
        },
        {
          "type": "DAILY",
          "usage": 0,
          "threshold": 700,
          "currency": "SGD",
          "expiresAt": "2022-12-15T16:00:00Z"
        },
        {
          "type": "MONTHLY",
          "usage": 0,
          "threshold": 2100,
          "currency": "SGD",
          "expiresAt": "2022-12-31T16:00:00Z"
        }
      ]
    }
  ]
}