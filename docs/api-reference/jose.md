# JOSECopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/api-reference/jose

---

JOSE
Copy

Wise uses the JOSE framework to accept and respond with signed and encrypted payloads. The following endpoints allow you to manage keys that apply to this process.

For more information, please speak with your Implementation team.

Operations

GET
/v1/auth/jose/response/public-keys
POST
/v1/auth/jose/request/public-keys
POST
/v1/auth/jose/playground/jws
POST
/v1/auth/jose/playground/jwe
POST
/v1/auth/jose/playground/jwe-direct-encryption
GET
/v1/auth/jose/playground/jwe
Get Wise public signing key

GET /v1/auth/jose/response/public-keys

This endpoint returns a public key issued by Wise for verifying signed HTTP responses and for encrypting payloads. These parameters must always be included.

For both signature verification and payload encryption, the process involves storing this public key after retrieval. In both cases, the stored public key should be used without calling this endpoint.

If verification of the signed request fails or you receive an encryption error, call this API once to issue a fresh key from Wise and then try verification one more time.

Parameters
versionstring

Fetch a specific public key version. If omitted the most recent public key is provided.

algorithmstring

Algorithm to be used for signature verification or payload encryption. This must match the algorithm used during request.

Signature verification (Scope: PAYLOAD_SIGNING): ES256, ES384, ES512, PS256, PS384, PS512
Payload encryption (Scope: PAYLOAD_ENCRYPTION): RSA_OAEP_256
scopestring

Scope of the signature. Value must be PAYLOAD_SIGNING or PAYLOAD_ENCRYPTION.

Response

Returns a public key object.

versionint

Version of the public key issued.

keyMaterial.algorithmstring

Algorithm to be used with the key. Values can be ES256, ES384, ES512, PS256, PS384, PS512, or RSA_OAEP_256.

keyMaterial.keyMaterialstring

Public key material.

scopestring

Scope of the key. Value will be PAYLOAD_SIGNING or PAYLOAD_ENCRYPTION.

Example Request
curl -X GET \
  https://api.wise-sandbox.com/v1/auth/jose/response/public-keys?algorithm=ES512&scope=PAYLOAD_SIGNING \
  -H 'Authorization: Bearer {{client-credentials-token}}'
Example Response
{
  "version": 1,
  "keyMaterial": {
    "algorithm": "ES512",
    "keyMaterial": "MIGbMBAGByqGSM49AgEGBSuBBAAjA4GGAAQBYAwVICxD0Paq7MOuO34omujHxSrQXZtiTQ/VMteqAeUfM4wE+vTSpbYCqb1pNhhcQpF+FJd2H8jB1H1zil7qLLcBw+yl4PrnLza1pmNLr+kqQVoVXVyVx/xxMK2WObLn8tHxXtW4k+bm1/ySF+0RQ265IZcw2i8YYX2FY59JkwE2Fac="
  },
  "scope": "PAYLOAD_SIGNING"
}
Note that the above endpoint requires a client credentials token, not a user level access token to authenticate. Please make sure you use your client details to fetch a valid client credentials token before performing any of these calls.
Add a new client public key

POST /v1/auth/jose/request/public-keys

This endpoint allows clients to upload their public keys for request payload signing.

Parameters
keyIdstring

Unique public key identifier in uuid format.

validFromstring

The key is valid from the date in the format yyyy-MM-dd HH:mm:ss. The time zone is always UTC.

validTillstring

The key is valid till the date in the format yyyy-MM-dd HH:mm:ss. The time zone is always UTC.

scopestring

Scope of the payload operation.
Supported scopes are: PAYLOAD_SIGNING, PAYLOAD_ENCRYPTION.

publicKeyMaterial.algorithmstring

Algorithm to be used for:

Signature verification (Scope: PAYLOAD_SIGNING): ES256, ES384, ES512, PS256, PS384, PS512
Payload encryption (Scope: PAYLOAD_ENCRYPTION): RSA_OAEP_256
publicKeyMaterial.keyMaterialstring

Public key material in DER (Distinguished Encoding Rules) format and base64 encoded.

Response

Returns a 201 - Created and response object

keyIdstring

Unique public key identifier in uuid format.

validFromstring

The key is valid from the date in the format yyyy-MM-dd HH:mm:ss. The time zone is always UTC.

validTillstring

The key is valid till the date in the format yyyy-MM-dd HH:mm:ss. The time zone is always UTC.

scopestring

Scope of the payload operation.

publicKeyMaterial.algorithmstring

Algorithm to be used for request signature verification or for response payload encryption.

publicKeyMaterial.keyMaterialstring

Public key material in DER (Distinguished Encoding Rules) format and base64 encoded.

clientIdnull

This field is currently not in use and will always return null.

deactivationTimestampnull

This field is currently not in use and will always return null.

Example Requests
curl \
  --location 'https://api.wise-sandbox.com/v1/auth/jose/request/public-keys' \
  --header 'Content-Type: application/json' \
  --header 'Authorization: Bearer {{client-credentials-token}}' \
  --data '{
    "keyId": "e87da464-8e5e-4380-8f2d-4e4e04052672",
    "scope": "PAYLOAD_SIGNING",
    "validFrom": "2023-04-27 00:00:00",
    "validTill": "2024-04-01 00:00:00",
    "publicKeyMaterial": {
      "algorithm": "ES512",
      "keyMaterial": "MIGbMBAGByqGSM49AgEGBSuBBAAjA4GGAAQBYAwVICxD0Paq7MOuO34omujHxSrQXZtiTQ/VMteqAeUfM4wE+vTSpbYCqb1pNhhcQpF+FJd2H8jB1H1zil7qLLcBw+yl4PrnLza1pmNLr+kqQVoVXVyVx/xxMK2WObLn8tHxXtW4k+bm1/ySF+0RQ265IZcw2i8YYX2FY59JkwE2Fac="
    }
  }'

curl \
  --location 'https://api.wise-sandbox.com/v1/auth/jose/request/public-keys' \
  --header 'Content-Type: application/json' \
  --header 'Authorization: Bearer {{client-credentials-token}}' \
  --data '{
    "keyId": "9d09e43f-3c16-4683-9c07-db7e992b2050",
    "scope": "PAYLOAD_ENCRYPTION",
    "validFrom": "2023-04-27 00:00:00",
    "validTill": "2024-04-01 00:00:00",
    "publicKeyMaterial": {
      "algorithm": "RSA_OAEP_256",
      "keyMaterial": "MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAwBjfPXePuZJr6jXEPibN8fpgysswqURHJGk5tod+T3SZBgVEXji0cuXF6xCdh7FMwIUROZ3ZsJFxOwyX8dNYzqEiiQy+C/wCr7/OzvRXQsy6qEQyW8fFsuEuqHRwb+ndAtz7HWZhq7P3K8XHCvJ72zeqZySXmxMYZDVwiwp+kMfRofLIivBjkN5DGWfn/7sxDLJr7/DdNgM1lMIHJtc3arffExROOnYkZ+UaUDxPo22Wnrdeb1h5S9s9m8Z46etMVEDbKJ7KEFppcRbMdckLnY3THZm/le5oxjrdVEDyhVioTC01NT0CTiqnNHfzB90ktWLsbKVww+bgDKAgx2x8EQIDAQAB"
    }
  }'
Example Response
{
  "clientId": null,
  "keyId": "e87da464-8e5e-4380-8f2d-4e4e04052672",
  "scope": "PAYLOAD_SIGNING",
  "validTill": "2023-04-27 00:00:00",
  "validFrom": "2024-04-01 00:00:00",
  "publicKeyMaterial": {
    "algorithm": "ES512",
    "keyMaterial": "MIGbMBAGByqGSM49AgEGBSuBBAAjA4GGAAQBYAwVICxD0Paq7MOuO34omujHxSrQXZtiTQ/VMteqAeUfM4wE+vTSpbYCqb1pNhhcQpF+FJd2H8jB1H1zil7qLLcBw+yl4PrnLza1pmNLr+kqQVoVXVyVx/xxMK2WObLn8tHxXtW4k+bm1/ySF+0RQ265IZcw2i8YYX2FY59JkwE2Fac="
  },
  "deactivationTimestamp": null
},
{
  "clientId": null,
  "keyId": "9d09e43f-3c16-4683-9c07-db7e992b2050",
  "scope": "PAYLOAD_ENCRYPTION",
  "validTill": "2023-04-27 00:00:00",
  "validFrom": "2024-04-01 00:00:00",
  "publicKeyMaterial": {
    "algorithm": "RSA_OAEP_256",
    "keyMaterial": "MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAwBjfPXePuZJr6jXEPibN8fpgysswqURHJGk5tod+T3SZBgVEXji0cuXF6xCdh7FMwIUROZ3ZsJFxOwyX8dNYzqEiiQy+C/wCr7/OzvRXQsy6qEQyW8fFsuEuqHRwb+ndAtz7HWZhq7P3K8XHCvJ72zeqZySXmxMYZDVwiwp+kMfRofLIivBjkN5DGWfn/7sxDLJr7/DdNgM1lMIHJtc3arffExROOnYkZ+UaUDxPo22Wnrdeb1h5S9s9m8Z46etMVEDbKJ7KEFppcRbMdckLnY3THZm/le5oxjrdVEDyhVioTC01NT0CTiqnNHfzB90ktWLsbKVww+bgDKAgx2x8EQIDAQAB"
  },
  "deactivationTimestamp": null
}
Note that the above endpoint requires a client credentials token, not a user level access token to authenticate. Please make sure you use your client details to fetch a valid client credentials token before performing any of these calls.
Playground JWS

POST /v1/auth/jose/playground/jws

This endpoint enables clients to send test signed HTTP requests and receive signed responses. Signing is mandatory for this API endpoint, any message that is not a JSON Web Signature (JWS) will be rejected.

Parameters
messagestring

Any text. For example: This is an example from docs.wise.com.

Response

Returns a 200 - OK and signed response object

messagestring

The response message will be prefixed with jose-playground-response- followed by the original message from the request. Please note that the message length is limited to 100 symbols.

methodstring

Original HTTP request method name: POST

Example Requests
curl -X POST \
  --location 'https://api.wise-sandbox.com/v1/auth/jose/playground/jws' \
  --header 'Authorization: Bearer <your api token>' \
  --header 'Content-Type: application/jose+json' \
  --header 'Accept: application/jose+json' \
  --header 'x-tw-jose-method: jws' \
  --data 'eyJhbGciOiJFUzUxMiIsInR5cCI6IkpXVCIsInVybCI6Ii92MS9hdXRoL2pvc2UvcGxheWdyb3VuZC9qd3MifQ.eyJtZXNzYWdlIjoiVGhpcyBpcyBhbiBleGFtcGxlIGZyb20gZG9jcy53aXNlLmNvbSJ9.AS9QHdkWvUEn0LxQEMPRBzlceN78J-Le-Qm1XIkkSBpsGdc0WM0MZTIGFEAJEcWeUR2M-abtd5DRdar4hLzs9apPAQ-GT70SIDV6pX9-4UKfIfzJ4g305zCoHflTfn-ijvI7XrVR_yr7xO9GJo86bfBqAX_m5uuxyU7Jy9gM1epd8HcC'
Example Response
eyJ2ZXJzaW9uIjoxLCJhbGciOiJFUzUxMiJ9.eyJtZXNzYWdlIjoiam9zZS1wbGF5Z3JvdW5kLXJlc3BvbnNlLVRoaXMgaXMgYW4gZXhhbXBsZSBmcm9tIGRvY3Mud2lzZS5jb20iLCJtZXRob2QiOiJQT1NUIn0.APYfoUySW_dPdBBVST3tJdaCBCMQZm0UR6g0vBqxjjX4WWkOdt6h045EXZKMT1m0JEMI5b7KpN14Mib9BxS7VWpLAPwYhTU5pJcQrtiK4fwaBMTT_DwipZk6ASXVevrMK_Tn9YSSkKDsv6X9qyBsnIXKy304Ex5QIHKmSGQT6wNSJ7Ye02
Example of Decoded Response
{
  "message": "jose-playground-response-This is an example from docs.wise.com",
  "method": "POST"
}
Playground JWE for POST requests

POST /v1/auth/jose/playground/jwe

This endpoint enables clients to send test encrypted HTTP requests and receive encrypted responses. Encryption is mandatory for this API endpoint, any message that is not a JSON Web Encryption (JWE) will be rejected.

Parameters
messagestring

Any text. For example: This is an example from docs.wise.com.

Response

Returns a 200 - OK and encrypted response object

messagestring

The response message will be prefixed with jose-playground-response- followed by the original message from the request. Please note that the message length is limited to 100 symbols.

methodstring

Original HTTP request method name: POST

Example Requests
curl -X POST \
  --location 'https://api.wise-sandbox.com/v1/auth/jose/playground/jwe' \
  --header 'Authorization: Bearer <your api token>' \
  --header 'Content-Type: application/jose+json' \
  --header 'Accept: application/jose+json' \
  --header 'x-tw-jose-method: jwe' \
  --data 'eyJlbmMiOiJBMjU2R0NNIiwiYWxnIjoiUlNBLU9BRVAtMjU2In0.W_WPBDXclMryaywqAB-_yC1hUYukKmZxByhE9d1G8hClc2HpewkryILGJW4uVTUeRdo1oVWd68TPIqi7bqMGUsrT-3MI4ggVSjC1rf8Lf1xTZ8-GHjSPso8tFBXnydOKzggi6fnfk98BIW76Rnxkn6sW79LH5NgN1spTOoh8-tI3i_wbHdqJOxTReaUNMYZobm7wxedZcRxhsaSrVqx2qdELeqkfwgvB1DRbHTF_PTe4W0ibMbcJivHjiDf6oAV9vXgVhYb66rhB43pgdFDv4nY1mkQC45R5T6CBdzv80EdAVOj1G4bktHyJWaJzHVsGozpxsNj3bt1AopyvDO8tsw.WLOO5WH4ZpBPi-8B.0g3eUpQPvRIaTbgUi6sH0WejsJ1nLWDGnDKTktZrkquLQlCMmIguj0I5UCyqXEo.URtTniRvfGxrKRLK63trug'
Example Response
eyJraWQiOiI2ZGE0OTQ5NS0xNjMyLTQzY2QtOWEwMy1kZDMyNDYwM2VjZDciLCJlbmMiOiJBMjU2R0NNIiwiYWxnIjoiUlNBLU9BRVAtMjU2In0.qwMqgFXRI84THQdCHhxNe5yvKOuKp2rAMH62xQ1OioPatnLfxqPHJUX0as13B3g3JKaqGgF0qMi7r3QbJ_cXNFBIg1yenCwWv4192J73-kEkCSyS-FflDCNfih1z1mm95ftAE7FhHJVG5tcSm_B0-bPU3SbjB3SDhR3QlLX5o_sQmYkQay8PKSssyBS5tCjuKdz-9QNvZtNADPujKrJGe5hE5r7ShcNwVUS2vpMXANiQTzZnQ2L3j_m07Rru-PgMxHTD2uR621B87YfTHonWDJe4XogA_pMWD_hzHVQKT7Il--2YOtv8PTnLge-uVvRTgqUiYUdvq85nPyDKsOQMnQ.zprknKK9W4iAX7YA.LJWA6oRcYj0LruM3GLPtZ0CtVNpI0Zs9WFrARDd464kKxSlu15LBiFFCFwOHbeKoRudfp8Br7MfRIlaTLHxPEE1xoBB8jZN8fakAcApU.OlMATtfF9XMtYJtTsGuBGQ02
Example of Decrypted Response
{
  "message": "jose-playground-response-This is an example from docs.wise.com",
  "method": "POST"
}
Playground JWE for direct encryption

POST /v1/auth/jose/playground/jwe-direct-encryption

This endpoint allows clients to send encrypted HTTP requests for testing purposes and receive responses encrypted with the original content encryption key. Encryption is mandatory for this API endpoint, any message that is not in JSON Web Encryption (JWE) format will be rejected.

Parameters
messagestring

Any text. For example: This is an example from docs.wise.com.

Response

Returns a 200 - OK and encrypted response object

messagestring

The response message will be prefixed with jose-playground-response- followed by the original message from the request. Please note that the message length is limited to 100 symbols.

methodstring

Original HTTP request method name: POST

Example Requests
curl -X POST \
  --location 'https://api.wise-sandbox.com/v1/auth/jose/playground/jwe-direct-encryption' \
  --header 'Authorization: Bearer <your api token>' \
  --header 'Content-Type: application/jose+json' \
  --header 'Accept: application/jose+json' \
  --header 'x-tw-jose-method: jwe' \
  --data 'eyJlbmMiOiJBMjU2R0NNIiwiYWxnIjoiUlNBLU9BRVAtMjU2In0.rVqOhX92u637hqwrw96rqA48e7NhMZVeWvUZwA4OAwOa_sBVcpXecd6qivPfZ-CuhRaD4gNKUlUJnTedBwOh5hcDZELRWThtwNiTZKaDS_ZNDjJf1r0VQPj65nT2ikfPAP-S6cYCy6JbWXivf7Jwq6lus-QydmxoLlVRvAuROFe-HzlH0-JhlwOdhPwbwl1AGx64qEir7oOn5VezJvpx3sscipm3w30mfoFc7pLlscMijMNFUwngXCLmgpno1rC_ZDzRcEycVbwvgKW75jO25UyEJif25MdE0UJUx4IT6MDviHqtBXO4OQpwhd_W6jVk-PlZ1WkZyOZqpi8HLKGo8Q.eFHqPV-mcBC82Ga2.W9o2BT7Q-vEUA2u3n4gaSdfQY_4svVj0-jwjcXmlBraZEKmtTW_A1ygvr8b9iHfS9fkxL8H_6S_oEcqzFqTKmNTzwe2V0cRY-kvzsKI.lO2gETmo2WocPZoTpU-pkQ'
Example Response
eyJlbmMiOiJBMjU2R0NNIiwiYWxnIjoiZGlyIn0..gTnxA4uYrCTWVNp1.jQ3dDidM2Z8RuYkgQt1ydJ6xYy5uG3CH72-pioHlZPqucxPWeN1pWwmceQKgpGulJmw3nYLRNkgCa9yQHMofpFIlV5HQJS6mQDxZP4G2kj3bFKvByckRYQQ3v3W7TkjfVL4NAVlrDqxx3G29-qblRwbyneMP.mVY_uctEfEAleVVbHjcqdA04
Response JOSE Headers
{
  "enc":"A256GCM",
  "alg":"dir"
}
Example of Decrypted Response
{
  "message": "jose-playground-response-This is an example from docs.wise.com",
  "method": "POST"
}
Playground JWE for GET requests

GET /v1/auth/jose/playground/jwe

This endpoint enables clients to send test HTTP GET requests and receive encrypted responses.

Response

Returns a 200 - OK and encrypted response object

messagestring

The response message is always jose-playground-response-.

methodstring

Original HTTP request method name: GET

Example Requests
curl -X GET \
  --location 'https://api.wise-sandbox.com/v1/auth/jose/playground/jwe' \
  --header 'Authorization: Bearer <your api token>' \
  --header 'Content-Type: application/jose+json' \
  --header 'Accept: application/jose+json' \
  --header 'x-tw-jose-method: jwe'
Example Response
eyJraWQiOiI2ZGE0OTQ5NS0xNjMyLTQzY2QtOWEwMy1kZDMyNDYwM2VjZDciLCJlbmMiOiJBMjU2R0NNIiwiYWxnIjoiUlNBLU9BRVAtMjU2In0.UC5mC6XSGZZuQzUALpM2QfDWyi-lcl4Wv3v8-9L04BFmqYTyiQMO-YMxiwzOdVbF-L01CaFcr_iuoM36fKp01onJAtsrvuGcnrHeh23kMwZ5k29ycGTckE0YYLe-WV9WtXguudfFC3Hri9v4FTPEE1_BbkrHACVufuhw_xWUWgYAekfm7dnmTI_3SmtExWZ_P-Xn6rkGXhsJ0Kq9LB-F3mnuvz_iglvZsW76RdLV_I4es-TfTnQgVOGO-N38SSS4wCF1qw2TWOkpNAe9kyAg6a8tWw8ZkpOxJlmGdp4jLuG_GDWlmtDrv9ntgeCLDRiTp4SLYL2MGYdmNXigZI5xrg.P_h3CuIiKxEzndER.Zf0wahZEHQSMcb4olkeT1NNzh3Alj34XbXVdREiqQH9CKDa-GnKvh1lXXYuvTO99vtBmNRM.iTzGQ3FNRIybDT_lJcnaeQ03
Example of Decrypted Response
{
  "message": "jose-playground-response-",
  "method": "GET"
}