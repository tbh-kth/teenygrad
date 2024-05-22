// ADD YOUR OPS JAY
//use teenygrad::device::Buffer;

struct LazyBuffer {
    device: &'static str,
    //realized: Buffer,
}

impl LazyBuffer {
    fn base(&self) {}
    fn dtype(&self) {}
    fn _np() {}
    fn shape() {}
    fn __repr__() {}

    fn is_unrealized_contiguous_const() {}
    fn copy_to_device(&self, _device: &str) {}

    fn from_cpus() {}
    fn loadop() {}

    fn contiguous() {}
    fn _const() {}

    fn cast() {}

    fn e() {}

    fn r() {}

    // MovementOps
    fn reshape() {}
    fn expand() {}
    fn shrink() {}
    fn permute() {}
    fn pad() {}
    fn stride() {}
}
