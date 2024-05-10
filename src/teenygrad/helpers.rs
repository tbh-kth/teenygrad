pub static OSX: bool = cfg!(target_os = "macos");

pub fn prod() {
    todo!();
}

pub fn dedup<T>(x: Vec<T>) -> Vec<T> {
    todo!();
    //    return x.into_iter().unique().collect();
    //    // retains original order, check if using hashmap is better
}

pub fn argfix<T>(x: Vec<T>) -> Vec<T> {
    todo!();
    //    if condtition {
    //        // Lost idk what i should make the condition be.
    //        return x[0].as_Vec();
    //    } else {
    //        return x;
    //    }
}

pub fn make_pair<T: Clone>(x: T, cint: usize) -> Vec<i32> {
    todo!();
    //    if x.is_interger() {
    //        return vec![x.clone(); cint];
    //    } else {
    //        return x.clone();
    //    }
}

pub fn flatten<T>(l: Vec<Vec<T>>) -> Vec<T> {
    return l.into_iter().flatten().collect();
}

pub fn fully_flatten<T>(l: Vec<Vec<T>>) {
    todo!();
}

pub fn argsort<T: Ord>(x: Vec<T>) -> Vec<i32> {
    todo!();
    //   return (0..x.len()).sort_by_key(|&i| &x[i]);
}

pub fn all_int<T>(t: Vec<T>) -> bool {
    todo!();
    //   return t.into_iter().all(|s| s.is_interger());
}

pub fn round_up(num: i32, amt: i32) -> i32 {
    return (num + amt - 1) / amt * amt;
}

pub fn merge_dicts() {
    todo!();
}

pub fn flat_mv() {
    todo!();
}
