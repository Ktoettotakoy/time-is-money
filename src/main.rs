use acc_app::file_handlers::txt_handlers::show_debug_data_from_file;
use acc_app::file_handlers::txt_handlers::transform_file_into_me_struct;
use acc_app::file_handlers::xls_handlers::xls_perform_workbook_update;
use rfd::FileDialog;

slint::include_modules!();

// hardcoded and exposed, yep
const DESTINATION_FOR_SAVED_SPREADSHEET: &str = "/Users/yaroslav.k0/Documents/myAccAppFileStorage"; // filepath where tmp files are going to be stored

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

            let tmp = ui.get_filepath();
            let filepath = tmp.as_str();

            println!("{}", filepath);
            let res = show_debug_data_from_file(filepath);
            println!("{}",res);
        }
    });

    // put monthly expenses into excel hook
    ui.on_put_me_into_excel({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();

            let tmp = ui.get_filepath();
            let filepath = tmp.as_str();

            if let Some(me) = transform_file_into_me_struct(filepath){
                let _success = xls_perform_workbook_update(me, DESTINATION_FOR_SAVED_SPREADSHEET);
            }
            // not finished
            // add response on the frontend
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
