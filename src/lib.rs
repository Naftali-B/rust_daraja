use reqwest::Client;
use serde::{Deserialize, Serialize};
use base64::{Engine as _, engine::general_purpose};
use chrono::Utc;
use std::error::Error;
use std::fs;
use openssl::x509::X509;
use openssl::rsa::Padding;

/// MPESA Daraja API client library.
///
/// This module provides a Rust interface to Safaricom's MPESA Daraja API,
/// supporting STK Push (C2B), B2C payments, balance queries, and transaction status checks.
pub mod mpesa {
    use super::*;

    /// A client for interacting with Safaricom's MPESA Daraja API.
    pub struct MpesaClient {
        consumer_key: String,
        consumer_secret: String,
        environment: String,
    }

    #[derive(Deserialize)]
    struct AccessTokenResponse {
        access_token: String,
        #[allow(dead_code)]
        expires_in: String,
    }

    // STK Push (C2B)
    #[derive(Serialize)]
    struct StkPushRequest {
        #[serde(rename = "BusinessShortCode")]
        business_short_code: String,
        #[serde(rename = "Password")]
        password: String,
        #[serde(rename = "Timestamp")]
        timestamp: String,
        #[serde(rename = "TransactionType")]
        transaction_type: String,
        #[serde(rename = "Amount")]
        amount: String,
        #[serde(rename = "PartyA")]
        party_a: String,
        #[serde(rename = "PartyB")]
        party_b: String,
        #[serde(rename = "PhoneNumber")]
        phone_number: String,
        #[serde(rename = "CallBackURL")]
        callback_url: String,
        #[serde(rename = "AccountReference")]
        account_reference: String,
        #[serde(rename = "TransactionDesc")]
        transaction_desc: String,
    }

    /// Response from an STK Push (C2B) request.
    #[derive(Deserialize)]
    pub struct StkPushResponse {
        #[serde(rename = "MerchantRequestID")]
        pub merchant_request_id: Option<String>,
        #[serde(rename = "CheckoutRequestID")]
        pub checkout_request_id: Option<String>,
        #[serde(rename = "ResponseCode")]
        pub response_code: String,
        #[serde(rename = "ResponseDescription")]
        pub response_description: String,
        #[serde(rename = "CustomerMessage")]
        pub customer_message: Option<String>,
    }

    /// Error response from any MPESA API call.
    #[derive(Deserialize)]
    pub struct ErrorResponse {
        #[serde(rename = "requestId")]
        pub request_id: String,
        #[serde(rename = "errorCode")]
        pub error_code: String,
        #[serde(rename = "errorMessage")]
        pub error_message: String,
    }

    // B2C
    #[derive(Serialize)]
    struct B2cRequest {
        #[serde(rename = "InitiatorName")]
        initiator_name: String,
        #[serde(rename = "SecurityCredential")]
        security_credential: String,
        #[serde(rename = "CommandID")]
        command_id: String,
        #[serde(rename = "Amount")]
        amount: String,
        #[serde(rename = "PartyA")]
        party_a: String,
        #[serde(rename = "PartyB")]
        party_b: String,
        #[serde(rename = "Remarks")]
        remarks: String,
        #[serde(rename = "QueueTimeOutURL")]
        queue_timeout_url: String,
        #[serde(rename = "ResultURL")]
        result_url: String,
        #[serde(rename = "Occasion")]
        occasion: String,
    }

    /// Response from a B2C payment request.
    #[derive(Deserialize)]
    pub struct B2cResponse {
        #[serde(rename = "ConversationID")]
        pub conversation_id: Option<String>,
        #[serde(rename = "OriginatorConversationID")]
        pub originator_conversation_id: Option<String>,
        #[serde(rename = "ResponseCode")]
        pub response_code: String,
        #[serde(rename = "ResponseDescription")]
        pub response_description: String,
    }

    // Balance Check
    #[derive(Serialize, Deserialize)]
    struct BalanceRequest {
        #[serde(rename = "Initiator")]
        initiator: String,
        #[serde(rename = "SecurityCredential")]
        security_credential: String,
        #[serde(rename = "CommandID")]
        command_id: String,
        #[serde(rename = "PartyA")]
        party_a: String,
        #[serde(rename = "IdentifierType")]
        identifier_type: String,
        #[serde(rename = "Remarks")]
        remarks: String,
        #[serde(rename = "QueueTimeOutURL")]
        queue_timeout_url: String,
        #[serde(rename = "ResultURL")]
        result_url: String,
    }

    /// Immediate response from a balance query.
    #[derive(Serialize, Deserialize)]
    pub struct BalanceQueryResponse {
        #[serde(rename = "ConversationID")]
        pub conversation_id: Option<String>,
        #[serde(rename = "OriginatorConversationID")]
        pub originator_conversation_id: Option<String>,
        #[serde(rename = "ResponseCode")]
        pub response_code: String,
        #[serde(rename = "ResponseDescription")]
        pub response_description: String,
    }

    /// Callback response for balance and transaction status queries.
    // Shared Callback Structs (for Balance and Transaction Status)
    #[derive(Serialize, Deserialize)]
    pub struct BalanceResponse { // shared
        #[serde(rename = "ResultType")]
        pub result_type: i32,
        #[serde(rename = "ResultCode")]
        pub result_code: String,
        #[serde(rename = "ResultDesc")]
        pub result_desc: String,
        #[serde(rename = "OriginatorConversationID")]
        pub originator_conversation_id: String,
        #[serde(rename = "ConversationID")]
        pub conversation_id: String,
        #[serde(rename = "TransactionID")]
        pub transaction_id: String,
        #[serde(rename = "ResultParameters")]
        pub result_parameters: ResultParameters,
    }

    /// Parameters in a callback response.
    #[derive(Serialize, Deserialize)]
    pub struct ResultParameters { // shared
        #[serde(rename = "ResultParameter")]
        pub result_parameter: Vec<ResultParameter>,
    }

    /// Key-value pair in a callback response.
    #[derive(Serialize, Deserialize)]
    pub struct ResultParameter { // shared
        #[serde(rename = "Key")]
        pub key: String,
        #[serde(rename = "Value")]
        pub value: String,
    }

    // Transaction Status
    #[derive(Serialize)]
    struct TransactionStatusRequest {
        #[serde(rename = "Initiator")]
        initiator: String,
        #[serde(rename = "SecurityCredential")]
        security_credential: String,
        #[serde(rename = "CommandID")]
        command_id: String,
        #[serde(rename = "TransactionID")]
        transaction_id: String,
        #[serde(rename = "PartyA")]
        party_a: String,
        #[serde(rename = "IdentifierType")]
        identifier_type: String,
        #[serde(rename = "ResultURL")]
        result_url: String,
        #[serde(rename = "QueueTimeOutURL")]
        queue_timeout_url: String,
        #[serde(rename = "Remarks")]
        remarks: String,
        #[serde(rename = "Occasion")]
        occasion: String,
    }

    /// Immediate response from a transaction status query.
    #[derive(Deserialize)]
    pub struct TransactionStatusResponse {
        #[serde(rename = "ConversationID")]
        pub conversation_id: Option<String>,
        #[serde(rename = "OriginatorConversationID")]
        pub originator_conversation_id: Option<String>,
        #[serde(rename = "ResponseCode")]
        pub response_code: String,
        #[serde(rename = "ResponseDescription")]
        pub response_description: String,
    }

    impl MpesaClient {
        /// Creates a new MPESA client.
        ///
        /// # Arguments
        /// * `consumer_key` - The API consumer key from Safaricom.
        /// * `consumer_secret` - The API consumer secret from Safaricom.
        /// * `environment` - `"sandbox"` or `"production"`.
        ///
        /// # Examples
        /// ```
        /// use mpesa_daraja::mpesa::MpesaClient;
        /// let client = MpesaClient::new("consumer_key", "consumer_secret", "sandbox");
        /// ```
        pub fn new(consumer_key: &str, consumer_secret: &str, environment: &str) -> Self {
            MpesaClient {
                consumer_key: consumer_key.to_string(),
                consumer_secret: consumer_secret.to_string(),
                environment: environment.to_string(),
            }
        }

        async fn get_access_token(&self) -> Result<String, Box<dyn Error>> {
            let client = Client::new();
            let auth = format!("{}:{}", self.consumer_key, self.consumer_secret);
            let auth_encoded = general_purpose::STANDARD.encode(auth);

            let url = if self.environment == "sandbox" {
                "https://sandbox.safaricom.co.ke/oauth/v1/generate?grant_type=client_credentials"
            } else {
                "https://api.safaricom.co.ke/oauth/v1/generate?grant_type=client_credentials"
            };

            let response = client
                .get(url)
                .header("Authorization", format!("Basic {}", auth_encoded))
                .send()
                .await?;

            let token_data: AccessTokenResponse = response.json().await?;
            Ok(token_data.access_token)
        }

        /// Initiates an STK Push (C2B) transaction, prompting the user to enter their PIN.
        ///
        /// # Arguments
        /// * `phone_number` - The customer's phone number (e.g., "2547XXXXXXXX").
        /// * `amount` - The amount to charge in KES.
        /// * `account_reference` - A reference for the transaction (e.g., invoice number).
        /// * `transaction_desc` - A description of the transaction.
        /// * `callback_url` - URL to receive the transaction result.
        /// * `short_code` - The business shortcode.
        /// * `passkey` - The passkey from Safaricom.
        pub async fn stk_push(
            &self,
            phone_number: &str,
            amount: u32,
            account_reference: &str,
            transaction_desc: &str,
            callback_url: &str,
            short_code: &str,
            passkey: &str,
        ) -> Result<StkPushResponse, Box<dyn Error>> {
            let client = Client::new();
            let access_token = self.get_access_token().await?;

            let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
            let password = general_purpose::STANDARD.encode(format!("{}{}{}", short_code, passkey, timestamp));

            let url = if self.environment == "sandbox" {
                "https://sandbox.safaricom.co.ke/mpesa/stkpush/v1/processrequest"
            } else {
                "https://api.safaricom.co.ke/mpesa/stkpush/v1/processrequest"
            };

            let request_body = StkPushRequest {
                business_short_code: short_code.to_string(),
                password,
                timestamp,
                transaction_type: "CustomerPayBillOnline".to_string(),
                amount: amount.to_string(),
                party_a: phone_number.to_string(),
                party_b: short_code.to_string(),
                phone_number: phone_number.to_string(),
                callback_url: callback_url.to_string(),
                account_reference: account_reference.to_string(),
                transaction_desc: transaction_desc.to_string(),
            };

            let response = client
                .post(url)
                .header("Authorization", format!("Bearer {}", access_token))
                .json(&request_body)
                .send()
                .await?;

            let text = response.text().await?;
            if text.contains("errorCode") {
                let error: ErrorResponse = serde_json::from_str(&text)?;
                return Err(format!(
                    "API Error: {} - {}",
                    error.error_code, error.error_message
                ).into());
            }

            let stk_response: StkPushResponse = serde_json::from_str(&text)?;
            Ok(stk_response)
        }

        /// Generates a security credential for B2C, balance, and transaction status APIs.
        ///
        /// # Arguments
        /// * `initiator_password` - The initiator's password.
        /// * `is_production` - Whether to use the production certificate.
        /// * `cert_path` - Optional path to the certificate file; defaults to "certs/production.cer" or "certs/sandbox.cer".
        pub fn generate_security_credential(
            initiator_password: &str,
            is_production: bool,
            cert_path: Option<&str>,
        ) -> Result<String, Box<dyn Error>> {
            let default_path = if is_production {
                "certs/production.cer"
            } else {
                "certs/sandbox.cer"
            };
            let path = cert_path.unwrap_or(default_path);

            let cert_content = fs::read(path)?;
            let cert = X509::from_pem(&cert_content)?;
            let public_key = cert.public_key()?;
            let rsa = public_key.rsa()?;

            let mut encrypted = vec![0; rsa.size() as usize];
            let len = rsa.public_encrypt(
                initiator_password.as_bytes(),
                &mut encrypted,
                Padding::PKCS1,
            )?;

            Ok(general_purpose::STANDARD.encode(&encrypted[..len]))
        }

        /// Initiates a B2C payment to a customer's phone number.
        ///
        /// # Arguments
        /// * `phone_number` - Recipient's phone number (e.g., "2547XXXXXXXX").
        /// * `amount` - Amount to send in KES.
        /// * `remarks` - Transaction remarks.
        /// * `result_url` - URL to receive the result callback.
        /// * `queue_timeout_url` - URL for timeout notifications.
        /// * `initiator_name` - The initiator username.
        /// * `security_credential` - Generated security credential.
        /// * `short_code` - Business shortcode.
        /// * `occasion` - Optional occasion description.
        pub async fn business_payment(
            &self,
            phone_number: &str,
            amount: u32,
            remarks: &str,
            result_url: &str,
            queue_timeout_url: &str,
            initiator_name: &str,
            security_credential: &str,
            short_code: &str,
            occasion: &str,
        ) -> Result<B2cResponse, Box<dyn Error>> {
            let client = Client::new();
            let access_token = self.get_access_token().await?;
            let url = if self.environment == "sandbox" {
                "https://sandbox.safaricom.co.ke/mpesa/b2c/v1/paymentrequest"
            } else {
                "https://api.safaricom.co.ke/mpesa/b2c/v1/paymentrequest"
            };

            let request_body = B2cRequest {
                initiator_name: initiator_name.to_string(),
                security_credential: security_credential.to_string(),
                command_id: "BusinessPayment".to_string(),
                amount: amount.to_string(),
                party_a: short_code.to_string(),
                party_b: phone_number.to_string(),
                remarks: remarks.to_string(),
                queue_timeout_url: queue_timeout_url.to_string(),
                result_url: result_url.to_string(),
                occasion: occasion.to_string(),
            };

            let response = client
                .post(url)
                .header("Authorization", format!("Bearer {}", access_token))
                .json(&request_body)
                .send()
                .await?;

            let text = response.text().await?;
            if text.contains("errorCode") {
                let error: ErrorResponse = serde_json::from_str(&text)?;
                return Err(format!("API Error: {} - {}", error.error_code, error.error_message).into());
            }

            let b2c_response: B2cResponse = serde_json::from_str(&text)?;
            Ok(b2c_response)
        }

        /// Queries the account balance for a shortcode.
        ///
        /// # Arguments
        /// * `initiator_name` - The initiator username.
        /// * `security_credential` - Generated security credential.
        /// * `short_code` - Business shortcode.
        /// * `remarks` - Request remarks.
        /// * `queue_timeout_url` - URL for timeout notifications.
        /// * `result_url` - URL to receive the result callback.
        pub async fn check_balance(
            &self,
            initiator_name: &str,
            security_credential: &str,
            short_code: &str,
            remarks: &str,
            queue_timeout_url: &str,
            result_url: &str,
        ) -> Result<BalanceQueryResponse, Box<dyn Error>> {
            let client = Client::new();
            let access_token = self.get_access_token().await?;
            let url = if self.environment == "sandbox" {
                "https://sandbox.safaricom.co.ke/mpesa/accountbalance/v1/query"
            } else {
                "https://api.safaricom.co.ke/mpesa/accountbalance/v1/query"
            };

            let request_body = BalanceRequest {
                initiator: initiator_name.to_string(),
                security_credential: security_credential.to_string(),
                command_id: "AccountBalance".to_string(),
                party_a: short_code.to_string(),
                identifier_type: "4".to_string(),
                remarks: remarks.to_string(),
                queue_timeout_url: queue_timeout_url.to_string(),
                result_url: result_url.to_string(),
            };

            let response = client
                .post(url)
                .header("Authorization", format!("Bearer {}", access_token))
                .json(&request_body)
                .send()
                .await?;

            let text = response.text().await?;
            let balance_response: BalanceQueryResponse = serde_json::from_str(&text)?;
            Ok(balance_response)
        }

        /// Checks the status of a previous transaction.
        ///
        /// # Arguments
        /// * `initiator_name` - The initiator username.
        /// * `security_credential` - Generated security credential.
        /// * `transaction_id` - The transaction ID to query.
        /// * `short_code` - Business shortcode.
        /// * `remarks` - Request remarks.
        /// * `result_url` - URL to receive the result callback.
        /// * `queue_timeout_url` - URL for timeout notifications.
        /// * `occasion` - Optional occasion description.
        pub async fn check_transaction_status(
            &self,
            initiator_name: &str,
            security_credential: &str,
            transaction_id: &str,
            short_code: &str,
            remarks: &str,
            result_url: &str,
            queue_timeout_url: &str,
            occasion: &str,
        ) -> Result<TransactionStatusResponse, Box<dyn Error>> {
            let client = Client::new();
            let access_token = self.get_access_token().await?;
            let url = if self.environment == "sandbox" {
                "https://sandbox.safaricom.co.ke/mpesa/transactionstatus/v1/query"
            } else {
                "https://api.safaricom.co.ke/mpesa/transactionstatus/v1/query"
            };

            let request_body = TransactionStatusRequest {
                initiator: initiator_name.to_string(),
                security_credential: security_credential.to_string(),
                command_id: "TransactionStatusQuery".to_string(),
                transaction_id: transaction_id.to_string(),
                party_a: short_code.to_string(),
                identifier_type: "4".to_string(),
                result_url: result_url.to_string(),
                queue_timeout_url: queue_timeout_url.to_string(),
                remarks: remarks.to_string(),
                occasion: occasion.to_string(),
            };

            let response = client
                .post(url)
                .header("Authorization", format!("Bearer {}", access_token))
                .json(&request_body)
                .send()
                .await?;

            let text = response.text().await?;
            let status_response: TransactionStatusResponse = serde_json::from_str(&text)?;
            Ok(status_response)
        }
    }
}