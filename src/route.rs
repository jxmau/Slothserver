

use serde::{Serialize, Deserialize};
use arkos::{server::{route::Route as ARoute}, core::method::HttpMethod};


use crate::response::Response;

#[derive(Debug, Serialize, Deserialize)]
pub struct Route {
    pub name : String,
    pub path : String,
    pub order: Option<u16>,
    pub method: String,
    pub response: Response,    
}

impl Route {
    
    /// Create a new Route.
    pub fn new(name: &str, path: &str, method: &str, order: Option<u16>) -> Self {
        Self {name: name.into(), order, path: path.into(), method: method.to_uppercase(), response: Response::default()}
    }

    /// Generate a route for an Arkos Route for the Server. 
    pub fn generate(&self) -> Result<ARoute, String> {

        let mut route = ARoute::new(&self.path, get_method(&self.method)?);
        let response = &self.response.generate()?;
        let response = response.to_owned().clone();
        route.set_static_response(response);

        Ok(route)
    }

    /// Set the order field
    pub fn set_order(&mut self, o: u16) {
        self.order = Some(o);
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
    /// Will return a default GET route on the path "/" with a default Response.
    fn default() -> Self {
        Self {name: "New Route".into(), order: Some(1), path: "/".into(), method: "GET".to_string(), response: Response::default()}
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