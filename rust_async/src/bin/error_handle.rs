use std::fs;
use std::net::TcpStream;
use std::num::ParseIntError;
use std::path::Path;

// type GenericResult<T> = Result<T, Box<dyn Error>>;
fn main() {
    // let _ = connect_and_validate("127:8080");
    // if let Ok(stream) = connect_and_validate("127:80") {
    //     println!("connected to server:{stream:#?}");
    // }
    let stream = load_config_and_connect();
    match stream {
        Ok(_stream) => {
            println!("Successfully connected to server");
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    let stream = anyhow_load_config_and_connect();
    match stream {
        Ok(_stream) => {
            println!("Successfully connected to server");
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
#[allow(dead_code)]
fn connect_and_validate(address: &str) -> Result<TcpStream, std::io::Error> {
    let stream = TcpStream::connect(address)?;
    Ok(stream)
}

fn parse_address(content: &str) -> Result<(String, u16), ParseIntError> {
    let parts: Vec<&str> = content.split(":").collect();
    let host = parts[0].to_string();
    let port = parts[1].parse::<u16>()?;
    Ok((host, port))
}

fn load_config_and_connect() -> anyhow::Result<TcpStream> {
    let path = Path::new("server_config.yaml");
    let text: String = fs::read_to_string(path)?;
    let (host, port) = parse_address(text.trim())?;
    let address = format!("{host}:{port}");
    let stream = TcpStream::connect(&address)?;
    Ok(stream)
}

fn anyhow_load_config_and_connect() -> anyhow::Result<TcpStream> {
    let path = Path::new("server_config.yaml");
    let text: String = fs::read_to_string(path)?;
    let address = text.trim();

    if address.is_empty() {
        anyhow::bail!("server config file cannot be empty");
    }

    if !address.contains(":") {
        return Err(anyhow::Error::msg("server config file must contain ':'"));
    }

    let stream = TcpStream::connect(&address)?;
    Ok(stream)
}
