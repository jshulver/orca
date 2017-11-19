error_chain! {
    errors {
        /// An error meaning you sent a bad request to reddit
        BadRequest(message: String) {
            description("Reddit failed to handle the request")
            display("Failed to handle request. Got response {}", message)
        }

        /// You tried to perform an action that was unauthorized. Make sure to set the auth field
        /// of a connection
        Unauthorized {
            description("A request was made without proper authorization")
            display("Made an unauthorized request")
        }

        /// The response recieved was formatted in an unexpected way and failed to parse
        ResponseError(response: String) {
            description("Got a response that was unexpected")
            display("Unexpected response: {}", response)
        }

        Unimplemented {
            description("Tried to do something unimplemented")
            display("Unimplented feature")
        }

        InvalidJson(jsonstr: String) {
            description("JSON recieved could not be parsed correctly")
            display("Invalid json: {}", jsonstr)
        }

        Other(msg: String) {
            description("An error occurred")
            display("{}", msg)
        }
    }
}

use failure::{Fail, Error as FError, err_msg};

#[derive(Debug, Fail)]
#[fail(display = "The requested resource could not be found")]
pub struct NotFound {}

#[derive(Debug, Fail)]
#[fail(display = "The requested resource is forbidden")]
pub struct Forbidden {}

#[derive(Debug, Fail)]
#[fail(display = "Got an unexpected reponse:\n{}", response)]
pub struct BadResponse {
    pub response: String,
}
