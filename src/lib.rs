extern crate pest;

#[macro_use]
extern crate pest_derive;

use pest::Parser;
use anyhow::{Result, Error};
use http_request::HttpRequest;
use http_response::HtttpResponse;

#[derive(Parser)]
#[grammar = "http.pest"]
pub struct HttpParser;

pub trait SetStatusLine {
    fn set_status_line(&mut self, status: &str) -> Result<&mut Self, Error>;
}

impl SetStatusLine for HtttpResponse {
    fn set_status_line(&mut self, status: &str) -> Result<&mut Self, Error> {
        let parsed = HttpParser::parse(Rule::status_line, status)?;

        for pair in parsed {
            match format!("{:?}",pair.as_rule()).as_str() {
                "version" => { self.set_version(pair.as_str()); },
                "status" => { self.set_status_code(pair.as_str()); },
                "message" => { self.set_message(pair.as_str()); },
                _ => {},
            }
        }

        Ok(self)
    }
}

pub struct Http;

impl Http {
    pub fn parse_request(http_request: &str) -> Result<HttpRequest, Error> {
        let parsed = HttpParser::parse(Rule::http_request, http_request)?;

        let mut request = HttpRequest::new();

        for pair in parsed {
            match format!("{:?}",pair.as_rule()).as_str() {
                "method" => { request.set_method(pair.as_str()); },
                "uri" => { request.set_uri(pair.as_str()); },
                "version" => { request.set_version(pair.as_str()); },
                "field_line" => { request.push_field_line(pair.as_str()); },
                _ => {},
            }
        }

        Ok(request)
    }

    pub fn parse_response(http_responce: &str) -> Result<HtttpResponse, Error> {
        let parsed = HttpParser::parse(Rule::http_response, http_responce)?;

        let mut response = HtttpResponse::new();

        for pair in parsed {
            match format!("{:?}",pair.as_rule()).as_str() {
                "status_code" => { response.set_status_code(pair.as_str()); },
                "status_messsage" => { response.set_message(pair.as_str()); },
                "version" => { response.set_version(pair.as_str()); },
                "field_line" => { response.push_field_line(pair.as_str()); },
                "response_body" => { response.set_body(pair.as_str()); },
                _ => {},
            }
        }

        Ok(response)
    }
}
