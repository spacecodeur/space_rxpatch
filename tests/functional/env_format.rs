use crate::functional::{RxPatch, TestCase};

#[test]
fn inject_into_empty_env_block() -> Result<(), Box<dyn std::error::Error>> {
    TestCase {
        fixture_file_path: "fixtures/env/00-00",
        file_format: "env",
        rx_patchs: &[RxPatch {
            i_want_to: "add new key/value to empty block",
            regex: r"#\sPart1\s((?:[a-zA-Z_0-9]+=[a-zA-Z_0-9]*\n)*)",
            new_content: r"key_1_1=value_1_1",
        }],
    }
    .check()?;

    Ok(())
}

#[test]
fn inject_into_existing_env_block() -> Result<(), Box<dyn std::error::Error>> {
    TestCase {
        fixture_file_path: "fixtures/env/01-00",
        file_format: "env",
        rx_patchs: &[
            RxPatch {
                i_want_to: "add new key/value to the begin of an existing block with alphabetical sorting",
                regex: r"#\sPart3\s((?:[a-zA-Z_0-9]+=[a-zA-Z_0-9]*\n)*)",
                new_content: r"key_3_0=value_3_0",
            },
            RxPatch {
                i_want_to: "add new key/value to existing block with alphabetical sorting",
                regex: r"#\sPart3\s((?:[a-zA-Z_0-9]+=[a-zA-Z_0-9]*\n)*)",
                new_content: r"key_3_3=value_3_3",
            },
            RxPatch {
                i_want_to: "add new key/value to the end of an existing block with alphabetical sorting",
                regex: r"#\sPart3\s((?:[a-zA-Z_0-9]+=[a-zA-Z_0-9]*\n)*)",
                new_content: r"key_3_5=value_3_5",
            },
            // RxPatch {
            //     i_want_to: "add new key/value to a non-existing block",
            //     regex: r"#\sPart0\s((?:[a-zA-Z_0-9]+=[a-zA-Z_0-9]*\n)*)",
            //     new_content: r"key_0_0=value_0_0",
            // }
        ],
    }
    .check()?;

    Ok(())
}

#[test]
fn modify_value_of_key_in_existing_env_block() -> Result<(), Box<dyn std::error::Error>> {
    TestCase {
        fixture_file_path: "fixtures/env/02-00",
        file_format: "env",
        rx_patchs: &[
            RxPatch {
                i_want_to: "modify the value of an unique key",
                regex: r"^ *key_4=([^\n]*)",
                new_content: r"value_40",
            },
            RxPatch {
                i_want_to: "modify the value of an unique key when the key has an empty value",
                regex: r"^ *key_0=([^\n]*)",
                new_content: r"value_0",
            },
            RxPatch {
                i_want_to: "modify the value of a non-unique key",
                regex: r"#\sPart3\s(?:.*\n)*?^ *key_3=([^\n]*)",
                new_content: r"value_30",
            },
            RxPatch {
                i_want_to: "modify the value of a non-unique key when the key has an empty value",
                regex: r"#\sPart4\s(?:.*\n)*?^ *key_2=([^\n]*)",
                new_content: r"value_2",
            },
        ],
    }
    .check()?;

    Ok(())
}
