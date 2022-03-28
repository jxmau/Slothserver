use serde::{Serialize, Deserialize};
use arkos::core::cookie::Cookie as ACookie;

#[derive(Debug, Serialize, Deserialize)]
pub struct Cookie {
    name: String,
    value: String,
}





impl Cookie {

    /// Generate an Arkos Cookie.
    pub fn generate(&self) -> ACookie {
        ACookie::new(self.name.to_owned(), self.value.to_owned())
    }

}