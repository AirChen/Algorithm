
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
pub fn sqrt(x: f64) -> f64 {    
    let xhalf = 0.5 * x;
    let mut i: u32 = x as u32;
    i = 0x5f375a86 - (i>>1);

    let mut xmut: f64 = f64::from_bits(i as u64);    
    xmut = xmut * (1.5 - xhalf * xmut * xmut);
    xmut = xmut * (1.5 - xhalf * xmut * xmut);
    xmut = xmut * (1.5 - xhalf * xmut * xmut);

    1.0/xmut
}

// float InvSqrt(float x)
// {
// 	float xhalf = 0.5f*x;
// 	int i = *(int*)&x; // get bits for floating VALUE 
// 	i = 0x5f375a86- (i>>1); // gives initial guess y0
// 	x = *(float*)&i; // convert bits BACK to float
// 	x = x*(1.5f-xhalf*x*x); // Newton step, repeating increases accuracy
// 	x = x*(1.5f-xhalf*x*x); // Newton step, repeating increases accuracy
// 	x = x*(1.5f-xhalf*x*x); // Newton step, repeating increases accuracy

// 	return 1/x;
// }

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
