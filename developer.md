**Developer Documentation**

How to create a simple middleware in rust

## Environment Variables

The server uses the following environment variables:

- `HOST`: The hostname or IP address on which the server should listen. If not set, it defaults to `127.0.0.1`.
- `PORT`: The port number on which the server should listen. If not set, it defaults to `8888`.

## Middleware

The server looks for middleware executables in the `middlewares/` directory. Each middleware executable should be a standalone program that reads request metadata from its standard input and writes its output to the standard output.

### Input Format

The middleware executables receive their input as a JSON string representing the `Metadata` struct:

```rust
#[derive(Serialize, Deserialize, Debug)]
struct Metadata {
    method: String,     // HTTP method (GET, POST, etc.)
    path: String,       // Request path
    query: Value,       // Query string parameters
    req_body: String,   // Path to the request body file
    res_body: String,   // Path to the response body file
    headers: HashMap<String, String>, // Request headers
    state: Value,       // State information (initially empty)
}
```

The `req_body` field contains the path to a temporary file where the request body is stored. The `res_body` field contains the path to a temporary file where the middleware should write the response body.

### Output Format

The middleware executables should write their output as a JSON string representing the `MiddlewareOutput` struct:

```rust
#[derive(Serialize, Deserialize, Debug)]
struct MiddlewareOutput {
    status: u16,        // HTTP status code
    headers: HashMap<String, String>, // Response headers
    state: Value,       // Updated state information
    terminate: bool,    // Flag to indicate if middleware execution should terminate
}
```

The `headers` field should contain the response headers to be sent back to the client. The `state` field can be used to pass data between middleware executables. If the `terminate` field is set to `true`, the middleware chain will stop executing after the current middleware.

### Example Middleware

Here's an example of a simple middleware written in Rust that echoes the request body back as the response:

```rust
use std::fs::File;
use std::io::Read;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, Value};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct Metadata {
    req_body: String,
    res_body: String,
    state: Value,
}

#[derive(Serialize, Deserialize, Debug)]
struct MiddlewareOutput {
    status: u16,
    headers: HashMap<String, String>,
    state: Value,
    terminate: bool,
}
This project is a Rust-based server that uses the Actix Web framework to handle incoming HTTP requests. The server executes a chain of middleware executables, which can modify the request and response data. The middleware chain is executed for each incoming request, and the final response is streamed back to the client.to_string(&mut input).unwrap();

    let metadata: Metadata = from_str(&input).unwrap();

    let mut req_body = Vec::new();
    File::open(&metadata.req_body)
        .unwrap()
        .read_to_end(&mut req_body)
        .unwrap();

    let mut res_body_file = File::create(&metadata.res_body).unwrap();
    res_body_file.write_all(&req_body).unwrap();

    let output = MiddlewareOutput {
        status: 200,
        headers: HashMap::new(),
        state: metadata.state,
        terminate: false,
    };

    println!("{}", serde_json::to_string(&output).unwrap());
}
```

This middleware reads the request body from the file specified in `req_body`, writes it back to the file specified in `res_body`, and returns a `200 OK` status with no additional headers.

To build this middleware, save the code to a file (e.g., `echo.rs`) and compile it:

```
rustc echo.rs
```

Then, move the compiled executable to the `middlewares/` directory, and make sure it has execute permissions:

```
mv echo middlewares/
chmod +x middlewares/echo
```

Now, when the server receives a request, it will execute the `echo` middleware, which will echo the request body back as the response.


## You can also refer to the included examples in the middlewares/ directory.