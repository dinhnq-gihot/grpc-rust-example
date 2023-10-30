use std::net::SocketAddr;

use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::{transport::Server, Request, Response, Status};

pub mod calculator {
    tonic::include_proto!("calculator");
}

use calculator::calculator_server::{Calculator, CalculatorServer};
use calculator::{
    AverageRequest, AverageResponse, MaxRequest, MaxResponse, PndRequest, PndResponse,
    SquareRequest, SquareResponse, SumReponse, SumRequest,
};

#[derive(Debug, Default)]
pub struct MyCalc {}

#[tonic::async_trait]
impl Calculator for MyCalc {
    async fn sum(&self, request: Request<SumRequest>) -> Result<Response<SumReponse>, Status> {
        println!("Got a request {:?}", request);

        let num1 = request.get_ref().num1;
        let num2 = request.get_ref().num2;

        let sum = num1 + num2;

        let response = calculator::SumReponse { result: sum };

        Ok(Response::new(response))
    }

    type PrimeNumberDecompisitionStream = ReceiverStream<Result<PndResponse, Status>>;

    async fn prime_number_decompisition(
        &self,
        request: Request<PndRequest>,
    ) -> Result<Response<Self::PrimeNumberDecompisitionStream>, Status> {
        let (sender, receiver) = mpsc::channel(32);

        println!("Got a request {:?}", request);

        let mut k = 2;
        let mut n = request.get_ref().number;

        tokio::spawn(async move {
            while n > 1 {
                if n % k == 0 {
                    let response = PndResponse { number: k };

                    sender.send(Ok(response)).await.unwrap();
                    n /= k;
                } else {
                    k += 1;
                }
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
        });

        Ok(Response::new(ReceiverStream::new(receiver)))
    }

    async fn average(
        &self,
        request: Request<tonic::Streaming<AverageRequest>>,
    ) -> Result<Response<AverageResponse>, Status> {
        let mut request_stream = request.into_inner();

        let mut sum = 0.0;
        let mut length = 0;

        while let Some(req) = request_stream.message().await? {
            println!(" ==> number = {}", req.number);
            sum += req.number;
            length += 1;
        }

        Ok(Response::new(AverageResponse {
            result: (sum / (length as f32)),
        }))
    }

    type FindMaxStream = ReceiverStream<Result<MaxResponse, Status>>;

    async fn find_max(
        &self,
        request: Request<tonic::Streaming<MaxRequest>>,
    ) -> Result<Response<Self::FindMaxStream>, Status> {
        let mut request_stream = request.into_inner();

        let mut max = 0;

        let (sender, receiver) = mpsc::channel(32);
        tokio::spawn(async move {
            while let Some(req) = request_stream.message().await.unwrap() {
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;

                let number = req.number;
                println!(" ==> Got a request number = {}", number);

                if number > max {
                    max = number;
                }
                sender.send(Ok(MaxResponse { result: max })).await.unwrap();
            }
        });

        Ok(Response::new(ReceiverStream::new(receiver)))
    }

    async fn square(
        &self,
        request: Request<SquareRequest>,
    ) -> Result<Response<SquareResponse>, Status> {
        println!("Got a request {:?}", request);

        let input = request.get_ref().number as f64;

        if input <= 0.0 {
            return Err(Status::invalid_argument(
                format!("Except number > 0, request num was {}", input).as_str(),
            ));
        }

        Ok(Response::new(SquareResponse {
            result: input.sqrt(),
        }))
    }

    async fn sum_timeout(
        &self,
        request: Request<SumRequest>,
    ) -> Result<Response<SumReponse>, Status> {
        println!("Got a request {:?}", request);

        tokio::time::sleep(std::time::Duration::from_secs(5)).await;

        let sum = request.get_ref().num1 + request.get_ref().num2;
        let response = calculator::SumReponse { result: sum };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = "127.0.0.1:50052".parse()?;
    let calc = MyCalc::default();

    Server::builder()
        .add_service(CalculatorServer::new(calc))
        .serve(addr)
        .await?;

    Ok(())
}
