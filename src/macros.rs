#[macro_export]
/// Creates a new Element of type File.
macro_rules! fi {
    ( $x:expr ) => {
        $crate::element::Element::new($x).format($crate::element::ElementFormat::File)
    };
}

#[macro_export]
/// Creates a new Element of type Directory.
macro_rules! directory {
    ( $x:expr ) => {
        $crate::element::Element::new($x).format($crate::element::ElementFormat::Directory)
    };
}

#[cfg(test)]
mod tests {
    use crate::element::{Element, ElementFormat};

    #[test]
    fn new_file() {
        assert_eq!(
            Element::new("file").format(ElementFormat::File),
            fi!("file")
        )
    }

    #[test]
    fn new_directory() {
        assert_eq!(
            Element::new("directory").format(ElementFormat::Directory),
            directory!("directory")
        )
    }
}
