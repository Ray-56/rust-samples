fn main() {
    let mut _mutable_integer = 7i32;

    {
        // Shaded by immutable `_mutable_integer`
        let _mutable_integer = _mutable_integer;

        // Report an error! `_mutable_integer` is frozen in this scope
        _mutable_integer = 50;

        // `_mutable_integer` goes out of scope
    }

    // Running normally! `_mutable_integer` is not frozen in this scope
    _mutable_integer = 3;
}