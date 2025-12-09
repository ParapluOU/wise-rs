# Spend LimitsCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/api-reference/spend-limits

---

Spend Limits
Copy

These APIs help you manage spend limits that are applied on your profile or card.

Operations

GET
/v4/spend/profiles/{{profileId}}/spend-limits
PATCH
/v4/spend/profiles/{{profileId}}/spend-limits
GET
/v4/spend/profiles/{{profileId}}/cards/{{cardToken}}/spend-limits
PATCH
/v4/spend/profiles/{{profileId}}/cards/{{cardToken}}/spend-limits
DELETE
/v4/spend/profiles/{{profileId}}/cards/{{cardToken}}/spend-limits/{{granularity}}
The Profile Limits resource
typetext

The type of transaction. One of PURCHASE, ATM_WITHDRAWAL. PURCHASE is a combined limit that applies to Contactless, Magnetic, Online purchase, Chip and PIN/mobile wallet transactions

valuemoney

Daily or monthly limit. The value should be between 0 and the max allowed

usagemoney

The total authorised amount till date

maxmoney

The max allowed limit for daily or monthly

resetAttext

The time when the limit get reset. ISO-8601 timestamp with timezone (Z)

Profile Limits Resource
{
  "type": "PURCHASE",
  "aggregateWindow": {
    "daily": {
      "value": {
        "amount": 20000.00,
        "currency": "GBP"
      },
      "usage": {
        "amount": 0.00,
        "currency": "GBP"
      },
      "max": {
        "amount": 30000.00,
        "currency": "GBP"
      },
      "resetAt": "2023-07-31T22:59:59.999999999Z"
    },
    "monthly": {
      "value": {
        "amount": 20000.00,
        "currency": "GBP"
      },
      "usage": {
        "amount": 0.00,
        "currency": "GBP"
      },
      "max": {
        "amount": 30000.00,
        "currency": "GBP"
      },
      "resetAt": "2023-07-31T22:59:59.999999999Z"
    }
  }
}
The Card Limits resource
valuemoney

Optional transaction, daily, monthly or lifetime limit

usagemoney

The total authorised amount during the specified window. The value is always 0 for transaction limit

resetAttext

The time when the limit gets reset. ISO-8601 timestamp with timezone (Z). A null value is returned for transaction and lifetime limit

Card Limits Resource
{
  "transaction": {
    "value": {
      "amount": 100.00,
      "currency": "GBP"
    },
    "usage": {
      "amount": 0.00,
      "currency": "GBP"
    },
    "resetAt": null
  },
  "daily": {
    "value": {
      "amount": 2000.00,
      "currency": "GBP"
    },
    "usage": {
      "amount": 100.00,
      "currency": "GBP"
    },
    "resetAt": "2023-09-29T22:59:59.999999999Z"
  },
  "monthly": null,
  "lifetime": {
    "value": {
      "amount": 5000.00,
      "currency": "GBP"
    },
    "usage": {
      "amount": 100.00,
      "currency": "GBP"
    },
    "resetAt": null
  }
}
Retrieve profile limits

GET /v4/spend/profiles/{{profileId}}/spend-limits

Retrieves the spend limits that are configured for a profileId.

Response

Returns a list of profile limits object

Example Request
curl -X GET \
  'https://api.wise-sandbox.com/v4/spend/profiles/{{profileId}}/spend-limits' \
  -H 'Authorization: Bearer <your api token>'
Example Response
{
  "spendLimits": [
    {
      "type": "PURCHASE",
      "aggregateWindow": {
        "daily": {
          "value": {
            "amount": 20000.00,
            "currency": "GBP"
          },
          "usage": {
            "amount": 0.00,
            "currency": "GBP"
          },
          "max": {
            "amount": 30000.00,
            "currency": "GBP"
          },
          "resetAt": "2023-07-31T22:59:59.999999999Z"
        },
        "monthly": {
          "value": {
            "amount": 20000.00,
            "currency": "GBP"
          },
          "usage": {
            "amount": 0.00,
            "currency": "GBP"
          },
          "max": {
            "amount": 30000.00,
            "currency": "GBP"
          },
          "resetAt": "2023-07-31T22:59:59.999999999Z"
        }
      }
    },
    {
      "type": "ATM_WITHDRAWAL",
      "aggregateWindow": {
        "daily": {
          "value": {
            "amount": 1000.00,
            "currency": "GBP"
          },
          "usage": {
            "amount": 0.00,
            "currency": "GBP"
          },
          "max": {
            "amount": 4000.00,
            "currency": "GBP"
          },
          "resetAt": "2023-07-31T22:59:59.999999999Z"
        },
        "monthly": {
          "value": {
            "amount": 1000.00,
            "currency": "GBP"
          },
          "usage": {
            "amount": 0.00,
            "currency": "GBP"
          },
          "max": {
            "amount": 4000.00,
            "currency": "GBP"
          },
          "resetAt": "2023-07-31T22:59:59.999999999Z"
        }
      }
    }
  ]
}
Update profile limits

PATCH /v4/spend/profiles/{{profileId}}/spend-limits

Update profile daily and monthly spend limits for PURCHASE or ATM_WITHDRAWAL.

Request
typetext

The type of transaction. One of PURCHASE, ATM_WITHDRAWAL

valuemoney

The amount allowed to be spent for the chosen type, both daily and monthly must be set

Response

See Retrieve profile limits

Example Request
curl -X PATCH \
  https://api.wise-sandbox.com/v4/spend/profiles/{{profileId}}/spend-limits \
  -H 'Authorization: Bearer <your api token>' \
  -H 'Content-Type: application/json' \
  -d '{
    "type": "PURCHASE",
    "aggregateWindow": {
      "daily": {
        "value": {
          "amount": 20000.00,
          "currency": "GBP"
        }
      },
      "monthly": {
        "value": {
          "amount": 20000.00,
          "currency": "GBP"
        }
      }
    }
  }'
Retrieve card limits

GET /v4/spend/profiles/{{profileId}}/cards/{{cardToken}}/spend-limits

Retrieves the spend limits that are configured for a card.

Response

Returns a list of card limits object

Example Request
curl -X GET \
  'https://api.wise-sandbox.com/v4/spend/profiles/{{profileId}}/cards/{{cardToken}}/spend-limits' \
  -H 'Authorization: Bearer <your api token>'
Example Response
{
  "transaction": {
    "value": {
      "amount": 100.00,
      "currency": "GBP"
    },
    "usage": {
      "amount": 0.00,
      "currency": "GBP"
    },
    "resetAt": null
  },
  "daily": {
    "value": {
      "amount": 2000.00,
      "currency": "GBP"
    },
    "usage": {
      "amount": 100.00,
      "currency": "GBP"
    },
    "resetAt": "2023-09-29T22:59:59.999999999Z"
  },
  "monthly": {
    "value": {
      "amount": 3000.00,
      "currency": "GBP"
    },
    "usage": {
      "amount": 100.00,
      "currency": "GBP"
    },
    "resetAt": "2023-09-30T22:59:59.999999999Z"
  },
  "lifetime": {
    "value": {
      "amount": 5000.00,
      "currency": "GBP"
    },
    "usage": {
      "amount": 100.00,
      "currency": "GBP"
    },
    "resetAt": null
  }
}
Create or update card limits

PATCH /v4/spend/profiles/{{profileId}}/cards/{{cardToken}}/spend-limits

Create or update card transaction, daily, monthly or lifetime spend limits.

Request
valuemoney

The amount allowed to be spent on the card during the specified period

Response

See Retrieve card limits

Example Request
curl -X PATCH \
  https://api.wise-sandbox.com/v4/spend/profiles/{{profileId}}/cards/{{cardToken}}/spend-limits \
  -H 'Authorization: Bearer <your api token>' \
  -H 'Content-Type: application/json' \
  -d '{
    "daily": {
      "value": {
        "amount": 1000.00,
        "currency": "GBP"
      }
    },
    "monthly": {
      "value": {
        "amount": 1000.00,
        "currency": "GBP"
      }
    }
  }'
Delete card limits

DELETE /v4/spend/profiles/{{profileId}}/cards/{{cardToken}}/spend-limits/{{granularity}}

Delete card spend limits. The granularity must be set to transaction, daily, monthly or lifetime.

Response

See Retrieve card limits

Example Request
curl -X DELETE \
  https://api.wise-sandbox.com/v4/spend/profiles/{{profileId}}/cards/{{cardToken}}/spend-limits/daily \
  -H 'Authorization: Bearer <your api token>'