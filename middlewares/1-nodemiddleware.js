#!/usr/bin/env node
const process = require("process");
const fs = require("fs");

/**
 * Represents metadata associated with a request or response.
 * @typedef {Object} Metadata
 * @property {string} method - HTTP method (GET, POST, etc.).
 * @property {string} path - Request path.
 * @property {Object|Array} query - Query string parameters.
 * @property {string} req_body - Path to the request body file.
 * @property {string} res_body - Path to the response body file.
 * @property {Object<string, string>} headers - Request headers.
 * @property {Object|Array} state - State information (initially empty).
 */
/**
 * Represents a response object with status, headers, state, and terminate flag.
 * @typedef {Object} ResponseObject
 * @property {number} status - HTTP status code.
 * @property {Object<string, string>} headers - Response headers.
 * @property {Object<string, string>} state - State information.
 * @property {boolean} terminate - Whether to terminate the execution flow.
 */

/**
 * @type {Metadata}
 */
const metadata = JSON.parse(process.argv.slice(2).join(" "));
let terminate = false;
/**
 * Uncomment all the commented lines from here on if you want to respond with this middleware
 */
// fs.writeFileSync(metadata.res_body, `<html>
// <head>
//     <title>Response from Node</title>
// </head>
// <body>
//     <p>This is a ${metadata.method} request to ${metadata.path}</p>
// </body>
// </html>`);
//
//terminate = true;

function next() {
    /**
     * @type {ResponseObject}
     */
    const response = {
        headers: { 
            // "Content-Type": "text/html"
        },
        state: {
            from: "node"
        },
        status: 200,
        terminate
    };
    console.log(JSON.stringify(response));
}

next();

