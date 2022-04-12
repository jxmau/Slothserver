use serde::{Serialize, Deserialize};

use crate::{route::Route};
use arkos::server::server::Server as AServer;


#[derive(Debug, Serialize, Deserialize)]
pub struct Server {
    pub port: u16,
    pub routes : Vec<Route>
}

impl Server {

    /// Create a new Server.
    pub fn new(port: u16) -> Self {
        Server {port, routes: vec![Route::default()]}
    } 

    /// Generate an Arkos Server.
    pub fn generate(&self) -> Result<AServer, String> {
        let mut server = AServer::new([127,0,0,1], self.port as u32).unwrap();

        for r in &self.routes {
            server.add_route(r.generate()?)
        }

        Ok(server)
    }

    /// Add a route to the Server
    pub fn add_route(&mut self, r: Route) {
        self.routes.push(r)
    }

    /// Get the routes.
    pub fn get_routes(&self) -> &Vec<Route> {
        &self.routes
    }

    /// Delete a route
    pub fn delete_route(&mut self, od: u16) {
        self.routes.remove(od.into());
    }

    /// Reorganize the od of the routes.
    pub fn reorganize_routes(&mut self) {
        let mut od = 1;
        for r in &mut self.routes {
            r.set_order(od);
            od = od + 1;
        }
    }

}

impl Default for Server {
    /// Return a Server on the port 8080
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

    #[test]
    pub fn reorganize() {
        let mut server = Server::default();
        server.add_route(Route::default());
        server.add_route(Route::default());
        server.add_route(Route::default());
        server.reogarnize_routes();
        let mut od = 1;
        for r in server.get_routes() {
            assert_eq!(r.order.unwrap(), od);
            od = 1 + od;
        }
        server.delete_route(2);
        server.reogarnize_routes();
        od = 1;
        for r in server.get_routes() {
            assert_eq!(r.order.unwrap(), od);
            od = 1 + od;
        }
    }


}
