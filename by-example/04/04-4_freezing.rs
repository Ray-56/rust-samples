fn main() {
    let mut _mutable_integer = 7i32;

    {
        // 被不可变的`_mutable_integer`遮蔽
        let _mutable_integer = _mutable_integer;

        // 报错！`_mutable_integer`在本作用域被冻结
        _mutable_integer = 50;

        // `_mutable_integer` 离开作用域
    }

    // 正常运行！`_mutable_integer`在这个作用域没有冻结
    _mutable_integer = 3;
}