use std::process::exit;

use macroquad::{
    material::{load_material, Material, MaterialParams},
    math::{vec2, Vec2},
    miniquad::{ShaderError, UniformType},
};
use num_complex::Complex;

use crate::Polynomial;

const VERTEX: &str = include_str!("shaders/vertex.vert");

pub struct NewtonFractal {
    max_iterations: u32,
    roots: Vec<Vec2>,
    colors: Vec<[f32; 3]>,
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
			VERTEX,
			include_str!("shaders/fractal.frag"),
            MaterialParams {
                uniforms: params,
                ..Default::default()
            },
        );

        if material.is_err() {
            match material.err().unwrap() {
                ShaderError::CompilationError {
                    shader_type,
                    error_message,
                } => {
                    println!("Error in {:?} shader:\n{}", shader_type, error_message);
                }
                err => println!("{:?}", err),
            }
            exit(1);
        }
        material.unwrap()
    }

    pub fn new(roots: Vec<Vec2>, colors: Vec<[f32; 3]>, max_iterations: u32) -> Option<Self> {
        if roots.len() != colors.len() {
            return None;
        }
        let polynomial = NewtonFractal::polynomial_from_roots(&roots);
        let derivative = polynomial.derivative();
        let material = NewtonFractal::create_material(roots.len());
        NewtonFractal::set_material_roots(&roots, &colors, &material);
        NewtonFractal::set_material_derivative_coeff(derivative.get_coefficients(), &material);
        NewtonFractal::set_material_max_iter(max_iterations, &material);

        Some(Self {
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

    fn set_material_roots(roots: &[Vec2], colors: &[[f32; 3]], material: &Material) {
        for i in 0..roots.len() {
            material.set_uniform(format!("root{}", i).as_str(), roots[i]);
            material.set_uniform(format!("color{}", i).as_str(), colors[i]);
        }
    }

    fn set_material_derivative_coeff(coefs: &[Complex<f32>], material: &Material) {
        for (i, coeff) in coefs.iter().enumerate() {
            material.set_uniform(format!("dcoeff{}", i).as_str(), vec2(coeff.re, coeff.im));
        }
    }

    fn set_material_max_iter(max_iter: u32, material: &Material) {
        material.set_uniform("maxIterations", max_iter);
    }

    pub fn set_roots(&mut self, roots: Vec<Vec2>, colors: Vec<[f32; 3]>) -> bool {
        if roots.len() != colors.len() {
            return false;
        }
		if self.roots.len() != roots.len() {
			self.material = NewtonFractal::create_material(roots.len());
		}
        self.roots = roots;
        self.colors = colors;
        let polynomial = NewtonFractal::polynomial_from_roots(&self.roots);
        self.derivative = polynomial.derivative();
        self.update();
        true
    }

    pub fn set_max_iterations(&mut self, max_iterations: u32) {
        self.max_iterations = max_iterations;
    }

	pub fn get_roots(&mut self) -> &mut [Vec2] {
		&mut self.roots
	}

	pub fn get_colors(&mut self) -> &mut [[f32; 3]] {
		&mut self.colors
	}

	pub fn get_colored_roots(&mut self) -> impl Iterator<Item = (&mut Vec2, &mut [f32; 3])> {
		self.roots.iter_mut().zip(self.colors.iter_mut())
	}

	pub fn update(&mut self) {
		NewtonFractal::set_material_roots(&self.roots, &self.colors, &self.material);
		NewtonFractal::set_material_derivative_coeff(
			self.derivative.get_coefficients(),
			&self.material,
		);
		NewtonFractal::set_material_max_iter(self.max_iterations, &self.material);
	}
}
