#[macro_export]
/// Creates a new Element of type File.
macro_rules! fi {
    ( $x:expr) => {
        Element::new($x).format(ElementFormat::File)
    };
}

#[macro_export]
/// Creates a new Element of type Directory.
macro_rules! dir {
    ( $x:expr) => {
        Element::new($x).format(ElementFormat::Directory)
    };
}
