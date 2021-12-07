#[macro_export]
macro_rules! fi {
    ( $x:expr) => {
        Element::new($x).format(Format::File)
    };
}

#[macro_export]
macro_rules! dir {
    ( $x:expr) => {
        Element::new($x).format(Format::Directory)
    };
}
