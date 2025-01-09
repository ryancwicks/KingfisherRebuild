//! Main Tauri application stratup.
use tauri::Builder;

//mod system_status;
mod dds_topics;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {

    Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup( |_app| {
            dds_topics::setup_dds_topics();
            //app.manage(system_status::setup_app_state());
            Ok(())
        })
        //.invoke_handler(tauri::generate_handler![system_status::connect_dds_topics])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
