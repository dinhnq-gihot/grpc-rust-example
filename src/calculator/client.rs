pub mod calculator_mod {
    tonic::include_proto!("calculator");
}

use calculator_mod::calculator_client::CalculatorClient;
use calculator_mod::{AverageRequest, MaxRequest, PndRequest, SquareRequest, SumRequest};
use std::error::Error;
use tonic::transport::Channel;
use tonic::Request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = CalculatorClient::connect("grpc://127.0.0.1:50052").await?;

    // run_sum(&mut client).await?;
    // run_average(&mut client).await?;
    // run_find_max(&mut client).await?;
    // run_square(&mut client, 0).await?;
    run_sum_timeout(2, 3).await?;

    Ok(())
}

// UNARY API
async fn run_sum(client: &mut CalculatorClient<Channel>) -> Result<(), Box<dyn Error>> {
    let request = tonic::Request::new(SumRequest { num1: 2, num2: 3 });

    let response = client.sum(request).await?;

    println!("RESPONSE={:?}", response.get_ref());

    Ok(())
}

// SERVER STREAMING API
async fn run_pnd(client: &mut CalculatorClient<Channel>) -> Result<(), Box<dyn Error>> {
    let request = tonic::Request::new(PndRequest { number: 120 });

    let mut response_stream = client
        .prime_number_decompisition(request)
        .await?
        .into_inner();

    while let Some(response) = response_stream.message().await? {
        println!("RESPONSE={:?}", response);
    }

    Ok(())
}

// CLIENT STREAMING API
async fn run_average(client: &mut CalculatorClient<Channel>) -> Result<(), Box<dyn Error>> {
    let arr = vec![3.0, 4.0];
    let mut request_array = vec![];

    for number in arr {
        request_array.push(AverageRequest { number });
    }

    let request = Request::new(tokio_stream::iter(request_array));

    match client.average(request).await {
        Ok(response) => println!("Average Result = {:?}", response.get_ref()),
        Err(e) => println!("something went wrong: {:?}", e),
    }

    Ok(())
}

// BIDIRECTIONAL STREAMING API
async fn run_find_max(client: &mut CalculatorClient<Channel>) -> Result<(), Box<dyn Error>> {
    let arr = vec![8, 5, 4, -8, 9, 2, -3, 10];
    let mut request_array = vec![];

    for number in arr {
        request_array.push(MaxRequest { number });
    }

    let request = Request::new(tokio_stream::iter(request_array));

    let mut response_stream = client.find_max(request).await.unwrap().into_inner();

    while let Some(response) = response_stream.message().await.unwrap() {
        println!("RESPONSE={:?}", response);

        // tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }

    Ok(())
}

// ERROR HANDLING
async fn run_square(
    client: &mut CalculatorClient<Channel>,
    number: i32,
) -> Result<(), Box<dyn Error>> {
    let request = Request::new(SquareRequest { number });

    let response = client.square(request).await?;

    println!("RESPONSE={:?}", response.get_ref().result);

    Ok(())
}

// TIMEOUT
async fn run_sum_timeout(
    // client: &mut CalculatorClient<Channel>,
    num1: i32,
    num2: i32,
) -> Result<(), Box<dyn Error>> {
    let channel = Channel::from_static("grpc://127.0.0.1:50052")
        .connect()
        .await?;
    let timeout_channel =
        tower::timeout::Timeout::new(channel, std::time::Duration::from_millis(1000));

    let mut client = CalculatorClient::new(timeout_channel);

    let request = tonic::Request::new(SumRequest { num1, num2 });

    match client.sum_timeout(request).await {
        Ok(response) => println!("RESPONSE={:?}", response.get_ref()),
        Err(status) => println!("error={:?}", status),
    }

    Ok(())
}
