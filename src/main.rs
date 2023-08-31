mod hello_world;

use std::time::Duration;

use crate::greeter_server::{Greeter, GreeterServer};
use hello_world::*;
use tokio::{
    signal::unix::{signal, SignalKind},
    spawn,
    sync::oneshot::{self, Receiver, Sender},
    time::sleep,
};
use tonic::{transport::Server, Request, Response, Status};

#[derive(Default)]
pub struct GreeterService;

#[tonic::async_trait]
impl Greeter for GreeterService {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        let name = request.into_inner().name;
        let reply = hello_world::HelloResponse {
            message: format!("Hello, {}!", name),
        };

        // I am adding a sleep here to demonstrate that the server doesn't wait for the request to finish
        // The steps I follow:
        // 1. Start the server
        // 2. find the PID of the process [ps aux | grep debug/hello | head -n 1 | awk '{print $2}']
        // 3. Initiate a request to this endpoint
        // 4. `kill <pid>`
        sleep(Duration::from_secs(5)).await;

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:8080".parse()?;
    let svc = GreeterServer::new(GreeterService);

    println!("Server listening on {}", addr);

    let (signal_tx, signal_rx) = signal_channel();
    spawn(wait_for_sigterm(signal_tx));

    let server = Server::builder()
        .add_service(svc)
        .serve_with_shutdown(addr, async {
            signal_rx.await.ok();
            println!("Graceful context shutdown");
        });

    server.await?;

    Ok(())
}

fn signal_channel() -> (Sender<()>, Receiver<()>) {
    oneshot::channel()
}

async fn wait_for_sigterm(tx: Sender<()>) {
    let _ = signal(SignalKind::terminate())
        .expect("failed to install signal handler")
        .recv()
        .await;
    println!("SIGTERM received: shutting down");
    let _ = tx.send(());
}
