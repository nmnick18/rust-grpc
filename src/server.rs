use tonic::{transport::Server, Request, Response, Status};
use payment::payment_transaction_server::{PaymentTransaction, PaymentTransactionServer};
use payment::{PaymentResponse, PaymentRequest};

pub mod payment {
    tonic::include_proto!("payment");

}


#[derive(Debug, Default)]
pub struct PaymentService {}

#[tonic::async_trait]
impl PaymentTransaction for PaymentService {
    async fn send_payment(
        &self,
        request: Request<PaymentRequest>,
    ) -> Result<Response<PaymentResponse>, Status> {
        println!("Request: {:?}", request);

        let _req = request.into_inner();
        let requestdata = Box::pin(_req).as_ref().amount;
        print!("Value: {:?}", requestdata);

        let reply = PaymentResponse{ 
            result: "Successful!".to_string(),
            paid_amount:format!("Paid: {}", requestdata)
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let paytm_svc = PaymentService::default();

    Server::builder()
        .add_service(PaymentTransactionServer::new(paytm_svc))
        .serve(addr)
        .await?;
    
    Ok(())
}
