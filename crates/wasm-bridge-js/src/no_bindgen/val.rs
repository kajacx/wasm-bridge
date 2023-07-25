#[derive(Debug, Clone)]
pub enum Val {
    I32(i32),
    I64(i64),

    // raw values, use f32::to_bits to fill it
    F32(u32),
    F64(u64),
}

impl Val {
    pub fn i32(&self) -> Option<i32> {
        match self {
            Self::I32(val) => Some(*val),
            // Can be f64, because we can't tell from JS's number type
            // TODO: check for overflows
            Self::F64(val) => Some(f64::from_bits(*val) as _),
            _ => None,
        }
    }
    pub fn i64(&self) -> Option<i64> {
        match self {
            Self::I64(val) => Some(*val),
            _ => None,
        }
    }
    pub fn f32(&self) -> Option<f32> {
        match self {
            Self::F32(val) => Some(f32::from_bits(*val)),
            // Can be f64, because we can't tell from JS's number type
            // TODO: check for overflows
            Self::F64(val) => Some(f64::from_bits(*val) as _),
            _ => None,
        }
    }

    pub fn f64(&self) -> Option<f64> {
        match self {
            Self::F64(val) => Some(f64::from_bits(*val)),
            _ => None,
        }
    }
}

impl From<i32> for Val {
    fn from(value: i32) -> Self {
        Self::I32(value)
    }
}

impl From<i64> for Val {
    fn from(value: i64) -> Self {
        Self::I64(value)
    }
}

impl From<f32> for Val {
    fn from(value: f32) -> Self {
        Self::F32(value.to_bits())
    }
}

impl From<f64> for Val {
    fn from(value: f64) -> Self {
        Self::F64(value.to_bits())
    }
}
