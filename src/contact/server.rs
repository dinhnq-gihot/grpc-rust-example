use grpc_demo::prisma::contact::{Data, Delete};
use grpc_demo::prisma::{contact, PrismaClient};
use std::error::Error;
use tonic::transport::Server;

pub mod contact_mod {
    tonic::include_proto!("contact");
}

use contact_mod::contact_service_server::{ContactService, ContactServiceServer};
use contact_mod::{
    Contact, DeleteRequest, DeleteResponse, InsertRequest, InsertResponse, ReadRequest,
    ReadResponse, SearchRequest, SearchResponse, UpdateRequest, UpdateResponse,
};
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub struct MyContact {
    prisma_client: PrismaClient,
}

impl MyContact {
    fn new(prisma_client: PrismaClient) -> Self {
        Self { prisma_client }
    }
}

fn convert_data_2_contact(data: &Data) -> Option<Contact> {
    return Some(Contact {
        phone_number: data.phone_number.clone(),
        name: data.name.clone(),
        address: data.address.clone(),
    });
}

#[tonic::async_trait]
impl ContactService for MyContact {
    async fn insert(
        &self,
        request: Request<InsertRequest>,
    ) -> Result<Response<InsertResponse>, Status> {
        println!("Got a request {:?}", request);

        let contract_info: Contact = request.into_inner().contact.unwrap();

        if contract_info.phone_number.is_empty() {
            return Err(Status::invalid_argument("Phone number is required"));
        }

        if contract_info.name.is_empty() {
            return Err(Status::invalid_argument("Name is required"));
        }

        let contact_result = self
            .prisma_client
            .contact()
            .create(
                contract_info.phone_number,
                contract_info.name,
                contract_info.address,
                vec![],
            )
            .exec()
            .await;

        match contact_result {
            Ok(contact_data) => {
                return Ok(Response::new(InsertResponse {
                    id: contact_data.id.clone(),
                    contact: convert_data_2_contact(&contact_data),
                }))
            }

            Err(error) => return Err(Status::already_exists(error.to_string())),
        }
    }

    async fn read(&self, request: Request<ReadRequest>) -> Result<Response<ReadResponse>, Status> {
        println!("Got a READ request {:?}", request);

        let phone_number = request.into_inner().phone_number;
        if phone_number.is_empty() {
            return Err(Status::invalid_argument("phone number is required"));
        }

        let result = self
            .prisma_client
            .contact()
            .find_unique(contact::phone_number::equals(phone_number))
            .exec()
            .await;

        let contact_result;
        match result {
            Ok(option_contact) => contact_result = option_contact,
            Err(error) => return Err(Status::resource_exhausted(error.to_string())),
        }

        match contact_result {
            Some(data) => {
                return Ok(Response::new(ReadResponse {
                    contact: convert_data_2_contact(&data),
                }))
            }
            None => return Ok(Response::new(ReadResponse { contact: None })),
        }
    }

    async fn update(
        &self,
        request: Request<UpdateRequest>,
    ) -> Result<Response<UpdateResponse>, Status> {
        println!("Got a PATCH request {:?}", request);

        if request.get_ref().phone_number.is_empty() {
            return Err(Status::invalid_argument("phone number is required"));
        }

        let result = self
            .prisma_client
            .contact()
            .find_unique(contact::phone_number::equals(
                request.get_ref().phone_number.clone(),
            ))
            .exec()
            .await;

        let contact_option;
        match result {
            Ok(option) => contact_option = option,
            Err(error) => return Err(Status::resource_exhausted(error.to_string())),
        }

        let existing_contact;
        match contact_option {
            Some(data) => existing_contact = data,
            None => return Err(Status::not_found("phone number not found")),
        }

        let mut update_data = vec![];
        if let Some(update_phone_number) = &request.get_ref().update_phone_number {
            update_data.push(contact::phone_number::set(update_phone_number.clone()));
        }
        if let Some(update_name) = &request.get_ref().update_name {
            update_data.push(contact::name::set(update_name.clone()));
        }
        if let Some(update_address) = &request.get_ref().update_address {
            update_data.push(contact::address::set(update_address.clone()));
        }

        let updated_contact = self
            .prisma_client
            .contact()
            .update(contact::id::equals(existing_contact.id), update_data)
            .exec()
            .await;

        match updated_contact {
            Ok(data) => {
                return Ok(Response::new(UpdateResponse {
                    contact: convert_data_2_contact(&data),
                }))
            }
            Err(error) => return Err(Status::resource_exhausted(error.to_string())),
        }
    }

    async fn delete(
        &self,
        request: Request<DeleteRequest>,
    ) -> Result<Response<DeleteResponse>, Status> {
        println!("Got a DELETE request {:?}", request);

        // if request.get_ref().phone_number.is_empty() {
        //     return Err(Status::invalid_argument("phone number is required"));
        // }

        // let result = self
        //     .prisma_client
        //     .contact()
        //     .find_unique(contact::phone_number::equals(
        //         request.get_ref().phone_number.clone(),
        //     ))
        //     .exec()
        //     .await;

        // let contact_option;
        // match result {
        //     Ok(option) => contact_option = option,
        //     Err(error) => return Err(Status::resource_exhausted(error.to_string())),
        // }

        // let existing_contact;
        // match contact_option {
        //     Some(data) => existing_contact = data,
        //     None => return Err(Status::not_found("phone number not found")),
        // }

        let deleted_contact_result = self
            .prisma_client
            .contact()
            .delete(contact::phone_number::equals(
                request.get_ref().phone_number.clone(),
            ))
            .exec()
            .await;

        match deleted_contact_result {
            Ok(data) => {
                return Ok(Response::new(DeleteResponse {
                    contact: convert_data_2_contact(&data),
                }));
            }
            Err(e) => return Err(Status::resource_exhausted(e.to_string())),
        }
    }

    async fn search(
        &self,
        request: Request<SearchRequest>,
    ) -> Result<Response<SearchResponse>, Status> {
        println!("Got a SEARCH request {:?}", request);

        let search_result = self
            .prisma_client
            .contact()
            .find_many(vec![contact::name::contains(
                request.get_ref().search_name.clone(),
            )])
            .exec()
            .await;

        match search_result {
            Ok(vec) => {
                return Ok(Response::new(SearchResponse {
                    results: {
                        let mut results = vec![];
                        for data in vec {
                            results.push(convert_data_2_contact(&data).unwrap());
                        }

                        results
                    },
                }));
            }
            Err(e) => return Err(Status::resource_exhausted(e.to_string())),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:50053".parse()?;
    let prisma_client: PrismaClient = PrismaClient::_builder().build().await?;

    let contact_service = MyContact::new(prisma_client);

    Server::builder()
        .add_service(ContactServiceServer::new(contact_service))
        .serve(addr)
        .await?;

    Ok(())
}
