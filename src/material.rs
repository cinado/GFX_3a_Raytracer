use std::rc::Rc;

use serde::{Deserialize, Deserializer};

use crate::{deserialization_helpers::deserialize_color, vec3::Color};

pub trait Material {
    fn get_color(&self) -> Color;
}

#[derive(Deserialize)]
pub struct MaterialSolid {
    #[serde(deserialize_with = "deserialize_color")]
    pub color: Color,
    pub phong: Phong,
    pub reflectance: Reflectance,
    pub transmittance: Transmittance,
    pub refraction: Refraction,
}

impl MaterialSolid {
    pub fn new() -> Self {
        Self {
            color: Color::from_values(1., 0.0, 0.0),
            phong: Phong {
                ka: 1.0,
                kd: 1.0,
                ks: 1.0,
                exponent: 1.0,
            },
            reflectance: Reflectance { r: 20.0 },
            transmittance: Transmittance { t: 15.0 },
            refraction: Refraction { iof: 1.0 },
        }
    }
}

impl Material for MaterialSolid {
    fn get_color(&self) -> Color {
        self.color.clone()
    }
}

#[derive(Deserialize)]
pub struct MaterialTextured {
    pub texture: Texture,
    pub phong: Phong,
    pub reflectance: Reflectance,
    pub transmittance: Transmittance,
    pub refraction: Refraction,
}

impl Material for MaterialTextured {
    fn get_color(&self) -> Color {
        Color::new()
    }
}

#[derive(Deserialize)]
pub struct Phong {
    #[serde(rename = "@ka")]
    pub ka: f32,
    #[serde(rename = "@kd")]
    pub kd: f32,
    #[serde(rename = "@ks")]
    pub ks: f32,
    #[serde(rename = "@exponent")]
    pub exponent: f32,
}

#[derive(Deserialize)]
pub struct Reflectance {
    #[serde(rename = "@r")]
    pub r: f32,
}

#[derive(Deserialize)]
pub struct Transmittance {
    #[serde(rename = "@t")]
    pub t: f32,
}

#[derive(Deserialize)]
pub struct Refraction {
    #[serde(rename = "@iof")]
    pub iof: f32,
}

#[derive(Deserialize)]
pub struct Texture {
    #[serde(rename = "@name")]
    pub name: String,
}

#[derive(Deserialize)]
pub enum MaterialEnum {
    #[serde(rename = "material_solid")]
    Solid(MaterialSolid),
    #[serde(rename = "material_textured")]
    Textured(MaterialTextured),
}

pub fn deserialize_material<'de, D>(deserializer: D) -> Result<Rc<dyn Material>, D::Error>
where
    D: Deserializer<'de>,
{
    let material: MaterialEnum = Deserialize::deserialize(deserializer)?;
    match material {
        MaterialEnum::Solid(material_solid) => Ok(Rc::new(material_solid) as Rc<dyn Material>),
        MaterialEnum::Textured(material_textured) => {
            Ok(Rc::new(material_textured) as Rc<dyn Material>)
        }
    }
}