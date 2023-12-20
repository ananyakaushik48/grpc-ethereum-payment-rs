// Implementing the grpc server
use tonic::{transport::Server, Request, Response, Status};

use payments::ethereum_server::{Ethereum, EthereumServer};
use payments::{EthPaymentResponse, EthPaymentRequest};

pub mod payments {
    tonic::include_proto!("payments");
}

#[derive(Debug, Default)]
pub struct EthereumService {}

#[tonic::async_trait]
impl Ethereum for EthereumService {
    async fn send_payment(
        &self,
        request: Request<EthPaymentRequest>,
    ) -> Result<Response<EthPaymentResponse>, Status> {
        println!("Got a request: {:?}", request);

        let req = request.into_inner();

        let reply = EthPaymentResponse {
            successful: true,
            message: format!("Sent {}ETH to {}.", req.amount, req.to_addr).into(),
        };

        Ok(Response::new(reply))
    }
}

//  Setting up the tokio async runtime
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let eth_service = EthereumService::default();

    Server::builder()
        .add_service(EthereumServer::new(eth_service))
        .serve(addr)
        .await?;
    Ok(())
}
