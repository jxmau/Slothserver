use std::collections::HashMap;
use arkos::{server::response::Response as AResponse, core::{status::StatusCode, content::ContentType}};

use log::trace;
use serde::{Serialize, Deserialize};
use serde_json::Value;

use crate::cookie::Cookie;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub status: u8,
    cookies : Vec<Cookie>, // Create a Cookie struct for modulability
    headers : HashMap<String, String>,
    body : Value
}

impl Response {
    /// Generate an Arkos Reason
    pub fn generate(&self) -> Result<AResponse, String> {
        let mut r = AResponse::default();
        r.status = StatusCode::from_str(&format!("{}", &self.status));
        for c in &self.cookies {
            r.add_cookie(c.generate())
        }
        for (key, val) in &self.headers {
            trace!(" Adding Header {key} {val} to the Response. ");
            if key.eq("Content-Type") {
                r.set_content_type(ContentType::Custom(val.to_string()))
            } else {
                r.add_header(key.to_owned(), val.to_owned());
            }

        }
        r.set_body(self.body.to_string().to_owned());
        trace!(" Response generated : {r:?} ");
        Ok(r)
    }    
}







// The Deserialize is done as a Test of the Route struct