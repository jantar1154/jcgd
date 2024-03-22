use dialog::DialogBox;

pub(crate) fn download_catgirl() {
    // Open file dialog and get a path
    let save_dialog = dialog::FileSelection::new("Please select a file")
        .mode(dialog::FileSelectionMode::Save)
        .title("Save catgirl as:")
        .show()
        .expect("Could not show a saving dialog");

    // If clicked close, function will end now,
    // otherwise copy the file from temp
    let path = match save_dialog {
        Some(str) => {str},
        None => {return;},
    };

    let temp_path = std::env::temp_dir();
    let from = match temp_path.to_str() {
        Some(path) => path,
        None => {
            panic!("Could not locate temp directory!");
        },
    };
    match std::fs::copy(format!("{}/jcgd/jcgd.jpeg", from), path) {
        Ok(yay) => yay,
        Err(nay) => {
            panic!("Error copying catgirl to harddisk! Did you delete the image from temp folder?\n{}", nay);
        },
    };
}