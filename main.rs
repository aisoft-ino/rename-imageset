use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;

fn rename_directory_images_json(
    directory_full_path: &str,
    new_name: &str,
) -> io::Result<()> {
    // rename current folder from the full path to new_name.imageset
    // ex: Resources/AppAssets.xcassets/play-shortcut.imageset ->
    //    Resources/AppAssets.xcassets/play-circle-35.imageset
    let mut new_directory_full_path = String::from(directory_full_path);
    // remove the last part of the path
    new_directory_full_path = new_directory_full_path.rsplitn(2, "/").collect::<Vec<&str>>()[1].to_string();
    // append the new name
    new_directory_full_path.push_str("/");
    new_directory_full_path.push_str(new_name);
    new_directory_full_path.push_str(".imageset");
    fs::rename(directory_full_path, Path::new(&new_directory_full_path))?;

    // Find all image files under the directory
    let mut image_files = Vec::new();
    let extensions = ["jpg", "png", "pdf"];
    for entry in fs::read_dir(&new_directory_full_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extensions.contains(&extension.to_str().unwrap()) {
                    image_files.push(path);
                }
            }
        }
    }

    // Read the contents of the file "Contents.json"
    let mut contents_json_file = File::open(format!("{}/Contents.json", new_directory_full_path))?;
    let mut contents_json = String::new();
    contents_json_file.read_to_string(&mut contents_json)?;

    // Rename the image files form <image_file>(@2x|@3x)?.<extension> to <new_name>(@2x|@3x).<extension>
    // Replace the old image file names with the new image file names in the contents of the file "Contents.json"
    for image_file in &image_files {
        let image_file_name = image_file.file_name().unwrap().to_str().unwrap();
        let mut new_image_file_name = String::from(new_name);

        // verify if image_file contains @2x
        if image_file.to_str().unwrap().contains("@2x") {
            new_image_file_name.push_str("@2x");
        } else if image_file.to_str().unwrap().contains("@3x") {
            new_image_file_name.push_str("@3x");
        } else {
            // do nothing
        }
        // push same extension
        new_image_file_name.push_str(".");
        new_image_file_name.push_str(image_file.extension().unwrap().to_str().unwrap());
        let new_image_file_path = format!("{}/{}", new_directory_full_path, new_image_file_name);
        fs::rename(image_file, Path::new(&new_image_file_path))?;
        contents_json = contents_json.replace(image_file_name, &new_image_file_name);
    }

    // write the contents back to the file "Contents.json" replacing the old contents
    let mut contents_json_file = File::create(format!("{}/Contents.json", new_directory_full_path))?;
    contents_json_file.write_all(contents_json.as_bytes())?;

    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    // the directory full path, ex: /Users/username/Developer/ios_app/src/Resources/AppAssets.xcassets/play-shortcut.imageset
    let directory_full_path = &args[1];
    // the new name, ex: play-circle-35
    let new_name = &args[2];

    rename_directory_images_json(directory_full_path, new_name).unwrap();
}
