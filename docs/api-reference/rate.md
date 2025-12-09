# RateCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/api-reference/rate

---

Rate
Copy

Current and historical exchange rates by currency routes.

Operations

GET
/v1/rates
GET
/v1/rates?source=EUR&target=USD
The Rate resource
Fields
ratedecimal

Exchange rate value.

sourcetext

Source(send) currency code

targettext

Target(receive) currency code

timetimestamp

Timestamp for exchange rate.

Rate Object
{
  "rate": 1.166,
  "source": "EUR",
  "target": "USD",
  "time": "2018-08-31T10:43:31+0000"
}
Retrieve current rates

Please note that the from, to, and group parameters are not testable in our Sandbox environment at this time.

GET /v1/rates

Fetch latest exchange rates of all currencies.


GET /v1/rates?source=EUR&target=USD

Fetch latest exchange rate of one currency pair.


GET /v1/rates?source=EUR&target=USD&time=2019-02-13T14:53:01

Fetch exchange rate of specific historical timestamp.


GET /v1/rates?source=EUR&target=USD&from=2019-02-13T14:53:01&to=2019-03-13T14:53:01&group=day

Fetch exchange rate history over period of time with daily interval.


GET /v1/rates?source=EUR&target=USD&from=2019-02-13T14:53:01&to=2019-03-13T14:53:01&group=hour

Fetch exchange rate history over period of time with hourly interval.


GET /v1/rates?source=EUR&target=USD&from=2019-02-13T14:53:01&to=2019-03-13T14:53:01&group=minute

Fetch exchange rate history over period of time with 1 minute interval.


This endpoint only supports Bearer authentication for non-Affiliate partners.

Request
sourcetext

Source(send) currency code.

targettext

Target(receive) currency code.

timetimestamp

Timestamp to get historic exchange rate.

fromtimestamp or date

Period start date/time to get exchange rate history.

totimestamp or date

Period end date/time to get exchange rate history.

grouptext

Interval: day hour minute

Response

List of exchange rate values which meet your query criteria.

Additional notes about date/time formatting used above

The request/response field(s) below support both Timestamp (combined date and time) and Date (date only) formats:

Field	Sample
from	2019-03-13T14:53:01 or 2019-03-13
to	2019-03-13T14:53:01+0100 or 2019-03-13+0100

The request/response field(s) below support only Timestamp (combined date and time):

Field	Sample
time	2019-03-13T14:53:01 or 2019-03-13T14:53:01+0100

Timezone offset is supported but optional.

Example Request (Bearer token)
curl -X GET \
  https://api.wise-sandbox.com/v1/rates?source=EUR&target=USD \
  -H 'Authorization: Bearer <your api token>'
Example Response
[
  {
    "rate": 1.166,
    "source": "EUR",
    "target": "USD",
    "time": "2018-08-31T10:43:31+0000"
  }
]