#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StructName {
    pub package_name: String,
    pub simple_name: String,
}

impl StructName {

}

pub struct StructNameBuilder {

}

impl StructNameBuilder {
  pub fn struct_builder(name: &'static str) {}
  pub fn object_builder(name: &'static str) {}
  pub fn interface_builder(name: &'static str) {}
  pub fn enum_builder(name: &'static str) {}
}
