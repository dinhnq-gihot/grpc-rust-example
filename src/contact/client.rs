pub mod contact {
    tonic::include_proto!("contact");
}

use contact::contact_service_client::ContactServiceClient;
use contact::{Contact, InsertRequest, InsertResponse, ReadRequest, UpdateRequest};
use std::error::Error;
use tonic::transport::Channel;
use tonic::{Request, Response};

use crate::contact::{DeleteRequest, SearchRequest};

async fn run_insert(client: &mut ContactServiceClient<Channel>) -> Result<(), Box<dyn Error>> {
    let contact_input = Contact {
        phone_number: "0334011231".to_owned(),
        name: "Dinh".to_owned(),
        address: "Nha Trang".to_owned(),
    };
    let request = Request::new(InsertRequest {
        contact: Some(contact_input),
    });

    let response = client.insert(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}

async fn run_read(client: &mut ContactServiceClient<Channel>) -> Result<(), Box<dyn Error>> {
    let phone_number = "0334015951".to_string();

    let request = Request::new(ReadRequest { phone_number });

    let response = client.read(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}

async fn run_update(client: &mut ContactServiceClient<Channel>) -> Result<(), Box<dyn Error>> {
    let phone_number = "0344447786".to_string();

    let request = Request::new(UpdateRequest {
        phone_number,
        update_phone_number: Some("0909819538".to_string()),
        update_name: None,
        update_address: None,
    });

    let response = client.update(request).await?;

    println!("RESPONSE={:#?}", response.get_ref().contact);

    Ok(())
}

async fn run_delete(client: &mut ContactServiceClient<Channel>) -> Result<(), Box<dyn Error>> {
    let phone_number = "0909819538".to_string();

    let request = Request::new(DeleteRequest { phone_number });

    let response = client.delete(request).await?;

    println!("RESPONSE={:#?}", response.get_ref().contact);

    Ok(())
}

async fn run_search(client: &mut ContactServiceClient<Channel>) -> Result<(), Box<dyn Error>> {
    let search_name = "Din".to_string();

    let request = Request::new(SearchRequest { search_name });

    let response = client.search(request).await?;

    println!("RESPONSE={:#?}", response.get_ref());

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut client = ContactServiceClient::connect("grpc://127.0.0.1:50053").await?;

    run_insert(&mut client).await?;
    // run_read(&mut client).await?;
    // run_update(&mut client).await?;
    // run_delete(&mut client).await?;
    // run_search(&mut client).await?;

    Ok(())
}
