use rand::random;
use reed_solomon_erasure::galois_8::ReedSolomon;
use std::net::SocketAddr;
use tokio::net::UdpSocket;

pub struct UdpNode {
    socket: UdpSocket,
    reed_solomon: ReedSolomon,
    loss_rate: f64, 
}

impl UdpNode {
    pub async fn new(addr: &str, loss_rate: f64) -> Self {
        let socket = UdpSocket::bind(addr)
            .await
            .expect("Failed to bind to address");
        // 9 data shards, 1 parity shards (total 10 shards)
        let reed_solomon = ReedSolomon::new(9, 1).expect("Invalid Reed-Solomon configuration");

        UdpNode {
            socket,
            reed_solomon,
            loss_rate,
        }
    }

    // Encode and send a message with Reed-Solomon erasure coding
    pub async fn send(&self, msg: &[u8], dest: SocketAddr) {
        let shards = self.encode_message(msg);

        for shard in shards {
            let rando = random::<f64>();
            if rando > self.loss_rate {
                println!("{}", rando > self.loss_rate);
                // Only send the shard if it "passes" the loss simulation
                self.socket
                    .send_to(&shard, dest)
                    .await
                    .expect("Failed to send shard");
            } else {
                println!("Simulated packet loss");
            }
        }
    }

    // Receive and decode message
    pub async fn receive(&self) -> Vec<u8> {
        let mut received_shards = vec![None; 10]; // Buffer for 9 data shards + 1 parity shards
        let mut buf = [0; 5]; // Adjust based on shard size
        for i in 0..received_shards.len() {
            if let Ok((len, _)) = self.socket.recv_from(&mut buf).await {
                received_shards[i] = Some(buf[..len].to_vec());
            }
        }

        println!("receive");
        received_shards.iter().for_each(|s| {
            println!("{:?}", s);
        });

        self.decode_message(received_shards)
            .expect("Failed to decode message")
    }

    // Encode message into Reed-Solomon shards
    fn encode_message(&self, msg: &[u8]) -> Vec<Vec<u8>> {
        let mut shards = self.split_into_shards(msg.to_vec(), 5);
        self.reed_solomon
            .encode(&mut shards)
            .expect("Failed to encode shards");
        shards.into_iter().map(|shard| shard.to_vec()).collect()
    }

    // Decode Reed-Solomon shards into the original message
    fn decode_message(
        &self,
        shards: Vec<Option<Vec<u8>>>,
    ) -> Result<Vec<u8>, reed_solomon_erasure::Error> {
        let mut shards: Vec<_> = shards
            .into_iter()
            .map(|opt| opt.map(Vec::into_boxed_slice))
            .collect();
        self.reed_solomon
            .reconstruct(&mut shards)
            .expect("Failed to reconstruct shards");

        let result: Vec<_> = shards.into_iter().flatten().collect();

        let flattened_result = self.flatten_boxes(result);

        Ok(flattened_result)
    }

    fn split_into_shards(&self, data: Vec<u8>, shard_size: usize) -> Vec<Vec<u8>> {
        data.chunks(shard_size) // Split data into chunks of size `shard_size`
            .map(|chunk| chunk.to_vec()) // Convert each chunk to a Vec<u8>
            .collect() // Collect into a Vec<Vec<u8>>
    }

    fn flatten_boxes(&self, boxed_data: Vec<Box<[u8]>>) -> Vec<u8> {
        boxed_data
            .into_iter() // Take ownership of `boxed_data` and consume it
            .flat_map(|boxed_array| boxed_array.into_vec()) // Convert each `Box<[u8]>` to `Vec<u8>` and flatten
            .collect() // Collect all elements into a single `Vec<u8>`
    }
}
