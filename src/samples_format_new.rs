pub trait ToF32New{
    fn to_f32_new(&self) -> f32;
}
impl ToF32New for i16{
    fn to_f32_new(&self) -> f32 {
        if *self > 0{
            *self as f32 / i16::MAX as f32
        }else{
            *self as f32 / -(i16::MIN as f32)
        }
    }
}