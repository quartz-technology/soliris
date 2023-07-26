#[derive(Default)]
pub struct Metadata<'a> {
    pub file_path: &'a str,
}

impl<'a> Metadata<'a> {
    pub fn new(file_path: &'a str) -> Self {
        Metadata { file_path }
    }
}

#[cfg(test)]
mod tests {
    use super::Metadata;

    #[test]
    fn it_creates_a_new_metadata_object() {
        let file_path = "beyond_the_aquila_rift";
        let metadata = Metadata::new(file_path);

        assert_eq!(metadata.file_path, file_path)
    }
}
