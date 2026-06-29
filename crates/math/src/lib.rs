//! OpenFiltration Mathematics Library
//!
//! This crate contains the mathematical foundations of the
//! OpenFiltration project.
//!
//! No physics.
//! No numerics.
//! Only mathematics.

pub mod point2;
pub mod scalar;
pub mod scalar_field;
pub mod vector2;
pub mod vector3;

pub use point2::Point2;
pub use scalar::Scalar;
pub use scalar_field::ScalarField;
pub use vector2::Vector2;
pub use vector3::Vector3;