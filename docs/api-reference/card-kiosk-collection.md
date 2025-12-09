# Card Kiosk CollectionCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/api-reference/card-kiosk-collection

---

Card Kiosk Collection
Copy

These APIs are designed to allow you to print and encrypt your card directly from a kiosk machine. The card information will be sent to our card manufacturer to configure and print the card on-site on a kiosk machine.

The card printing process will automatically begin once the request is received by our card manufacturer.

During the printing process, you will be notified via webhook about any card production status change.

Before using these APIs, make sure to read the guide on kiosk collection.

Please reach out to your Implementation Manager for more information on these APIs.

Operations

PUT
/v3/spend/profiles/{{profileId}}/cards/{{cardToken}}/production
GET
/v3/spend/profiles/{{profileId}}/cards/{{cardToken}}/production
Production status flow

These statuses represent the lifecycle of a card production.

READY - The card is ready for production. The produce card endpoint can be called.
IN_PROGRESS - The card is in production at the kiosk machine.
PRODUCED - The card is produced and collected. This is a final state.
PRODUCTION_ERROR - The card production failed. The produce card endpoint can be called again to retry the card production.

A card with production status PRODUCED will trigger an asynchronous call to update the associated card order to PRODUCED status.

Production status flow
Produce card

PUT /v3/spend/profiles/{{profileId}}/cards/{{cardToken}}/production

This endpoint sends the card production request to a kiosk machine. To confirm that card information has been successfully created, please listen to card-production-status-change webhook with status READY.

Cards that were created over 60 days ago will result in a 422 error code and cannot be retried, this is due to the data being obfuscated on our side. In this case a new card order has to be created.
Request
kioskIdtext

Identifier that specifies on which kiosk the card should be produced.

Response
200 - OK - The card information have been successfully sent to the kiosk machine.
422 - Unprocessable Content - The card information can't be processed by the kiosk machine. For more details, check the error code returned.
statustext

The value will be IN_PROGRESS if the card information has been successfully sent to the kiosk machine, otherwise it will be REQUEST_ERROR.

kioskIdtext

Identifier that specifies which kiosk machine is producing the card.

occurredAttext (ISO-8601)

Time when the card production request has been successfully sent to the kiosk machine.

errorCode (optional)text

Code returned when sending the card production request is not successful. The possible values are listed in request errors.

description (optional)text

Detailed description of the error code.

Request
curl -X PUT \
  'https://api.wise-sandbox.com/v3/spend/profiles/{{profileId}}/cards/{{cardToken}}/production' \
  -H 'Authorization: Bearer <your api token>' \
  -H 'Content-Type: application/json' \
  -d '{
    "kioskId": "WIS00001"
  }'
Successful Response
{
    "status": "IN_PROGRESS",
    "kioskId": "WIS00001",
    "occurredAt" : "2024-01-01T12:24:56.121Z"
}
Error Response
{
    "status": "REQUEST_ERROR",
    "kioskId": "WIS00001",
    "occurredAt" : "2024-01-01T12:24:56.121Z",
    "errorCode": "PIN_VERIFICATION_FAILED",
    "description": "The PIN cannot be verified because the server is unreachable",
}
Retrieve card production status

GET /v3/spend/profiles/{{profileId}}/cards/{{cardToken}}/production

You should rely on the card production status change webhook to be notified on the production status.

In some situations, you might want to use this endpoint to synchronously retrieve the production status of a card.

Response
statustext

Current production status. The possible values can be found in the production status flow diagram.

kioskIdtext

Identifier that specifies which kiosk machine is producing the card. When the status is READY, this value is null.

occurredAttext (ISO-8601)

Time when the card production request has been sent to the kiosk machine.

errorCodetext

Code returned when card production is not successful. The possible values are listed in production errors. Nullable.

descriptiontext

Detailed description of the error code. Nullable.

Request
curl -X GET \
  'https://api.wise-sandbox.com/v3/spend/profiles/{{profileId}}/cards/{{cardToken}}/production' \
  -H 'Authorization: Bearer <your api token>' \
  -H 'Content-Type: application/json'
Card ready for production response
{
    "status": "READY",
    "kioskId": null,
    "occurredAt": "2024-01-01T12:24:56.124Z",
    "errorCode": null,
    "description": "Card ready for production"
}
Request errors

These errors are related to the card production request when using the produce card endpoint

UNKNOWN_ERROR

An error occurred on the server.

REQUEST_ALREADY_EXISTS

The request has already been submitted.

KIOSK_ID_NOT_FOUND

The kiosk ID does not exist. Please verify that this kiosk ID is correctly set up.

CARD_TYPE_NOT_FOUND

The card type does not exist. Please ensure this card type is correctly set up.

INVALID_FIELD_VALUE

Some field values are unexpected.

INVALID_PIN_FORMAT

The PIN does not follow ISO0 or ISO2 standard.

EMPTY_OR_NULL_FIELD_VALUE

Some required fields are empty or null.

EMPTY_OR_NULL_OR_WRONG_SIZE_FIELD_VALUE

Some required fields are empty, null, or have incorrect lengths.

CHARSET_NOT_SUPPORTED

The character set for some fields is not supported.

INVALID_FIELD_FORMAT

Some field formats are invalid.

INVALID_FIELD_ENCODING

The encoding for some fields is not expected.

CDATA_ENCODING_OR_FORMATING_ERROR

The encoding or formatting for XML text field values is incorrect.

PIN_LENGTH_ERROR

The PIN length is incorrect.

PIN_VERIFICATION_FAILED

The PIN cannot be verified because the server is unreachable.

DATA_PREPARATION_FAILED

The server failed to parse the request.

PARTIAL_DATA_RECEIVED

Only part of the expected data has been received.

NO_BRANCH_LINKED_TO_KIOSK

The kiosk has not been linked to any branch.

IMAGE_SERVER_ERROR

The image server is unreachable or failed to process the data.

PAN_ALREADY_EXISTS

The PAN already exists.

PRINTER_OR_SATELLITE_NOT_READY

The printer or satellite is not ready to start the production.

REQUEST_CREATED_BUT_NOT_STARTED

The request has been created, but production is pending until the printer or satellite is ready

UNABLE_TO_ACCEPT_REQUEST

The system is currently busy and cannot accept new requests at this time

Production errors

These errors are related to the final production status of a card.

CB_NOT_AVAILABLE

Main server error: central base not available.

CB_DB_NOT_AVAILABLE

Main server error: database not available.

CB_NETWORK_NOT_AVAILABLE

Main server error: network not available.

CB_AUTHENTICATION_FAILED

Main server error: user authentication verification failed.

CB_SERVICE_NOT_ALLOWED

Main server error: the accessed service requires a higher security level.

CB_TIME

Main server error: timeout occurred on the Central Base side.

DP_NOT_AVAILABLE

Data processing error: the Data preparation module is not available.

DP_IO_ERROR

Data processing error: Input/Output error when communicating with the DP.

DP_TIMEOUT

Data processing error: timeout error.

SAT_SERVER_NOT_REACHABLE

Satellite agent error: the server is not reachable.

SAT_AUTHENTICATION_FAILED

Satellite agent error: user authentication failed on satellite.

SAT_NETWORK_NOT_AVAILABLE

Satellite agent error: the network is not available.

PRT_NOT_REACHABLE

Printer error: printer is not reachable.

PRT_SETUP_ERROR

Printer error: printer setup is incorrect.

PRT_TIMEOUT

Printer error: printer timeout.

PRT_RIBBON

Printer error: ribbon error.

PRT_LOCK_ERROR

Printer error: printer physically unlocked.

PRT_RIBBON_MISSING

Printer error: ribbon is missing.

PRT_RIBBON_ENDED

Printer error: ribbon has ended.

PRT_COVER_OPEN

Printer error: printer cover is open.

PRT_PAUSED

Printer error: printer paused.

PRD_UNEXPECTED_DATA

Production error: production data is incorrect.

PRD_FEEDER_EMPTY

Production error: feeder is empty.

PRD_FEEDER_JAM

Production error: feeder card jam.

PRD_HOPPER_FULL

Production error: hopper is full.

PRD_HOPPER_DOOR

Production error: hopper door is open.

PRD_HOPPER_JAM

Production error: hopper card jam.

PRD_MAGSTRIPE

Production error: error occurred during magstripe encoding.

PRD_SMARTCARD

Production error: error occurred during chip personalization.

PRD_EMBOSSER

Production error: error occurred during embossing.

PRD_TIMEOUT

Production error: production timeout has been reached.

PRD_REJECT_FULL

Production error: reject box is full.

PRD_SMARTCARD_CARD_NOT_IN_READER

Production error: card is not in reader.

PRD_FINAL_VALIDATION_NOK

Production error: user has rejected card production.

INV_NOT_INITIALIZED

Production error: inventory not initialized.

UNKNOWN_ERROR

Unknown error.