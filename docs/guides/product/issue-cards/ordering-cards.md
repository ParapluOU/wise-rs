# Ordering cardsCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/product/issue-cards/ordering-cards

---

Ordering cards
Copy

How to order cards for your customers

This guide will teach you how to create and issue a new card for an account-owner. The card is associated to the account and all transactions will be reflected on the account's balance.

Card Eligibility

After creating a profile, cards can now be ordered. The first step is to retrieve the available card programs. A card program gives information on the type of card, the network that the card is on, as well as the default currency of the card.

If there are no card programs returned in the API call, it means that the profile cannot order any additional cards.

Type of Card
Virtual - No physical card will be printed. This card can be used for online transactions and added to digital wallets.
Physical - A physical card will be printed. On top of virtual card capabilities, physical cards can also be used at physical terminals and ATMs.
Approach the Sales team for more information on physical cards
Ordering a Card

After retrieving the card program, use the card order API to create a card order. It is also best practice to use the card address validation API to ensure the formatting of the address is correct.

The address is required for both virtual and physical cards as it will be used for AVS (address verification service) checks for transactions without a card present.

When the status of the card order is CARD_DETAILS_CREATED, a card will now be available to be retrieved.

The card token is the unique identifier for this card.

For physical card orders, it is recommended to subscribe to the card order status change webhook event to be notified of card order status changes.

You can cancel a card order by updating the card order status to CANCELLED with a reason. This can only be done when the card order is still in the PLACED status.

Card order status transitions
Card Order Limits

In order to control the number of cards a profile can create, we have card order limits in place. On production, the number of card orders is limited to 1 active physical card and 3 active virtual cards per personal profile. On sandbox, we allow up 10 physical cards and 30 virtual cards for testing. Additionally, no more than 3 virtual cards can be ordered per day. This limit includes existing cards and card orders.

Reach out to your account manager if you require a higher limit on cards per user

Sensitive Card Details

Retrieving the sensitive card details (PAN, PIN, CVV) of a card requires us to do so in a PCI-DSS compliant way, follow our sensitive card details guide on how to do so.

To retrieve sensitive card details, the card must be in either ACTIVE or FROZEN status. A 403 response will be returned when getting sensitive card details for a card in any other status.

Card PIN

Some card terminals and ATMs require users to enter their card PIN to authenticate transactions. By default, both physical and virtual card PINs are randomly set.

Optionally, we offer a feature to pre-set the PIN of the card during the card order process. Reach out to your account manager for this feature as Wise will first need to configure this to be done on our side.

Activate a physical card

To mitigate the possibility of fraudulent transactions on the card in the event of misdelivery of the card, users need to activate their card before they can use it for certain transactions. We offer two activation methods: Chip and PIN or Partner Control.

For Chip and PIN activation, contactless and magnetic stripe transactions attempted prior to the first successful Chip and PIN transaction will be declined. This activation method has been deprecated. For Partner Control activation, all transactions will be declined until the card status has been set to ACTIVE.

Activate a card using Chip and PIN (deprecated)

Physical cards issued with an ACTIVE card status require the cardholder to make a Chip and PIN transaction (inserting the card into the terminal and keying in the card PIN) to start using the card for contactless and magnetic stripe transactions.

Before the first Chip and PIN transaction, all spending permissions are enabled except POS_CONTACTLESS and POS_MAGSTRIPE, which are locked. At this point, the card is already ACTIVE and can be used for certain transactions.

After the first Chip and PIN transaction has been successfully performed, POS_CONTACTLESS and POS_MAGSTRIPE permissions will be enabled, with no change to other permissions. The card order status is moved to COMPLETED.

Activate a card using Partner Control

When Cards are issued with an INACTIVE card status, they can't be used to perform any transactions. You must activate the card to start using it by updating the card status to ACTIVE. After you have activated the card, it can immediately be used for transactions - as long as the spending permission for that transaction type is enabled.

Activating a physical card moves the card order status to COMPLETED. You should ensure that the card order has reached the PRODUCED status using the card order status change webhook before allowing activation. The PRODUCED status indicates that the card order has been produced and picked up by our delivery vendors to be dispatched to the cardholder.

You should also have confirmation from the cardholder that they have received their physical card before updating the card status to ACTIVE. This can be done by requiring the cardholder to key in the last 4 digits of their physical card upon receiving it. This can be validated against the lastFourDigits in the Card resource. To implement this, you should ensure that card details such as the PAN (Primary Account Number) should initially be hidden from the end customer.

Only physical cards of certain card programs can be activated over API. Check with our team whether this is supported in your integration.
Replace a physical card

You can replace your physical card if it is expiring or if it has been damaged. This can be done by creating a new card order with the replacementDetails object in the payload to define the cardToken to replace, as well as the reason for replacement.

A replacement card order must be of the same CardProgram as that of the card to be replaced.

The existing card can still be used until the replacement card has been activated. Once the replacement card is activated, the existing card will be blocked. If the existing card was tokenised, the payment token will be updated on replacement card activation without requiring any additional user action other than activating the replacement card.