use crate::{color::Color, utils::FuzzyEq};

pub trait Illuminated {}

#[derive(Clone, Copy, Debug)]
pub struct Phong {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Illuminated for Phong {}

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

pub enum PhongAttribute {
    Color(Color),
    Ambient(f64),
    Diffuse(f64),
    Specular(f64),
    Shininess(f64),
}

impl Phong {
    pub fn new(attrs: &[PhongAttribute]) -> Self {
        let mut res = Phong::default();
        for attr in attrs {
            match *attr {
                PhongAttribute::Color(c) => res.color = c,
                PhongAttribute::Ambient(a) => res.ambient = a,
                PhongAttribute::Diffuse(d) => res.diffuse = d,
                PhongAttribute::Specular(s) => res.specular = s,
                PhongAttribute::Shininess(s) => res.shininess = s,
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
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
        let m = Phong::new(&[
            PhongAttribute::Ambient(0.2),
            PhongAttribute::Diffuse(1.0),
            PhongAttribute::Specular(0.7),
            PhongAttribute::Ambient(0.4),
        ]);

        assert_fuzzy_eq!(Color::new(1.0, 1.0, 1.0), m.color);
        assert_fuzzy_eq!(0.4, m.ambient);
        assert_fuzzy_eq!(1.0, m.diffuse);
        assert_fuzzy_eq!(0.7, m.specular);
        assert_fuzzy_eq!(200.0, m.shininess);
    }
}
