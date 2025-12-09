# Mutual TLSCopy for LLMCopy page as Markdown for LLMsView as MarkdownOpen this page as MarkdownOpen in ChatGPTGet insights from ChatGPTOpen in ClaudeGet insights from ClaudeConnect to CursorInstall MCP server on CursorConnect to VS CodeInstall MCP server on VS Code

Source: https://docs.wise.com/guides/developer/auth-and-security/mtls

---

Mutual TLS
Copy

Learn how to set-up mTLS with Wise.

Mutual certificate validation is a security process between two parties, such as with Wise and a client. Once both parties have successfully validated each others certificates, they proceed to establish a secure, encrypted communication channel.

Some more resources on mutual certificate validation:

What is mutual TLS (mTLS)?
Generate and Secure Your Private Keys
Private And Public Keys
Generating a Certificate
Step 1: Generate a CSR

You can use the following command to generate a Certificate Signing Request as an example, and replace < name of your organisation>. The command output a private key called sandbox-PRIVATE-KEY.key and a Certificate sign request called sandbox-certificate-request.csr

Sample Private Key and CSR Generation Command
openssl req -nodes -newkey rsa:4096 -keyout sandbox-PRIVATE-KEY.key -out sandbox-certificate-request.csr -subj "/C=EU/O=<name of your organisation>"

CSR Requirements
The key algorithm should be RSA or ECC.
For an RSA key, the length should be at minimum 2048 bits (we support 2048, 3072, and 4096 bits).
For an ECC key, the length should be either 256 or 384 bits.
You will need to set an organisation name, as the example command above shows.
We will set a CN (Common Name) for you, which will be your Client ID. If you put anything else in, it will be overriden to be your Client ID.
The CSR should be in PEM format.
We do not allow any comments or plain text metadata in the CSR.
Do not share your private key with anyone-- including us.
Step 2: Upload the CSR and Download a Certificate

Once you have generated your CSR:

Log in to, and select the "Authentication" section in the menu on the left.
If you are testing in Sandbox, please use Developer Hub Sandbox and log in with the Sandbox account that you shared when you requested client credentials.
If you are implementing in Production, please use Developer Hub and log in with the Production account that you shared when you requested client credentials.
Navigate to Generate certificate screen
type a certificate name
copy and paste the content of your CSR
After submission of the Generate certificate form, Certificate Details screen opens displaying the client certificate. Copy the content of it into a file with pem extension, for example, for sandbox: sandbox-CERTIFICATE.pem and securely store that certificate in your key store.
In Developer Hub, in the Authentication section, you will also find a button "Get Wise Certificate". Depending on the environment, by clicking the button will download a Wise certificate for Sandbox or for Production. Securely store Wise certificate in your trust store.
Test the certificate.
Certificate Limitations
The certificate returned to you when you submit a CSR on Dev Hub is valid for 1 year.
We only allow up to 10 active certificates at a time.
All certificates must have unique names.
Setting up mTLS
Sandbox

You can get the Wise public test certificate from Developer Hub. Once you have obtained the OAuth credentials and your certificate, you can test mTLS right away by changing the host to: https://api-mtls.wise-sandbox.com.

Production

Once you have completed your integration to Sandbox please follow the same process for Production. You can use mTLS right away by changing the host to: https://api-mtls.transferwise.com.

Once integration is complete, Wise will only accept API calls via the mTLS endpoints. This ensures mTLS is enforced at all times.
Testing mTLS certificates

Open a terminal/cmd window, navigate to the newly created cert folder.

To test the integrity of the private key and generated certificate, you can try executing:

Example Command
openssl rsa -in sandbox-PRIVATE-KEY.key -noout -modulus
openssl x509 -in sandbox-CERTIFICATE.pem -noout -modulus


and compare that the outputs are the same.

To test an example request, you can try executing:

Example Command
curl --key sandbox-PRIVATE-KEY.key \
     --cert sandbox-CERTIFICATE.pem \
     --cacert wise.ca-certificate.sandbox.pem \
    https://api-mtls.wise-sandbox.com/v1/authenticated/playground \
    -H 'Authorization: Bearer <access_token>'