use acc_app::file_handlers::txt_handlers::process_data_from_file;
use rfd::FileDialog;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {


    let ui = AppWindow::new()?;
    // ui.on_request_increase_value({
    //     let ui_handle = ui.as_weak();
    //     move || {
    //         let ui = ui_handle.unwrap();
    //         ui.set_counter(ui.get_counter() + 1);
    //     }
    // });

    ui.on_request_acc_data({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            
            let shared = ui.get_filepath();
            
            let filepath = shared.as_str();
            println!("{}", filepath);
            
            let res = process_data_from_file(filepath);
            println!("{}",res);
        }
    });

    ui.on_choose_file({  
        let ui_handle = ui.as_weak();
        move || {
            let filename:String;
            let mut filepath:String = String::new();
            // Open a file dialog and get the selected file path
            if let Some(file_path) = FileDialog::new().pick_file() {
                if let Some(name) = file_path.file_name() {
                    // Convert OsStr to String
                    filename = name.to_string_lossy().into_owned();
                    filepath = file_path.to_string_lossy().into_owned();
                } else {
                    // Handle case where file name can't be extracted
                    filename = "Unknown file".into();
                }
            } else {
                // Handle file selection cancellation
                filename = "Error choosing a file".into();
            }

            let ui = ui_handle.unwrap();
            ui.set_filename(filename.into()); 
            ui.set_filepath(filepath.into()); 
        }
    });

    ui.run()
}

