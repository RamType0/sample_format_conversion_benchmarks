use std::mem;

/// Format that each sample has.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SampleFormat {
    /// The value 0 corresponds to 0.
    I16,
    /// The value 0 corresponds to 32768.
    U16,
    /// The boundaries are (-1.0, 1.0).
    F32,
}

impl SampleFormat {
    /// Returns the size in bytes of a sample of this format.
    #[inline]
    pub fn sample_size(&self) -> usize {
        match *self {
            SampleFormat::I16 => mem::size_of::<i16>(),
            SampleFormat::U16 => mem::size_of::<u16>(),
            SampleFormat::F32 => mem::size_of::<f32>(),
        }
    }
}

/// Trait for containers that contain PCM data.
pub unsafe trait Sample: Copy + Clone {
    /// The `SampleFormat` corresponding to this data type.
    const FORMAT: SampleFormat;

    /// Turns the sample into its equivalent as a floating-point.
    fn to_f32_mine(&self) -> f32;
    /// Converts this sample into a standard i16 sample.
    fn to_i16_mine(&self) -> i16;
    /// Converts this sample into a standard u16 sample.
    fn to_u16_mine(&self) -> u16;

    /// Converts any sample type to this one by calling `to_i16`, `to_u16` or `to_f32`.
    fn from<S>(s: &S) -> Self
    where
        S: Sample;
}

unsafe impl Sample for u16 {
    const FORMAT: SampleFormat = SampleFormat::U16;

    #[inline]
    fn to_f32_mine(&self) -> f32 {
        self.to_i16_mine().to_f32_mine()
    }

    #[inline]
    fn to_i16_mine(&self) -> i16 {
        (*self as i16).wrapping_add(i16::MIN)
    }

    #[inline]
    fn to_u16_mine(&self) -> u16 {
        *self
    }

    #[inline]
    fn from<S>(sample: &S) -> Self
    where
        S: Sample,
    {
        sample.to_u16_mine()
    }
}

unsafe impl Sample for i16 {
    const FORMAT: SampleFormat = SampleFormat::I16;

    #[inline]
    fn to_f32_mine(&self) -> f32 {
        const POSITIVE_MULTIPLIER:f32 = 1.0 / i16::MAX as f32;
        const NEGATIVE_MULTIPLIER:f32 = 1.0 / -(i16::MIN as f32);
        const NEGATIVE_OFFSET:f32 =  NEGATIVE_MULTIPLIER - POSITIVE_MULTIPLIER;
        let sign_bit = (*self as u16 >> 15) as f32;
        let multiplier = NEGATIVE_OFFSET.mul_add(sign_bit, POSITIVE_MULTIPLIER);
        *self as f32 * multiplier
    }

    #[inline]
    fn to_i16_mine(&self) -> i16 {
        *self
    }

    #[inline]
    fn to_u16_mine(&self) -> u16 {
        self.wrapping_add(i16::MIN) as u16
    }

    #[inline]
    fn from<S>(sample: &S) -> Self
    where
        S: Sample,
    {
        sample.to_i16_mine()
    }
}
const F32_TO_16BIT_INT_MULTIPLIER:f32 = u16::MAX as f32 * 0.5;
unsafe impl Sample for f32 {
    const FORMAT: SampleFormat = SampleFormat::F32;

    #[inline]
    fn to_f32_mine(&self) -> f32 {
        *self
    }

    #[inline]
    fn to_i16_mine(&self) -> i16 {
        (*self * F32_TO_16BIT_INT_MULTIPLIER).floor() as i16
    }

    #[inline]
    fn to_u16_mine(&self) -> u16 {
        self.mul_add(F32_TO_16BIT_INT_MULTIPLIER,F32_TO_16BIT_INT_MULTIPLIER).round() as u16
    }

    #[inline]
    fn from<S>(sample: &S) -> Self
    where
        S: Sample,
    {
        sample.to_f32_mine()
    }
}

#[cfg(test)]
mod test {
    use super::Sample;

    #[test]
    fn i16_to_i16() {
        assert_eq!(0i16.to_i16_mine(), 0);
        assert_eq!((-467i16).to_i16_mine(), -467);
        assert_eq!(32767i16.to_i16_mine(), 32767);
        assert_eq!((-32768i16).to_i16_mine(), -32768);
    }

    #[test]
    fn i16_to_u16() {
        assert_eq!(0i16.to_u16_mine(), 32768);
        assert_eq!((-16384i16).to_u16_mine(), 16384);
        assert_eq!(32767i16.to_u16_mine(), 65535);
        assert_eq!((-32768i16).to_u16_mine(), 0);
    }

    #[test]
    fn i16_to_f32() {
        assert_eq!(0i16.to_f32_mine(), 0.0);
        assert_eq!((-16384i16).to_f32_mine(), -0.5);
        assert_eq!(32767i16.to_f32_mine(), 1.0);
        assert_eq!((-32768i16).to_f32_mine(), -1.0);
    }

    #[test]
    fn u16_to_i16() {
        assert_eq!(32768u16.to_i16_mine(), 0);
        assert_eq!(16384u16.to_i16_mine(), -16384);
        assert_eq!(65535u16.to_i16_mine(), 32767);
        assert_eq!(0u16.to_i16_mine(), -32768);
    }

    #[test]
    fn u16_to_u16() {
        assert_eq!(0u16.to_u16_mine(), 0);
        assert_eq!(467u16.to_u16_mine(), 467);
        assert_eq!(32767u16.to_u16_mine(), 32767);
        assert_eq!(65535u16.to_u16_mine(), 65535);
    }

    #[test]
    fn u16_to_f32() {
        assert_eq!(0u16.to_f32_mine(), -1.0);
        assert_eq!(32768u16.to_f32_mine(), 0.0);
        assert_eq!(65535u16.to_f32_mine(), 1.0);
    }

    #[test]
    fn f32_to_i16() {
        assert_eq!(0.0f32.to_i16_mine(), 0);
        assert_eq!((-0.5f32).to_i16_mine(), i16::MIN / 2);
        assert_eq!(1.0f32.to_i16_mine(), i16::MAX);
        assert_eq!((-1.0f32).to_i16_mine(), i16::MIN);
    }

    #[test]
    fn f32_to_u16() {
        assert_eq!((-1.0f32).to_u16_mine(), 0);
        assert_eq!(0.0f32.to_u16_mine(), 32768);
        assert_eq!(1.0f32.to_u16_mine(), 65535);
    }

    #[test]
    fn f32_to_f32() {
        assert_eq!(0.1f32.to_f32_mine(), 0.1);
        assert_eq!((-0.7f32).to_f32_mine(), -0.7);
        assert_eq!(1.0f32.to_f32_mine(), 1.0);
    }
}
