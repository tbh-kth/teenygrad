//use teenygrad::dtype::Dtype

struct Devices;
impl Devices {
    pub const DEFAULT: &'static str = "CPU";
    pub const DEVICES: [&'static str; 1] = ["CPU"];
    pub fn cannicalize(_device: String) -> String {
        return String::from("CPU");
    }
}

// THIS IS THE COMPLEX SHIT
//struct Buffer;
//impl Buffer {
//    pub fn main(arg: Type) -> RetType {
//        // def __init__(self, device:str, size:int, dtype:DType, opaque:Any=None, options=None):
//        // self.device, self.size, self.dtype = device, size, dtype
//        // self._buf = opaque[1] if isinstance(opaque, tuple) else opaque
//    }
//    pub fn copyin(arg: Type) -> RetType {
//        // def copyin(self, buf): self._buf = np.frombuffer(buf, dtype=self.dtype.np)
//    }
//    pub fn as_buffer(arg: Type) -> RetType {
//       //  def as_buffer(self): return np.require(self._buf, requirements=["C"]).data
//    }
//}
