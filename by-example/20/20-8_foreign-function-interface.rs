use std::fmt;

// This extern code block is linked to the libm library
#[link(name = "m")]
extern {
    // This external function is used to calculate the square root of a single-precision complex number.
    fn csqrtf(z: Complex) -> Complex;

    // This is used to calculate the complex cosine of a single precision complex number
    fn ccosf(z: Complex) -> Complex;
}

// Since calling functions in other languages ​​is considered unsafe, we usually write a safe wrapper for them.
fn cos(z: Complex) -> Complex {
    unsafe { ccosf(z) }
}

fn main() {
    // z = -1 + 0i
    let z = Complex { re: -1., im: 0. };

    // Calling external language functions is an unsafe operation
    let z_sqrt = unsafe {
        csqrtf(z)
    };

    println!("the square root of {:?} is {:?}", z, z_sqrt);

    // Calling a secure API wrapper that does not perform secure operations
    println!("cos({:?}) = {:?}", z, cos(z));
}

// The simplest implementation of single precision complex numbers
#[repr(C)]
#[derive(Clone, Copy)]
struct Complex {
    re: f32,
    im: f32,
}

impl fmt::Debug for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.im < 0. {
            write!(f, "{}-{}i", self.re, -self.im)
        } else {
            write!(f, "{}+{}i", self.re, self.im)
        }
    }
}