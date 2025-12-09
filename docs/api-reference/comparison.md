# ComparisonCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/api-reference/comparison

---

Comparison
Copy

The comparison API can be used to request price and speed information about various money transfer providers. This includes not only Wise but other providers in the market.

Price Estimation

The quotes (price and speed) provided by this API are based off of real quotes collected from 3rd party websites. We collect both the advertised exchange rate and fee for each provider for various amounts. When a comparison is requested, we calculate the markup percentage of the collected exchange rate on the mid-market rate at the time of collection. We then apply this markup to the current mid-market rate to provide a realistic estimate of what each provider offers. We collect data for all providers around once per hour to ensure we provide as accurate and up-to-date information as possible.

Note: Today, we only provide estimations for FX transactions with a Bank Transfer pay-in and pay-out option. This is important to stress as many providers offer significantly different fees and exchange rates when used debit/credit card, cash etc.

For more details on the data collection process please see the following page: https://wise.com/gb/compare/disclaimer

If you have questions or suspect any data to be inaccurate or incomplete please contact us at: comparison@wise.com

Delivery Estimation

Similar to price, we collect speed data for most (if not all) providers which we have price information for. Many providers display speed estimates to their customers in a number of different ways.

Some examples:

"The transfer should be complete within 2-5 days"
"The money should arrive in your account within 48 hours"
"Should arrive by 26th Aug"
"Could take up to 4 working days"

The below API intends to model these in a consistent format by providing a minimum and maximum range for all delivery estimations. An estimate that states "up to X" will have "max" set to a duration but "min" as null; "from X" will have "min" set to a duration and "max" as null. Finally, for those providers who offer a specific, point in time estimation (like Wise), the API will surface a duration where "min" and "max" are equal.

Quotes structure

In order to provide the most flexible and accurate data for clients, we surface a denormalized list of quotes per provider where each quote represents a unique collection of comparison "dimensions".

A single given provider may expose multiple quotes for the same currency route. The most common example is where a provider offers different pricing for two different countries which use the same currency, e.g.:

Provider X:

GBP EUR 1000 [GB -> ES] fee: 10, rate: 1.5
GBP EUR 1000 [GB -> DE] fee: 8, rate: 1.5
GBP EUR 1000 [GB -> FR] fee: 10, rate: 1.35

The same principle applies for speed, i.e. a provider may have different speed estimates for different target countries. Hence, we expose these as discrete quotes, where a quote is a unique combination of route, country, speed and price factors.

A client may choose to reduce this set of quotes down to a single or several quotes in order to display a relevant quote to a given user. An example where we take the cheapest quote for a given currency route (and also surface the target country) can be seen at the below link:

https://wise.com/gb/compare/?sourceCurrency=GBP&targetCurrency=EUR&sendAmount=1000

Operations

GET
/v4/comparisons/?sourceCurrency=GBP&targetCurrency=EUR&sendAmount=10000
The Comparison Resource
v4
Fields
idinteger

Provider ID

aliastext

Provider alias (lowercase slug name)

nametext

Provider name (presentational / formal name)

logostext

List of URLs pointing to provider logos (svg, png)

typetext

Provider type: bank, money transfer provider, or travel money

partnerboolean

Whether a partner of Wise or not

quotesarray

An array of estimated quotes per provider

quotes.ratedecimal

The live estimated exchange rate for the provider for this quote

quotes.feedecimal

The estimated fee for the provider for this quote

quotes.dateCollectedtimestamp

The date of collection for the original quote

quotes.sourceCountrytext

Source country (ISO 3166-1 Alpha-2 code)

quotes.targetCountrytext

Target country (ISO 3166-1 Alpha-2 code)

quotes.markupdecimal

Percentage markup on the quoted rate compared to the mid-market rate

quotes.receivedAmountdecimal

The total estimated received amount for the provider for this quote (non-null only if a send amount request was made)

quotes.sendAmountdecimal

The total estimated send amount for the provider for this quote (non-null only if a recipient gets amount request was made)

quotes.deliveryEstimationobject

Delivery estimation details - i.e. a speed estimate

quotes.deliveryEstimation.durationobject

Duration range

quotes.deliveryEstimation.duration.miniso 8601 duration format

Minimum quoted time for transaction delivery

quotes.deliveryEstimation.duration.maxiso 8601 duration format

Maximum quoted time for transaction delivery

quotes.deliveryEstimation.providerGivesEstimateboolean

Whether a provider publicly surfaces a speed estimate or not

quotes.isConsideredMidMarketRateboolean

Whether the provider's rate is close enough to the true mid-market rate to be classified as a mid-market rate

Comparison Object
{
  "sourceCurrency": "GBP",
  "targetCurrency": "EUR",
  "sourceCountry": null,
  "targetCountry": null,
  "providerCountry": null,
  "providerTypes": [
    "bank",
    "moneyTransferProvider"
  ],
  "amount": 10000.0,
  "amountType": "SEND",
  "providers": [
    {
      "id": 39,
      "alias": "wise",
      "name": "Wise",
      "logos": {
        "normal": {
          "svgUrl": "https://wise.com/public-resources/assets/logos/wise-personal/logo.svg",
          "pngUrl": "https://wise.com/public-resources/assets/logos/wise-personal/logo.png"
        },
        "inverse": {
          "svgUrl": "https://wise.com/public-resources/assets/logos/wise-personal/logo_inverse.svg",
          "pngUrl": "https://wise.com/public-resources/assets/logos/wise-personal/logo_inverse.png"
        },
        "white": {
          "svgUrl": "https://wise.com/public-resources/assets/logos/wise-personal/logo_white.svg",
          "pngUrl": "https://wise.com/public-resources/assets/logos/wise-personal/logo_white.png"
        },
        "circle": {
          "svgUrl": "https://dq8dwmysp7hk1.cloudfront.net/logos/wise-mark.svg",
          "pngUrl": null
        },
        "altText": "Wise"
      },
      "type": "moneyTransferProvider",
      "partner": false,
      "quotes": [
        {
          "rate": 1.15989,
          "fee": 37.12,
          "dateCollected": "2019-10-22T14:36:43Z",
          "sourceCountry": null,
          "targetCountry": null,
          "markup": 0.0,
          "receivedAmount": 11555.84,
          "sendAmount": null,
          "deliveryEstimation": {
            "duration": {
              "min": "PT20H8M16.305111S",
              "max": "PT20H8M16.305111S"
            },
            "providerGivesEstimate": true
          },
          "isConsideredMidMarketRate": true
        }
      ]
    },
    {
      "id": 1,
      "alias": "barclays",
      "name": "Barclays",
      "logos": {
        "normal": {
          "svgUrl": "https://dq8dwmysp7hk1.cloudfront.net/logos/barclays.svg",
          "pngUrl": "https://dq8dwmysp7hk1.cloudfront.net/logos/barclays.png"
        },
        "inverse": {
          "svgUrl": null,
          "pngUrl": null
        },
        "white": {
          "svgUrl": "https://dq8dwmysp7hk1.cloudfront.net/logos/barclays--white.svg",
          "pngUrl": "https://dq8dwmysp7hk1.cloudfront.net/logos/barclays--white.png"
        },
        "circle": {
          "svgUrl": "https://dq8dwmysp7hk1.cloudfront.net/logos/barclays-mark.svg",
          "pngUrl": null
        },
        "altText": "Barclays"
      },
      "type": "bank",
      "partner": false,
      "quotes": [
        {
          "rate": 1.12792426,
          "fee": 0.0,
          "dateCollected": "2019-10-22T14:00:04Z",
          "sourceCountry": "GB",
          "targetCountry": "ES",
          "markup": 2.75592858,
          "receivedAmount": 11279.24,
          "sendAmount": null,
          "deliveryEstimation": {
            "duration": {
              "min": "PT24H",
              "max": "PT24H"
            },
            "providerGivesEstimate": true
          },
          "isConsideredMidMarketRate": false
        },
        ...
        {
          "rate": 1.12792426,
          "fee": 0.0,
          "dateCollected": "2019-10-22T14:00:04Z",
          "sourceCountry": "GB",
          "targetCountry": "FI",
          "markup": 2.75592858,
          "receivedAmount": 11279.24,
          "sendAmount": null,
          "deliveryEstimation": {
            "duration": {
              "min": "PT24H",
              "max": "PT24H"
            },
            "providerGivesEstimate": true
          },
          "isConsideredMidMarketRate": false
        }
      ]
    },
    ...
  ]
}
Request a Comparison Quote
v4

GET /v4/comparisons/?sourceCurrency={{sourceCurrency}}&targetCurrency={{targetCurrency}}&sendAmount={{sendAmount}}

Request
sourceCurrencytext

ISO 4217 source currency code

targetCurrencytext

ISO 4217 target currency code

sendAmountdecimal

(Optional - must provide either sendAmount or recipientGetsAmount) Amount to send in source currency

recipientGetsAmountdecimal

(Optional - must provide either sendAmount or recipientGetsAmount) Amount to be received in target currency

midMarketRatedecimal

(Optional) Current mid-market rate between the source and target currency

wiseFeedecimal

(Optional) Wise's fee, if the Wise quote is known

wiseEstimatedDeliverytimestamp

(Optional) Wise's estimated delivery timestamp, if the Wise quote is known

wiseQuoteCreatedTimetimestamp

(Optional) Timestamp when the Wise quote was created

payInMethodtext

(Optional) Change the pay-in method of the Wise quote only (all other quotes will remain as bank transfer)

sourceCountrytext

(Optional) Filter by source country (ISO 3166-1 Alpha-2 code)

targetCountrytext

(Optional) Filter by target country (ISO 3166-1 Alpha-2 code)

providerCountrytext

(Optional) Filter by provider country (ISO 3166-1 Alpha-2 code)

filter"POPULAR"

(Optional) Filter by most popular competitors

numberOfProvidersinteger

(Optional) Number of popular competitors to return if filter is set to POPULAR- default value is 4

providerTypeslist of any of "bank","moneytransferprovider","travelMoney"

(Optional) Filter by provider type

providerstext

(Optional) Filter by specific provider aliases

excludePartners"true" or "false"

(Optional) Exclude Wise's partner banks - default true

includeWise"true"

(Optional) Overrides filters to ensure Wise data is returned, even if exclusionary filters are applied

Response

Returns a comparison object.

Example Request
curl -X GET \
  'https://api.wise.com/v4/comparisons/?sourceCurrency=GBP&targetCurrency=EUR&sendAmount=10000'