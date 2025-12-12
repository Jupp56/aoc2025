#[derive(Clone, Debug, PartialEq, Eq)]
struct Device {
    name: String,
    outputs: Vec<String>,
}

fn main() {
    let input = include_str!("../input/real");

    let lines: Vec<&str> = input.lines().collect();

    let mut devices = Vec::new();

    for line in lines {
        let mut s = line.split(": ");
        let name = s.next().unwrap().to_owned();

        let outputs: Vec<String> = s.next().unwrap().split(" ").map(|x| x.to_owned()).collect();

        devices.push(Device { name, outputs });
    }

    part_1(&devices);
}

fn part_1(devices: &[Device]) {
    let current_device = devices.iter().find(|x| x.name == "you").unwrap();

    let res = find_paths(current_device, devices, &mut Vec::new());

    println!("Found {res} paths");
}

fn find_paths(
    current_device: &Device,
    devices: &[Device],
    visited_devices: &mut Vec<Device>,
) -> usize {
    if visited_devices.contains(current_device) {
        return 0;
    }

    visited_devices.push(current_device.clone());

    let devices_len = visited_devices.len();

    let mut counter = 0;
    for device in &current_device.outputs {
        if device == "out" {
            counter += 1;
            continue;
        }

        counter += find_paths(
            devices.iter().find(|x| x.name == *device).unwrap(),
            devices,
            visited_devices,
        );

        visited_devices.truncate(devices_len);
    }

    counter
}
