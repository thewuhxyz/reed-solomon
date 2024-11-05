use std::net::SocketAddr;
use tokio::time::{sleep, Duration};
use node::{self, UdpNode};

#[tokio::main]
async fn main() {
    // Node addresses
    let addr2: SocketAddr = "127.0.0.1:8082".parse().unwrap();

    // Loss rate of 5%
    let node1 = UdpNode::new("127.0.0.1:8081", 0.05).await;

    // Spawn task for node2 to listen and receive
    tokio::spawn(async move {
        loop {
            // Send a message from Node 1 to Node 2
            let msg = b"Hello from Node 1 with Reed-Solomon encoding!";

            let prep_msg = extend_array(msg);
            node1.send(prep_msg.as_slice(), addr2).await;

            println!(
                "Node 1 sent: {:?}",
                String::from_utf8_lossy(msg)
            );
            sleep(Duration::from_secs(3)).await;
        }
    });

    // Keep the main function alive indefinitely
    tokio::signal::ctrl_c().await.unwrap();
    println!("Process terminated.");
}

fn extend_array(input: &[u8; 45]) -> [u8; 50] {
    let mut new_array = [0u8; 50]; // Create a new array of size 50, initialized with zeros
    new_array[..45].copy_from_slice(input); // Copy the first 45 elements from the input array
    // The last 5 elements will remain as zeros, as needed
    new_array
}