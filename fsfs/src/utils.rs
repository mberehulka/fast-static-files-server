pub const fn str_to_usize(v: &str) -> usize {
    let mut res = 0;
    let env = v.as_bytes();
    let mut i = 0;
    let mut j = 10_usize.pow(env.len() as u32);
    let l = env.len();
    while i < l {
        j /= 10;
        let b = env[i];
        res += (b - b'0') as usize * j;
        i += 1;
    }
    res
}
pub const fn str_to_u64(v: &str) -> u64 {
    let mut res = 0;
    let env = v.as_bytes();
    let mut i = 0;
    let mut j = 10_u64.pow(env.len() as u32);
    let l = env.len();
    while i < l {
        j /= 10;
        let b = env[i];
        res += (b - b'0') as u64 * j;
        i += 1;
    }
    res
}

macro_rules! env_usize {
    ($name: expr) => {
        crate::utils::str_to_usize(env!($name))
    };
}
macro_rules! env_u64 {
    ($name: expr) => {
        crate::utils::str_to_u64(env!($name))
    };
}