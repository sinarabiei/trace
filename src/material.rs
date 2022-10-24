use crate::color;
use crate::color::Color;
use crate::light::Light;
use crate::pattern::Pattern;
use crate::point::Point;
use crate::prelude::is_equal;
use crate::shape::Shape;
use crate::vector::Vector;

#[derive(Debug)]
pub struct Material {
    pub pattern: Option<Box<dyn Pattern>>,
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Material {
    pub fn new() -> Self {
        Self {
            pattern: None,
            color: color![1, 1, 1],
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }

    pub fn lighting(
        &self,
        object: &dyn Shape,
        light: Light,
        point: Point,
        eye: Vector,
        normal: Vector,
        in_shadow: bool,
    ) -> Color {
        let color = match &self.pattern {
            Some(pattern) => pattern.at_object(object, point),
            None => self.color,
        };
        let effective_color = color * light.intensity;
        let light_vector = (light.position - point).normalize();
        let ambient = effective_color * self.ambient;
        let light_dot_normal = light_vector.dot(normal);
        let diffuse: Color;
        let specular: Color;
        if light_dot_normal < 0.0 || in_shadow {
            diffuse = color![0, 0, 0];
            specular = color![0, 0, 0];
        } else {
            diffuse = effective_color * self.diffuse * light_dot_normal;
            let reflect_vector = (-light_vector).reflect(normal);
            let reflect_dot_eye = reflect_vector.dot(eye);
            if reflect_dot_eye < 0.0 || is_equal(reflect_dot_eye, 0.0) {
                specular = color![0, 0, 0];
            } else {
                let factor = reflect_dot_eye.powf(self.shininess);
                specular = light.intensity * self.specular * factor;
            }
        }
        ambient + diffuse + specular
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            pattern: None,
            color: color![1, 1, 1],
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

impl PartialEq for Material {
    fn eq(&self, rhs: &Self) -> bool {
        self.color == rhs.color
            && is_equal(self.ambient, rhs.ambient)
            && is_equal(self.diffuse, rhs.diffuse)
            && is_equal(self.specular, rhs.specular)
            && is_equal(self.shininess, rhs.shininess)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pattern::stripe::Stripe;
    use crate::sphere::Sphere;
    use crate::{color, point, vector};
    use std::f64::consts::SQRT_2;

    #[test]
    fn test_lighting() {
        // Lighting with the eye between the light and the surface
        let material = Material::new();
        let position = Point::zero();
        let eye = vector![0, 0, -1];
        let normal = vector![0, 0, -1];
        let light = Light {
            position: point![0, 0, -10],
            intensity: color![1, 1, 1],
        };
        let in_shadow = false;
        let object = Sphere::new();
        assert_eq!(
            material.lighting(&object, light, position, eye, normal, in_shadow),
            color![1.9, 1.9, 1.9]
        );

        // Lighting with the eye between light and surface, eye offset 45 degrees
        let material = Material::new();
        let position = Point::zero();
        let eye = vector![0, SQRT_2 / 2.0, -SQRT_2 / 2.0];
        let normal = vector![0, 0, -1];
        let light = Light {
            position: point![0, 0, -10],
            intensity: color![1, 1, 1],
        };
        let in_shadow = false;
        let object = Sphere::new();
        assert_eq!(
            material.lighting(&object, light, position, eye, normal, in_shadow),
            color![1, 1, 1]
        );

        // Lighting with eye opposite surface, light offset 45 degrees
        let material = Material::new();
        let position = Point::zero();
        let eye = vector![0, 0, -1];
        let normal = vector![0, 0, -1];
        let light = Light {
            position: point![0, 10, -10],
            intensity: color![1, 1, 1],
        };
        let in_shadow = false;
        let object = Sphere::new();
        assert_eq!(
            material.lighting(&object, light, position, eye, normal, in_shadow),
            color![0.7364, 0.7364, 0.7364]
        );

        // Lighting with eye in the path of the reflection vector
        let material = Material::new();
        let position = Point::zero();
        let eye = vector![0, -SQRT_2 / 2.0, -SQRT_2 / 2.0];
        let normal = vector![0, 0, -1];
        let light = Light {
            position: point![0, 10, -10],
            intensity: color![1, 1, 1],
        };
        let in_shadow = false;
        let object = Sphere::new();
        assert_eq!(
            material.lighting(&object, light, position, eye, normal, in_shadow),
            color![1.6364, 1.6364, 1.6364]
        );

        // Lighting with the light behind the surface
        let material = Material::new();
        let position = Point::zero();
        let eye = vector![0, 0, -1];
        let normal = vector![0, 0, -1];
        let light = Light {
            position: point![0, 0, 10],
            intensity: color![1, 1, 1],
        };
        let in_shadow = false;
        let object = Sphere::new();
        assert_eq!(
            material.lighting(&object, light, position, eye, normal, in_shadow),
            color![0.1, 0.1, 0.1]
        );

        // Lighting with the surface in shadow
        let material = Material::new();
        let eyev = vector![0, 0, -1];
        let normalv = vector![0, 0, -1];
        let light = Light {
            position: point![0, 0, -10],
            intensity: color![1, 1, 1],
        };
        let in_shadow = true;
        let object = Sphere::new();
        assert_eq!(
            material.lighting(&object, light, position, eyev, normalv, in_shadow),
            color![0.1, 0.1, 0.1]
        );

        // Lighting with a pattern applied
        let mut material = Material::new();
        material.pattern = Some(Box::new(Stripe::default()));
        material.ambient = 1.0;
        material.diffuse = 0.0;
        material.specular = 0.0;
        let eyev = vector![0, 0, -1];
        let normalv = vector![0, 0, -1];
        let light = Light {
            position: point![0, 0, -10],
            intensity: color![1, 1, 1],
        };
        let object = Sphere::new();
        assert_eq!(
            material.lighting(&object, light, point![0.9, 0, 0], eyev, normalv, false),
            Color::WHITE
        );
        assert_eq!(
            material.lighting(&object, light, point![1.1, 0, 0], eyev, normalv, false),
            Color::BLACK
        );
    }
}
