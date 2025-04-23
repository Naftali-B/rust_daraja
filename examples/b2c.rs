
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
    let consumer_key = "xxxxxxxxxxxxxxxxxxxx";
    let consumer_secret = "xxxxxxxxxxxxxxxxxxxx";
    let initiator_password = "xxxxxxx";
    let cert_path = "src/certs/production.cer";
    let short_code = "xxxxxxx";
    let initiator_name = "xxxxxx";
    let result_url = "https://xxxxxx";
    let queue_timeout_url = "https://xxxxxx";

    let client = MpesaClient::new(consumer_key, consumer_secret, "production"); // or "sandbox"

    match MpesaClient::generate_security_credential(initiator_password, true, Some(cert_path)) { // true for production // None in place of Some(cert_path) defaults to default path in the generate_security_credential function
        Ok(security_credential) => {

            match client
                .business_payment(
                    "2547XXXXXXXX", // phone_number,
                    1, // Amount
                    "Test B2C Payment",
                    result_url,
                    queue_timeout_url,
                    initiator_name,
                    &security_credential,
                    short_code,
                    "Test Occasion",
                )
                .await
            {
                Ok(response) => {
                    println!("B2C Response: {}", response.response_description);
                    if let Some(conv_id) = response.conversation_id {
                        println!("Conversation ID: {}", conv_id);
                    } else {
                        println!("No Conversation ID returned");
                    }
                }
                Err(e) => println!("Error making payment: {}", e),
            }
        }
        Err(e) => println!("Error generating security credential: {}", e),
    }
}