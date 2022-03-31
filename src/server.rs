use serde::{Serialize, Deserialize};

use crate::{route::Route};
use arkos::server::server::Server as AServer;


#[derive(Debug, Serialize, Deserialize)]
pub struct Server {
    port: u16,
    routes : Vec<Route>
}

impl Server {

    pub fn new(port: u16) -> Self {
        Server {port, routes: vec![Route::default()]}
    } 

    pub fn generate(&self) -> Result<AServer, String> {
        let mut server = AServer::new([127,0,0,1], self.port as u32).unwrap();

        for r in &self.routes {
            server.add_route(r.generate()?)
        }

        Ok(server)
    }
}

impl Default for Server {

    fn default() -> Self {
        Server { port: 8080, routes: vec![Route::default()]}
    }
}

#[cfg(test)]
mod server_test{


    use super::*;

    #[test]
    pub fn deserialize(){
        let file = r#"{"port": 8080,"routes": []}"#;
        let d = serde_json::from_str::<Server>(file);
        assert!(d.is_ok())
    }
}