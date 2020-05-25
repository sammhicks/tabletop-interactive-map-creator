use std::rc::Rc;

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
pub struct Material {
    name: String,
    pub href: String,
    pub size: usize,
}

impl Material {
    pub fn get_name(&self) -> String {
        format!("tile{}", self.name)
    }

    pub fn as_url(&self) -> String {
        format!("url(#tile{})", self.name)
    }
}

impl std::fmt::Display for Material {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)
    }
}

#[derive(Clone, Debug, Default)]
pub struct Materials(Rc<Vec<Material>>);

impl Materials {
    pub fn as_vec(&self) -> Vec<Material> {
        self.0.as_ref().clone()
    }
}

impl std::ops::Deref for Materials {
    type Target = [Material];
    fn deref(&self) -> &[Material] {
        self.0.deref()
    }
}

impl<'de> serde::Deserialize<'de> for Materials {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self(Rc::new(Vec::<Material>::deserialize(deserializer)?)))
    }
}
