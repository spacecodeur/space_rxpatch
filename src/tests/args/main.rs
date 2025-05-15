use crate::{error::InsertError, rxpatch, utils::file_content_to_string};

use super::test_cases::*;

#[test]
fn main() -> Result<(), InsertError> {
    for (_case_idx, test_case) in DATAS.iter().enumerate() {
        let base_path = format!("src/tests/args/files/{}", test_case.file_path);

        for (patch_idx, rx_patch) in test_case.rx_patchs.iter().enumerate() {
            // Construire le chemin du fichier de résultat attendu
            let result_file = format!(
                "src/tests/args/files/{}-{:02}{}",
                test_case
                    .file_path
                    .rsplit_once('.')
                    .map(|(name, _)| name) // Enlève l'extension
                    .unwrap_or(&test_case.file_path) // Si pas d'extension, garde le nom complet
                    .trim_end_matches("-00"),
                patch_idx + 1,
                test_case
                    .file_path
                    .rsplit_once('.')
                    .map(|(_, ext)| format!(".{}", ext)) // Réajoute l'extension
                    .unwrap_or_default() // Si pas d'extension, rien à ajouter
            );

            let expected_content = file_content_to_string(&result_file)?;

            println!("Running test: {}", base_path);

            // Appeler votre fonction rxpatch
            let result = rxpatch(
                &base_path,
                test_case.replacement_type,
                rx_patch.regex,
                rx_patch.new_content,
            )?;

            // println!("APPEL : ");
            // println!(
            //     "rxpatch(\"{}\",\"{}\",\"{}\",\"{}\")",
            //     &base_content.escape_debug().to_string(),
            //     test_case.replacement_type,
            //     rx_patch.regex.escape_debug().to_string(),
            //     rx_patch.new_content
            // );

            // println!("###");
            // println!("{}", result);
            // println!("===??===");
            // println!("{}", expected_content);
            // println!("###\n");

            // Vérifier le résultat
            assert_eq!(
                result.escape_debug().to_string(),
                expected_content.escape_debug().to_string(),
                "Test failed: {}\nExpected: {}\nGot: {}",
                base_path,
                expected_content.escape_debug().to_string(),
                result.escape_debug().to_string()
            );
        }
    }
    Ok(())
}
