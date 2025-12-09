# FaceTecCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/api-reference/facetec

---

FaceTec
Copy

Wise leverages FaceTec's facial biometric technology for authentication, offering a seamless integration experience through these APIs.

Operations

GET
/v1/facetec/public-key
Get Public Key

GET /v1/facetec/public-key

Retrieve Wise's FaceTec public key to be used when exporting 3D Facemap from your FaceTec host to Wise.

The exported FaceMap can be used to Enrol FaceMap.

Response

Plain text containing public key.

Example Request
curl -X GET \
  https://api.wise-sandbox.com/v1/facetec/public-key \
  -H 'Authorization: Bearer <your api token>' 
Example Response
-----BEGIN PUBLIC KEY-----
Public Key Content
-----END PUBLIC KEY----- 