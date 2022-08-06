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

#[derive(Debug, PartialEq, Default, Getters)]
pub struct HtttpResponse {
    status: String,
    message: String,
    version: String,
    field: Vec<String>,
    body: String,
}

impl HtttpResponse {
    pub fn new() -> HtttpResponse {
        Default::default()
    }

    pub fn set_status_line(&mut self, status: &str) -> &mut Self{
        let parsed = HttpParser::parse(Rule::status_line, status)
        .unwrap_or_else(|e| panic!("{}", e));

        for pair in parsed {
            match format!("{:?}",pair.as_rule()).as_str() {
                "version" => { self.version =  pair.as_str().to_string(); },
                "status" => { self.status =  pair.as_str().to_string(); },
                "message" => { self.message =  pair.as_str().to_string(); },
                _ => {},
            }
        }

        self
    }

    pub fn set_status_code(&mut self, status: &str) -> &mut Self{
        self.status = String::from(status);

        self
    }

    pub fn set_message(&mut self, message: &str) -> &mut Self{
        self.message = String::from(message);

        self
    }

    pub fn set_version(&mut self, version: &str) -> &mut Self{
        self.version = String::from(version);

        self
    }

    pub fn set_body(&mut self, body: &str) -> &mut Self{
        self.body = String::from(body);

        self
    }

    pub fn push_field_line(&mut self, field_line: &str) -> &mut Self{
        self.field.push(String::from(field_line));

        self
    }

    pub fn append_field(&mut self, field: &mut Vec<String>) -> &mut Self{
        self.field.append(field);

        self
    }

    pub fn is_empty(&self) -> bool {
        self.status == "" ||
        self.message == "" ||
        self.version == "" ||
        self.field.len() == 0 ||
        self.body == ""
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

impl std::fmt::Display for HtttpResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter)-> std::fmt::Result {
        let mut request = format!("{} {} {}", self.version, self.status, self.message);

        for field_line in self.field.clone() {
            request = request + "\r\n" + &field_line;
        }

        request = format!("{}\r\n\r\n{}", request, self.body);

        write!(f, "{}", request)
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

    pub fn parse_response(http_responce: &str) -> HtttpResponse {
        let parsed = HttpParser::parse(Rule::http_response, http_responce)
        .unwrap_or_else(|e| panic!("{}", e));

        let mut request = HtttpResponse::new();

        for pair in parsed {
            match format!("{:?}",pair.as_rule()).as_str() {
                "status_code" => { request.status =  pair.as_str().to_string(); },
                "status_messsage" => { request.message =  pair.as_str().to_string(); },
                "version" => { request.version =  pair.as_str().to_string(); },
                "field_line" => { request.field.push(pair.as_str().to_string()); },
                "response_body" => { request.body =  pair.as_str().to_string(); },
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

    #[test]
    fn success_http_responce0() {
        let http_response = "HTTP/1.1 200 OK\r\n\
            Content-Length: 299\r\n\
            \r\n\
            function test(e) {\r\n\
            \tconsole.log(e);\r\n\
            }\r\n\
            \r\n\
            // Add event listener on keydown\r\n\
            document.addEventListener('keydown', (event) => {\r\n\
            \tvar name = event.key;\r\n\
            \tvar code = event.code;\r\n\
            \t// Alert the key name and key code on keydown\r\n\
            \tconsole.log(`Key pressed ${name} \r\n Key code value: ${code}`);\r\n\
            \r\n\
            }, false);";

        HttpParser::parse(Rule::http_response, http_response)
        .expect("unsuccessful parse")
        .next();
    }

    #[test]
    fn success_http_responce1() {
        let http_response = "HTTP/1.1 404 NOT FOUND\r\n\
            Content-Length: 206\r\n\
            \r\n\
            <!DOCTYPE html>\r\n\
            <html lang=\"en\">\r\n\
            \t<head>\r\n\
            \t\t<meta charset=\"utf-8\">\r\n\
            \t\t<title>Hello!</title>\r\n\
            \t</head>\r\n\
            \t<body>\r\n\
            \t\t<h1>Oops!</h1>
            \t\t<p>Sorry, I don't know what you're asking for.</p>
            \t</body>\r\n\
            </html>";

        HttpParser::parse(Rule::http_response, http_response)
        .expect("unsuccessful parse")
        .next();
    }

    #[test]
    fn success_create_http_responce_struct0() {
        let http_response = "HTTP/1.1 200 OK\r\n\
            Content-Length: 299\r\n\
            \r\n\
            function test(e) {\r\n\
            \tconsole.log(e);\r\n\
            }\r\n\
            \r\n\
            // Add event listener on keydown\r\n\
            document.addEventListener('keydown', (event) => {\r\n\
            \tvar name = event.key;\r\n\
            \tvar code = event.code;\r\n\
            \t// Alert the key name and key code on keydown\r\n\
            \tconsole.log(`Key pressed ${name} \r\n Key code value: ${code}`);\r\n\
            \r\n\
            }, false);";

        let response = Http::parse_response(http_response);

        println!("{}", response);
    }

    #[test]
    fn success_build_http_responce_struct0() {
        let mut response = HtttpResponse::new();

        response
        .set_version("HTTP/1.1")
        .set_status_code("200")
        .set_message("OK")
        .push_field_line("Content-Length: 299")
        .set_body("function test(e) {\r\n\
            \tconsole.log(e);\r\n\
            }\r\n\
            \r\n\
            // Add event listener on keydown\r\n\
            document.addEventListener('keydown', (event) => {\r\n\
            \tvar name = event.key;\r\n\
            \tvar code = event.code;\r\n\
            \t// Alert the key name and key code on keydown\r\n\
            \tconsole.log(`Key pressed ${name} \r\n Key code value: ${code}`);\r\n\
            \r\n\
            }, false);");

        println!("{}", response);
    }

    #[test]
    fn success_build_http_responce_struct1() {
        let mut response = HtttpResponse::new();

        response
        .set_status_line("HTTP/1.1 200 OK")
        .push_field_line("Content-Length: 299")
        .set_body("function test(e) {\r\n\
            \tconsole.log(e);\r\n\
            }\r\n\
            \r\n\
            // Add event listener on keydown\r\n\
            document.addEventListener('keydown', (event) => {\r\n\
            \tvar name = event.key;\r\n\
            \tvar code = event.code;\r\n\
            \t// Alert the key name and key code on keydown\r\n\
            \tconsole.log(`Key pressed ${name} \r\n Key code value: ${code}`);\r\n\
            \r\n\
            }, false);");

        println!("{}", response);
    }
}
