#!/usr/bin/env node
const process = require("process");

/**
 * Represents metadata associated with a request or response.
 * @typedef {Object} Metadata
 * @property {string} method - HTTP method (GET, POST, etc.).
 * @property {string} path - Request path.
 * @property {Object|Array} query - Query string parameters.
 * @property {string} reqBody - Path to the request body file.
 * @property {string} resBody - Path to the response body file.
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

function next() {
    /**
     * @type {ResponseObject}
     */
    const response = {
        headers: {},
        state: {
            from: "node"
        },
        status: 200,
        terminate: false
    };
    console.log(JSON.stringify(response));
}

next();

