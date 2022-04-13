use serde::{Serialize, Deserialize};
use arkos::core::cookie::Cookie as ACookie;

#[derive(Debug, Serialize, Deserialize)]
pub struct Cookie {
    pub name: String,
    pub value: String,
}





impl Cookie {

    /// Generate an Arkos Cookie.
    pub fn generate(&self) -> ACookie {
        ACookie::new(self.name.to_owned(), self.value.to_owned())
    }

    /// Create a new cookie
    pub fn new(name: &str, value: &str) -> Self {
        Self {name: name.into(), value: value.into() }
    }

}