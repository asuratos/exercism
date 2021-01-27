#[macro_export]
macro_rules! hashmap {
    () => {
        ::std::collections::HashMap::new()
    };
    ($($k:expr => $v:expr),+ $(,)?) => {
        {
            let mut hash = ::std::collections::HashMap::new();
            $(
                hash.insert($k, $v);
            )+
            hash
        }
    };
}
