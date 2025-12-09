# WebhookCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/api-reference/webhook

---

Webhook
Copy

Webhooks are available via application and profile level subscriptions. This set of APIs allows you to create, list, and delete subscriptions as well as test your webhook subscription endpoints.

For more information on creating and managing webhooks, and specific event types, please see Webhooks & Notifications.

Operations

POST
/v3/applications/{{clientKey}}/subscriptions
GET
/v3/applications/{{clientKey}}/subscriptions/{{subscriptionId}}
GET
/v3/applications/{{clientKey}}/subscriptions
DELETE
/v3/applications/{{clientKey}}/subscriptions/{{subscriptionId}}
POST
/v3/applications/{{clientKey}}/subscriptions/{{subscriptionId}}/test-notifications
POST
/v3/profiles/{{profileId}}/subscriptions
GET
/v3/profiles/{{profileId}}/subscriptions/{{subscriptionId}}
GET
/v3/profiles/{{profileId}}/subscriptions
DELETE
/v3/profiles/{{profileId}}/subscriptions/{{subscriptionId}}
The Webhook Resource

All webhook event notification payloads have the same high-level structure. Top-level properties are common to all events. The data property is an object that can contain various properties. The exact properties that the data object contains depends on the event type and schema version of the event.

Common Properties
dataobject

Event type- and schema version-specific details

subscription_idtext

ID of the webhook subscription that triggered the event notification

event_typestring

Event type (what event happened in our system)

schema_versionstring

Schema version (what notification structure is being used to model the event)

sent_atdatetime

When the event notification was sent from our system

Webhook Payload
{
  "data": {},
  "subscription_id": "01234567-89ab-cdef-0123-456789abcdef",
  "event_type": "event#type",
  "schema_version": "2.0.0",
  "sent_at": "2020-01-01T12:34:56Z"
}
Create Application Webhook Subscription

POST /v3/applications/{{clientKey}}/subscriptions

{{clientKey}} can be received upon obtaining client credentials from our tech support.

All fields listed below are required for creating a webhook subscription.

Request
nametext

A custom name for your webhook to ease with identification

trigger_ontext

Choose from a list of available events

delivery.versiontext

The event representation semantic version

delivery.urltext

Required. The URL where your server will be listening for events.

Response
IDtext

UUID that uniquely identifies the subscription

nametext

A custom name for your webhook to ease with identification

trigger_ontext

Choose from a list of available events

delivery.versiontext

The event representation semantic version

delivery.urltext

The URL where your server will be listening for events.

scope.domaintext

Scope of this subscription, always "application" in this case

scope.idtext

Client key used to create this subscription

created_by.typetext

Creator type. Always application in this case

created_by.idtext

Client ID of the creator. Not always the same as the client key

created_attext

Timestamp of when the subscription was created

Example Request
curl -X POST \
  "https://api.wise-sandbox.com/v3/applications/{{clientKey}}/subscriptions" \
  -H "Authorization: Bearer <your client credentials token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Webhook Subscription #1",
    "trigger_on": "transfers#state-change",
    "delivery": {
      "version": "2.0.0",
      "url": "https://your.webhook.url/12345"
    }
  }'
Example Response
{
  "id": "72195556-e5cb-495e-a010-b37a4f2a3043",
  "name": "Webhook Subscription #1",
  "delivery": {
    "version": "2.0.0",
    "url": "https://your.webhook.url/12345"
  },
  "trigger_on": "transfers#state-change",
  "scope": {
    "domain": "application",
    "id": "<your client key>"
  },
  "created_by": {
    "type": "application",
    "id": "<your client ID>"
  },
  "created_at": "2019-10-10T13:55:57Z"
}
Retrieve Application Webhook Subscription

Retrieves a subscription by its identifier.

Example Request
curl -X GET \
  "https://api.transferwise.com/v3/applications/{{clientKey}}/subscriptions/{{subscriptionId}}" \
  -H "Authorization: Bearer <your client credentials token>"
Example Response
{
  "id": "f215f353-35fd-405b-b27f-4fd603c905ed",
  "name": "Webhook Subscription #1",
  "delivery": {
    "version": "2.0.0",
    "url": "https://your.webhook.url/12345"
  },
  "trigger_on": "balances#credit",
  "scope": {
    "domain": "application",
    "id": "<your client key>"
  },
  "created_by": {
    "type": "application",
    "id": "<your client ID>"  // clientId and key are not always the same
  },
  "created_at": "2008-09-15T15:53:00Z"
}
List Application Webhook Subscriptions

List all your subscriptions.

Example Request
curl -X GET \
  "https://api.transferwise.com/v3/applications/{{clientKey}}/subscriptions" \
  -H "Authorization: Bearer <your client credentials token>"
Example Response
[
  {
    "id": "e889e085-3677-4110-be26-3e9160ac9f25",
    "name": "#1 subscription",
    "delivery": {
      "version": "2.0.0",
      "url": "https://your.webhook.url/12345"
    },
    "trigger_on": "transfers#state-change",
    "created_by": {
      "type": "application",
      "id": "<your client ID>"
    },
    "scope": {
      "domain": "application",
      "id": "<your client key>"
    }
  },
  {
    "id": "eabeb3f5-c134-4a1c-99e2-86a1163daf1b",
    "name": "#2 subscription",
    "delivery": {
      "version": "2.0.0",
      "url": "https://your.webhook.url/12345"
    },
    "trigger_on": "transfers#state-change",
    "created_by": {
      "type": "application",
      "id": "<your client ID>"
    },
    "scope": {
      "domain": "application",
      "id": "<your client key>"
    }
  }
]
Delete Application Webhook Subscription

Deletes a subscription by its identifier.

Example Request
curl -X DELETE \
  "https://api.transferwise.com/v3/applications/{{clientKey}}/subscriptions/{{subscriptionId}}" \
  -H "Authorization: Bearer <your client credentials token>"
Example Response
No Content
Test Notifications for Application Subscriptions

Test notifications can be generated for existing application subscriptions using the API.

Test notifications will have the correct structure for their source subscription's event type and version, and will contain "dummy" data. These data include random UUIDs, entity IDs of zero, current dates and times, and hard-coded status codes.

Test notifications are delivered with the usual notification HTTP request headers, including a unique delivery ID for the notification, and a "test notification" flag set to true. You can check for the presence of this test flag to determine that an incoming notification is a test notification which should not be processed as real data. See the section Event HTTP requests for more information on request headers.

When test notifications are created with the API, they are queued for sending in the same way as non-test notifications. This means that there may be some delay in notification delivery, and delivery failures will result in attempts to redeliver the notification later. The API returns the delivery IDs of the notifications that have been successfully queued for sending, which can be correlated with the delivery ID header values for notifications you later receive.

Please note that this test notification API is only available for application-based subscriptions. Profile-based subscriptions do not currently support this testing feature.

Example Request
curl -X POST \
  "https://api.wise-sandbox.com/v3/applications/{{clientKey}}/subscriptions/{{subscriptionId}}/test-notifications" \
  -H "Authorization: Bearer <your client credentials token>"
Example Response
[
  {
    "delivery_id": "4a6b9810-4279-4de5-8d8d-1a6cf3b92a75",
    "created_at": "2019-03-28T11:22:33Z"
  }
]
Example Test Notification
x-signature: bnho0q9JhjR6IPJIOZqWVP...
x-delivery-id: 4a6b9810-4279-4de5-8d8d-1a6cf3b92a75
x-test-notification: true

{
  "data": {
    "resource": {
      "id": 0,
      "profile_id": 0,
      "account_id": 0,
      "type": "transfer"
    },
    "current_state": "processing",
    "previous_state": "incoming_payment_waiting",
    "occurred_at": "2019-03-28T11:22:33Z"
  },
  "subscription_id": "39f241b7-293d-439e-beb3-4bf947bd4ff8",
  "event_type": "transfers#state-change",
  "schema_version": "2.0.0",
  "sent_at": "2019-03-28T11:22:33Z"
}
Create Profile Webhook Subscription

POST /v3/profiles/{{profileId}}/subscriptions

{{profileId}} - ID of the profile you are subscribing to.

All fields listed below are required for creating a webhook subscription.

Request
nametext

A custom name for your webhook to ease with identification

trigger_ontext

Choose from a list of available events

delivery.versiontext

The event representation semantic version

delivery.urltext

Required. The URL where your server will be listening for events.

Response
IDtext

UUID that uniquely identifies the subscription

nametext

A custom name for your webhook to ease with identification

trigger_ontext

Choose from a list of available events

delivery.versiontext

The event representation semantic version

delivery.urltext

The URL where your server will be listening for events.

scope.domaintext

Scope of this subscription, always "application" in this case

scope.idtext

Client key used to create this subscription

created_by.typetext

Creator type. Always application in this case

created_by.idtext

Client ID of the creator. Not always the same as the client key

created_attext

Timestamp of when the subscription was created

Example Request
curl -X POST \
  "https://api.wise-sandbox.com/v3/profiles/{{profileId}}/subscriptions" \
  -H "Authorization: Bearer <your API token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Webhook Subscription #1",
    "trigger_on": "transfers#state-change",
    "delivery": {
      "version": "2.0.0",
      "url": "https://your.webhook.url/12345"
    }
  }'
Example Response
{
  "id": "72195556-e5cb-495e-a010-b37a4f2a3043",
  "name": "Webhook Subscription #1",
  "delivery": {
    "version": "2.0.0",
    "url": "https://your.webhook.url/12345"
  },
  "trigger_on": "transfers#state-change",
  "scope": {
    "domain": "application",
    "id": "<your client key>"
  },
  "created_by": {
    "type": "application",
    "id": "<your client ID>"
  },
  "created_at": "2019-10-10T13:55:57Z"
}
Retrieve Profile Webhook Subscription

Retrieves a subscription by its identifier.

Example Request
curl -X GET \
  "https://api.transferwise.com/v3/profiles/{{profileId}}/subscriptions/{{subscriptionId}}" \
  -H "Authorization: Bearer <your API token>"
Example Response
{
  "id": "f215f353-35fd-405b-b27f-4fd603c905ed",
  "name": "Webhook Subscription #1",
  "delivery": {
    "version": "2.0.0",
    "url": "https://your.webhook.url/12345"
  },
  "trigger_on": "balances#credit",
  "scope": {
    "domain": "profile",
    "id": "<profile ID>"
  },
  "created_by": {
    "type": "user",
    "id": "<your user ID>"
  },
  "created_at": "2008-09-15T15:53:00Z"
}
List Profile Webhook Subscriptions

List all your subscriptions.

Example Request
curl -X GET \
  "https://api.transferwise.com/v3/profiles/{{profileId}}/subscriptions" \
  -H "Authorization: Bearer <your API token>"
Example Response
[
  {
    "id": "e889e085-3677-4110-be26-3e9160ac9f25",
    "name": "#1 subscription",
    "delivery": {
      "version": "2.0.0",
      "url": "https://your.webhook.url/12345"
    },
    "trigger_on": "transfers#state-change",
    "created_by": {
      "type": "user",
      "id": "<your user ID>"
    },
    "scope": {
      "domain": "profile",
      "id": "<profile ID>"
    }
  },
  {
    "id": "eabeb3f5-c134-4a1c-99e2-86a1163daf1b",
    "name": "#2 subscription",
    "delivery": {
      "version": "2.0.0",
      "url": "https://your.webhook.url/12345"
    },
    "trigger_on": "transfers#state-change",
    "created_by": {
      "type": "user",
      "id": "<your user ID>"
    },
    "scope": {
      "domain": "profile",
      "id": "<profile ID>"
    }
  }
]
Delete Profile Webhook Subscription

Deletes a subscription by its identifier.

Example Request
curl -X DELETE \
  "https://api.transferwise.com/v3/profiles/{{profileId}}/subscriptions/{{subscriptionId}}" \
  -H "Authorization: Bearer <your API token>"
Example Response
No Content