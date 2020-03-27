#[cfg(test)]
mod main_tests {
    use std::fs;
    use std::path::Path;

    use crate::replace_tag_for_service_image;

    #[test]
    fn result_file_should_contains_expected_tag() {
        let result_file_name = "docker-compose200.yml";
        let result_file = Path::new(&result_file_name);

        let source_file = Path::new("tests/docker-compose.yml-source");
        let expected_file = Path::new("tests/docker-compose.yml-expected");

        if result_file.exists() {
            fs::remove_file(result_file).expect("unable to remove docker-compose.yml file");
        }

        fs::copy(source_file, result_file_name)
           .expect("unable to prepare docker-compose.yml file");

        let result = replace_tag_for_service_image(result_file, "backend", "master-123");

        let expected_content = fs::read_to_string(expected_file)
                    .expect("unable to read content from tests/docker-compose.yml-expected");

        let result_content = fs::read_to_string(result_file)
            .expect("unable to read content from result file");

        assert_eq!(result, true);
        assert_eq!(result_content, expected_content);

        if result_file.exists() {
            fs::remove_file(result_file).expect("unable to remove docker-compose.yml file");
        }
    }

    #[test]
    fn yaml_files_with_different_spaces_should_be_supported() {
        let result_file_name = "docker-compose300.yml";
        let result_file = Path::new(&result_file_name);

        let source_file = Path::new("tests/less-spaces.yml-source");
        let expected_file = Path::new("tests/less-spaces.yml-expected");

        if result_file.exists() {
            fs::remove_file(result_file).expect("unable to remove docker-compose300.yml file");
        }

        fs::copy(source_file, result_file_name)
            .expect("unable to prepare docker-compose.yml file");

        let result = replace_tag_for_service_image(result_file, "backend", "master-221");

        let expected_content = fs::read_to_string(expected_file)
            .expect("unable to read content from 'tests/less-spaces.yml-expected'");

        let result_content = fs::read_to_string(result_file)
            .expect("unable to read content from result file");

        assert_eq!(result, true);
        assert_eq!(result_content, expected_content);

        if result_file.exists() {
            fs::remove_file(result_file).expect("unable to remove docker-compose300.yml file");
        }
    }
}
