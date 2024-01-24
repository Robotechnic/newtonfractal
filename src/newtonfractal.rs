use std::process::exit;

use macroquad::{
    material::{load_material, Material, MaterialParams},
    math::{vec2, Vec2},
    miniquad::{ShaderError, UniformType},
};
use num_complex::Complex;

use crate::Polynomial;

const VERTEX: &str = include_str!("shaders/vertex.vert");

const FRAGMENT_HEADER: &str = "
#version 400

#define cx_mul(a, b) vec2(a.x*b.x-a.y*b.y, a.x*b.y+a.y*b.x)
#define cx_div(a, b) vec2(((a.x*b.x+a.y*b.y)/(b.x*b.x+b.y*b.y)),((a.y*b.x-a.x*b.y)/(b.x*b.x+b.y*b.y)))

precision highp float;
in vec2 complex;

uniform int maxIterations;

vec2 cx_pow(vec2 a, float n) {
    float angle = atan(a.y, a.x);
    float r = length(a);
    float real = pow(r, n) * cos(n*angle);
    float im = pow(r, n) * sin(n*angle);
    return vec2(real, im);
}
";

const FRAGMENT_FOOTER: &str = "
void newton(inout vec2 z) {
	z = z - cx_div(evaluate_polynomial(z), evaluate_derivative(z));
}

void iterate(inout vec2 z) {
	for (int i = 0; i < maxIterations; i++) {
		newton(z);
	}
}

void main() {
	vec2 z = complex;
	iterate(z);
	closestRoot(z, gl_FragColor);
}
";

pub struct NewtonFractal {
    max_iterations: u32,
    roots: Vec<Vec2>,
    colors: Vec<[f32; 3]>,
    derivative: Polynomial,
    material: Material,
	real_range: Vec2,
	imag_range: Vec2,
}

impl NewtonFractal {
    fn polynomial_from_roots(roots: &[Vec2]) -> Polynomial {
        let mut poly = Polynomial::new(vec![Complex::new(1.0, 0.0)]);
        for root in roots {
            poly.add_root(Complex::new(root.x, root.y));
        }
        poly
    }

	fn build_uniforms(len: usize) -> String {
		let mut result = String::new();
		for i in 0..len {
			result.push_str(format!("uniform vec2 root{};\n", i).as_str());
			result.push_str(format!("uniform vec3 color{};\n", i).as_str());
			result.push_str(format!("uniform vec2 dcoeff{};\n", i).as_str());
		}
		result
	}

	fn build_closes_root(len: usize) -> String {
		let mut result = String::new();
		result.push_str("void closestRoot(vec2 z, out vec4 color) {\n");
		for i in 0..len {
			result.push_str(format!("float dist{} = length(z - root{});\n", i, i).as_str());
		}

		for i in 0..len {
			if i != len - 1 {
				result.push_str("if (");
				for j in (i + 1)..len {
					result.push_str(format!("dist{} < dist{}", i, j).as_str());
					if j != len - 1 {
						result.push_str(" && ");
					}
				}
				result.push_str(")");
			}
			result.push_str(format!("{{\ncolor = vec4(color{}, 1.0);\n", i).as_str());
			result.push_str("}\n");
			if i != len - 1 {
				result.push_str("else ");
			}
		}
		result.push_str("}\n");
		result
	}

	fn build_evaluate_polynomial(len: usize) -> String {
		let mut result = "vec2 evaluate_polynomial(vec2 z) {\nreturn ".to_string();

		for _ in 0..len - 1 {
			result.push_str("cx_mul(");
		}

		result.push_str("(z - root0)");
		for i in 1..len {
			result.push_str(format!(", (z - root{}))", i).as_str());
		}

		result.push_str(";\n}\n");
		result
	}

	fn build_derivative_evaluation(len: usize) -> String {
		let mut result = String::new();
		result.push_str("vec2 evaluate_derivative(vec2 z) {\nreturn ");
		for i in 0..len {
			if i != 0 {
				result.push_str("+");
			}
			if i == len - 1 {
				result.push_str(format!("dcoeff{}", i).as_str());
			} else if i == len - 2 {
				result.push_str(format!("cx_mul(dcoeff{}, z)", i).as_str());
			} else {
				result.push_str(format!("cx_mul(dcoeff{}, cx_pow(z, {}))", i, len - i - 1).as_str());
			}
		}
		result.push_str(";\n}\n");
		result
	}

	fn build_fragment_shader(len: usize) -> String {
		let mut result = String::new();
		result.push_str(FRAGMENT_HEADER);
		result.push_str(NewtonFractal::build_uniforms(len).as_str());
		result.push_str(NewtonFractal::build_closes_root(len).as_str());
		result.push_str(NewtonFractal::build_evaluate_polynomial(len).as_str());
		result.push_str(NewtonFractal::build_derivative_evaluation(len).as_str());
		result.push_str(FRAGMENT_FOOTER);
		result
	}

    fn create_material(len: usize) -> Material {
        let mut params = Vec::new();
        params.push(("maxIterations".to_owned(), UniformType::Int1));
		params.push(("realRange".to_owned(), UniformType::Float2));
		params.push(("imagRange".to_owned(), UniformType::Float2));
        for i in 0..len {
            params.push((format!("root{}", i), UniformType::Float2));
            params.push((format!("color{}", i), UniformType::Float3));
            params.push((format!("dcoeff{}", i), UniformType::Float2));
        }
        let material = load_material(
			VERTEX,
			&NewtonFractal::build_fragment_shader(len),
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

    pub fn new(roots: Vec<Vec2>, colors: Vec<[f32; 3]>, max_iterations: u32, real_range: Vec2, imag_range: Vec2) -> Option<Self> {
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
			real_range,
			imag_range,
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

	fn set_material_range(real_range: Vec2, imag_range: Vec2, material: &Material) {
		material.set_uniform("realRange", real_range);
		material.set_uniform("imagRange", imag_range);
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

	pub fn get_real_range(&self) -> Vec2 {
		self.real_range
	}

	pub fn get_imag_range(&self) -> Vec2 {
		self.imag_range
	}

	pub fn get_real_range_mut(&mut self) -> &mut Vec2 {
		&mut self.real_range
	}

	pub fn get_imag_range_mut(&mut self) -> &mut Vec2 {
		&mut self.imag_range
	}

	pub fn update(&mut self) {
		NewtonFractal::set_material_roots(&self.roots, &self.colors, &self.material);
		NewtonFractal::set_material_derivative_coeff(
			self.derivative.get_coefficients(),
			&self.material,
		);
		NewtonFractal::set_material_max_iter(self.max_iterations, &self.material);
		NewtonFractal::set_material_range(self.real_range, self.imag_range, &self.material);
	}
}
