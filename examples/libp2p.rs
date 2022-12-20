use bytecodec::DecodeExt;
use httpcodec::{HttpVersion, ReasonPhrase, Request, RequestDecoder, Response, StatusCode};
use std::io::{Read, Write};
use wasmedge_wasi_socket::{Shutdown, TcpListener, TcpStream};
// use libp2p::swarm::{NetworkBehaviour, Swarm, SwarmEvent};
// use libp2p::{identity, ping, Multiaddr, PeerId};
// use libp2p_swarm::keep_alive;
// use wasmedge_bindgen::*;
// use wasmedge_bindgen_macro::*;

fn handle_http(req: Request<String>) -> bytecodec::Result<Response<String>> {
    Ok(Response::new(
        HttpVersion::V1_0,
        StatusCode::new(200)?,
        ReasonPhrase::new("")?,
        format!("echo: {}", req.body()),
    ))
}

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buff = [0u8; 1024];
    let mut data = Vec::new();

    loop {
        let n = stream.read(&mut buff)?;
        data.extend_from_slice(&buff[0..n]);
        if n < 1024 {
            break;
        }
    }

    let mut decoder =
        RequestDecoder::<httpcodec::BodyDecoder<bytecodec::bytes::Utf8Decoder>>::default();

    let req = match decoder.decode_from_bytes(data.as_slice()) {
        Ok(req) => handle_http(req),
        Err(e) => Err(e),
    };

    let r = match req {
        Ok(r) => r,
        Err(e) => {
            let err = format!("{:?}", e);
            Response::new(
                HttpVersion::V1_0,
                StatusCode::new(500).unwrap(),
                ReasonPhrase::new(err.as_str()).unwrap(),
                err.clone(),
            )
        }
    };

    let write_buf = r.to_string();
    stream.write(write_buf.as_bytes())?;
    stream.shutdown(Shutdown::Both)?;
    Ok(())
}

fn main() {
    println!("Hello, world!");
}

#[no_mangle]
pub extern "C" fn print_hello() {
    println!("Hello, world! no mangle!");
}

#[no_mangle]
pub extern "C" fn http_server() {
    let port = std::env::var("PORT").unwrap_or("1234".to_string());
    println!("new connection at {}", port);
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port), false).unwrap();
    loop {
        let _ = handle_client(listener.accept(false).unwrap().0);
        ()
    }
}

// #[wasmedge_bindgen]
#[no_mangle]
pub extern "C" fn p2p() {
    println!("Starting...");
    #[cfg(feature = "p2p")]
    #[cfg_attr(docsrs, doc(cfg(feature = "p2p")))]
    use libp2p::{
        // identity,
    //     // mdns::{Mdns, MdnsConfig, MdnsEvent},
    //     swarm::{Swarm, SwarmEvent},
        PeerId,
    };
    // let local_key = identity::Keypair::generate_ed25519();
    // let local_peer_id = PeerId::from(local_key.public());
    // println!("Local peer id: {local_peer_id:?}");
}