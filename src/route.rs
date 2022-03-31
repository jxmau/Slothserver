

use serde::{Serialize, Deserialize};
use arkos::{server::{route::Route as ARoute}, core::method::HttpMethod};


use crate::response::Response;

#[derive(Debug, Serialize, Deserialize)]
pub struct Route {
    name : String,
    path : String,
    order: Option<u16>,
    method: String,
    response: Response,    
}

impl Route {

    pub fn new(name: &str, path: &str, method: &str, order: Option<u16>) -> Self {
        Self {name: name.into(), order, path: path.into(), method: method.to_uppercase(), response: Response::default()}
    }


    pub fn generate(&self) -> Result<ARoute, String> {

        let mut route = ARoute::new(&self.path, get_method(&self.method)?);
        let response = &self.response.generate()?;
        let response = response.to_owned().clone();
        route.set_static_response(response);

        Ok(route)
    }

}


pub fn get_method(m: &str) -> Result<HttpMethod, String> {
    Ok(
        match m {
            "PUT" => HttpMethod::PUT,
            "POST" => HttpMethod::POST,
            "GET" => HttpMethod::GET,
            "DELETE" => HttpMethod::DELETE,
            _ => return Err(format!(" Method {m} is not supported. ")),
        }
    )
}

impl Default for Route {

    fn default() -> Self {
        Self {name: "New Route".into(), order: None, path: "/".into(), method: "GET".to_string(), response: Response::default()}
    }
    
}


#[cfg(test)]
mod route_test{


    use super::*;

    #[test]
    pub fn deserialize(){

        let s = r#"
        
        {
            "name": "Route",
            "path": "/",
            "method": "GET",
            "response" : {
                "status": 200,
                "cookies": [],
                "headers": { "name": "value"},
                "body":""
            }
        }
        "#;

        let d = serde_json::from_str::<Route>(s);
        assert!(d.is_ok());
    }

}