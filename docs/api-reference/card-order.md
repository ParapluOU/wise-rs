# Card OrderCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/api-reference/card-order

---

Card Order
Copy

With this set of APIs, you will be able to create cards for your customers. You can also retrieve and view the status of your current card orders, as well as the list of available card programs for the user.

On production, each personal profile can order at most 1 physical card and 3 virtual cards. On sandbox, we allow up to 10 physical cards and 30 virtual cards for testing purpose. However, no more than 3 virtual cards can be ordered per day. This limit includes cards and card orders.

Operations

POST
/v3/spend/profiles/{{profileId}}/card-orders
GET
/v3/spend/profiles/{{profileId}}/card-orders/{{cardOrderId}}
GET
/v3/spend/profiles/{{profileId}}/card-orders?pageNumber=1&pageSize=10
GET
/v3/spend/profiles/{{profileId}}/card-orders/availability
GET
/v3/spend/profiles/{{profileId}}/card-orders/{{cardOrderId}}/requirements
POST
/v3/spend/address/validate
POST
/twcard-data/v1/sensitive-card-data/preset-pin
PUT
/v3/spend/profiles/{{profileId}}/card-orders/{{cardOrderId}}/status
Card order resource
Resource
idnumber

ID of the card order

profileIdnumber

Profile ID

clientIdtext

Client ID

cardProgramCardProgram

Card program of the card holder.

addressAddress

Address set during card order

cardTokentext

Token of the card associated with card order. Nullable.

replacesCardtext

A string for replacement card. Not supported at the moment.

creationTimetext

Time when the card order is created

modificationTimetext

Time when the card order was last modified

statusCardStatus

Status of the card order before card is issued.
Checkout card order status for more information.

cardHolderNametext

Name of the card holder.

phoneNumbertext

Phone number associated with the card order.

lifetimeLimitnumber

Maximum amount of spending on the card once issued. Nullable.

deliveryEstimatetext (ISO-8601)

The estimated time when the card will be delivered. There are few scenarios to be mindful of:

For virtual card the delivery estimate will be close to the creationTime. As it does not require delivery.
For physical card in PLACED status, the delivery estimate is calculated assuming that the order requirements will be fulfilled today (refreshed daily).
For physical card after PLACED status, we provide a best effort estimation, and it should not be used as delivery timing as we will have separate delivery tracking (subject to region availability) for physical card that is coming soon.
deliveryDetails (optional)DeliveryDetails

Delivery details of a physical card order. For virtual card, this value is null

Card order resource
{
  "id": 142,
  "profileId": 123456,
  "clientId": "{{clientId}}",
  "cardProgram": {
    "name": "VISA_DEBIT_BUSINESS_UK_1_PHYSICAL_CARDS_API",
    "scheme": "VISA",
    "defaultCurrency": "GBP",
    "cardType": "PHYSICAL"
  },
  "address": {
    "firstLine": "56 Shoreditch High St",
    "secondLine": "The Tea Bldg",
    "thirdLine": null,
    "city": "London",
    "postCode": "E1 6JJ",
    "state": null,
    "country": "GB"
  },
  "cardToken": "4dc0be88-903f-49e4-8237-735f1139e3dd",
  "replacesCard": null,
  "creationTime": "2023-07-31T01:43:24.596321434Z",
  "modificationTime": "2023-07-31T01:43:24.596321825Z",
  "status": "PRODUCED",
  "cardHolderName": "John Smith",
  "phoneNumber": "+441234567890",
  "lifetimeLimit": 100,
  "deliveryEstimate": "2023-10-30T07:11:00.848681Z",
  "deliveryDetails": {
    "deliveryVendor": null,
    "trackingUrl": null,
    "trackingNumber": null,
    "deliveryOption": "POSTAL_SERVICE_STANDARD"
  }
}
Card program resource

A Card Program is what Wise refers to all the cards that you will be issuing with us, grouped by product type and by issuing country.

nametext

The name of the card program

schemetext

The network of the card program. One of MASTERCARD or VISA

defaultCurrencytext

The default currency assigned to the card program

cardTypetext

The type of the card. It can be one of VIRTUAL_NON_UPGRADEABLE or PHYSICAL

Card program resource
{
  "cardPrograms": [
    {
      "name": "VISA_DEBIT_BUSINESS_UK_1_CARDS_API",
      "scheme": "VISA",
      "defaultCurrency": "GBP",
      "cardType" : "VIRTUAL_NON_UPGRADEABLE"
    }
  ]
}
Delivery option resource
The delivery method will be defined during scoping phase. Please reach out to your Implementation Manager for more information

POSTAL_SERVICE_STANDARD is typically utilized by default. This type of delivery can't be traceable, hence deliveryVendor, trackingUrl, and trackingNumber value will return null.

POSTAL_SERVICE_WITH_TRACKING is available in certain regions. In Brazil, this is the default delivery method. Tracking details can only be known for physical card when the status is PRODUCED or COMPLETED.

We also offer a KIOSK_COLLECTION delivery method. Please refer to this guide for more information.

The deliveryOption field should only be specified for KIOSK_COLLECTION during card order creation.

Resource
deliveryVendor (optional)text

The name of the delivery vendor

trackingUrl (optional)text

The url to track the card delivery

trackingNumber (optional)text

The tracking number of the card delivery

deliveryOptiontext

The delivery option used on the card order

Delivery details resource
{
  "deliveryDetails": {
    "deliveryVendor": "DHL",
    "trackingUrl": "https://www.dhl.com/gb-en/home/tracking/tracking-express.html?submit=1&tracking-id=1999473803",
    "trackingNumber": "1999473803",
    "deliveryOption": "POSTAL_SERVICE_WITH_TRACKING"
  }
}
Card order status

The card order response will contain the status field. The initial status is PLACED or REQUIREMENT_FULFILLED depending on the requirement fulfillment state.

The possible 'status' values are:

PLACED- The card order is created. The card will be generated once it has fulfilled all the requirements
REQUIREMENTS_FULFILLED - The card order has fulfilled all the requirements and the card should be generated in a short while
CARD_DETAILS_CREATED - The card has been generated
PRODUCED - The physical card has been produced and waiting to be picked up by delivery vendor (physical card only)
COMPLETED - The card has been activated and is ready to use. The card order is completed
CANCELLED - The card order has been cancelled. This can happen if you reach out to Wise Support to cancel a card order
RETURNED - Delivery failed, the physical card has been returned and will be blocked (physical card only)
Card order status state machine
Card address resource

To create a card order, make sure to follow country specific address fields and validation. If your country is not listed in the dropdown below, select Other.

For virtual cards, the address field will be used as a billing address. It will be used for AVS checks in countries where is it required.

For physical cards, the address field will be used as a delivery address. It will be used to deliver your card as well as AVS checks in countries where is it required.

Please select a country:

Create a card order

POST /v3/spend/profiles/{{profileId}}/card-orders

The program field value is retrieved from retrieve all card programs endpoint.

This request requires an extra field in the header, `X-idempotence-uuid`. This should be generated and used for any subsequent retries in the event that the initial request fails.

When you issue a card under a business profile, the cardholder will automatically default to the business representative.

If the cardholder is not the business representative, create a cardholder personal profile and add the profileId of the cardholder profile to the cardHolderProfileId field on the card order request.

Request
programtext

The name of the card program

cardHolderNametext

The cardholder's name

embossedNametext

The cardholder's name to print on the card (physical card only). The field length should be between 1 and 22 characters (spaces included).

phoneNumber (optional)text

For partners onboarded after 1/3/2025, we will use the profile phone number for any Card-related One-Time Password (OTP) requests. See 3ds. Ensure that the phone number is valid and starts with a "+" followed by the country code. For example, a valid phone number would be +6588888888.

addressAddress

The cardholder's billing address or delivery address. See address

deliveryOption (optional)text

The option to deliver your card. See delivery option

lifetimeLimit (optional)integer

Optionally sets a lifetime spending limit on the card. A lifetime limit of 0 means that a card cannot be used until the lifetime limit is updated.

cardHolderProfileId (optional)integer

The cardholder profile for this card. This is used for business profiles.

replacementDetails (optional)ReplacementDetails

The replacement details for this card. This object must contain:

cardToken of the card to replace
reason for replacing the card - one of CARD_DAMAGED or CARD_EXPIRING.
Response

Returns a Card Order

Request
curl -X POST \
  https://api.wise-sandbox.com/v3/spend/profiles/{{profileId}}/card-orders \
  -H 'Authorization: Bearer <your api token>' \
  -H 'Content-Type: application/json' \
  -H 'X-idempotence-uuid: 054064c9-e01e-49fb-8fd9-b0990b9442f4' \
  -d '{
    "program": "VISA_DEBIT_BUSINESS_UK_1_PHYSICAL_CARDS_API",
    "cardHolderName": "John Smith,
    "embossedName": "Smith John", // Physical card order
    "address": {
      "firstLine": "56 Shoreditch High St",
      "secondLine": "The Tea Bldg",
      "thirdLine": null,
      "city": "London",
      "postCode": "E1 6JJ",
      "state": null,
      "country": "GB"
    },
    "deliveryOption": "POSTAL_SERVICE_STANDARD",
    "lifetimeLimit": 100,
    "cardHolderProfileId": 654321,
    "replacementDetails": {
      "cardToken": "4a75fdb7-5791-49ac-832c-81c4347e4df0",
      "reason": "CARD_DAMAGED"
    }
  }'
Response
{
  "id": 142,
  "profileId": 123456,
  "clientId": "{{clientId}}",
  "cardProgram": {
    "name": "VISA_DEBIT_BUSINESS_UK_1_PHYSICAL_CARDS_API",
    "scheme": "VISA",
    "defaultCurrency": "GBP",
    "cardType": "PHYSICAL"
  },
  "address": {
    "firstLine": "56 Shoreditch High St",
    "secondLine": "The Tea Bldg",
    "thirdLine": null,
    "city": "London",
    "postCode": "E1 6JJ",
    "state": null,
    "country": "GB"
  },
  "cardToken": "4dc0be88-903f-49e4-8237-735f1139e3dd",
  "replacesCard": null,
  "creationTime": "2024-09-19T06:49:49.145021Z",
  "modificationTime": "2024-09-19T06:49:49.145021Z",
  "status": "PRODUCED",
  "cardHolderName": "John Smith",
  "phoneNumber": "+441234567890",
  "lifetimeLimit": 100,
  "deliveryEstimate": "2023-10-30T07:11:00.848681Z",
  "deliveryDetails": {
    "deliveryVendor": "DHL",
    "trackingUrl": "https://www.dhl.com/gb-en/home/tracking/tracking-express.html?submit=1&tracking-id=1999473803",
    "trackingNumber": "1999473803"
  },
  "cardHolderProfileId": 654321
}
Retrieve a card order

GET /v3/spend/profiles/{{profileId}}/card-orders/{{cardOrderId}}

Retrieve a card order based on the cardOrderId.

Response

Returns a Card Order.

Request
curl -X GET \
  https://api.wise-sandbox.com/v3/spend/profiles/{{profileId}}/card-orders/{{cardOrderId}} \
  -H 'Authorization: Bearer <your api token>'
Response
{
  "id": 142,
  "profileId": 123456,
  "clientId": "{{clientId}}",
  "cardProgram": {
    "name": "VISA_DEBIT_BUSINESS_UK_1_PHYSICAL_CARDS_API",
    "scheme": "VISA",
    "defaultCurrency": "GBP",
    "cardType": "PHYSICAL"
  },
  "address": {
    "firstLine": "56 Shoreditch High St",
    "secondLine": "The Tea Bldg",
    "thirdLine": null,
    "city": "London",
    "postCode": "E1 6JJ",
    "state": null,
    "country": "GB"
  },
  "cardToken": "4dc0be88-903f-49e4-8237-735f1139e3dd",
  "replacesCard": null,
  "creationTime": "2024-07-22T07:29:33.720717Z",
  "modificationTime": "2024-08-29T15:28:31.078399Z",
  "status": "PRODUCED",
  "cardHolderName": "John Smith",
  "phoneNumber": "+441234567890",
  "lifetimeLimit": 100,
  "deliveryEstimate": "2023-10-30T07:11:00.848681Z",
  "deliveryDetails": {
    "deliveryVendor": "DHL",
    "trackingUrl": "https://www.dhl.com/gb-en/home/tracking/tracking-express.html?submit=1&tracking-id=1999473803",
    "trackingNumber": "1999473803"
  }
}
Retrieve all card orders

GET /v3/spend/profiles/{{profileId}}/card-orders?pageSize=10&pageNumber=1

Retrieve a list of card orders created for the profileId specified in the endpoint.

Request
pageSize (optional)integer

The maximum number of card orders to return per page. This number can be between 10 - 100, and will default to 10

pageNumber (optional)integer

The page number to retrieve the next set of card orders. The number has to be greater or equal to 1, and will default to 1

Response

totalCount is the total number of card orders for this profile.

Returns collection of card order for this profileId

Request
curl -X GET \
  https://api.wise-sandbox.com/v3/spend/profiles/{{profileId}}/card-orders?pageSize=10&pageNumber=1 \
  -H 'Authorization: Bearer <your api token>'
Response
{
  "totalCount": 5,
  "cardOrders": [
    {
      "id": 142,
      "profileId": 123456,
      "clientId": "clientId",
      "cardProgram": {
        "name": "VISA_DEBIT_BUSINESS_UK_1_CARDS_API",
        "scheme": "VISA",
        "defaultCurrency": "GBP",
        "cardType" : "VIRTUAL_NON_UPGRADEABLE",
      },
      "address": {
        "firstLine": "56 Shoreditch High St",
        "secondLine": "The Tea Bldg",
        "thirdLine": null,
        "city": "London",
        "postCode": "E1 6JJ",
        "state": null,
        "country": "GB"
      },
      "cardToken": null,
      "replacesCard": null,
      "creationTime": "2024-07-22T07:29:33.720717Z",
      "modificationTime": "2024-08-29T15:28:31.078399Z",
      "status": "REQUIREMENTS_FULFILLED",
      "cardHolderName": "John Smith",
      "phoneNumber": "+441234567890",
      "lifetimeLimit": 100
    }
  ]
}
Retrieve all card programs

GET /v3/spend/profiles/{{profileId}}/card-orders/availability

Retrieve the list of available card programs for each profileId.

Response

Returns a collection of card program resources available to the profileId.

Request
curl -X GET \
  https://api.wise-sandbox.com/v3/spend/profiles/{{profileId}}/card-orders/availability \
  -H 'Authorization: Bearer <your api token>' \
  -H 'Content-Type: application/json'
Response
{
  "cardPrograms": [
    {
      "name": "VISA_DEBIT_BUSINESS_UK_1_CARDS_API",
      "scheme": "VISA",
      "defaultCurrency": "GBP",
      "cardType" : "VIRTUAL_NON_UPGRADEABLE"
    }
  ]
}
Retrieve all card order requirements

GET /v3/spend/profiles/{{profileId}}/card-orders/{{cardOrderId}}/requirements

Retrieve all card requirements for a cardOrderId. A valid card order needs all its requirements status to be COMPLETED.

Response

The type of requirements are:

PIN (optional): Set a pin on a virtual or physical card order. Contact the team if you need this feature.
VERIFICATION: Verify your customer by providing KYC evidences. Refer to the KYC guide
ADDRESS: Provide a valid address for your card order. Refer to address validation

A requirement status has the following values:

NOT_INITIATED
NEEDS_ACTION
PENDING
COMPLETED
FAILED
Request
curl -X GET \
  https://api.wise-sandbox.com/v3/spend/profiles/{{profileId}}/card-orders/{{cardOrderId}}/requirements \
  -H 'Authorization: Bearer <your api token>'
Response
{
  "requirements": [
    {
      "type": "PIN",
      "status": "NOT_INITIATED"
    },
    {
      "type": "VERIFICATION",
      "status": "PENDING"
    },
    {
      "type": "ADDRESS",
      "status": "COMPLETED"
    }
  ]
}
Validate an address

POST /v3/spend/address/validate

To create a card order, make sure to follow country specific address fields and validation. Find more about each country specifics in this guide.

For virtual cards, the address field will be used as a billing address. It will be used for AVS checks in countries where is it required.

For physical cards, the address field will be used as a delivery address. It will be used to deliver your card and for AVS checks in countries where is it required.

We do not support PO BOX addresses

Request
curl -X POST \
  https://api.wise-sandbox.com/v3/spend/address/validate \
  -H 'Authorization: Bearer <your api token>' \
  -H 'Content-Type: application/json' \
  -d '{
    "firstLine": "92 Jubilee Market Hall, Tavistock St, London WC2E 8BD, United Kingdom ",
    "secondLine": "Covent Garden",
    "thirdLine": "null",
    "city": "",
    "postCode": "wrong-postcode",
    "state": null,
    "country": "GB"
  }'
Set a PIN

This endpoint will be accessible for partners that require to set a PIN during the card order flow.

Please follow this guide to use this endpoint.

To use this endpoint, make sure to set the api token and the card order id in the headers.

Request
keyVersionnumber

The version of the key to use. It is always set to 1.

encryptedPayloadtext

Your JWE payload.

Response

Returns the card order ID.

Request
curl -X POST \
  https://twcard.wise-sandbox.com/twcard-data/v1/sensitive-card-data/preset-pin \
  -H 'Authorization: Bearer <your api token>' \
  -H 'x-tw-twcard-order-id: <your card order ID>' \
  -d '{
    "keyVersion": 1,
    "encryptedPayload": <your JWE> 
  }'
Response
{ 
  "cardOrderId": "<your card order ID>",
}
Update the status of a card order

PUT /v3/spend/profiles/{{profileId}}/card-orders/{{cardOrderId}}/status

Update the status of a card order based on its cardOrderId. The status can be updated to one of the following:

CANCELLED
COMPLETED (deprecated)

A card order can be updated to COMPLETED for all statuses except CANCELLED and COMPLETED.

The behaviour for card order cancellation differs across card order statuses and is listed below:

PLACED: Card order can be cancelled with no further action required.
REQUIREMENTS_FULFILLED, CARD_DETAILS_CREATED, PRODUCED: Card order can be cancelled, and its associated card will be blocked. However, the physical card may have already been produced and may be in delivery. If so, the card will still reach the end user's address. This should be communicated to the end user to prevent confusion.
COMPLETED, RETURNED: Card orders in these statuses cannot be cancelled.
Updating a card order status to `COMPLETED` is deprecated. Check with our team whether this is supported in your integration.
Request
statustext

The intended new status of the card order. One of CANCELLED or COMPLETED (deprecated)

Response

Returns a 202 - Accepted

Example Request
curl -X PUT \
  https://api.wise-sandbox.com/v3/spend/profiles/{{profileId}}/card-orders/{{cardOrderId}}/status \
  -H 'Authorization: Bearer <your api token>'
  -d '{
    "status": "CANCELLED"
  }'