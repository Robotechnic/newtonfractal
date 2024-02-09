use macroquad::{material::Material, math::Vec2, miniquad::error};
use num_complex::Complex;

use crate::{FractalShader, Polynomial};

pub struct NewtonFractal {
    max_iterations: u32,
    roots: Vec<Vec2>,
    colors: Vec<[f32; 3]>,
    derivative: Polynomial,
    material: FractalShader,
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

    pub fn new(
        roots: Vec<Vec2>,
        colors: Vec<[f32; 3]>,
        max_iterations: u32,
        real_range: Vec2,
        imag_range: Vec2,
    ) -> Option<Self> {
        if roots.len() != colors.len() {
            return None;
        }
        let polynomial = NewtonFractal::polynomial_from_roots(&roots);
        let derivative = polynomial.derivative();
        let mut material = FractalShader::new(roots.len())?;
        material.set_material_roots(&roots, &colors);
        material.set_material_derivative_coeff(derivative.get_coefficients());
        material.set_material_max_iter(max_iterations);

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
        self.material.get_material()
    }

    pub fn add_root(&mut self, root: Vec2, color: [f32; 3]) {
        let material = FractalShader::new(self.roots.len() + 1);
        if material.is_none() {
            error!("Failed to create material");
            return;
        }
        self.material = material.unwrap();
        self.roots.push(root);
        self.colors.push(color);
        self.update();
    }

    pub fn remove_root(&mut self, index: usize) {
        let material = FractalShader::new(self.roots.len() - 1);
        if material.is_none() {
            error!("Failed to create material");
            return;
        }
        self.material = material.unwrap();
        self.roots.remove(index);
        self.colors.remove(index);
        self.update();
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
		let polynomial = NewtonFractal::polynomial_from_roots(&self.roots);
		self.derivative = polynomial.derivative();
		self.material.set_material_range(self.real_range, self.imag_range);
        self.material.set_material_roots(&self.roots, &self.colors);
        self.material.set_material_derivative_coeff(self.derivative.get_coefficients());
        self.material.set_material_max_iter(self.max_iterations);
    }

    pub fn len(&self) -> usize {
        self.roots.len()
    }

    pub fn is_empty(&self) -> bool {
        self.roots.is_empty()
    }
}
