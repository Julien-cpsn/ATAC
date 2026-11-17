use std::process::Command;

fn atac(directory: &std::path::Path, args: &[&str]) {
    let output = Command::new(env!("CARGO_BIN_EXE_atac"))
        .arg("--directory")
        .arg(directory)
        .args(args)
        .output()
        .unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
}

#[test]
fn postman_import_is_saved_next_to_existing_collections() {
    let root = std::env::temp_dir().join(format!("atac-cli-import-postman-{}", std::process::id()));
    let directory = root.join("collections");
    std::fs::create_dir_all(&directory).unwrap();

    let postman = root.join("postman.json");
    std::fs::write(&postman, r#"{
        "info": {
            "name": "Shop API",
            "schema": "https://schema.getpostman.com/json/collection/v2.1.0/collection.json"
        },
        "item": [{ "name": "List orders", "request": { "method": "GET", "url": "https://api.example.com/orders" } }]
    }"#).unwrap();

    atac(&directory, &["collection", "new", "Existing"]);
    atac(&directory, &["import", "postman", postman.to_str().unwrap()]);

    let saved = directory.join("Shop API.json").exists();
    std::fs::remove_dir_all(&root).unwrap();
    assert!(saved, "imported collection was not saved");
}
