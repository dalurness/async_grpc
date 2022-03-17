use tokio::sync::mpsc;
use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;
use tokio::task;
use tonic::Request;


use futures::future::join_all;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let mut v= vec![];
    let (tx, mut rx) = mpsc::channel(32);



    let manager = tokio::spawn(async move {
        // Establish a connection to the server
        let mut client = GreeterClient::connect("http://[::1]:50051").await.unwrap();
    
        // Start receiving messages
        while let Some(next) = rx.recv().await {
            match next {
                Ok(option) => {
                    match option {
                        Some(m) => {
                            let response = client.say_hello(m).await.unwrap();
                            println!("RESPONSE={:?}", response);
                        },
                        None => {}
                    }
                }
                Err(e) => println!("{:?}", e)
            }
        }
    });




    for i in 0..100 {
        let tx = tx.clone();
        v.push(task::spawn(async move {
            send_message(i, tx);
        }));
    }
    let results = join_all(v);
    

    Ok(())
}

async fn send_message(n: usize, s: mpsc::Sender<Request<HelloRequest>>) {
    let to_send = "D$$$ Ur Daddy ".to_owned() + &n.to_string();
    let request = tonic::Request::new(HelloRequest {
        name: to_send,
    });

    s.send(request).await;    
}