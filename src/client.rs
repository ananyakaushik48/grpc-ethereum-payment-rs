use payments::ethereum_client::EthereumClient;
use payments::EthPaymentRequest;

pub mod payments {
    tonic::include_proto!("payments");
}

// updating main to use the async runtime with tokio
#[tokio::main]
async fn main () -> Result<(), Box<dyn std::error::Error>> {
    // Setting up the connection to the grpc server for the client
    let mut client = EthereumClient::connect(
        "http://[::1]:50051"
    ).await?;

    let request = tonic::Request::new(
        EthPaymentRequest {
            from_addr: "123456".to_owned(),
            to_addr:"765432".to_owned(),
            amount: 12,
        }
    );
    let response = client.send_payment(request).await?;

    println!("Response={:?}", response);
    Ok(())
}