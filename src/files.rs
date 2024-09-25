use std::fs;
use std::fs::File;
use std::io::prelude::*;

use serde_json::{Result, Value};

use crate::timesheet::{Timesheet, TimesheetEntry};

pub fn make_loggr_dir() -> std::io::Result<()> {
    if assert_loggr_dir() {
        return Ok(());
    }

    // Change to home env
    let _f = std::fs::create_dir("/home/gabriel/.loggr")?;
    Ok(())
}

fn assert_loggr_dir() -> bool {
    let path = std::path::Path::new("/home/gabriel/.loggr/");
    return path.exists() && path.is_dir();
}

pub fn load_timesheet(timesheet_name: String) -> Result<Timesheet> {
    let content = fs::read_to_string(format!("/home/gabriel/.loggr/{timesheet_name}.csv"))
        .expect("Should have been able to read the file");
    let loaded: Value = serde_json::from_str(&content)?;

    let mut entry_arr: Vec<TimesheetEntry> = vec![];
    let value_vec: Vec<Value> = loaded["entries"].as_array().unwrap().to_vec();

    for entry_index in 0..value_vec.len() {
        let val = value_vec[entry_index].clone();
        entry_arr.push(TimesheetEntry::new(
            val["date"].to_string(),
            val["time_in"].to_string(),
            val["time_out"].to_string(),
        ));
    }

    let parsed = Timesheet::new(timesheet_name, entry_arr);
    Ok(parsed)
}

pub fn save_timesheet(timesheet: Timesheet) -> std::io::Result<()> {
    let timesheet_name = &timesheet.name;
    let mut file = File::create(format!("/home/gabriel/.loggr/{timesheet_name}.csv"))?;

    let serialized = serde_json::to_string(&timesheet).unwrap();
    file.write_all(serialized.as_bytes())?;
    Ok(())
}
