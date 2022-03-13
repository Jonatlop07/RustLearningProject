pub mod ctx;
pub mod renderer;
pub mod form;
pub mod http;
pub mod hitcounter;
pub mod api;

pub use hitcounter::HitCounter;

pub const PASSWORD_COOKIE: &str = "password";

use handlebars::RenderError;
use serde_json::Error;

#[derive(rocket::Responder)]
pub enum PageError {
    #[response(status = 500)]
    Serialization(String),
    #[response(status = 500)]
    Render(String),
    #[response(status = 404)]
    NotFound(String),
    #[response(status = 500)]
    Internal(String)
}

impl From<RenderError> for PageError {
    fn from(err: RenderError) -> Self {
        PageError::Render(format!("{}", err))
    }
}

impl From<Error> for PageError {
    fn from(err: Error) -> Self {
        PageError::Serialization(format!("{}", err))
    }
}

#[cfg(test)]
pub mod test {
    use crate::test::async_runtime;
    use crate::RocketConfig;
    use rocket::local::blocking::Client;
    
    pub fn config() -> RocketConfig {
        use crate::web::{hitcounter::HitCounter, renderer::Renderer};
        let rt = async_runtime();
        let renderer = Renderer::new("templates/".into());
        let database = crate::data::test::new_db(rt.handle());
        let maintenance = crate::domain::maintenance::Maintenance::spawn(
            database.get_pool().clone(), 
            rt.handle().clone()
        );
        let hit_counter = HitCounter::new(
            database.get_pool().clone(),
            rt.handle().clone()
        );
        
        RocketConfig {
            renderer,
            database,
            hit_counter,
            maintenance
        }
    }
    
    pub fn client() -> Client {
        let config = config();
        Client::tracked(crate::rocket(config))
            .expect("failed to build rocket instance")
    }
}