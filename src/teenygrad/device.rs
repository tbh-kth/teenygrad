// from typing import Optional, Any
// from teenygrad.dtype import DType
// import numpy as np
use teenygrad::dtype::Dtype


struct Devices;
impl Devices {
    let DEFAULT = "CPU";
    let _devices = ["CPU"];
    fn cannicalize(_device: String) -> String {
        return String::from("CPU");
    }
}


// THIS IS THE COMPLEX SHIT
struct Buffer;
impl Buffer {
    fn main(arg: Type) -> RetType {
        // def __init__(self, device:str, size:int, dtype:DType, opaque:Any=None, options=None):
        // self.device, self.size, self.dtype = device, size, dtype
        // self._buf = opaque[1] if isinstance(opaque, tuple) else opaque
    }
    fn copyin(arg: Type) -> RetType {
        // def copyin(self, buf): self._buf = np.frombuffer(buf, dtype=self.dtype.np)
    }
    fn as_buffer(arg: Type) -> RetType {
       //  def as_buffer(self): return np.require(self._buf, requirements=["C"]).data
    }
}

