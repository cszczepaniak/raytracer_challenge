use crate::{color::Color, light::PointLight, point::Point, utils::FuzzyEq, vector::Vector};

pub trait Illuminated {
    fn lighting(
        &self,
        light: &PointLight,
        position: Point,
        eye_vector: Vector,
        normal_vector: Vector,
        shadow_state: ShadowState,
    ) -> Color;
}

#[derive(Clone, Copy, Debug)]
pub enum Material {
    Phong(Phong),
}

impl Illuminated for Material {
    fn lighting(
        &self,
        light: &PointLight,
        position: Point,
        eye_vector: Vector,
        normal_vector: Vector,
        shadow_state: ShadowState,
    ) -> Color {
        match self {
            Material::Phong(p) => {
                p.lighting(light, position, eye_vector, normal_vector, shadow_state)
            }
        }
    }
}

impl From<Phong> for Material {
    fn from(p: Phong) -> Self {
        Material::Phong(p)
    }
}

impl Default for Material {
    fn default() -> Self {
        Material::Phong(Phong::default())
    }
}

impl FuzzyEq for Material {
    fn fuzzy_eq(&self, other: Self) -> bool {
        match self {
            Material::Phong(p) => match other {
                Material::Phong(op) => p.fuzzy_eq(op),
            },
        }
    }
}

pub enum ShadowState {
    Shadow,
    Clear,
}

#[derive(Clone, Copy, Debug)]
pub struct Phong {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Illuminated for Phong {
    fn lighting(
        &self,
        light: &PointLight,
        position: Point,
        eye_vector: Vector,
        normal_vector: Vector,
        shadow_state: ShadowState,
    ) -> Color {
        let ambient_light: Color;
        let diffuse_light: Color;
        let specular_light: Color;

        let effective_color = self.color * light.intensity;
        let light_vector = (light.position - position).normalize();

        ambient_light = effective_color * self.ambient;

        if let ShadowState::Shadow = shadow_state {
            return ambient_light;
        }

        let light_dot_normal = light_vector.dot(&normal_vector);
        if light_dot_normal < 0.0 {
            // Light is on the other side of the surface
            diffuse_light = Color::new(0.0, 0.0, 0.0);
            specular_light = Color::new(0.0, 0.0, 0.0);
        } else {
            // Light is on the side the surface is pointing to
            diffuse_light = effective_color * self.diffuse * light_dot_normal;

            let reflect_vector = -light_vector.reflect(normal_vector);
            let reflect_dot_eye = reflect_vector.dot(&eye_vector);

            if reflect_dot_eye <= 0.0 {
                specular_light = Color::new(0.0, 0.0, 0.0);
            } else {
                let factor = reflect_dot_eye.powf(self.shininess);
                specular_light = light.intensity * self.specular * factor;
            }
        }

        ambient_light + diffuse_light + specular_light
    }
}

impl Default for Phong {
    fn default() -> Self {
        Phong {
            color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

impl FuzzyEq for Phong {
    fn fuzzy_eq(&self, other: Self) -> bool {
        self.color.fuzzy_eq(other.color)
            && self.ambient.fuzzy_eq(other.ambient)
            && self.diffuse.fuzzy_eq(other.diffuse)
            && self.specular.fuzzy_eq(other.specular)
            && self.shininess.fuzzy_eq(other.shininess)
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::FRAC_1_SQRT_2;

    use crate::assert_fuzzy_eq;
    use crate::utils::FuzzyEq;

    use super::*;

    #[test]
    fn default_phong_material() {
        let m = Phong::default();

        assert_fuzzy_eq!(Color::new(1.0, 1.0, 1.0), m.color);
        assert_fuzzy_eq!(0.1, m.ambient);
        assert_fuzzy_eq!(0.9, m.diffuse);
        assert_fuzzy_eq!(0.9, m.specular);
        assert_fuzzy_eq!(200.0, m.shininess);
    }

    #[test]
    fn phong_material_can_be_constructed_with_properties() {
        let m = Phong {
            ambient: 0.2,
            diffuse: 1.0,
            specular: 0.7,
            ..Phong::default()
        };

        assert_fuzzy_eq!(Color::new(1.0, 1.0, 1.0), m.color);
        assert_fuzzy_eq!(0.2, m.ambient);
        assert_fuzzy_eq!(1.0, m.diffuse);
        assert_fuzzy_eq!(0.7, m.specular);
        assert_fuzzy_eq!(200.0, m.shininess);
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface() {
        let m = Phong::default();
        let position = Point::new(0.0, 0.0, 0.0);

        let eye = Vector::new(0.0, 0.0, -1.0);
        let normal = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));

        assert_fuzzy_eq!(
            Color::new(1.9, 1.9, 1.9),
            m.lighting(&light, position, eye, normal, ShadowState::Clear)
        )
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface_eye_offset_by_45_degrees() {
        let m = Phong::default();
        let position = Point::new(0.0, 0.0, 0.0);

        let eye = Vector::new(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2);
        let normal = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));

        assert_fuzzy_eq!(
            Color::new(1.0, 1.0, 1.0),
            m.lighting(&light, position, eye, normal, ShadowState::Clear)
        )
    }

    #[test]
    fn lighting_with_the_eye_opposite_surface_light_offset_by_45_degrees() {
        let m = Phong::default();
        let position = Point::new(0.0, 0.0, 0.0);

        let eye = Vector::new(0.0, 0.0, -1.0);
        let normal = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let actual_result = m.lighting(&light, position, eye, normal, ShadowState::Clear);

        let expected_result = Color::new(0.7364, 0.7364, 0.7364);

        assert_fuzzy_eq!(expected_result, actual_result);
    }

    #[test]
    fn lighting_with_the_eye_in_path_of_the_reflection_vector() {
        let m = Phong::default();
        let position = Point::new(0.0, 0.0, 0.0);

        let eye = Vector::new(0.0, -FRAC_1_SQRT_2, -FRAC_1_SQRT_2);
        let normal = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let actual_result = m.lighting(&light, position, eye, normal, ShadowState::Clear);

        let expected_result = Color::new(1.6364, 1.6364, 1.6364);

        assert_fuzzy_eq!(expected_result, actual_result);
    }

    #[test]
    fn lighting_with_light_behind_the_surface() {
        let m = Phong::default();
        let position = Point::new(0.0, 0.0, 0.0);

        let eye = Vector::new(0.0, 0.0, -1.0);
        let normal = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0));

        let actual_result = m.lighting(&light, position, eye, normal, ShadowState::Clear);

        let expected_result = Color::new(0.1, 0.1, 0.1);

        assert_fuzzy_eq!(actual_result, expected_result);
    }

    #[test]
    fn lighting_with_surface_in_shadow() {
        let m = Phong {
            ambient: 0.22,
            ..Phong::default()
        };
        let position = Point::new(0.0, 0.0, 0.0);

        let eye = Vector::new(0.0, 0.0, -1.0);
        let normal = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));

        assert_fuzzy_eq!(
            Color::new(0.22, 0.22, 0.22),
            m.lighting(&light, position, eye, normal, ShadowState::Shadow)
        )
    }
}
