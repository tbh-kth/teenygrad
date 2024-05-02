use itertools::Itertools;

static OSX: bool = cfg(!target_os = "macos");

pub fn prod() {
    todo!();
}

pub fn dedup<T>(x: Vec<T>) -> Vec<T> {
    return x.into_iter().unique().collect();
    // retains original order, check if using hashmap is better
}

pub fn argfix<t>(x: Vec<t>) -> Vec<_> {
    if condtition {
        // Lost idk what i should make the condition be.
        return x[0].as_Vec();
    } else {
        return x;
    }
}

pub fn make_pair<T: Clone>(x: T, cint: usize) -> Vec<i32> {
    if x.is_interger() {
        return vec![x.clone(); cint];
    } else {
        return x.clone();
    }
}

pub fn flatten<T>(l: Vec<Vec<T>>) -> Vec<T> {
    return l.into_iter().flatten();
}

pub fn fully_flatten<T>(l: Vec<Vec<T>>) {
    todo!();
}

pub fn argsort<T: Ord>(x: Vec<T>) -> Vec<i32> {
    return (0..x.len()).sort_by_key(|&i| &x[i]);
}

pub fn all_int<T>(t: Vec<T>) -> bool {
    return t.into_iter().all(|s| s.is_interger());
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

/*
from typing import Union, Tuple, Sequence, Any, Iterable, Dict, TypeVar
import os, functools, platform, operator

T = TypeVar("T")
U = TypeVar("U")
def prod(x:Iterable[T]) -> Union[T,int]: return functools.reduce(operator.mul, x, 1)
def dedup(x): return list(dict.fromkeys(x))   # retains list orderi
def argfix(*x): return tuple(x[0]) if x and x[0].__class__ in (tuple, list) else x
def make_pair(x:Union[int, Tuple[int, ...]], cnt=2) -> Tuple[int, ...]: return (x,)*cnt if isinstance(x, int) else x
def fully_flatten(l): return [item for sublist in l for item in (fully_flatten(sublist) if isinstance(sublist, (tuple, list)) else [sublist])]
def argsort(x): return type(x)(sorted(range(len(x)), key=x.__getitem__)) # https://stackoverflow.com/questions/3382352/equivalent-of-numpy-argsort-in-basic-python
def merge_dicts(ds:Iterable[Dict[T,U]]) -> Dict[T,U]:
  assert len(kvs:=set([(k,v) for d in ds for k,v in d.items()])) == len(set(kv[0] for kv in kvs)), f"cannot merge, {kvs} contains different values for the same key"  # noqa: E501
  return {k:v for d in ds for k,v in d.items()}
def flat_mv(mv:memoryview): return mv if len(mv) == 0 else mv.cast("B", shape=(mv.nbytes,))

@functools.lru_cache(maxsize=None)
def getenv(key, default=0): return type(default)(os.getenv(key, default))

DEBUG, WINO, IMAGE = getenv("DEBUG"), getenv("WINO"), 0
CI = os.getenv("CI", "") != ""
*/
