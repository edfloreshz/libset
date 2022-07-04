#[macro_export]
/// Creates a new Element of type File.
/// ```
/// use libset::new_file;
/// use libset::element::Element;
///
/// let dir: Element = new_file!("settings");
/// ```
macro_rules! new_file {
    ( $x:expr ) => {
        $crate::element::Element::new($x).set_type($crate::element::ElementType::File)
    };
}

#[macro_export]
/// Creates a new Element of type Directory.
/// ```
/// use libset::new_dir;
/// use libset::element::Element;
///
/// let dir: Element = new_dir!("settings");
/// ```
macro_rules! new_dir {
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
            new_file!("file")
        )
    }

    #[test]
    fn new_directory() {
        assert_eq!(
            Element::new("directory").set_type(ElementType::Directory),
            new_dir!("directory")
        )
    }
}
