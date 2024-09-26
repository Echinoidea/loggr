use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timesheet {
    pub name: String,
    pub entries: Vec<TimesheetEntry>,
}

impl Timesheet {
    pub fn new(name: String, entries: Vec<TimesheetEntry>) -> Timesheet {
        Timesheet { name, entries }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimesheetEntry {
    pub date: String,
    pub time_in: String,
    pub time_out: String,
}

// todo)) implement entry description
impl TimesheetEntry {
    pub fn new(date: String, time_in: String, time_out: String) -> TimesheetEntry {
        TimesheetEntry {
            date,
            time_in,
            time_out,
        }
    }
}
