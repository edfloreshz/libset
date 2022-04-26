#[macro_export]
/// Creates a new Element of type File.
/// ```
/// use libset::fi;
/// use libset::element::Element;
///
/// let dir: Element = fi!("settings");
/// ```
macro_rules! fi {
    ( $x:expr ) => {
        $crate::element::Element::new($x).set_type($crate::element::ElementType::File)
    };
}

#[macro_export]
/// Creates a new Element of type Directory.
/// ```
/// use libset::directory;
/// use libset::element::Element;
///
/// let dir: Element = directory!("settings");
/// ```
macro_rules! directory {
    ( $x:expr ) => {
        $crate::element::Element::new($x).set_type($crate::element::ElementType::Directory)
    };
}

#[cfg(test)]
mod tests {
    use crate::element::{Element, ElementType};

    #[test]
    fn new_file() {
        assert_eq!(
            Element::new("file").set_type(ElementType::File),
            fi!("file")
        )
    }

    #[test]
    fn new_directory() {
        assert_eq!(
            Element::new("directory").set_type(ElementType::Directory),
            directory!("directory")
        )
    }
}
