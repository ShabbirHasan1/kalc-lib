use std::ops::{Deref, DerefMut};
pub struct Complex {
    pub real: f64,
    pub imag: f64,
}
pub struct Float(pub f64);
impl Deref for Float {
    type Target = f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Float {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
