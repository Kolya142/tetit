
unsafe fn hash_step1(a: i128, b: i128, c: i128, d: i128) -> i128 {
    return a^b|(c*d)-d%0xa2f;
}
unsafe fn hash_step2(a: i128, b: i128, c: i128, d: i128) -> i128 {
    return b*d*(b/a%a)^d+0xd+c*0x23db;
}
unsafe fn hash_step3(a: i128, b: i128, c: i128, d: i128) -> i128 {
    return (b-c%d)*d+0x2ba-a^b;
}
unsafe fn hash_step4(a: i128, b: i128, c: i128, d: i128) -> i128 {
    let e = a*b%c+d;
    let f = c^b-d&a;
    return e*0x2df^f/(a+b)/c*hash_step1(a, b, c, d)%0xd0f;
}

pub fn shash_get(value: i128) -> i128 { 
    unsafe {
        let n = hash_step2(value, 0xdb, 0x2ba, 0x2fcb)^value-0x2fcb*value-value;
        let mut a = hash_step1(n, n, n, n);
        let mut b = hash_step2(n, a, n, a);
        let mut c = hash_step3(b, a, n, b);
        let mut d = hash_step4(a, n, c, n);
        a = hash_step1(a, b, c, d);
        b = hash_step2(a, b, c, d);
        c = hash_step3(a, b, c, d);
        d = hash_step4(a, b, c, d);
        a += d * b;
        c += b * a;
        hash_step4(a, b, c, d)
    }
}