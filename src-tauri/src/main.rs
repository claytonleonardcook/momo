// ! Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rusqlite::{Connection, Result};
use include_sqlite_sql::{include_sql, impl_sql};

include_sql!("sql/Person.sql");

#[tauri::command]
fn greet(name: &str) -> Result<String, String> {
    if name.is_empty() {
        return Err("Provide a valid name!".into());
    }

    let conn = Connection::open_in_memory().map_err(|_err| "Couldn't connect to database!")?;

    conn.create_person_table().map_err(|_err| "Couldn't create person table!")?;

    conn.insert_person("Steven").map_err(|_err| "Couldn't insert Steven!")?;

    conn.get_all_people(|row| {
        let name : &str = row.get_ref("name")?.as_str()?;

        print!("{}", name);

        Ok(())
    }).map_err(|_err| "Couldn't get all people!")?;
    
    Ok(format!("Hello, {}! You've been greeted from Rust!", name))
}

fn main() -> Result<()> {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
