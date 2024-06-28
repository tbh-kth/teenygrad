// Helper functions and data types
#![allow(dead_code)]

pub static _OSX: bool = cfg!(target_os = "macos");

fn prod<T: std::ops::Mul<Output = T> + Default>(iter: impl Iterator<Item = T>) -> T {
    iter.fold(T::default(), |acc, x| acc * x)
}

// God save me from satan's deduplicate function.
pub fn dedup<T: std::cmp::PartialEq>(x: Vec<T>) -> Vec<T> {
    let mut result = vec![];
    for item in x {
        if !result.contains(&item) {
            result.push(item);
        }
    }
    result
}

// idk cant make it work. i will work on it later when we are doing tensor.rs
//pub fn argfix<T: Clone>(x: Vec<T>) -> Vec<T> {
//    if x[0].length > 1 {
//        vec![x[0].clone()] // Then flatten it idk
//    } else {
//        x
//    }
//}

pub fn make_pair<T: Clone>(mut x: Vec<T>, cint: T) -> Vec<T> {
    if all_int(x.clone()) {
        x.push(cint);
    }
    x
}

pub fn flatten<T>(l: Vec<Vec<T>>) -> Vec<T> {
    l.into_iter().flat_map(|x| x.into_iter()).collect()
}

// Maybe I can make it smaller
// https://stackoverflow.com/questions/69764050/how-to-get-the-indices-that-would-sort-a-vec
pub fn argsort<T: Ord>(x: Vec<T>) -> Vec<usize> {
    let mut indices = (0..x.len()).collect::<Vec<_>>();
    indices.sort_by_key(|&i| &x[i]);
    indices
}

// Check if we need to allow unsigned int here
fn all_int<T>(_: Vec<T>) -> bool {
    ["i8", "i16", "i32", "i64"].contains(&std::any::type_name::<T>())
}

pub fn round_up(num: i32, amt: i32) -> i32 {
    (num + amt - 1) / amt * amt
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct DType {
    pub priority: i32,
    pub itemsize: i32,
    pub name: &'static str,
    pub fmt: char,
    pub count: i32,
}

impl std::fmt::Debug for DType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "dtypes.{}", self.name)
    }
}
impl DType {
    pub fn is_int(&self) -> bool {
        matches!(
            *self,
            INT8 | INT16 | INT32 | INT64 | UINT8 | UINT16 | UINT32 | UINT64
        )
    }

    pub fn is_float(&self) -> bool {
        matches!(*self, FLOAT16 | FLOAT32 | FLOAT64)
    }

    pub fn is_unsigned(&self) -> bool {
        matches!(*self, UINT8 | UINT16 | UINT32 | UINT64)
    }
}

pub const BOOL: DType = DType {
    priority: 0,
    itemsize: 1,
    name: "bool",
    fmt: '?',
    count: 1,
};

pub const FLOAT16: DType = DType {
    priority: 9,
    itemsize: 2,
    name: "half",
    fmt: 'e',
    count: 1,
};
pub const HALF: DType = FLOAT16;

pub const FLOAT32: DType = DType {
    priority: 11,
    itemsize: 4,
    name: "float",
    fmt: 'f',
    count: 1,
};

pub const FLOAT: DType = FLOAT32;

pub const FLOAT64: DType = DType {
    priority: 12,
    itemsize: 8,
    name: "double",
    fmt: 'd',
    count: 1,
};

pub const DOUBLE: DType = FLOAT64;

pub const INT8: DType = DType {
    priority: 1,
    itemsize: 1,
    name: "char",
    fmt: 'b',
    count: 1,
};

pub const UINT8: DType = DType {
    priority: 2,
    itemsize: 1,
    name: "unsigned char",
    fmt: 'B',
    count: 1,
};

pub const INT16: DType = DType {
    priority: 3,
    itemsize: 2,
    name: "short",
    fmt: 'h',
    count: 1,
};

pub const UINT16: DType = DType {
    priority: 4,
    itemsize: 2,
    name: "unsigned short",
    fmt: 'H',
    count: 1,
};

pub const INT32: DType = DType {
    priority: 5,
    itemsize: 4,
    name: "int",
    fmt: 'i',
    count: 1,
};

pub const UINT32: DType = DType {
    priority: 6,
    itemsize: 4,
    name: "unsigned int",
    fmt: 'I',
    count: 1,
};

pub const INT64: DType = DType {
    priority: 7,
    itemsize: 8,
    name: "long",
    fmt: 'l',
    count: 1,
};

pub const UINT64: DType = DType {
    priority: 8,
    itemsize: 8,
    name: "unsigned long",
    fmt: 'L',
    count: 1,
};

pub const BFLOAT16: DType = DType {
    priority: 10,
    itemsize: 2,
    name: "__bf16",
    fmt: 'g',
    count: 1,
};
