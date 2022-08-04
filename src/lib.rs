extern crate pest;

#[macro_use]
extern crate pest_derive;

#[macro_use]
extern crate derive_getters;

use pest::Parser;

#[derive(Parser)]
#[grammar = "http.pest"]
pub struct HttpParser;

#[derive(Debug, PartialEq, Default, Getters)]
pub struct HtttpRequest {
    uri: String,
    method: String,
    version: String,
    field: Vec<String>,
}

impl HtttpRequest {
    pub fn new() -> HtttpRequest {
        Default::default()
    }

    pub fn is_empty(&self) -> bool {
        self.uri == "" ||
        self.method == "" ||
        self.version == ""
    }

    pub fn is_valid(&self) -> bool {
        if self.is_empty() {
            return false;
        }
        else{
            return true;
        }
    }

}

impl std::fmt::Display for HtttpRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter)-> std::fmt::Result {
        if self.field.len() == 0 && !self.is_empty() {
            write!(f, "{} {} {}\r\n", self.method, self.uri, self.version)
        }
        else {
            let mut request = format!("{} {} {}", self.method, self.uri, self.version);

            for field_line in self.field.clone() {
                request = request + "\r\n" + &field_line;
            }

            write!(f, "{}", request)
        }
    }
}

pub struct Http;

impl Http {
    pub fn parse_request(http_request: &str) -> HtttpRequest {
        let parsed = HttpParser::parse(Rule::http_request, http_request)
        .unwrap_or_else(|e| panic!("{}", e));

        let mut request = HtttpRequest::new();

        for pair in parsed {
            match format!("{:?}",pair.as_rule()).as_str() {
                "method" => { request.method =  pair.as_str().to_string(); },
                "uri" => { request.uri =  pair.as_str().to_string(); },
                "version" => { request.version =  pair.as_str().to_string(); },
                "field_line" => { request.field.push(pair.as_str().to_string()); },
                _ => {},
            }
        }

        request
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_http_method0(){
        let http_method = "GET";

        HttpParser::parse(Rule::method, http_method)
        .expect("unsuccessful parse")
        .next();
    }

    #[test]
    fn success_http_method1(){
        let http_method = "POST";

        HttpParser::parse(Rule::method, http_method)
        .expect("unsuccessful parse")
        .next();
    }

    #[test]
    fn success_http_uri0(){
        let http_uri = "/";

        HttpParser::parse(Rule::uri, http_uri)
        .expect("unsuccessful parse")
        .next();
    }

    #[test]
    fn success_http_uri1(){
        let http_uri = "/index.html";

        HttpParser::parse(Rule::uri, http_uri)
        .expect("unsuccessful parse")
        .next();
    }

    #[test]
    fn success_http_uri2(){
        let http_uri = "/test.js";

        HttpParser::parse(Rule::uri, http_uri)
        .expect("unsuccessful parse")
        .next();
    }

    #[test]
    fn success_http_uri3(){
        let http_uri = "/home/settings";

        HttpParser::parse(Rule::uri, http_uri)
        .expect("unsuccessful parse")
        .next();
    }

    #[test]
    #[should_panic]
    fn fail_http_uri0(){
        let http_uri = "";

        HttpParser::parse(Rule::uri, http_uri)
        .expect("unsuccessful parse")
        .next();
    }

    #[test]
    #[should_panic]
    fn fail_http_uri1(){
        let http_uri = " ";

        HttpParser::parse(Rule::uri, http_uri)
        .expect("unsuccessful parse")
        .next();
    }

    #[test]
    #[should_panic]
    fn fail_http_uri2(){
        let http_uri = "\n";

        HttpParser::parse(Rule::uri, http_uri)
        .expect("unsuccessful parse")
        .next();
    }

    #[test]
    #[should_panic]
    fn fail_http_uri3(){
        let http_uri = "/home/settings{";

        HttpParser::parse(Rule::uri, http_uri)
        .expect("unsuccessful parse")
        .next();
    }

    #[test]
    #[should_panic]
    fn fail_http_uri4(){
        let http_uri = "/\t";

        HttpParser::parse(Rule::uri, http_uri)
        .expect("unsuccessful parse")
        .next();
    }

    #[test]
    #[should_panic]
    fn fail_http_uri5(){
        let http_uri = "/\\";

        HttpParser::parse(Rule::uri, http_uri)
        .expect("unsuccessful parse")
        .next();
    }

    #[test]
    fn success_http_version_number0(){
        let version_number = "1.1";

        HttpParser::parse(Rule::version_number, version_number)
        .expect("unsuccessful parse")
        .next();
    }

    #[test]
    fn success_http_version_number1(){
        let version_number = "1.1.0";

        HttpParser::parse(Rule::version_number, version_number)
        .expect("unsuccessful parse")
        .next();
    }

    #[test]
    #[should_panic]
    fn fail_http_version_number0(){
        let version_number = "1.";

        HttpParser::parse(Rule::version_number, version_number)
        .expect("unsuccessful parse")
        .next();
    }

    #[test]
    #[should_panic]
    fn fail_http_version_number1(){
        let version_number = ".1";

        HttpParser::parse(Rule::version_number, version_number)
        .expect("unsuccessful parse")
        .next();
    }

    #[test]
    fn success_http_version(){
        let http_version = "HTTP/1.1";

        HttpParser::parse(Rule::version, http_version)
        .expect("unsuccessful parse")
        .next();
    }

    #[test]
    fn success_http_field_name0() {
        let field_name = "Sec-Fetch-Dest";

        HttpParser::parse(Rule::field_name, field_name)
        .expect("unsuccessful parse")
        .next();
    }

    #[test]
    #[should_panic]
    fn fail_http_field_name0() {
        let field_name = "Sec-Fetch-Dest-";

        HttpParser::parse(Rule::field_name, field_name)
        .expect("unsuccessful parse")
        .next();
    }

    #[test]
    fn success_http_field_line0() {
        let field_line = "Sec-Fetch-Mode: navigate";

        HttpParser::parse(Rule::field_line, field_line)
        .expect("unsuccessful parse")
        .next();
    }

    #[test]
    fn success_http_field0() {
        let field = "Sec-Fetch-Mode: navigate";

        HttpParser::parse(Rule::field, field)
        .expect("unsuccessful parse")
        .next();
    }

    #[test]
    fn success_http_field1() {
        let field = "Sec-Fetch-Mode: navigate\r\n\
            Sec-Fetch-Mode: navigate";

        HttpParser::parse(Rule::field, field)
        .expect("unsuccessful parse")
        .next();
    }

    #[test]
    fn success_http_field2() {
        let field = "Host: 127.0.0.1:7878";

        HttpParser::parse(Rule::field, field)
        .expect("unsuccessful parse")
        .next();
    }

    #[test]
    #[should_panic]
    fn fail_http_field0() {
        let field = "Sec-Fetch-Mode: navigate\r\n";

        HttpParser::parse(Rule::field, field)
        .expect("unsuccessful parse")
        .next();
    }

    #[test]
    fn success_http_request0() {
        let request = "GET / HTTP/1.1\r\n";

        HttpParser::parse(Rule::http_request, request)
        .expect("unsuccessful parse")
        .next();
    }

    #[test]
    fn success_http_request1() {
        let http_request = "GET / HTTP/1.1\r\n\
            Host: 127.0.0.1:7878\r\n\
            User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:103.0) Gecko/20100101 Firefox/103.0\r\n\
            Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8\r\n\
            Accept-Language: en-US,en;q=0.5\r\n\
            Accept-Encoding: gzip, deflate, br\r\n\
            Connection: keep-alive\r\n\
            Upgrade-Insecure-Requests: 1\r\n\
            Sec-Fetch-Dest: document\r\n\
            Sec-Fetch-Mode: navigate\r\n\
            Sec-Fetch-Site: cross-site";

        HttpParser::parse(Rule::http_request, http_request)
        .expect("unsuccessful parse")
        .next();
    }

    #[test]
    fn success_create_http_request_struct0() {
        let http_request = "GET / HTTP/1.1\r\n\
            Host: 127.0.0.1:7878\r\n\
            User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:103.0) Gecko/20100101 Firefox/103.0\r\n\
            Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8\r\n\
            Accept-Language: en-US,en;q=0.5\r\n\
            Accept-Encoding: gzip, deflate, br\r\n\
            Connection: keep-alive\r\n\
            Upgrade-Insecure-Requests: 1\r\n\
            Sec-Fetch-Dest: document\r\n\
            Sec-Fetch-Mode: navigate\r\n\
            Sec-Fetch-Site: cross-site";

        let request = Http::parse_request(http_request);

        println!("{}", request);
    }

    #[test]
    fn success_create_http_request_struct1() {
        let http_request = "GET / HTTP/1.1\r\n";

        let request = Http::parse_request(http_request);

        println!("{}", request);
    }
}
