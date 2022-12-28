pub mod location;
pub mod node;
pub mod server;
pub mod user;

pub use self::{
    location::LocationRelations, node::NodeRelations, server::ServerRelations, user::UserRelations,
};
