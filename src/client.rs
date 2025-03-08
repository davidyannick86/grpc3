use todo_service::{AddTaskRequest, ListTasksRequest, todo_service_client::TodoServiceClient};

pub mod todo_service {
    tonic::include_proto!("todo");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = TodoServiceClient::connect("http://[::1]:50051").await?;

    let request = AddTaskRequest {
        task: "Learn Rust".to_string(),
    };

    let request2 = AddTaskRequest {
        task: "Learn Go".to_string(),
    };

    TodoServiceClient::add_task(&mut client, request).await?;
    TodoServiceClient::add_task(&mut client, request2).await?;

    let request = ListTasksRequest {};

    let response = client.list_tasks(request).await?;

    response.into_inner().tasks.iter().for_each(|task| {
        println!("Task ID: {}, Task: {}", task.id, task.task);
    });

    Ok(())
}
