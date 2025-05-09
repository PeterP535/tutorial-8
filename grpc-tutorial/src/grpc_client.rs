use tonic::{transport::Server, Request, Response, Status};
pub mod services {
    tonic::include_proto!("services");
}
use services::{
    payment_service_server::{PaymentService, PaymentServiceServer},
    transaction_service_server::{TransactionService, TransactionServiceServer},
    chat_service_server::{ChatService, ChatServiceServer},
    PaymentRequest, PaymentResponse,
    TransactionRequest, TransactionResponse,
    ChatMessage, ChatResponse,
};


#[derive(Default)]
pub struct MyPaymentService {}


#[tonic::async_trait]
impl PaymentService for MyPaymentService {
    async fn process_payment(
        &self,
        request: Request<PaymentRequest>,
    ) -> Result<Response<PaymentResponse>, Status> {
        println!("Received payment request: {:?}", request);

        Ok(Response::new(PaymentResponse { success: true }))
    }
}


#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let payment_service = MyPaymentService::default();
    let transaction_service = MyTransactionService::default();
    let chat_service = MyChatService::default();

    Server::builder()
        .add_service(PaymentServiceServer::new(payment_service))
        .serve(addr)
        .await?;

    Ok(())


}