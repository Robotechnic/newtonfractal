#![warn(clippy::all, rust_2018_idioms)]

mod polynomial;
pub use polynomial::Polynomial;

mod newtonfractal;
pub use newtonfractal::NewtonFractal;

mod fractal_shader;
pub use fractal_shader::FractalShader;
