use std::process::exit;

use egui::vec2;
use macroquad::{
    material::{load_material, Material, MaterialParams},
    math::{Vec2, Vec3},
    miniquad::{ShaderError, ShaderSource, UniformType},
    Error,
};
use num_complex::Complex;

use crate::Polynomial;

const VERTEX: &str = include_str!("shaders/vertex.vert");

pub struct NewtonFractal {
    max_iterations: u32,
    roots: Vec<Vec2>,
    colors: Vec<Vec3>,
    derivative: Polynomial,
    material: Material,
}

impl NewtonFractal {
    fn polynomial_from_roots(roots: &[Vec2]) -> Polynomial {
        let mut poly = Polynomial::new(vec![Complex::new(1.0, 0.0)]);
        for root in roots {
            poly.add_root(Complex::new(root.x, root.y));
        }
        poly
    }

    fn create_material(len: usize) -> Material {
        let mut params = Vec::new();
        params.push(("maxIterations".to_owned(), UniformType::Int1));
        for i in 0..len {
            params.push((format!("root{}", i), UniformType::Float2));
            params.push((format!("color{}", i), UniformType::Float3));
            params.push((format!("dcoeff{}", i), UniformType::Float2));
        }
        let material = load_material(
            ShaderSource::Glsl {
                vertex: VERTEX,
                fragment: include_str!("shaders/fractal.frag"),
            },
            MaterialParams {
                uniforms: params,
                ..Default::default()
            },
        );

        if material.is_err() {
            match material.err().unwrap() {
                Error::ShaderError(ShaderError::CompilationError {
                    shader_type,
                    error_message,
                }) => {
                    println!("Error in {:?} shader:\n{}", shader_type, error_message);
                }
                err => println!("{:?}", err),
            }
            exit(1);
        }
        material.unwrap()
    }

    pub fn new(roots: Vec<Vec2>, colors: Vec<Vec3>, max_iterations: u32) -> Result<Self, ()> {
        if roots.len() != colors.len() {
            return Err(());
        }
        let polynomial = NewtonFractal::polynomial_from_roots(&roots);
        let derivative = polynomial.derivative();
        let material = NewtonFractal::create_material(roots.len());
        NewtonFractal::set_material_roots(&roots, &colors, &material);
        NewtonFractal::set_material_derivative_coeff(&derivative.get_coefficients(), &material);
		NewtonFractal::set_material_max_iter(max_iterations, &material);

        Ok(Self {
            roots,
            colors,
            derivative,
            material,
            max_iterations,
        })
    }

    pub fn get_material(&self) -> &Material {
        &self.material
    }

    fn set_material_roots(roots: &[Vec2], colors: &[Vec3], material: &Material) {
        for i in 0..roots.len() {
            material.set_uniform(format!("root{}", i).as_str(), roots[i]);
            material.set_uniform(format!("color{}", i).as_str(), colors[i]);
        }
    }

    fn set_material_derivative_coeff(coefs: &[Complex<f32>], material: &Material) {
        for i in 0..coefs.len() {
			println!("dcoeff{}: {}", i, coefs[i]);
            material.set_uniform(
                format!("dcoeff{}", i).as_str(),
                vec2(coefs[i].re, coefs[i].im),
            );
        }
    }

    fn set_material_max_iter(max_iter: u32, material: &Material) {
        material.set_uniform("maxIterations", max_iter);
    }

    pub fn set_roots(&mut self, roots: Vec<Vec2>, colors: Vec<Vec3>) -> bool {
        if roots.len() != colors.len() {
            return false;
        }
        self.roots = roots;
        self.colors = colors;
        let polynomial = NewtonFractal::polynomial_from_roots(&self.roots);
        self.derivative = polynomial.derivative();
        NewtonFractal::set_material_roots(&self.roots, &self.colors, &mut self.material);
		NewtonFractal::set_material_derivative_coeff(
			&self.derivative.get_coefficients(),
			&mut self.material,
		);
        true
    }

    pub fn set_max_iterations(&mut self, max_iterations: u32) {
        self.max_iterations = max_iterations;
        NewtonFractal::set_material_max_iter(self.max_iterations, &mut self.material);
    }
}
