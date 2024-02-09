use macroquad::{
    material::{load_material, Material, MaterialParams},
    math::{vec2, Vec2},
    miniquad::{error, ShaderError, UniformType},
};
use num_complex::Complex;

pub struct FractalShader {
    shader: Material,
}

const VERTEX: &str = include_str!("shaders/vertex.vert");

const FRAGMENT_HEADER: &str = "#version 300 es

#define cx_mul(a, b) vec2(a.x*b.x-a.y*b.y, a.x*b.y+a.y*b.x)
#define cx_div(a, b) vec2(((a.x*b.x+a.y*b.y)/(b.x*b.x+b.y*b.y)),((a.y*b.x-a.x*b.y)/(b.x*b.x+b.y*b.y)))

precision highp float;
in vec2 complex;
out vec4 fragColor;

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
	closestRoot(z, fragColor);
}
";

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
            result.push(')');
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
            result.push('+');
        }
        if i == len - 1 {
            result.push_str(format!("dcoeff{}", i).as_str());
        } else if i == len - 2 {
            result.push_str(format!("cx_mul(dcoeff{}, z)", i).as_str());
        } else {
            result.push_str(format!("cx_mul(dcoeff{}, cx_pow(z, {}.0))", i, len - i - 1).as_str());
        }
    }
    result.push_str(";\n}\n");
    result
}

fn build_fragment_shader(len: usize) -> String {
    let mut result = String::new();
    result.push_str(FRAGMENT_HEADER);
    result.push_str(build_uniforms(len).as_str());
    result.push_str(build_closes_root(len).as_str());
    result.push_str(build_evaluate_polynomial(len).as_str());
    result.push_str(build_derivative_evaluation(len).as_str());
    result.push_str(FRAGMENT_FOOTER);
    result
}

fn create_material(len: usize) -> Option<Material> {
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
        build_fragment_shader(len).as_str(),
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
                error!("Error in {:?} shader:\n{}", shader_type, error_message);
            }
            err => error!("{:?}", err),
        }
        return None;
    }
    Some(material.unwrap())
}

impl FractalShader {
    pub fn new(nb_roots: usize) -> Option<Self> {
        let material = create_material(nb_roots)?;
        Some(Self { shader: material })
    }

    pub fn get_material(&self) -> &Material {
        &self.shader
    }

    pub fn set_material_roots(&mut self, roots: &[Vec2], colors: &[[f32; 3]]) {
        for i in 0..roots.len() {
            self.shader
                .set_uniform(format!("root{}", i).as_str(), roots[i]);
            self.shader
                .set_uniform(format!("color{}", i).as_str(), colors[i]);
        }
    }

    pub fn set_material_derivative_coeff(&mut self, coefs: &[Complex<f32>]) {
        for (i, coeff) in coefs.iter().enumerate() {
            self.shader
                .set_uniform(format!("dcoeff{}", i).as_str(), vec2(coeff.re, coeff.im));
        }
    }

    pub fn set_material_max_iter(&mut self, max_iter: u32) {
        self.shader.set_uniform("maxIterations", max_iter);
    }

    pub fn set_material_range(&mut self, real_range: Vec2, imag_range: Vec2) {
        self.shader.set_uniform("realRange", real_range);
        self.shader.set_uniform("imagRange", imag_range);
    }
}
