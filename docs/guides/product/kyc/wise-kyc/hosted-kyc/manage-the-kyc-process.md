# Manage the KYC processCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/kyc/wise-kyc/hosted-kyc/manage-the-kyc-process

---

Manage the KYC process
Copy
Monitor KYC review status webhooks

In order to provide the hosted KYC experience to your customer, you must subscribe to the kyc-reviews#state-change webhook.

When the KYC process begins - either proactively or at the moment of your customer's first transfer - you receive a webhook message indicating that the KYC status has changed. This status is either WAITING_CUSTOMER_INPUT or PASSED/PASSED_WITH_REQUIREMENTS.

KYC review state change event
Event type: kyc-reviews#state-change
Profile level subscriptions: Not Supported
Application level subscriptions: Supported

This event is triggered when a KYC Review state has changed.

Schema
data.resource.iduuid

ID of the KYC Review.

data.resource.statestring

Status of the KYC Review. See KYC Review Status for possible values and state diagram.

data.resource.profileIdinteger

ID of the profile KYC Review belongs to.

data.resource.requiredBydatetime

Timestamp by which the underlying requirement set needs to be verified to not block the customer. Only relevant if the status is PASSED_WITH_REQUIREMENTS.

data.resource.createdAtdatetime

Timestamp marking the creation of the KYC Review.

data.resource.updatedAtdatetime

Timestamp marking the last update of the KYC Review.

data.resource.triggerReference[n].typestring

Type of the underlying action/process this KYC Review is for. Usually a reference to which product this KYC Review is for (like QUOTE or TRANSFER) or a reference to a KYC process on the profile that isn’t related to a specific product (like REFRESH_CYCLE or REPAPERING). Currently, only types QUOTE and TRANSFER are supported.

data.resource.triggerReference[n].triggerData.idstring

ID of the underlying product object. e.g. if type is TRANSFER then this would be transfer ID. This ID might be null if underlying action is a process like REPAPERING.

Example `kyc-reviews#state-change` event
{
  "data": {
    "resource": {
      "id": "46e1a5c4-4a9b-4563-39d3-18174d3ac0f8",
      "state": "WAITING_CUSTOMER_INPUT",
      "profileId": 22016766,
      "requiredBy": "2024-09-03T16:22:02.257725",
      "createdAt": "2024-09-03T16:22:02.257725",
      "updatedAt": "2024-09-03T16:29:41.147522",
      "triggerReferences": [
        {
          "type": "QUOTE",
          "triggerData": {
            "id": "ba83s43a-f623-46f0-956d-196c13e2ab01"
          }
        }
      ]
    }
  },
  "subscription_id": "8df95817-8085-40aa-9bda-e3bf46e7a21a",
  "event_type": "kyc-reviews#state-change",
  "schema_version": "2.0.0",
  "sent_at": "2024-09-03T16:29:42Z"
}

Refer to the KYC Review Status guide for all possible KYC review status values.

Generate the hosted KYC link

If the KYC review status is WAITING_CUSTOMER_INPUT, action is required from your customer. They complete this in the Wise hosted KYC screens. To generate the link for the Hosted KYC flow, use PATCH /v1/profiles/{profileId}/kyc-reviews/{kycReviewId}.

This endpoint's response includes the link for the Hosted KYC flow, along with a redirectUrl field. Wise redirects the customer to this redirectUrl once the Hosted KYC flow completes.

Update KYC review to get a hosted KYC link

PATCH /v1/profiles/{profileId}/kyc-reviews/{kycReviewId}

Updates the KYC review with a redirect url.

Returns KYC review object with a link field containing a url where the end customer needs to be directed in order to complete the Hosted KYC flow.

Once Hosted KYC flow is completed by the end customer, they will be redirected to the redirectUrl provided in this API call. During the redirection, the redirectUrl will be appended with query parameters: status=success or status=error, indicating whether Hosted KYC flow was completed successfully or not.

Request Fields
redirectUrlstring

URL where the user will be redirected at the end of the flow. Can contain query params and path fragments. It has to be a valid URL as per RFC2396.

Response

Possible HTTP status codes

200 - OK

Returns updated KYC Review object

400 - Bad Request

Invalid request. (e.g. not valid URI)

401 - Unauthorized

User is not authorized to access the resource

404 - Not Found

KYC Review not found

Example Request
curl -L -X PATCH \
  'https://api.wise-sandbox.com/v1/profiles/{personalProfileId}/kyc-reviews/{kycReviewId}' \
  -H 'Content-Type: application/json' \
  -H 'Authorization: Bearer <user access token>' \
  -d '{
    "redirectUrl": "https://example.com"
  }'
Example Response
{
  "id": "46e1a5c4-4a9b-4563-39d3-18174d3ac0f8",
  "createdAt": "2024-09-03T16:22:02.257725",
  "updatedAt": "2024-09-03T16:29:41.147522",
  "requiredBy": "2024-09-03T16:22:02.257725",
  "status": "WAITING_CUSTOMER_INPUT",
  "link": {
    "value": "https://wise-sandbox.com/embedded-flows/verification?token=d7332edb-25bf-41af-a4e9-09f5efe39ded&checkId=3120073",
    "expiresAt": "2024-09-03T16:29:41.203225146"
  },
  "triggerReferences": [{
    "type": "QUOTE",
    "triggerData": {
      "id": "ba83s43a-f623-46f0-956d-196c13e2ab01"
    }
  }],
  "redirectUrl": "https://example.com"
}

For a complete reference, see the KYC Review API reference.

Direct the customer to the hosted KYC page

Direct your customer to the Wise Hosted KYC page using the link received in the response to the PATCH /v1/profiles/{profileId}/kyc-reviews/{kycReviewId}. The integration method depends on your platform:

Web: Open the link in a new browser tab or redirect the customer to the link.
Android: Embed the page within your app using Custom Tabs.
iOS: Embed the page within your app using Safari View Controller.

This page guides the customer through a UI with multiple forms and may include liveness checks. Customers might also be asked to upload documents, such as IDs. The exact requirements vary based on the country of residence, source currency, target currency, and transfer amount. During implementation, you can configure the UI styling to match your branding.

Note: The Hosted KYC link expires 5 minutes after generation. To get a new link, call the Update KYC Review endpoint again.

Handle hosted KYC flow completion

Once the customer provides all necessary information in the Hosted KYC flow, Wise redirects them back to the redirectUrl you provided. When redirecting, the redirectUrl will be appended with either status=success or status=error as a query parameter , indicating if the Hosted KYC flow has been completed successfully.

If the flow completes successfully, you receive a KYC review status webhook with the status PROCESSING.

Manage KYC review outcomes

Wise reviews the information provided by the customer. To ensure a review by one of our agents, the transfer must be in a funded state. You can fund the transfer by following the Send Money - Funding guide.

Based on the review result:

Information accepted: If the customer's provided information is accepted, you receive a webhook message with a PASSED KYC review status. The customer can then proceed with the underlying action that triggered the KYC process (e.g., a transfer).
More information required: If more information is needed from the customer, you receive a webhook message with a WAITING_CUSTOMER_INPUT KYC review status. In this case, repeat the process with a new redirect link to direct the customer back to the Hosted KYC flow to provide additional information.