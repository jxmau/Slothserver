use server::Server;

pub mod cookie;
pub mod response;
pub mod route;
pub mod server;

/// Will parse, generate and serve the Arkos server.
pub fn serve(content: &str) -> Result<(), String> {
    let server : Server = match serde_json::from_str(content) {
        Ok(s) => s,
        Err(_e) => return Err("The deserialization of the file failed.".to_string())
    };
    let server = server.generate()?;
    server.serve();
    Ok(())
}