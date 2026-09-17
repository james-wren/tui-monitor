use std::vec;
use sysinfo::{Disks, System};

// Function to get any needed system info (ram or cpu for now)
pub fn get_info(name: &str, mut sys: System) -> Vec<u64>{
    sys.refresh_all(); // Refreshes system to get updated data

    // Returns appropriate live data based on the chosen component
    match name {
        "ram" => vec![sys.total_memory(), sys.used_memory()],
        "cpu" => vec![(sys.global_cpu_usage() * 100.0) as u64],
        _ => vec![0]
    }
}

// A struct contaning disk info data
pub struct DiskStruct {
    pub name: String,
    pub total: u64,
    pub available: u64
}

// Function that returns a list of disk info structs
pub fn get_disks() -> Vec<DiskStruct> {
    let disks = Disks::new_with_refreshed_list(); // Makes a list of updated disks
    
    let mut disk_list = vec![]; //Initializing list to be returned

    // Loops through the list of disks and creates a struct based on them
    for disk in &disks {
        let disk_info = DiskStruct {
            name: disk.name().to_string_lossy().to_string(),
            total: disk.total_space(),
            available: disk.available_space()
        };

        disk_list.push(disk_info); // Adds the struct to the list of structs
    }

    disk_list // Returns the list
}

// A struct containing process info data
pub struct ProccessStruct {
    pub name: String,
    pub disk_usage: u64,
    pub cpu_usage: u64,
    pub ram_usage: u64,
    pub pid: u32
}

// Function to get a list of new proccess structs
pub fn get_proccesses(mut sys: System) -> Vec<ProccessStruct> {
    sys.refresh_all(); // refreshes system to get new data

    let mut process_list = vec![]; // Initializes lsit of proccess struts

    // Loops through each process and adds it to a struct
    for (pid, process) in sys.processes() {
        let process_info = ProccessStruct {
            name: process.name().to_string_lossy().to_string(),
            disk_usage: process.disk_usage().total_written_bytes,
            cpu_usage: (process.cpu_usage()* 100.0) as u64,
            ram_usage: process.memory(),
            pid: pid.as_u32(),
        };

        process_list.push(process_info); // Adds the struct to the list
    }

    process_list // Returns the list
}