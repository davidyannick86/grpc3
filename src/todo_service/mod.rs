use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use todo::{AddTaskResponse, CompleteTaskResponse, ListTasksResponse};
use tonic::{Response, Status};
use tracing::info;

pub mod todo {
    tonic::include_proto!("todo");
}

#[derive(Debug, Default)]
pub struct MyService {
    tasks: Arc<Mutex<HashMap<String, String>>>,
}

impl MyService {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[tonic::async_trait]
impl todo::todo_service_server::TodoService for MyService {
    async fn add_task(
        &self,
        request: tonic::Request<todo::AddTaskRequest>,
    ) -> Result<tonic::Response<todo::AddTaskResponse>, tonic::Status> {
        let req = request.into_inner();
        let id = uuid::Uuid::new_v4().to_string();
        let mut tasks = self.tasks.lock().unwrap();

        info!("Got a request: {:?}", req);

        tasks.insert(id.clone(), req.task.clone());

        info!("Added task: {:?}", req.task);

        Ok(Response::new(AddTaskResponse { id }))
    }

    async fn list_tasks(
        &self,
        request: tonic::Request<todo::ListTasksRequest>,
    ) -> Result<tonic::Response<todo::ListTasksResponse>, tonic::Status> {
        info!("Got a request: {:?}", request);

        let tasks = self.tasks.lock().unwrap();

        let task_list = tasks
            .iter()
            .map(|(id, task)| todo::Task {
                id: id.clone(),
                task: task.clone(),
            })
            .collect();

        info!("Listing tasks: {:?}", task_list);

        Ok(Response::new(ListTasksResponse { tasks: task_list }))
    }

    async fn complete_task(
        &self,
        request: tonic::Request<todo::CompleteTaskRequest>,
    ) -> Result<tonic::Response<todo::CompleteTaskResponse>, tonic::Status> {
        let req = request.into_inner();
        let mut tasks = self.tasks.lock().unwrap();

        info!("Got a request: {:?}", req);
        if !tasks.contains_key(&req.id) {
            return Err(Status::not_found("task not found"));
        }

        tasks.remove(&req.id);

        info!("Completed task with id: {}", req.id);

        Ok(Response::new(CompleteTaskResponse {}))
    }
}
