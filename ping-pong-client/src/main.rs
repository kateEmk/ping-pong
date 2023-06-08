use std::net::Ipv6Addr;
use std::net::SocketAddr;

use wtransport::ClientConfig;
use wtransport::Endpoint;

use errors::ClientError;

pub mod errors;

const PORT: u16 = 4433;
const HOST: Ipv6Addr = Ipv6Addr::LOCALHOST;
const LOCALHOST: &str = "localhost";


#[tokio::main]
async fn main() -> Result<(), ClientError> {
    let addr = SocketAddr::new(Ipv6Addr::UNSPECIFIED.into(), 0);
    let config =
        ClientConfig::builder().with_bind_address(addr);

    println!("Bind address: {:?}.", addr);

    let conn = match Endpoint::client(config) {
        Ok(endpoint) => match endpoint.connect(SocketAddr::new(HOST.into(), PORT),
                                               LOCALHOST) {
            Ok(conn) => {
                match conn.await {
                    Ok(connection) => connection,
                    Err(_) => return Err(ClientError::TimeOut),
                }
            }
            Err(_) => return Err(ClientError::LocallyClosed),
        },
        Err(_) => return Err(ClientError::QuicError),
    };

    println!("Connected: port – {:?}, host – {:?}.", PORT, HOST);

    let mut stream = match conn.open_bi().await {
        Ok(s) => s,
        Err(_) => return Err(ClientError::StreamOpeningError)
    };

    let _res = match stream.0.write_all(b"ping").await {
        Ok(_) => {
            match stream.0.finish().await {
                Ok(_) => Ok(()),
                Err(_) => return Err(ClientError::TimeOut),
            }
        }
        Err(_) => Err(ClientError::TimeOut),
    };
    Ok(())
}