pub static _OSX: bool = cfg!(target_os = "macos");

pub fn prod() {
    todo!();
}

pub fn dedup<T>(_x: Vec<T>) -> Vec<T> {
    todo!();
    //x.iter().dedup()
    //    return x.into_iter().unique().collect();
    //    // retains original order, check if using hashmap is better
}

pub fn argfix<T>(_x: Vec<T>) -> Vec<T> {
    todo!();
    //    if condtition {
    //        // Lost idk what i should make the condition be.
    //        return x[0].as_Vec();
    //    } else {
    //        return x;
    //    }
}

pub fn make_pair<T: Clone>(_x: T, _cint: usize) -> Vec<i32> {
    todo!();
    //    if x.is_interger() {
    //        return vec![x.clone(); cint];
    //    } else {
    //        return x.clone();
    //    }
}

pub fn flatten<T>(l: Vec<Vec<T>>) -> Vec<T> {
    l.into_iter().flat_map(|x| x.into_iter()).collect()
}

pub fn argsort<T: Ord>(_x: Vec<T>) -> Vec<i32> {
    todo!();
    //return x
    //    .iter()
    //    .enumerate()
    //    .map(|(i, &v)| (v, i))
    //    .collect::<Vec<_>>()
    //    .sort_by_key(|&(v, _)| v)
    //    .into_iter()
    //    .map(|(_, i)| i)
    //    .collect();
}

pub fn all_int<T>(_t: Vec<T>) -> bool {
    //t.into_iter().all(|&item| item.is_signed())
    todo!();
}

pub fn round_up(num: i32, amt: i32) -> i32 {
    (num + amt - 1) / amt * amt
}

impl std::fmt::Debug for DType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "dtypes.{}", self.name)
    }
}

pub struct DType {
    pub priority: i32,
    pub itemsize: i32,
    pub name: &'static str,
    pub fmt: char,
    pub count: i32,
}

pub const BOOL: DType = DType {
    priority: 0,
    itemsize: 1,
    name: "bool",
    fmt: '?',
    count: 1,
};

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

pub const FLOAT16: DType = DType {
    priority: 9,
    itemsize: 2,
    name: "half",
    fmt: 'e',
    count: 1,
};

pub const BFLOAT16: DType = DType {
    priority: 10,
    itemsize: 2,
    name: "__bf16",
    fmt: 'g',
    count: 1,
};

pub const FLOAT32: DType = DType {
    priority: 11,
    itemsize: 4,
    name: "float",
    fmt: 'f',
    count: 1,
};

pub const FLOAT64: DType = DType {
    priority: 12,
    itemsize: 8,
    name: "double",
    fmt: 'd',
    count: 1,
};
