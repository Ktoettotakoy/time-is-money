use acc_app::file_handlers::txt_handlers::process_data_from_file;

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
        move || {
            let res = process_data_from_file("./src/data/acc.txt");
            println!("{}",res);
        }
    });

    ui.run()
}

