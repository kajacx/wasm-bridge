#[derive(Debug, Clone)]
pub enum Val {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
}

impl Val {
    pub fn i32(&self) -> Option<i32> {
        match self {
            Self::I32(val) => Some(*val),
            // Can be f64, because we can't tell from JS's number type
            // TODO: check for overflows
            Self::F64(val) => Some(*val as _),
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
            Self::F32(val) => Some(*val),
            // Can be f64, because we can't tell from JS's number type
            // TODO: check for overflows
            Self::F64(val) => Some(*val as _),
            _ => None,
        }
    }

    pub fn f64(&self) -> Option<f64> {
        match self {
            Self::F64(val) => Some(*val),
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
        Self::F32(value)
    }
}

impl From<f64> for Val {
    fn from(value: f64) -> Self {
        Self::F64(value)
    }
}
