use payment::payment_transaction_client::PaymentTransactionClient;
use payment::PaymentRequest;

pub mod payment{
    tonic::include_proto!("payment");
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = PaymentTransactionClient::connect("https://[::1]:50051").await?;
    let req = tonic::Request::new(
        PaymentRequest{
            person_from: "bob".to_string() ,
            person_to: "jacoma".to_string(),
            amount: 20
        }
    );
    let response = client.send_payment(req).await?;

    print!("response: {:?}", response);
    
    Ok(())
}