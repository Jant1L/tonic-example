use payments::bitcoin_client::BitcoinClient;
use payments::BtcPaymentRequest;


pub mod payments {
    tonic::include_proto!("payments");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = BitcoinClient::connect("http://[::1]:50051").await?;

    println!("please input the address of the sender");
    let mut from_address = String::new();
    std::io::stdin().read_line(&mut from_address).expect("read_line error!");

    println!("please input the address of the reciever");
    let mut to_address = String::new();
    std::io::stdin().read_line(&mut to_address).expect("read_line error!");

    println!("please input the number of the amount");
    let mut amount_number = String::new();
    std::io::stdin().read_line(&mut amount_number).expect("read_line error!");

    let request = tonic::Request::new(
        BtcPaymentRequest {
            from_addr: from_address.to_owned(),
            to_addr: to_address.to_owned(),
            amount: amount_number.to_owned(),
        }
    );

    let response = client.send_payment(request).await?;

    println!("Response={:?}", response);
    Ok(())
}