use mpesa_daraja::mpesa::MpesaClient;

#[tokio::main]
async fn main() {

    // // ideally
    // dotenv().ok();

    // let consumer_key = env::var("CONSUMER_KEY").expect("Missing CONSUMER_KEY");
    // let consumer_secret = env::var("CONSUMER_SECRET").expect("Missing CONSUMER_SECRET");
    // let initiator_password = env::var("INITIATOR_PASSWORD").expect("Missing INITIATOR_PASSWORD");
    // let cert_path = env::var("CERT_PATH").unwrap_or("src/certs/production.cer".to_string());
    // let short_code = env::var("SHORT_CODE").expect("Missing SHORT_CODE");
    // let initiator_name = env::var("INITIATOR_NAME").expect("Missing INITIATOR_NAME");
    // let result_url = env::var("RESULT_URL").expect("Missing RESULT_URL");
    // let queue_timeout_url = env::var("QUEUE_TIMEOUT_URL").expect("Missing QUEUE_TIMEOUT_URL");

    // hardcoded for testing
    let consumer_key = "xxxxxx";
    let consumer_secret = "xxxxxxxx";
    let initiator_password = "xxxxxxxxx";
    let cert_path = "xxxxxxxx";
    let short_code = "xxxxxxx";
    let initiator_name = "xxxxxxx";
    let result_url = "https://xxxxxx.xx/xxx";
    let queue_timeout_url = "https://xxxxx.xx/xxx";

    let client = MpesaClient::new(consumer_key, consumer_secret, "production");

    let security_credential = MpesaClient::generate_security_credential(initiator_password, true, Some(cert_path))
        .expect("Failed to generate security credential");

    // Test Balance
    match client.check_balance(
        initiator_name,
        &security_credential,
        short_code,
        "Balance Inquiry",
        queue_timeout_url,
        result_url,
    ).await {
        Ok(response) => {
            println!("Balance Query Response: {}", response.response_description);
            println!("Conversation ID: {:?}", response.conversation_id);
        }
        Err(e) => println!("Error checking balance: {}", e),
    }
}