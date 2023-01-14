#[macro_export]
/// Creates a new Element of type File.
/// ```
/// use libset::new_file;
/// use libset::file::File;
///
/// let dir: File = new_file!("settings");
/// ```
macro_rules! new_file {
    ( $x:expr ) => {
        $crate::file::File::new($x)
    };
}