//! A field defined by rotating another field.

use cgmath::{Point3, Vector3, Rotation, Basis3};

use field;

#[derive(Clone)]
#[allow(missing_docs)]
pub struct T<Field> {
  pub rotation: Basis3<f32>,
  pub field: Field,
}

impl<Field> field::T for T<Field> where Field: field::T {
  fn density(&mut self, p: &Point3<f32>) -> f32 {
    let p = self.rotation.invert().rotate_point(*p);
    field::T::density(&mut self.field, &p)
  }

  fn normal(&mut self, p: &Point3<f32>) -> Vector3<f32> {
    let p = self.rotation.invert().rotate_point(*p);
    field::T::normal(&mut self.field, &p)
  }
}
