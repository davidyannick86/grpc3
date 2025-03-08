mod todo_service;

use todo_service::MyService;
use todo_service::todo;

use tracing::info;

#[tokio::main]
async fn main() {
    let addr = "[::1]:50051".parse().unwrap();
    let service = MyService::new();
    let svc = todo::todo_service_server::TodoServiceServer::new(service);

    tracing_subscriber::FmtSubscriber::builder().init();

    info!("Starting server on {}", addr);

    tonic::transport::Server::builder()
        .add_service(svc)
        .serve(addr)
        .await
        .unwrap();
}
