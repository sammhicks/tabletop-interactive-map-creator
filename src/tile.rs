use std::rc::Rc;

#[derive(Debug, serde::Deserialize)]
pub struct MaterialInner {
    name: String,
    href: String,
    size: usize,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Material(Rc<MaterialInner>);

impl Material {
    pub fn name(&self) -> String {
        format!("tile{}", self.0.name)
    }

    pub fn url_reference(&self) -> String {
        format!("url(#tile{})", self.0.name)
    }

    pub fn href(&self) -> String {
        self.0.href.clone()
    }

    pub fn size(&self) -> usize {
        self.0.size
    }
}

impl std::fmt::Display for Material {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.name)
    }
}

impl std::cmp::PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

#[derive(Clone, Debug, Default, serde::Deserialize)]
pub struct Materials(Rc<Vec<Material>>);

impl Materials {
    pub fn as_vec(&self) -> Vec<Material> {
        self.0.as_ref().clone()
    }
}

impl std::convert::AsRef<[Material]> for Materials {
    fn as_ref(&self) -> &[Material] {
        self.0.as_ref()
    }
}
