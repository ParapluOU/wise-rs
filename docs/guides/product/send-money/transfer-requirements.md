# Transfer Requirements | Wise Platform

Source: https://docs.wise.com/guides/product/send-money/transfer-requirements

---

Transfer Requirements
Copy

Each transfer can have additional requirements that are needed in order to process the transfer. To determine the transfer requirements needed, your application should use the transfer requirements endpoints to request the requirements for the transfer.

Requirements are returned in a dynamic form, with form types, restrictions, and validation included in a consistent method. This allows your application to present these to users in a consistent way.

Once all transfer requirements have been gathered, they should be added to the details section of the create transfer API call.

For a full reference, please see the request transfer requirements API reference.

Including transfer requirements that are not optional is important, as it helps ensure that transfers are able to process as quickly as possible with further requests for information reduced. Please ensure that this process is part of your integration.