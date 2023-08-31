use tonic::Request;

mod hello_world {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = tonic::transport::Channel::from_static("http://127.0.0.1:8080")
        .connect()
        .await?;

    let mut client = hello_world::greeter_client::GreeterClient::new(channel);

    let request = Request::new(hello_world::HelloRequest {
        name: "John".to_string(),
    });

    let response = client.say_hello(request).await?;

    println!("Response: {:?}", response.get_ref());

    Ok(())
}
