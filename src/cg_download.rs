use rfd::FileDialog;

pub(crate) fn download_catgirl() {
    // Open file dialog and get a path
    let path = FileDialog::new()
        .set_file_name("catgirl.jpeg")
        .set_title("Select where to save your catgirl")
        .add_filter("JPEG image", &["jpeg"])
        .save_file();

    // If clicked close, function will end now,
    // otherwise copy the file from temp
    let path = match path {
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