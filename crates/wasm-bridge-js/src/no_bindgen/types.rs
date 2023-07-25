#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct FuncType {
    params: Vec<ValType>,
    results: Vec<ValType>,
}

impl FuncType {
    pub fn new(
        params: impl IntoIterator<Item = ValType>,
        results: impl IntoIterator<Item = ValType>,
    ) -> Self {
        Self {
            params: params.into_iter().collect(),
            results: results.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum ValType {
    I32,
    I64,
    F32,
    F64,
}
