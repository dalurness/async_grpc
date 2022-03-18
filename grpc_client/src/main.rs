use tokio::sync::mpsc;
use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;
use tokio::task;
use tonic::Request;
pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let mut v= vec![];
    let (tx, mut rx): (mpsc::Sender<Request<HelloRequest>>, mpsc::Receiver<Request<HelloRequest>>) = mpsc::channel(6);

    let manager = tokio::spawn(async move {
        // Establish a connection to the server
        let mut client = GreeterClient::connect("http://[::1]:50051").await.unwrap();
    
        // Start receiving messages

        println!("Starting manager...");
        loop {
            let val = rx.recv().await;
            match val {
                Some(message) => {
                    println!("Sending...");
                    let _response = client.say_hello(message).await.unwrap();
                },
                None => {
                    println!(" ITS OVER ");
                    break;
                },
            }
            //println!("RESPONSE={:?}", response);

        }
    });

    for i in 0..100 {
        let tx = tx.clone();
        v.push(task::spawn(async move {
            send_message(i, tx).await;
        }));
    }
    for process in v {
        process.await.unwrap();
        println!("Done Waiting");
    }

    manager.await.unwrap();

    //manager
    Ok(())
}

async fn send_message(n: usize, s: mpsc::Sender<Request<HelloRequest>>) {
    let to_send = "D$$$ Ur Daddy ".to_owned() + &n.to_string();
    let request = tonic::Request::new(HelloRequest {
        name: to_send,
    });
    s.send(request).await.unwrap();
    println!("Created new message");
}
