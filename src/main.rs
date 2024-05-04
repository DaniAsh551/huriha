mod metadata;
mod middlewareoutput;
mod util;
use actix_http::StatusCode;
use actix_web::web::BytesMut;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpResponseBuilder, HttpServer};
use tokio::fs::File;
use std::env;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use serde_json::Value;
use std::collections::HashMap;
use futures_util::StreamExt;
use futures::TryStreamExt;
use tokio_util::codec::{BytesCodec, FramedRead};
use tempfile::NamedTempFile;
use tokio::io;
use metadata::Metadata;
use middlewareoutput::MiddlewareOutput;
use util::is_executable;


// Handle incoming HTTP requests
async fn handle_request(req: HttpRequest, mut payload: web::Payload) -> Result<HttpResponse, Error> {
    // Create temporary files for request and response bodies
    let mut req_body_file = NamedTempFile::new().unwrap();
    let res_body_file = NamedTempFile::new().unwrap();

    // Prepare metadata struct with request information
    let metadata = Metadata {
        method: req.method().to_string(),
        path: req.path().to_string(),
        query: req.query_string().parse().unwrap_or(Value::Object(Default::default())),
        req_body: req_body_file.path().to_str().unwrap().to_string(),
        res_body: res_body_file.path().to_str().unwrap().to_string(),
        headers: req.headers().iter().map(|(k, v)| (k.to_string(), v.to_str().unwrap().to_string())).collect(),
        state: Value::Object(Default::default()),
    };

    // Read request body into a BytesMut buffer
    let mut stream = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        stream.extend_from_slice(&chunk);
    }

    // Write request body to the temporary file
    req_body_file.write_all(&stream)?;

    // Execute middleware chain and get the output
    let middleware_out = execute_middlewares(metadata).await?;

    // Build the HTTP response
    let status = StatusCode::from_u16(middleware_out.status).unwrap();
    let mut resp_builder = HttpResponseBuilder::new(status);

    // Add response headers
    let headers = middleware_out.headers;
    for (key, value) in headers {
        resp_builder.insert_header((key.to_string(), value.to_string()));
    }

    // Read response body from the temporary file and stream it in the response
    let tokio_file = tokio::fs::File::from_std(res_body_file.into_file());
    let frame = FramedRead::new(tokio_file, BytesCodec::new()).map_ok(BytesMut::freeze);
    let res = resp_builder.streaming(frame);

    Ok(res)
}

// Execute middleware chain
async fn execute_middlewares(mut metadata: Metadata) -> Result<MiddlewareOutput, Error> {
    // Initialize middleware output with default values
    let mut middleware_out = MiddlewareOutput {
        status: 200,
        headers: HashMap::<String, String>::new(),
        state: Value::Object(Default::default()),
        terminate: false,
    };

    // Get the directory containing middleware executables
    let middlewares_dir = Path::new("middlewares");

    // Collect all executable files from the middleware directory
    let mut executables: Vec<_> = middlewares_dir
        .read_dir()?
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if is_executable(&path) {
                Some(path)
            } else {
                // TODO: read res_body into stream and pass it to res
                None
            }
        })
        .collect();

    // Sort the executables
    executables.sort();

    // Execute each middleware executable in order
    for executable in executables {
        // Run the middleware executable and capture its output
        let output = Command::new(executable)
            .arg(serde_json::to_string(&metadata)?)
            .output()
            .expect("Failed to parse command output");

        let output_str = String::from_utf8_lossy(&output.stdout).to_string();
        println!("{}", output_str);

        // Parse the middleware output
        let middleware_output: MiddlewareOutput = serde_json::from_str(&output_str).unwrap();

        middleware_out = middleware_output;
        metadata.state = middleware_out.state.clone();

        // If the middleware signals termination, break out of the loop
        if middleware_out.terminate {
            break;
        }
    }

    // Add some default headers to the response
    let resp_file = File::open(metadata.res_body).await.unwrap();
    let resp_len = resp_file.metadata().await.unwrap().len();
    middleware_out.headers.insert("Content-Length".to_string(), resp_len.to_string());
    middleware_out.headers.insert("Powered-By".to_string(), "Huriha".to_string());

    Ok(middleware_out)
}

// Main entry point of the application
#[actix_web::main]
async fn main() -> io::Result<()> {
    // Get the host and port from environment variables, or use defaults
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let server_host = host.clone();

    let port = env::var("PORT")
        .unwrap_or_else(|_| "8888".to_string())
        .parse()
        .unwrap();

    // Set up the Actix Web server
    let server = HttpServer::new(
        || App::new().default_service(web::to(handle_request)),
        )
        .bind((host, port))?
        .run();

    println!("Huriha Server listening on: {server_host}:{port}");

    // Run the server and wait for it to complete
    server.await?;

    Ok(())
}