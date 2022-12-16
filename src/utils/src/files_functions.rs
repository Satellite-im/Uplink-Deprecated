use std::{path::{PathBuf, Path}, io::Cursor, ffi::OsStr};

use dioxus::desktop::DesktopContext;
use futures::StreamExt;
use mime::*;
use tokio_util::io::ReaderStream;
use warp::{constellation::Progression, error::Error};
use image::io::Reader as ImageReader;

use crate::Storage;

pub async fn upload_file(file_storage: Storage, file_path: PathBuf, eval_script: DesktopContext) {
    let mut filename = match file_path
        .file_name()
        .map(|file| file.to_string_lossy().to_string())
    {
        Some(file) => file,
        None => return,
    };

    let local_path = Path::new(&file_path).to_string_lossy().to_string();
    let mut count_index_for_duplicate_filename = 1;
    let mut file_storage = file_storage.clone();
    let original = filename.clone();
    let file = PathBuf::from(&original);

    let current_directory = match file_storage.current_directory() {
        Ok(current_dir) => current_dir, 
        _ => return
    };

    loop {
        if !current_directory.has_item(&filename) {
            break;
        }
        let file_extension = file.extension().and_then(OsStr::to_str).map(str::to_string);
        let file_stem = file.file_stem().and_then(OsStr::to_str).map(str::to_string);

        filename = match (file_stem, file_extension) {
            (Some(file_stem), Some(file_extension)) => {
                format!("{file_stem} ({count_index_for_duplicate_filename}).{file_extension}")
            }
            _ => format!("{original} ({count_index_for_duplicate_filename})"),
        };

        log::info!("Duplicate name, changing file name to {}", &filename);
        count_index_for_duplicate_filename += 1;
    }

    let tokio_file = match tokio::fs::File::open(&local_path).await {
        Ok(file) => file,
        Err(error) => {
            log::error!("Error on get tokio file, cancelling upload action, error: {error}");
            return;
        }
    };

    let total_size_for_stream = match tokio_file.metadata().await {
        Ok(data) => Some(data.len() as usize),
        Err(error) => {
            log::error!("Error getting metadata: {:?}", error);
            None
        }
    };

    let file_stream = ReaderStream::new(tokio_file)
        .filter_map(|x| async { x.ok() })
        .map(|x| x.into());

    match file_storage
        .put_stream(&filename, total_size_for_stream, file_stream.boxed())
        .await
    {
        Ok(mut upload_progress) => {
            while let Some(upload_progress) = upload_progress.next().await {
                match upload_progress {
                    Progression::CurrentProgress {
                        name,
                        current,
                        total,
                    } => {
                        log::info!("Written {} MB for {name}", current / 1024 / 1024);
                        if let Some(total) = total {
                            let mut selector_without_percentage =
                                "document.getElementById('dropzone').value = '".to_owned();

                            let percentage =
                                ((((current as f64) / (total as f64)) * 100.) as usize).to_string();
                            selector_without_percentage.push_str(&percentage);

                            let ending_string = "% uploaded'";
                            selector_without_percentage.push_str(ending_string);

                            eval_script.eval(&selector_without_percentage);

                            log::info!(
                                "{}% completed",
                                (((current as f64) / (total as f64)) * 100.) as usize
                            )
                        }
                    }
                    Progression::ProgressComplete { name, total } => {
                        log::info!(
                            "{name} has been uploaded with {} MB",
                            total.unwrap_or_default() / 1024 / 1024
                        );
                    }
                    Progression::ProgressFailed {
                        name,
                        last_size,
                        error,
                    } => {
                        log::info!(
                            "{name} failed to upload at {} MB due to: {}",
                            last_size.unwrap_or_default(),
                            error.unwrap_or_default()
                        );
                    }
                }
            }
            match set_thumbnail_if_file_is_image(file_storage.clone(), filename.clone()).await {
                Ok(success) => log::info!("{:?}", success), 
                Err(error) => log::error!("Error on update thumbnail: {:?}", error), 
            }  
            log::info!("{:?} file uploaded!", &filename);
        }, 
        Err(error) => log::error!("Error when upload file: {:?}", error)
        
    }


    
}

async fn set_thumbnail_if_file_is_image(
    file_storage: Storage,
    filename_to_save: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let item = file_storage.current_directory()?.get_item(&filename_to_save)?;
    let parts_of_filename: Vec<&str> = filename_to_save.split('.').collect();

    let file = file_storage.get_buffer(&filename_to_save).await?;

    // Guarantee that is an image that has been uploaded
    ImageReader::new(Cursor::new(&file))
        .with_guessed_format()?
        .decode()?;

    // Since files selected are filtered to be jpg, jpeg, png or svg the last branch is not reachable
    let mime = match parts_of_filename
        .iter()
        .map(|extension| extension.to_lowercase())
        .last()
    {
        Some(m) => match m.as_str() {
            "png" => IMAGE_PNG.to_string(),
            "jpg" => IMAGE_JPEG.to_string(),
            "jpeg" => IMAGE_JPEG.to_string(),
            "svg" => IMAGE_SVG.to_string(),
            &_ => "".to_string(),
        },
        None => "".to_string(),
    };

    if !file.is_empty() || !mime.is_empty() {
        let prefix = format!("data:{};base64,", mime);
        let base64_image = base64::encode(&file);
        let img = prefix + base64_image.as_str();
        item.set_thumbnail(&img);
        Ok(format_args!("{} thumbnail updated with success!", item.name()).to_string())
    } else {
        Err(Box::from(Error::InvalidItem))
    }
}

pub fn format_item_size(item_size: usize) -> String {
    if item_size == 0 {
        return String::from("0 bytes");
    }
    let base_1024: f64 = 1024.0;
    let size_f64: f64 = item_size as f64;

    let i = (size_f64.log10() / base_1024.log10()).floor();
    let size_formatted = size_f64 / base_1024.powf(i);

    let item_size_suffix = ["bytes", "KB", "MB", "GB", "TB"][i as usize];
    let mut size_formatted_string = format!(
        "{size:.*} {size_suffix}",
        1,
        size = size_formatted,
        size_suffix = item_size_suffix
    );
    if size_formatted_string.contains(".0") {
        size_formatted_string = size_formatted_string.replace(".0", "");
    }
    size_formatted_string
}

pub fn format_item_name(item_name: String, file_kind: Option<String>, is_folder: bool) -> String {
    let mut new_item_name = item_name.clone();
    let item = PathBuf::from(&new_item_name);

    if is_folder {
        if new_item_name.len() > 10 {
            new_item_name = match &new_item_name.get(0..5) {
                Some(name_sliced) => format!(
                    "{}...{}",
                    name_sliced,
                    &new_item_name[new_item_name.len() - 3..].to_string(),
                ),
                None => new_item_name.clone(),
            };
        }
    } else {
        let file_kind = file_kind.unwrap_or_default();
        let file_stem = item
        .file_stem()
        .and_then(OsStr::to_str)
        .map(str::to_string)
        .unwrap_or_default();

    if file_stem.len() > 10 {
        new_item_name = match &item_name.get(0..5) {
            Some(name_sliced) => format!(
                "{}...{}.{}",
                name_sliced,
                &file_stem[file_stem.len() - 3..].to_string(),
                file_kind
            ),
            None => item_name.clone(),
        };
    }
    }
new_item_name
}