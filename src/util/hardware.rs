use bytesize::ByteSize;
use sysinfo::{CpuExt, DiskExt, NetworkExt,   System, SystemExt};
use crate::domain::vo::{CpuVO, DiskVO, Memory, ServerVO, SysVO};


pub fn get_server_info() -> ServerVO {
    let mut sys = System::new_all();

// First we update all information of our `System` struct.
    sys.refresh_all();

// We display all disks' information:
    //   println!("=> disks:");
    let mut sys_file = vec![];
    for d in sys.disks() {
        let disk = DiskVO {
            dir_name: d.name().to_str().unwrap().to_string(),
            free: ByteSize::b(d.available_space()).to_string(),
            sys_type_name: String::from_utf8(d.file_system().to_vec()).unwrap(),
            type_name: d.mount_point().to_str().unwrap().to_string(),
            total: ByteSize::b(d.total_space()).to_string(),
            used: ByteSize::b(d.total_space() - d.available_space()).to_string(),
            usage: format!("{:.1}", 100.0 - 100.0 * (d.total_space() - d.available_space()) as f64 / d.total_space() as f64),
        };
        sys_file.push(disk);
        //     println!("{:?}", disk);
        //   println!("{:?}", disk_);
    }

// Network interfaces name, data received and data transmitted:
  //   println!("=> networks:");
    for (interface_name, data) in sys.networks() {
        println!("{}: {}/{} B", interface_name, data.received(), data.transmitted());
    }

// Components temperature:
    println!("=> components:");
    for component in sys.components() {
        println!("{:?}", component);
    }

    let mem = Memory {
        free: ByteSize::b(sys.used_memory()).to_string().trim_end_matches(" GB").to_string(),//fixme .trim_end_matches(" GB").to_string()为了兼容前端，以后需要删除
        total: ByteSize::b(sys.total_memory()).to_string().trim_end_matches(" GB").to_string(),
        used: ByteSize::b(sys.total_memory() - sys.used_memory()).to_string().trim_end_matches(" GB").to_string(),
        usage: format!("{:.1}", 100.0 - 100.0 * sys.used_memory() as f64 / sys.total_memory() as f64),
    };
   // println!("{:?}", mem);


    sys.refresh_cpu();
    let cpu_usage = sys.global_cpu_info().cpu_usage();
    let cpu = CpuVO {
        cpu_num: sys.physical_core_count().unwrap() as u8,
        free: 100.0 - cpu_usage,
        sys: 0.0,
        total: 0,
        used: cpu_usage,
        wait: 0.0,
    };
    println!("{:?}", cpu);
    let sys_vo = SysVO {
        computer_ip: "".to_string(),
        computer_name: sys.host_name().unwrap(),
        os_arch: "".to_string(),
        os_name: format!("{} {}", sys.name().unwrap(), sys.os_version().unwrap()),
    };
    println!("{:?}", sys_vo);


    ServerVO {
        cpu,
        mem,
        sys: sys_vo,
        sys_files: sys_file,
    }
// Number of CPUs:
    //   println!("NB CPUs: {}", sys.cpus().len());

// Display processes ID, name na disk usage:
//     for (pid, process) in sys.processes() {
//         println!("[{}] {} {:?}", pid, process.name(), process.disk_usage());
//     }

    // let sys = System::new();
    // //let sys_files = vec![];
    // match sys.mounts() {
    //     Ok(mounts) => {
    //         println!("\nMounts:");
    //         for mount in mounts.iter() {
    //             let disk = Disk {
    //                 dir_name: mount.fs_mounted_on.clone(),
    //                 free:mount.free.to_string(),
    //                 sys_type_name: mount.fs_type.clone(),
    //                 type_name: mount.fs_mounted_from.clone(),
    //                 total:mount.total.to_string(),
    //                 used: saturating_sub_bytes(mount.total, mount.free).to_string(),
    //                 usage: format!("{:.1}", 100.0 - 100.0 * (saturating_sub_bytes(mount.total, mount.free).as_u64() as f64) / mount.total.as_u64() as f64),
    //             };
    //             println!("{:?}", disk);
    //             // println!("{} ---{}---> {} (available {} of {})",
    //             //          mount.fs_mounted_from, mount.fs_type, mount.fs_mounted_on, mount.avail, mount.total);
    //         }
    //     }
    //     Err(x) => println!("\nMounts: error: {}", x)
    // }
    //
    // match sys.mount_at("/") {
    //     Ok(mount) => {
    //         println!("\nMount at /:");
    //         println!("{} ---{}---> {} (available {} of {})",
    //                  mount.fs_mounted_from, mount.fs_type, mount.fs_mounted_on, mount.avail, mount.total);
    //     }
    //     Err(x) => println!("\nMount at /: error: {}", x)
    // }
    //
    // match sys.block_device_statistics() {
    //     Ok(stats) => {
    //         for blkstats in stats.values() {
    //             println!("{}: {:?}", blkstats.name, blkstats);
    //         }
    //     }
    //     Err(x) => println!("\nBlock statistics error: {}", x)
    // }
    //
    // match sys.networks() {
    //     Ok(netifs) => {
    //         println!("\nNetworks:");
    //         for netif in netifs.values() {
    //             println!("{} ({:?})", netif.name, netif.addrs);
    //         }
    //     }
    //     Err(x) => println!("\nNetworks: error: {}", x)
    // }
    //
    // match sys.networks() {
    //     Ok(netifs) => {
    //         println!("\nNetwork interface statistics:");
    //         for netif in netifs.values() {
    //             println!("{} statistics: ({:?})", netif.name, sys.network_stats(&netif.name));
    //         }
    //     }
    //     Err(x) => println!("\nNetworks: error: {}", x)
    // }
    //
    // // match sys.battery_life() {
    // //     Ok(battery) =>
    // //         print!("\nBattery: {}%, {}h{}m remaining",
    // //                battery.remaining_capacity * 100.0,
    // //                battery.remaining_time.as_secs() / 3600,
    // //                battery.remaining_time.as_secs() % 60),
    // //     Err(x) => print!("\nBattery: error: {}", x)
    // // }
    // //
    // // match sys.on_ac_power() {
    // //     Ok(power) => println!(", AC power: {}", power),
    // //     Err(x) => println!(", AC power: error: {}", x)
    // // }
    //
    // match sys.memory() {
    //     Ok(mem) => {
    //
    //         let mem = Memory {
    //             free:mem.free.to_string(),
    //             total:mem.total.to_string(),
    //             used: saturating_sub_bytes(mem.total, mem.free).to_string(),
    //             usage: format!("{:.1}", 100.0 - 100.0 * (saturating_sub_bytes(mem.total, mem.free).as_u64() as f64) / mem.total.as_u64() as f64),
    //         };
    //         println!("{:?}", mem);
    //         //println!("\nMemory: {} used / {} ({} bytes) total ({:?})", saturating_sub_bytes(mem.total, mem.free), mem.total, mem.total.as_u64(), mem.platform_memory)
    //     }
    //     Err(x) => println!("\nMemory: error: {}", x)
    // }
    //
    // match sys.swap() {
    //     Ok(swap) => println!("\nSwap: {} used / {} ({} bytes) total ({:?})", saturating_sub_bytes(swap.total, swap.free), swap.total, swap.total.as_u64(), swap.platform_swap),
    //     Err(x) => println!("\nSwap: error: {}", x)
    // }
    //
    // match sys.load_average() {
    //     Ok(loadavg) => println!("\nLoad average: {} {} {}", loadavg.one, loadavg.five, loadavg.fifteen),
    //     Err(x) => println!("\nLoad average: error: {}", x)
    // }
    //
    // match sys.uptime() {
    //     Ok(uptime) => println!("\nUptime: {:?}", uptime),
    //     Err(x) => println!("\nUptime: error: {}", x)
    // }
    //
    // match sys.boot_time() {
    //     Ok(boot_time) => println!("\nBoot time: {}", boot_time),
    //     Err(x) => println!("\nBoot time: error: {}", x)
    // }
    //
    // match sys.cpu_load_aggregate() {
    //     Ok(cpu) => {
    //         println!("\nMeasuring CPU load...");
    //         thread::sleep(Duration::from_secs(1));
    //         let cpu = cpu.done().unwrap();
    //
    //         println!("CPU load: {}% user, {}% nice, {}% system, {}% intr, {}% idle ",
    //                  cpu.user * 100.0, cpu.nice * 100.0, cpu.system * 100.0, cpu.interrupt * 100.0, cpu.idle * 100.0);
    //         let cpu=Cpu{
    //             cpu_num: 0,
    //             free: cpu.idle * 100.0,
    //             sys:  cpu.system * 100.0,
    //             total: 0,
    //             used: cpu.user * 100.0,
    //             wait: 0.0,
    //         };
    //         println!("{:?}",cpu);
    //
    //     }
    //     Err(x) => println!("\nCPU load: error: {}", x)
    // }
    //
    // match sys.cpu_temp() {
    //     Ok(cpu_temp) => println!("\nCPU temp: {}", cpu_temp),
    //     Err(x) => println!("\nCPU temp: {}", x)
    // }
    //
    // match sys.socket_stats() {
    //     Ok(stats) => println!("\nSystem socket statistics: {:?}", stats),
    //     Err(x) => println!("\nSystem socket statistics: error: {}", x)
    // }
}

