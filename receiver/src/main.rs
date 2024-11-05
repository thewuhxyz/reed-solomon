use node::{self, UdpNode};

#[tokio::main]
async fn main() {
    // Loss rate of 10%
    let node2 = UdpNode::new("127.0.0.1:8082", 0.05).await;

    // Spawn task for node2 to listen and receive
    tokio::spawn(async move {
        loop {
            let received_msg = node2.receive().await;
            println!(
                "Node 2 received: {:?}",
                String::from_utf8_lossy(received_msg.as_slice())
            );
        }
    });

    // Keep the main function alive indefinitely
    tokio::signal::ctrl_c().await.unwrap();
    println!("Process terminated.");
}
