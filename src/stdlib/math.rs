//! Python math module implementation
//! 
//! This module provides mathematical functions and constants.
//! Implementation matches Python's math module API.

use crate::PyException;
use std::f64::consts;
use crate::python_function;

// Mathematical constants
pub const pi: f64 = consts::PI;
pub const e: f64 = consts::E;
pub const tau: f64 = consts::TAU;
pub const inf: f64 = f64::INFINITY;
pub const nan: f64 = f64::NAN;

python_function! {
    /// math.ceil - ceiling function
    pub fn ceil<T>(x: T) -> i64
    where [T: Into<f64>]
    [signature: (x)]
    [concrete_types: (f64) -> i64]
    {
        x.into().ceil() as i64
    }
}

python_function! {
    /// math.floor - floor function
    pub fn floor<T>(x: T) -> i64
    where [T: Into<f64>]
    [signature: (x)]
    [concrete_types: (f64) -> i64]
    {
        x.into().floor() as i64
    }
}

python_function! {
    /// math.trunc - truncate to integer
    pub fn trunc<T>(x: T) -> i64
    where [T: Into<f64>]
    [signature: (x)]
    [concrete_types: (f64) -> i64]
    {
        x.into().trunc() as i64
    }
}

python_function! {
    /// math.fabs - absolute value (float)
    pub fn fabs<T>(x: T) -> f64
    where [T: Into<f64>]
    [signature: (x)]
    [concrete_types: (f64) -> f64]
    {
        x.into().abs()
    }
}

python_function! {
    /// math.sqrt - square root
    pub fn sqrt<T>(x: T) -> Result<f64, PyException>
    where [T: Into<f64>]
    [signature: (x)]
    [concrete_types: (f64) -> Result<f64, crate::PyException>]
    {
        let val = x.into();
        if val < 0.0 {
            Err(crate::value_error("math domain error"))
        } else {
            Ok(val.sqrt())
        }
    }
}

python_function! {
    /// math.pow - power function
    pub fn pow<T, U>(x: T, y: U) -> f64
    where [T: Into<f64>, U: Into<f64>]
    [signature: (x, y)]
    [concrete_types: (f64, f64) -> f64]
    {
        x.into().powf(y.into())
    }
}

python_function! {
    /// math.exp - exponential function
    pub fn exp<T>(x: T) -> f64
    where [T: Into<f64>]
    [signature: (x)]
    [concrete_types: (f64) -> f64]
    {
        x.into().exp()
    }
}

python_function! {
    /// math.exp2 - 2^x
    pub fn exp2<T>(x: T) -> f64
    where [T: Into<f64>]
    [signature: (x)]
    [concrete_types: (f64) -> f64]
    {
        x.into().exp2()
    }
}

python_function! {
    /// math.expm1 - exp(x) - 1
    pub fn expm1<T>(x: T) -> f64
    where [T: Into<f64>]
    [signature: (x)]
    [concrete_types: (f64) -> f64]
    {
        x.into().exp_m1()
    }
}

python_function! {
    /// math.log - natural logarithm
    pub fn log<T>(x: T, base: Option<f64>) -> Result<f64, PyException>
    where [T: Into<f64>]
    [signature: (x, base=None)]
    [concrete_types: (f64, Option<f64>) -> Result<f64, crate::PyException>]
    {
        let val = x.into();
        if val <= 0.0 {
            return Err(crate::value_error("math domain error"));
        }
        
        match base {
            Some(b) if b <= 0.0 || b == 1.0 => Err(crate::value_error("math domain error")),
            Some(b) => Ok(val.ln() / b.ln()),
            None => Ok(val.ln()),
        }
    }
}

python_function! {
    /// math.log2 - base-2 logarithm
    pub fn log2<T>(x: T) -> Result<f64, PyException>
    where [T: Into<f64>]
    [signature: (x)]
    [concrete_types: (f64) -> Result<f64, crate::PyException>]
    {
        let val = x.into();
        if val <= 0.0 {
            Err(crate::value_error("math domain error"))
        } else {
            Ok(val.log2())
        }
    }
}

python_function! {
    /// math.log10 - base-10 logarithm
    pub fn log10<T>(x: T) -> Result<f64, PyException>
    where [T: Into<f64>]
    [signature: (x)]
    [concrete_types: (f64) -> Result<f64, crate::PyException>]
    {
        let val = x.into();
        if val <= 0.0 {
            Err(crate::value_error("math domain error"))
        } else {
            Ok(val.log10())
        }
    }
}

python_function! {
    /// math.log1p - log(1 + x)
    pub fn log1p<T>(x: T) -> Result<f64, PyException>
    where [T: Into<f64>]
    [signature: (x)]
    [concrete_types: (f64) -> Result<f64, crate::PyException>]
    {
        let val = x.into();
        if val <= -1.0 {
            Err(crate::value_error("math domain error"))
        } else {
            Ok(val.ln_1p())
        }
    }
}

// Trigonometric functions
python_function! {
    /// math.sin - sine
    pub fn sin<T>(x: T) -> f64
    where [T: Into<f64>]
    [signature: (x)]
    [concrete_types: (f64) -> f64]
    {
        x.into().sin()
    }
}

python_function! {
    /// math.cos - cosine
    pub fn cos<T>(x: T) -> f64
    where [T: Into<f64>]
    [signature: (x)]
    [concrete_types: (f64) -> f64]
    {
        x.into().cos()
    }
}

python_function! {
    /// math.tan - tangent
    pub fn tan<T>(x: T) -> f64
    where [T: Into<f64>]
    [signature: (x)]
    [concrete_types: (f64) -> f64]
    {
        x.into().tan()
    }
}

python_function! {
    /// math.asin - arc sine
    pub fn asin<T>(x: T) -> Result<f64, PyException>
    where [T: Into<f64>]
    [signature: (x)]
    [concrete_types: (f64) -> Result<f64, crate::PyException>]
    {
        let val = x.into();
        if val < -1.0 || val > 1.0 {
            Err(crate::value_error("math domain error"))
        } else {
            Ok(val.asin())
        }
    }
}

python_function! {
    /// math.acos - arc cosine
    pub fn acos<T>(x: T) -> Result<f64, PyException>
    where [T: Into<f64>]
    [signature: (x)]
    [concrete_types: (f64) -> Result<f64, crate::PyException>]
    {
        let val = x.into();
        if val < -1.0 || val > 1.0 {
            Err(crate::value_error("math domain error"))
        } else {
            Ok(val.acos())
        }
    }
}

python_function! {
    /// math.atan - arc tangent
    pub fn atan<T>(x: T) -> f64
    where [T: Into<f64>]
    [signature: (x)]
    [concrete_types: (f64) -> f64]
    {
        x.into().atan()
    }
}

python_function! {
    /// math.atan2 - arc tangent of y/x
    pub fn atan2<T, U>(y: T, x: U) -> f64
    where [T: Into<f64>, U: Into<f64>]
    [signature: (y, x)]
    [concrete_types: (f64, f64) -> f64]
    {
        y.into().atan2(x.into())
    }
}

// Hyperbolic functions
python_function! {
    /// math.sinh - hyperbolic sine
    pub fn sinh<T>(x: T) -> f64
    where [T: Into<f64>]
    [signature: (x)]
    [concrete_types: (f64) -> f64]
    {
        x.into().sinh()
    }
}

python_function! {
    /// math.cosh - hyperbolic cosine
    pub fn cosh<T>(x: T) -> f64
    where [T: Into<f64>]
    [signature: (x)]
    [concrete_types: (f64) -> f64]
    {
        x.into().cosh()
    }
}

python_function! {
    /// math.tanh - hyperbolic tangent
    pub fn tanh<T>(x: T) -> f64
    where [T: Into<f64>]
    [signature: (x)]
    [concrete_types: (f64) -> f64]
    {
        x.into().tanh()
    }
}

python_function! {
    /// math.asinh - inverse hyperbolic sine
    pub fn asinh<T>(x: T) -> f64
    where [T: Into<f64>]
    [signature: (x)]
    [concrete_types: (f64) -> f64]
    {
        x.into().asinh()
    }
}

python_function! {
    /// math.acosh - inverse hyperbolic cosine
    pub fn acosh<T>(x: T) -> Result<f64, PyException>
    where [T: Into<f64>]
    [signature: (x)]
    [concrete_types: (f64) -> Result<f64, crate::PyException>]
    {
        let val = x.into();
        if val < 1.0 {
            Err(crate::value_error("math domain error"))
        } else {
            Ok(val.acosh())
        }
    }
}

python_function! {
    /// math.atanh - inverse hyperbolic tangent
    pub fn atanh<T>(x: T) -> Result<f64, PyException>
    where [T: Into<f64>]
    [signature: (x)]
    [concrete_types: (f64) -> Result<f64, crate::PyException>]
    {
        let val = x.into();
        if val <= -1.0 || val >= 1.0 {
            Err(crate::value_error("math domain error"))
        } else {
            Ok(val.atanh())
        }
    }
}

// Angular conversion
python_function! {
    /// math.degrees - convert radians to degrees
    pub fn degrees<T>(x: T) -> f64
    where [T: Into<f64>]
    [signature: (x)]
    [concrete_types: (f64) -> f64]
    {
        x.into().to_degrees()
    }
}

python_function! {
    /// math.radians - convert degrees to radians
    pub fn radians<T>(x: T) -> f64
    where [T: Into<f64>]
    [signature: (x)]
    [concrete_types: (f64) -> f64]
    {
        x.into().to_radians()
    }
}

// Special functions
python_function! {
    /// math.factorial - factorial
    pub fn factorial(x: i64) -> Result<i64, PyException>
    [signature: (x)]
    [concrete_types: (i64) -> Result<i64, crate::PyException>]
    {
        if x < 0 {
            return Err(crate::value_error("factorial() not defined for negative values"));
        }
        
        if x > 20 {
            return Err(crate::overflow_error("factorial() result too large"));
        }
        
        let mut result = 1i64;
        for i in 1..=x {
            result = result.saturating_mul(i);
        }
        Ok(result)
    }
}

python_function! {
    /// math.gcd - greatest common divisor
    pub fn gcd(a: i64, b: i64) -> i64
    [signature: (a, b)]
    [concrete_types: (i64, i64) -> i64]
    {
        fn gcd_impl(mut a: i64, mut b: i64) -> i64 {
            while b != 0 {
                let temp = b;
                b = a % b;
                a = temp;
            }
            a.abs()
        }
        gcd_impl(a, b)
    }
}

python_function! {
    /// math.lcm - least common multiple
    pub fn lcm(a: i64, b: i64) -> i64
    [signature: (a, b)]
    [concrete_types: (i64, i64) -> i64]
    {
        if a == 0 || b == 0 {
            0
        } else {
            (a / gcd(a, b) * b).abs()
        }
    }
}

// Classification functions
python_function! {
    /// math.isfinite - check if x is finite
    pub fn isfinite<T>(x: T) -> bool
    where [T: Into<f64>]
    [signature: (x)]
    [concrete_types: (f64) -> bool]
    {
        x.into().is_finite()
    }
}

python_function! {
    /// math.isinf - check if x is infinite
    pub fn isinf<T>(x: T) -> bool
    where [T: Into<f64>]
    [signature: (x)]
    [concrete_types: (f64) -> bool]
    {
        x.into().is_infinite()
    }
}

python_function! {
    /// math.isnan - check if x is NaN
    pub fn isnan<T>(x: T) -> bool
    where [T: Into<f64>]
    [signature: (x)]
    [concrete_types: (f64) -> bool]
    {
        x.into().is_nan()
    }
}

python_function! {
    /// math.isclose - check if values are close
    pub fn isclose<T, U>(a: T, b: U, rel_tol: Option<f64>, abs_tol: Option<f64>) -> bool
    where [T: Into<f64>, U: Into<f64>]
    [signature: (a, b, rel_tol=None, abs_tol=None)]
    [concrete_types: (f64, f64, Option<f64>, Option<f64>) -> bool]
    {
        let a = a.into();
        let b = b.into();
        let rel_tol = rel_tol.unwrap_or(1e-9);
        let abs_tol = abs_tol.unwrap_or(0.0);
        
        if a == b {
            return true;
        }
        
        if a.is_infinite() || b.is_infinite() || a.is_nan() || b.is_nan() {
            return false;
        }
        
        let diff = (a - b).abs();
        diff <= abs_tol.max(rel_tol * a.abs().max(b.abs()))
    }
}

python_function! {
    /// math.copysign - return a float with the magnitude of x and the sign of y
    pub fn copysign<T, U>(magnitude: T, sign: U) -> f64
    where [T: Into<f64>, U: Into<f64>]
    [signature: (magnitude, sign)]
    [concrete_types: (f64, f64) -> f64]
    {
        magnitude.into().copysign(sign.into())
    }
}

python_function! {
    /// math.frexp - return mantissa and exponent
    pub fn frexp<T>(x: T) -> (f64, i32)
    where [T: Into<f64>]
    [signature: (x)]
    [concrete_types: (f64) -> (f64, i32)]
    {
        let val = x.into();
        if val == 0.0 {
            return (val, 0);
        }
        
        let bits = val.to_bits();
        let exponent = ((bits >> 52) & 0x7ff) as i32 - 1023;
        let mantissa = f64::from_bits((bits & 0x800fffffffffffff) | 0x3fe0000000000000);
        
        (mantissa, exponent + 1)
    }
}

python_function! {
    /// math.ldexp - return x * (2**i)
    pub fn ldexp<T>(x: T, i: i32) -> f64
    where [T: Into<f64>]
    [signature: (x, i)]
    [concrete_types: (f64, i32) -> f64]
    {
        x.into() * (2.0_f64).powi(i)
    }
}

python_function! {
    /// math.modf - return fractional and integer parts
    pub fn modf<T>(x: T) -> (f64, f64)
    where [T: Into<f64>]
    [signature: (x)]
    [concrete_types: (f64) -> (f64, f64)]
    {
        let val = x.into();
        let integer_part = val.trunc();
        let fractional_part = val - integer_part;
        (fractional_part, integer_part)
    }
}

python_function! {
    /// math.fmod - floating point remainder
    pub fn fmod<T, U>(x: T, y: U) -> Result<f64, PyException>
    where [T: Into<f64>, U: Into<f64>]
    [signature: (x, y)]
    [concrete_types: (f64, f64) -> Result<f64, crate::PyException>]
    {
        let x = x.into();
        let y = y.into();
        
        if y == 0.0 {
            Err(crate::value_error("math domain error"))
        } else {
            Ok(x % y)
        }
    }
}

python_function! {
    /// math.remainder - IEEE remainder
    pub fn remainder<T, U>(x: T, y: U) -> Result<f64, PyException>
    where [T: Into<f64>, U: Into<f64>]
    [signature: (x, y)]
    [concrete_types: (f64, f64) -> Result<f64, crate::PyException>]
    {
        let x = x.into();
        let y = y.into();
        
        if y == 0.0 {
            Err(crate::value_error("math domain error"))
        } else {
            let n = (x / y).round();
            Ok(x - n * y)
        }
    }
}