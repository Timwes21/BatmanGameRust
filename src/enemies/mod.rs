pub mod enemies;
pub mod knife_guy;
pub mod gun_guy;
pub mod enemy;
pub mod mime;
pub mod rocket_launcher;
pub mod fire_mime;
pub mod axe_guy;

pub use gun_guy::GunGuy;
pub use knife_guy::KnifeGuy;
pub use mime::Mime;
pub use enemy::{Enemy, Action};
pub use enemies::Enemies;
pub use rocket_launcher::RocketGuy;
pub use fire_mime::FireMime;
pub use axe_guy::AxeGuy;