enum Scalar {
    Float(f64),
    Int(i32),
    Bool(bool),
}

struct DType {
    priority: i32,
    itemsize: i32,
    name: String,
    fmt: Option<i32>,
    count: i32,
}

impl DType {
    fn repr() {}
    fn vec() {}
    fn scalar() {}
    fn np() {}
}

struct ImageDType;
impl ImageDType {}

struct PtrDTyp;
impl PtrDTyp {
    fn init() {}
    fn repr() {}
    fn hash() {}
    fn eq() {}
    fn ne() {}
}

fn cast_scalar(scalar: Scalar, dtype: DType) {}

struct dtypes;
impl dtypes {
    fn is_float() {}
    fn is_init() {}
    fn is_unsigned() {}
    fn from_np() {}
    fn from_py() {}
    fn fields() {}
}

fn _get_recursive_parents(dtype: DType) {}

fn least_upper_dtype(dtype: DType) {}

fn least_upper_float(dtype: DType) {}
