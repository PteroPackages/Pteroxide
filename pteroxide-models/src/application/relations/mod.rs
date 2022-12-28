pub mod egg;
pub mod location;
pub mod nest;
pub mod node;
pub mod server;
pub mod user;

pub use self::{
    egg::EggRelations, location::LocationRelations, nest::NestRelations, node::NodeRelations,
    server::ServerRelations, user::UserRelations,
};
