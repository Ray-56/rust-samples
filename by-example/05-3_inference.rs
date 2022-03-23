fn main() {
    // 因为有类型说明，编译器知道`elem`的类型是 u8
    let elem = 5u8;

    // 创建一个空向量（vector，即不定长的，可以增长的数组）
    let mut vec = Vec::new();
    // 现在编译器还不知道`vec`的具体类型，只知道它是某种东西构成的向量（`vec<_>`）

    // 在向量中插入`elem`
    vec.push(elem);
    // 现在编译器知道`vec`是 u8 的向量了（`Vec<u8>`）
    
}