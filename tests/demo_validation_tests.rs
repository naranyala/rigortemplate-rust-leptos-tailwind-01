use std::fs;

#[test]
fn test_demos_have_source_code() {
    let demo_dir = "src/ui/pages/demos";
    let entries = fs::read_dir(demo_dir).expect("Failed to read demo directory");

    for entry in entries {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();

        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("rs") {
            let file_name = path.file_name().unwrap().to_string_lossy();
            
            if file_name == "mod.rs" {
                continue;
            }

            let content = fs::read_to_string(&path).expect("Failed to read demo file");
            
            // Check if it contains the SOURCE constant definition
            assert!(
                content.contains("pub const SOURCE"),
                "Demo file {} must define 'pub const SOURCE'",
                file_name
            );

            // Heuristic check for empty SOURCE string: look for SOURCE: &str = "" or SOURCE: &str = ' '
            // This is a simple check; in a real scenario, we might use a parser.
            let has_empty_source = content.contains("SOURCE: &str = \"\"") || content.contains("SOURCE: &str = ''");
            assert!(!has_empty_source, "Demo file {} has an empty SOURCE string", file_name);
        }
    }
}
