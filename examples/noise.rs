#[cfg(feature="noise")]
use custom_noise::noise::NoiseInstance;
use wasmedge_wasi_socket::{Shutdown, TcpListener, TcpStream};

fn main() {
    static SECRET: &[u8] = b"we care a lot";
    let local_address = "127.0.0.1:33100";
    let listener = TcpListener::bind(local_address, true).unwrap();
    println!("Listening on address : {:?}", local_address);
}