# Raise dispute with dynamic flowCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/issue-cards/card-disputes-dynamic-flow

---

Raise dispute with dynamic flow
Copy

Customers can raise disputes to recover funds from fraudulent transactions or problematic products and services.

For the dispute submission process, Wise offers Dynamic Flow, a way to easily build embeddable forms into your application. There are 2 ways you can implement our dynamic flow for disputes:

Customer Initiated
Partner Customer Support Initiated
Customer Initiated

The customer initiated flow is used when you would like to show the dynamic flow on your customer facing app.

Partner Customer Support Initiated

The partner customer support initiated flow is used when you would like to show the dynamic flow to your internal app used only accessible to your employees.

If you think that a card has been compromised, block or freeze the card.

After submission of the dispute, Wise's agents will get in contact with your support team should they need anymore information.

Setting up the submit dispute API for Dynamic Forms

You will need to implement a POST API with the following format:
POST https://{{yourApiUrl}}/v3/spend/profiles/{{profileId}}/dispute-form/flows/{{scheme}}/{{reason}}

This API should forward the call to POST https://{{wiseUrl}}/v3/spend/profiles/{{profileId}}/dispute-form/flows/{{scheme}}/{{reason}} along with the request body. This is required as the dynamic form returned by us will automatically be configured to call your POST API In order to redirect the Dynamic Flow JavaScript library to your domain please use baseUrl or fetcher as part of the dynamic flows setup


See example backend implementation

The response to the submit call of the dynamic flow will include the x-df-exit: true HTTP header. This header is used by the JavaScript framework to add an option to exit the dynamic flow and redirect the user to a different page (or exit a WebView depending on the client's implementation). In order to intercept the last page on the frontend, an onClose function should be added to JavaScript, for example:

const onClose = () => {
  console.log("DF is exiting");
  window.location.href = "https://www.google.com/";
};
Retrieve a list of dispute reasons

GET /v3/spend/profiles/{{profileId}}/dispute-form/reasons

Retrieves the list of possible reasons for submitting a dispute.

If a reason code has subOptions, those should be used for submitting disputes.

Example Request
curl -X GET \
  https://api.wise-sandbox.com/v3/spend/profiles/{{profileId}}/dispute-form/reasons \
  -H 'Authorization: Bearer <your api token>'
Response

Returns a collection of Reason.

Example Response
[
  {
    "code": "ATM_DISPENSED_NO_FUNDS",
    "description": "I didn't receive the money from the ATM or cash machine",
    "isFraud": false,
    "supportsMultipleTransactions": false,
    "tooltip": "Choose this if the ATM did not dispense the money or gave you less than expected"
  },
  {
    "code": "WRONG_AMOUNT",
    "description": "I was charged the wrong amount or currency",
    "isFraud": false,
    "supportsMultipleTransactions": false,
    "tooltip": "Choose this if you were overcharged or the payment was in a different currency than you were expecting"
  },
  {
    "code": "TROUBLE_WITH_GOODS_SERVICES",
    "description": "There's a problem with the goods or service I ordered",
    "isFraud": false,
    "supportsMultipleTransactions": false,
    "tooltip": "Choose this if the goods or service never arrived, or if the product was defective or different from what you expected"
  },
  {
    "code": "MERCHANT_CHARGED_AGAIN",
    "description": "I got an unexpected charge from a merchant",
    "isFraud": false,
    "supportsMultipleTransactions": false,
    "tooltip": "Choose this for subscription charges, when you've paid twice for one purchase, or when you know the merchant from a past transaction but aren't sure why they charged you"
  },
  {
    "code": "NO_REFUND",
    "description": "I haven't received the refund",
    "isFraud": false,
    "supportsMultipleTransactions": false,
    "tooltip": "Choose this when you've been promised a refund and it hasn't arrived"
  },
  {
    "code": "UNAUTHORIZED",
    "description": "I did not make, authorize, or participate in this transaction",
    "tooltip": "Choose this if you don't know the merchant or have never purchased anything from them",
    "subOptions": [
      {
        "code": "UNEXPECTEDLY_CHARGED_AGAIN",
        "description": "A past merchant unexpectedly charged me again",
        "isFraud": false,
        "supportsMultipleTransactions": false
      },
      {
        "code": "UNWANTED_SUBSCRIPTION",
        "description": "I've been charged for a subscription without my permission",
        "isFraud": false,
        "supportsMultipleTransactions": false
      },
      {
        "code": "CARD_POSSESSION",
        "description": "I don't recognise a transaction",
        "isFraud": true,
        "supportsMultipleTransactions": true,
        "tooltip": "Choose this if you had your card at the time of the transactions, or if you think your card details have been compromised"
      },
      {
        "code": "CARD_NO_POSSESSION",
        "description": "My card was lost or stolen",
        "isFraud": true,
        "supportsMultipleTransactions": true,
        "tooltip": "Choose this if you didn't have your card at the time of the transactions"
      }
    ]
  }
]
Retrieving the dynamic flow for disputes

POST /v3/spend/profiles/{{profileId}}/dispute-form/flows/step/{{scheme}}/{{reason}}?transactionId={{transactionId}}

Retrieves the JSON for initiating the dispute flow. This endpoint should be used in conjuction with Wise's Dynamic Flow framework.

The JSON data in the response must be passed into the Dynamic Flow Framework which handles the rest of the multi-step dispute submission including the generation of the subsequent pages (if needed) and the creation of the dispute, along with all the required documents.

A sample implementation of the dynamic flow for Disputes can be found here.

Path and Request Parameters
schemetext

The network of the card that was used to make this transaction. One of MASTERCARD or VISA

reasontext

Dispute reason code supplied by the dispute reasons API

transactionIdtext

The ID of the transaction that is being disputed. It can be a comma separated list of IDs if the reason code has the supportsMultipleTransactions flag

Request Body
emailtext

Email used to receive communications regarding the dispute from Wise (ex. your support team's email)

Setting up the API

You will need to implement a GET API with the following format:
GET https://{{yourApiUrl}}/v3/spend/profiles/{{profileId}}/dispute-form/flows/step/{{scheme}}/{{reason}}?transactionId={{transactionId}}

This API should forward the call to POST https://{{wiseUrl}}/v3/spend/profiles/{{profileId}}/dispute-form/flows/step/{{scheme}}/{{reason}}transactionId={{transactionId}} along with the request parameters. This is required as the dynamic flow returned by Wise will automatically be configured to call your GET API In order to redirect the Dynamic Flow JavaScript library to your domain please use baseUrl or fetcher as part of the dynamic flow setup


See example backend implementation

Example Request
curl -X POST \
  'https://api.wise-sandbox.com/v3/spend/profiles/{{profileId}}/dispute-form/flows/step/{{scheme}}/{{reason}}?transactionId={{transactionId}}' \
  -H 'Authorization: Bearer <your api token>' \
  -H 'Content-Type: application/json' \
  -d '{
    "email": support@partner.com 
  }'
Response

Returns information required to populate the form with the correct information. Note how the action field contains the url and method to the next step

Example Response
{
  "key": "TROUBLE_WITH_GOODS_SERVICES",
  "type": "form",
  "title": "There's a problem with the goods or service I ordered",
  "actions": [],
  "schemas": [],
  "layout": [
    {
      "type": "decision",
      "options": [
        {
          "title": "I never got the goods or service I ordered",
          "action": {
            "url": "/v3/spend/profiles/12345/dispute-form/flows/visa/no-goods-or-services?transactionId=6789",
            "method": "GET"
          },
          "disabled": false,
          "description": "Choose this if the order was cancelled or never arrived"
        },
        {
          "title": "Something is wrong with the goods or service I ordered",
          "action": {
            "url": "/v3/spend/profiles/12345/dispute-form/flows/visa/something-wrong-what-was-received?transactionId=6789",
            "method": "GET"
          },
          "disabled": false
        },
        {
          "title": "I think there might be an issue with the merchant",
          "action": {
            "url": "/v3/spend/profiles/12345/dispute-form/flows/visa/scam?transactionId=6789",
            "method": "GET"
          },
          "disabled": false,
          "description": "Choose this if you haven't heard from the merchant, or have found scam reviews"
        }
      ]
    }
  ]
}
Submitting disputes

POST /v3/spend/profiles/{{profileId}}/dispute-form/flows/{{scheme}}/{{reason}}

Submit the dispute.

Path Variables
schemetext

The network of the card that was used to make this transaction. One of MASTERCARD or VISA

reasontext

Dispute reason code supplied by the dispute reasons API

Information required for a dispute submission is different per dispute reason. For more information, please use the dropdown and select a dispute reason.

View Request for Dispute Reason:

Select a Dispute Reason
Response

The submit API will return back a response to be used with dynamic flow. If you are using the API without dynamic flow, the response can be ignored.

Example Response
{
  "key": "final",
  "type": "form",
  "title": "Done!",
  "actions": [
    {
      "title": "Continue",
      "exit": true,
      "$id": "continue"
    }
  ],
  "schemas": [],
  "layout": [
    {
      "width": "md",
      "components": [
        {
          "url": "https://wise.com/web-art/assets/illustrations/email-success-large%402x.png",
          "type": "image"
        } 
      ],
      "type": "box"
    },
    {
      "margin": "lg",
      "align": "center",
      "type": "info",
      "markdown": "Thanks for reporting this transaction. It's pre-authorised right now, but as soon as it becomes \"spent\" we'll begin our investigation."
    },
    {
      "type": "button",
      "action": {
        "$ref": "continue"
      }
    }
  ]
}
Example backend implementation for dynamic flow disputes (Java)
Partner Implementation
import com.fasterxml.jackson.databind.node.JsonNodeFactory;
import com.fasterxml.jackson.databind.node.ObjectNode;
import lombok.extern.slf4j.Slf4j;
import org.springframework.http.MediaType;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;
import org.springframework.web.reactive.function.client.WebClient;
import reactor.core.publisher.Mono;

import javax.validation.Valid;

@Slf4j
@RestController
@RequestMapping(value = "/v3/spend/profiles/{profileId}/dispute-form", produces = MediaType.APPLICATION_JSON_VALUE)
public class MyPartnerProxy {
    private static String TARGET_BASE_URL = "https://api.wise-sandbox.com";
    private static String ACCESS_TOKEN = "Bearer 492b992e-85dd-4671-8095-b0d1d2235d07";

    private static String STEP_URL = "/v3/spend/profiles/{profileId}/dispute-form/flows/step/{scheme}/{reason}";
    private static String SUBMIT_URL = "/v3/spend/profiles/{profileId}/dispute-form/flows/{scheme}/{reason}";

    private WebClient webClient;

    public MyPartnerProxy() {
        this.webClient = WebClient.builder()
                .baseUrl(TARGET_BASE_URL)
                .defaultHeader("Authorization", ACCESS_TOKEN)
                .build();
    }

    @GetMapping("/flows/step/{scheme}/{reason}")
    public ResponseEntity<Object> getStep(final @Valid @PathVariable("profileId") Long profileId,
                                          final @PathVariable("scheme") String scheme,
                                          final @PathVariable("reason") String reason,
                                          final @RequestParam("transactionId") String transactionId) {
        ObjectNode data = JsonNodeFactory.instance.objectNode();
        data.put("email", "support@partner.com");

        Object stepResponse = webClient.post()
                .uri(STEP_URL, uriBuilder -> uriBuilder
                        .queryParam("transactionId", transactionId)
                        .build(profileId, scheme, reason)
                )
                .body(Mono.just(data), ObjectNode.class)
                .accept(MediaType.APPLICATION_JSON)
                .retrieve()
                .bodyToMono(Object.class)
                .block();
        return ResponseEntity.ok(stepResponse);
    }

    @PostMapping("/flows/{scheme}/{reason}")
    public ResponseEntity<Object> submit(final @Valid @PathVariable("profileId") Long profileId,
                                         final @PathVariable("scheme") String scheme,
                                         final @PathVariable("reason") String reason,
                                         @RequestBody final Object payload) {
        Object submitResponse = webClient.post()
                .uri(SUBMIT_URL, uriBuilder -> uriBuilder
                        .build(profileId, scheme, reason)
                )
                .body(Mono.just(payload), Object.class)
                .accept(MediaType.APPLICATION_JSON)
                .retrieve()
                .bodyToMono(Object.class)
                .block();
        return ResponseEntity.ok(submitResponse);
    }

}
Customizing your Dynamic Flow

Wise's Dynamic Flow uses Bootstrap for responsive UI, based on a 12-grid column layout. The defaults for currency selection, amount and file upload are col-xs-12, col-sm-6, and col-xs-12 for the rest. The CSS classes used are listed below. These class selectors can be used to override existing CSS

To override the existing CSS, add a new stylesheet and place the reference to that file after a Bootstrap CSS reference (if any), so that it will override any previous style properties. Alternatively, add !important to a property/value to override ALL previous styling rules for that property. Avoid using this for all properties, as the stylesheet will be large and more difficult to debug. In this code example, modify the CSS file imports in App.js to view the different overriding style sheets.

Please note that some of the form elements also use core Bootstrap classes, so be careful when you modify the styles.

An interactive demo can be found here.

Input labels
class	description
.control-label.d-inline	All input labels
.d-inline	Text, selection and file upload input labels
.d-inline-block	Text input labels
.np-checkbox__text	Checkbox text
Input
class	elements
.form-group	Form input and labels
.form-control	Text input
.d-inline-flex	Text in dropdown selection
.form-control-placeholder	Dropdown selection placeholder text
.tw-form-control	textarea input
.droppable, .tw-droppable-md or .droppable-md	File upload
.droppable-default-card or .droppable-card-content	Content for file upload
.droppable-complete-card	Completed file upload
.m-b-3	Text in file upload block
Button
class	elements
.btn.np-dropdown-toggle	Dropdown selection
.sr-only	Radio buttons
.promoted-selection	To select names
.tw-radio-button.checked .tw-radio-check	For checked buttons
.tw-radio-check	All radio buttons
.np-radio__text	Text for radio buttons
.btn.btn-primary.btn-sm	File upload buttons
.btn.btn-sm.btn-accent	Cancel button during file upload
.np-checkbox, .checkbox	Checkbox button
.btn.btn-md.np-btn-block.btn-priority-2	"Continue" button
Icon
class	elements
.chevron-color, or tw-icon-chevron-up	Arrow icon for dropdown selection
.circle, .circle-sm , .circle-inverse or .tw-icon-upload	File upload icon
.tw-icon-info-circle	Information icon for alert
.np-checkbox-button	Checkbox selection icon
Alert
class	elements
.alert-detach.alert-danger	Alert for form validation
.d-flex.alert-neutral	"You may be responsible for dispute administration fees" alert
Others
class	elements
.text-xs-center.m-b-3	"I was charged the wrong amount or currency"
.np-drawer-header--title	"Search ..." title in currency selection
.np-select-filter	Search bar in currency selection
legend	"Please upload" above file upload
.np-popover__content or .np-bottom-sheet--content	Popover content upon clicking information icon beside "Confirmation of the correct price"