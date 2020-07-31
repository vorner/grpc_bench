use tonic::{transport::Server, Request, Response, Status};
use tokio::runtime::Builder;

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let reply = hello_world::HelloReply {
            message: request.into_inner().name,
        };
        Ok(Response::new(reply))
    }
}

fn guess_threads_env() -> Option<usize> {
    let env = std::env::var("THREADS").ok()?;
    let threads = env.parse().ok()?;
    Some(threads)
}

async fn server() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse().unwrap();
    let greeter = MyGreeter::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let threads = guess_threads_env().unwrap_or_else(num_cpus::get);

    let mut runtime = Builder::new()
        .enable_all()
        .threaded_scheduler()
        .core_threads(threads)
        .build()?;

    runtime.block_on(server())?;

    Ok(())
}
