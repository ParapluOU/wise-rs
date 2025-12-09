# Initiate a returnCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/receive-money/initiate-a-return

---

Initiate a return
Copy

You can initiate returns over API for payments that have been received via Swift. The flow looks like this:

To implement returns:

Initiate a return via the Returns API
Monitor the return status via the Transfer State Change webhook
Check reconciliation of the return using Balance Statement API
Initiate a return

Once you identify the transfer you wish to return, a payment return can be initiated by calling the Returns API and providing the transfer ID of the transfer you wish to return.

Ensure that you have sufficient funds in your balance to cover the return. The balance will be debited asynchronously so the API may return success even in cases of insufficient funds. transfer-state-change events can be used to confirm the return was successfully initiated, this is described in the next section.

Wise has controls in place to ensure that a transfer can only be returned once. If a return is attempted on a transfer that is being returned or has been returned already, the API will return an error.

Monitor the return status

We do not require partners to monitor the return status. Returns are usually processed instantly and in the unlikely event of a failure, our operations team will reach out to the partner to resolve the issue. There are no actions required from the partner once a payment return is initiated, hence we do not consider it necessary to monitor the return status.

However, if you wish to monitor the return status, you can subscribe to the Transfer State Change webhook. The transfer here will be the original inbound transfer. Transfer lifecycle extends beyond settlement, hence we use the same resource for the return.

A transfer has many states, but only the following are relevant for returns:

outgoing_payment_sent: The status of a settled transfer, this also indicates that the return has not been initiated yet.
bounced_back: Once the funds have been debited from the balance, the transfer will be set to this state. Wise will then start processing the return.
refunded: The return has been successfully processed and the payment in en route to the originator. Since the return is also sent over Swift, it may take a while to reach.
Reconcile the return

Once the return has been processed, you can reconcile the return by checking the Balance Statement API. Using the transferId of the original inbound transfer, you can find the return in the balance statement, and it should be reflected as a debit.