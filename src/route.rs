

use serde::{Serialize, Deserialize};
use arkos::{server::{route::Route as ARoute}, core::method::HttpMethod};


use crate::response::Response;

#[derive(Debug, Serialize, Deserialize)]
pub struct Route {
    name : String,
    path : String,
    method: String,
    response: Response,    
}

impl Route {

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