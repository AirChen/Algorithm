
pub fn pow(base: u128, idx: u16) -> u128 {
    let mut tu = 1;
    let mut dir = idx;
    let mut v = base;
    while dir != 0 {
        if dir&1 != 0 {
            tu *= v;
        }
        v *= v;
        dir >>= 1;
    }
    tu as u128
}

pub fn powf(base: f64, idx: u16) -> f64 {
    let mut tu: f64 = 1.0;
    let mut dir = idx;
    let mut v = base;
    while dir != 0 {
        if dir&1 != 0 {
            tu *= v;
        }
        v *= v;
        dir >>= 1;
    }
    tu
}

// https://diducoder.com/sotry-about-sqrt.html
// https://stackoverflow.com/questions/59081890/is-it-possible-to-write-quakes-fast-invsqrt-function-in-rust
pub fn sqrt(x: f32) -> f32 {    
    let xhalf = 0.5 * x;
    
    let mut i = x.to_bits();        
    i = 0x5f375a86 - (i>>1);

    let mut xmut: f32 = f32::from_bits(i);    
    xmut = xmut * (1.5 - xhalf * xmut * xmut);
    xmut = xmut * (1.5 - xhalf * xmut * xmut);
    xmut = xmut * (1.5 - xhalf * xmut * xmut);

    1.0/xmut
}

fn abs(x: f64) -> f64 {
    if x < 0.0 { -x } else { x }     
}

// 求出根号a的近似值：首先随便猜一个近似值x，然后不断令x等于x和a/x的平均数，迭代个六七次后x的值就已经相当精确了。
pub fn sqrt_newton(x: f64) -> f64 {
    let mut val = x;
    let mut last = 0.0;

    loop {
        last = val;
        val = (val + x/val)/2.0;

        if abs(val - last) <= 0.001 {
            break;
        }
    }

    val
}

// 二分法
pub fn sqrt_bisection(x: f64) -> f64 {
    if x < 0.0 {
        return x;
    }

    let mut low: f64 = 0.0;
    let mut up: f64 = x;
    let mut mid: f64 = (low + up)/2.0;
    let mut last: f64 = 0.0;
    loop {
        if mid * mid > x {
            up = mid;
        } else {
            low = mid;
        }
        last = mid;
        mid = ( low + up )/2.0;

        if abs(mid - last) <= 0.01 {
            break;
        }
    }

    mid
}

pub fn log2(value: isize) -> i8 {
    let table: [i8; 64] = 
        [
            63,  0, 58,  1, 59, 47, 53,  2,
            60, 39, 48, 27, 54, 33, 42,  3,
            61, 51, 37, 40, 49, 18, 28, 20,
            55, 30, 34, 11, 43, 14, 22,  4,
            62, 57, 46, 52, 38, 26, 32, 41,
            50, 36, 17, 19, 29, 10, 13, 21,
            56, 45, 25, 31, 35, 16,  9, 12,
            44, 24, 15,  8, 23,  7,  6,  5
        ];

    let mut happy_value = value;
    happy_value |= happy_value >> 1;
    happy_value |= happy_value >> 2;
    happy_value |= happy_value >> 4;
    happy_value |= happy_value >> 8;
    happy_value |= happy_value >> 16;
    happy_value |= happy_value >> 32;

    let idx = ((happy_value - (happy_value >> 1)) * 0x07EDD5E59A4E28C2) >> 58;

    table[idx as usize]
}
