use chrono::{TimeZone, Utc};
use humansize::BINARY;
use serde_derive::Serialize;
use std::time::Duration;
use sysinfo::{Disks, System};

#[derive(Serialize)]
pub struct Status {
    status: i8,
    description: String,
    name: String,
    host_name: String,
    os_version: String,
    kernel: String,
    uptime: String,
    boot_time: String,
    cpu: CPU,
    mem: Mem,
    disks: Vec<Disk>,
}

#[derive(Serialize)]
struct CPU {
    cpus: usize,
    global_cpu_usage: f32,
    load1: f64,
    load5: f64,
    load15: f64,
}

#[derive(Serialize)]
struct Mem {
    total: String,
    used: String,
    free: String,
    available: String,
}

#[derive(Serialize)]
struct Disk {
    name: String,
    kind: String,
    mount_point: String,
    file_system: String,
    is_removable: bool,
    total: String,
    available: String,
}

impl Status {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        let formatter = humansize::make_format(BINARY);
        let load_avg = System::load_average();
        let disks = Disks::new_with_refreshed_list();
        let mut disk_list: Vec<Disk> = Vec::new();
        for d in disks.list() {
            let disk = Disk {
                name: d.name().to_string_lossy().into_owned(),
                kind: d.kind().to_string(),
                mount_point: d.mount_point().to_string_lossy().into_owned(),
                file_system: d.file_system().to_string_lossy().into_owned(),
                is_removable: d.is_removable(),
                total: formatter(d.total_space()),
                available: formatter(d.available_space()),
            };
            disk_list.push(disk);
        }

        let uptime = humantime::format_duration(Duration::new(System::uptime(), 0)).to_string();

        let boot_time = Utc
            .timestamp_opt(System::boot_time() as i64, 0)
            .unwrap()
            .to_rfc3339();

        Self {
            status: 0,
            description: String::from("running"),
            name: System::name().unwrap(),
            host_name: System::host_name().unwrap(),
            os_version: System::long_os_version().unwrap(),
            kernel: System::kernel_version().unwrap(),
            uptime,
            boot_time,
            cpu: CPU {
                cpus: sys.cpus().len(),
                global_cpu_usage: sys.global_cpu_usage(),
                load1: load_avg.one,
                load5: load_avg.five,
                load15: load_avg.fifteen,
            },
            mem: Mem {
                total: formatter(sys.total_memory()),
                used: formatter(sys.used_memory()),
                free: formatter(sys.free_memory()),
                available: formatter(sys.available_memory()),
            },
            disks: disk_list,
        }
    }
}

pub fn get_status() -> Status {
    Status::new()
}
