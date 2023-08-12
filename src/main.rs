use std::net::TcpListener; //Used to listen any connection locally

fn main() {
    println!("Basic Webserver in Rust");
    create_tcp_listener();
}

fn create_tcp_listener() {
    //First thing webserver do is to listen any TCP connection
    println!("Connecting...");
    const HOST: &str = "127.0.0.1"; //IP address of the local machine
    const PORT: &str = "8477"; //PORT on which TCP listens to the connection

    //Now we're going to create an endpoint by concatenating, HOST & PORT
    let endpoint: String = HOST.to_owned() + ":" + PORT; //to_owned() method basically makes a reference value to owned value or heap allocated

    let listener = TcpListener::bind(endpoint);

    match listener {
        Ok(listener) => {
            println!("Port is listening to {}", PORT);
            //looping over to listener to listen every connection
            for stream in listener.incoming() {
                match stream {
                    Ok(_stream) => {
                        println!("Connection established with stream:{:?}", _stream);
                    }
                    Err(e) => {
                        println!("Error occured while connecting:{}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("Err occured:{}", e);
        }
    }
}
