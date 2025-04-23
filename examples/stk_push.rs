use mpesa_daraja::mpesa::MpesaClient;

#[tokio::main]
async fn main() {

    // // ideally
    // dotenv().ok();

    // let consumer_key = env::var("CONSUMER_KEY").expect("Missing CONSUMER_KEY");
    // let consumer_secret = env::var("CONSUMER_SECRET").expect("Missing CONSUMER_SECRET");
    // let short_code = env::var("SHORT_CODE").expect("Missing SHORT_CODE");
    // let passkey = env::var("PASS_KEY").expect("Missing PASS_KEY");
    // let callback_url = env::var("RESULT_URL").expect("Missing RESULT_URL");

    // hardcoded for testing
    let consumer_key = "xxxxxxxxxx";
    let consumer_secret = "xxxxxxxxxxxxx";
    let short_code = "xxxxxxxxxxxxx";
    let passkey = "xxxxxxxxxxxxxx";
    let callback_url = "https://xxxxxxxxx.xx/xxxxxxx";

    let client = MpesaClient::new(consumer_key, consumer_secret, "production"); // or sandbox
    let response = client
        .stk_push(
            "2547XXXXXXXX", // phone_number,
            1, // Amount (1 KES for testing)
            "TestRef",
            "Test STK Push in Rust",
            callback_url,
            short_code,
            passkey,
        )
        .await;

    match response {
        Ok(resp) => {
            println!("STK Push Response: {}", resp.response_description);
            if let Some(checkout_id) = resp.checkout_request_id {
                println!("Checkout Request ID: {}", checkout_id);
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}