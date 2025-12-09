# Direct Debit: Simulating in SandboxCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/send-money/funding/direct-debit/simulation

---

Direct Debit: Simulating in Sandbox
Copy

This guide details the simulation endpoints available exclusively in the sandbox environment to help you test how your system handles various status changes and failure scenarios without initiating real-world financial transactions. This is crucial for verifying your logic for handling disconnected instruments, failed payments, and successful settlements.

Payment Instrument Status Simulation

You can simulate a Payment Instrument becoming disconnected to test your error handling logic for this scenario. A Payment Instrument marked as DISCONNECTED is no longer authorised and cannot be used for initiating Payins.

Method	Endpoint
POST	/v2/simulation/profiles/{{profile_id}}/payment-instruments/{{payment_instrument_id}}
Simulate a Disconnected Payment Instrument

To simulate a Payment Instrument being marked as DISCONNECTED (e.g., due to the customer cancelling their authorisation via their bank), send the following request:

Sample request:

{
  "scenario": "DISCONNECTED"
}

After a successful request (204 response code), you should receive a payment-instruments#status-change webhook, and a subsequent GET request for the Payment Instrument will show the status as DISCONNECTED.

Payin Status Simulation

These endpoints allow you to fast-forward a Payin into a key terminal status (FAILED or SETTLED) for testing purposes.

Method	Endpoint
POST	/v2/simulation/profiles/{{profile_id}}/payment-instruments/{{payment_instrument_id}}/payins/{{payin_id}}
Simulate a Failed Payin

To test how your system handles a payment failure, use the FAILED scenario. When a Payin fails, the status is set to FAILED, and the response includes a failureReason field for more granular reasoning.

Sample request for failed status:

{
  "scenario": "FAILED"
}

The FAILED status means the payment has failed due to cancellation or issues encountered while processing the payment.

Simulate a Settled Payin

To simulate a successful payment where the funds are available for the associated transfer, use the SETTLED scenario.

Sample request for settled status:

{
  "scenario": "SETTLED"
}

Upon a successful simulation, you should receive the relevant payment-instruments-payins#status-change webhook notification.