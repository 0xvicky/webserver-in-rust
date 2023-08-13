use std::fs;
use std::io::prelude;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener; //Used to listen any connection locally
use std::net::TcpStream;

fn main() {
    println!("Basic Webserver in Rust");
    create_tcp_listener();
}

fn create_tcp_listener() {
    //First thing webserver do is to listen any TCP connection
    println!("Connecting...");
    const HOST: &str = "127.0.0.1"; //IP address of the local machine
    const PORT: &str = "7878"; //PORT on which TCP listens to the connection

    //Now we're going to create an endpoint by concatenating, HOST & PORT
    let endpoint: String = HOST.to_owned() + ":" + PORT; //to_owned() method basically makes a reference value to owned value or heap allocated

    let listener = TcpListener::bind(endpoint);

    match listener {
        Ok(listener) => {
            println!("Port is listening to {}", PORT);
            //looping over to liste ner to listen every connection, incoming() basically iterates over the incoming connections on the network
            for stream in listener.incoming() {
                match stream {
                    Ok(_stream) => {
                        println!("Connection established with stream:{:?}", _stream);
                        handle_connection(_stream);
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

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024]; //array of 1024 elements of 0,
    let stream_res = stream.read(&mut buffer);

    match stream_res {
        Ok(res) => {
            println!("Request:{:?}", String::from_utf8_lossy(&buffer[..]));
            println!("Result:{}", res);
        }
        Err(e) => {
            println!("Error occured while reading the connection:{}", e);
        }
    }

    //RESPONDING TO THE REQUEST FROM THE CLIENT
    let res_content = fs::read_to_string("index.html");
    match res_content {
        Ok(res) => {
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                res.len(),
                res
            );
            let stream_wrt: Result<usize, std::io::Error> = stream.write(response.as_bytes()); //writing response to the connection or stream
            match stream_wrt {
                Ok(_) => {
                    let stream_flsh = stream.flush(); //sending the buffer response instantly to the client if in the streamline.
                    match stream_flsh {
                        Ok(_) => {}
                        Err(e) => {
                            println!("Error occured:{}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("Error occured while writing the res:{}", e);
                }
            }
        }
        Err(e) => {
            println!("Error occured:{}", e);
        }
    }
}
