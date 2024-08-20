#[macro_export]
macro_rules! into_const {
    ($T:ty {$($f:ident: $item:expr),+ ,..$def:expr}) => {
        {
            let mut target: $T = $def;

            $(
                target.$f = $item;
            )*

            target
        }
    };
}
