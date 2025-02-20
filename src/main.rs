use std::env::args;
use rdkafka::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

fn create_producer(bootstrap_server: &str) -> FutureProducer {
    ClientConfig::new()
        .set("bootstrap.servers", bootstrap_server)
        .set("queue.buffering.max.ms", "0")
        .create().expect("Failed to create client")
}

#[tokio::main]
async fn main() {
    let mut stdout = tokio::io::stdout();
    let mut input_lines = BufReader::new(tokio::io::stdin()).lines();
 
    stdout.write(b"Welcome to Kafka chat!\n").await.unwrap();
 
 
    // Creates a producer, reading the bootstrap server from the first command-line argument
    // or defaulting to localhost:9092
    let producer = create_producer(&args().skip(1).next()
        .unwrap_or("localhost:9092".to_string()));
 
    let mut stdout = tokio::io::stdout();
    let mut input_lines = BufReader::new(tokio::io::stdin()).lines();
 
    loop {
        // Write a prompt to stdout
        stdout.write(b"> ").await.unwrap();
        stdout.flush().await.unwrap();
 
        // Read a line from stdin
        match input_lines.next_line().await.unwrap() {
            Some(line) => {
                // Send the line to Kafka on the 'chat' topic
                producer.send(FutureRecord::<(), _>::to("chat")
                  .payload(&line), Timeout::Never)
                    .await
                    .expect("Failed to produce");
            }
            None => break,
        }
    }
}