use handlers::{dockerfile_handler, env_handler, json_handler};

pub mod common_context;
pub mod file_type;
pub mod handlers;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Contexts {
    Common(common_context::Context),
    Env(env_handler::Context),
    Dockerfile(dockerfile_handler::Context),
    JSON(json_handler::Context),
}
