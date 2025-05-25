// pub mod docker_compose_format;
pub mod env_format;
// pub mod rust_format;

use std::process::{Command, Output};

#[derive(Debug)]
pub struct TestCase<'a> {
    pub fixture_file_path: &'a str,
    pub file_format: &'a str,
    pub rx_patchs: &'a [RxPatch<'a>],
}

#[derive(Debug)]
pub struct RxPatch<'a> {
    pub i_want_to: &'a str,
    pub regex: &'a str,
    pub new_content: &'a str,
}

impl TestCase<'_> {
    pub fn check(self) -> Result<(), Box<dyn std::error::Error>> {
        let absolute_functional_tests_dir = "tests/functional";
        let absolute_fixture_file_path = format!(
            "{}/{}",
            absolute_functional_tests_dir, self.fixture_file_path
        );

        dbg!(&self.rx_patchs);

        for (patch_idx, rx_patch) in self.rx_patchs.iter().enumerate() {
            let result_file = format!(
                "{}/{}-{:02}",
                absolute_functional_tests_dir,
                self.fixture_file_path.trim_end_matches("-00"),
                patch_idx + 1
            );

            let expected_content = std::fs::read_to_string(&result_file)?;

            let result = Self::run_app_with_args(
                &absolute_fixture_file_path,
                self.file_format,
                rx_patch.regex,
                rx_patch.new_content,
            )
            .expect("App should run without error");

            assert_eq!(
                result.escape_debug().to_string(),
                expected_content.escape_debug().to_string(),
                "Test failed: {}\nExpected: {}\nGot: {}",
                absolute_fixture_file_path,
                expected_content.escape_debug().to_string(),
                result.escape_debug().to_string()
            );
        }

        Ok(())
    }

    fn run_app_with_args(
        file_path: &str,
        file_force_format: &str,
        regex: &str,
        new_content: &str,
    ) -> Result<String, String> {
        let mut binding = Command::new("cargo");
        let command = binding
            .args(&["run", "--quiet", "--"])
            .arg("--file")
            .arg(file_path)
            .arg("--regex")
            .arg(regex)
            .arg("--new-content")
            .arg(new_content)
            .arg("--file-force-format")
            .arg(file_force_format);

        let command_str = format!(
            "cargo run --quiet -- --file '{}' --regex '{}' --new-content '{}' --file-force-format '{}'",
            file_path, regex, new_content, file_force_format
        );

        dbg!(regex);
        println!("{}", command_str);

        let output = command.output().expect("Failed to run cargo");

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).into_owned())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).into_owned())
        }
    }
}
