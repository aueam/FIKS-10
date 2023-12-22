use std::fs::read_to_string;

#[derive(Clone, Debug)]
struct Task {
    task_header: TaskHeader,
    devices: Vec<Device>
}

#[derive(Clone, Debug)]
struct TaskHeader {
    h: i16,
    w: i16,
    n: i32
}

#[derive(Clone, Debug)]
struct Device {
    y: i16,
    x: i16,
    c: String
}

impl Task {
    fn solve(&self) -> String {
        let devices_in_correct_order = assign_devices_to_coordinates_on_side(
            find_coordinates_on_sides(self.task_header.h, self.task_header.w),
            &self.devices
        );

        let mut device_route_buffer: Vec<Device> = Vec::new();
        for device in &devices_in_correct_order {

            match device_route_buffer.last() {
                None => device_route_buffer.push(device.clone()),
                Some(last_device) => {
                    if last_device.c == device.c {
                        device_route_buffer.pop();
                    } else {
                        device_route_buffer.push(device.clone());
                    }
                }
            }
        }

        if device_route_buffer.is_empty() {
            return "pujde to".to_owned();
        }
        "ajajaj".to_owned()
    }
}

impl TaskHeader {
    fn new(line: &str) -> Self {

        let numbers = line.split_whitespace();

        let mut h: i16 = 0;
        let mut w: i16 = 0;
        let mut n: i32 = 0;

        for (index, number) in numbers.enumerate() {
            match index {
                0 => h = number.parse::<i16>().unwrap(),
                1 => w = number.parse::<i16>().unwrap(),
                2 => n = number.parse::<i32>().unwrap(),
                _ => panic!()
            }
        }

        if !(1..=10000).contains(&h) {
            panic!("\"h\" is less than 1 or more than 10000")
        }

        if !(1..=10000).contains(&w) {
            panic!("\"w\" is less than 1 or more than 10000")
        }

        if !(1..=40000).contains(&n) {
            panic!("\"n\" is less than 1 or more than 10000")
        }

        Self { h, w, n }
    }
}

impl Device {
    fn new(task: &TaskHeader, line: &str) -> Self {

        let numbers = line.split_whitespace();

        let mut y: i16 = 0;
        let mut x: i16 = 0;
        let mut c: String = String::new();

        for (index, number) in numbers.enumerate() {
            match index {
                0 => y = number.parse::<i16>().unwrap(),
                1 => x = number.parse::<i16>().unwrap(),
                2 => c = number.to_owned(),
                _ => panic!()
            }
        }

        if y < 0 || y > task.h {
            panic!("\"y\" is less than 0 or more than \"h\"")
        }

        if x < 0 || x > task.w {
            panic!("\"y\" is less than 0 or more than \"h\"")
        }


        Self { y, x, c }
    }
}

fn main() {
    let mut lines = Vec::new();

    for line in read_to_string("input.txt").unwrap().lines() {
        lines.push(line.to_string())
    }

    let mut ports = 0;
    let mut tasks: Vec<Task> = Vec::new();
    let mut current_task: Task = Task { task_header: TaskHeader { h: 0, w: 0, n: 0, }, devices: vec![] };
    for (index, line) in lines.iter().enumerate() {
        if index == 0 {
            continue
        }

        if ports == 0 {
            let new_task_header = TaskHeader::new(line);
            current_task = Task { task_header: new_task_header.clone(), devices: vec![] };
            ports = new_task_header.n;
        } else {
            let device = Device::new(&current_task.task_header, line);
            current_task.devices.push(device);
            ports -= 1;

            // save task
            if ports == 0 {
                tasks.push(current_task.clone())
            }
        }
    }

    // solve tasks
    let mut answers: Vec<String> = Vec::new();
    for task in tasks {
        answers.push(task.solve());
    }

    for answer in answers {
        println!("{}", answer)
    }
}

// returns devices in correct order around the room
fn assign_devices_to_coordinates_on_side(coords_on_side: Vec<(i16, i16)>, devices: &Vec<Device>) -> Vec<Device> {
    let mut devices_in_order: Vec<Device> = Vec::new();

    for (y, x) in coords_on_side {
        for device in devices {
            if device.y == y && device.x == x {
                devices_in_order.push(device.clone());
            }
        }
    }

    devices_in_order
}

//                                                    y    x
fn find_coordinates_on_sides(h: i16, w: i16) -> Vec<(i16, i16)> {
    let mut coordinates = Vec::new();

    // TOP SIDE
    for i in 0..w+1 {
        coordinates.push((0, i))
    }

    // RIGHT SIDE
    for i in 0..h+1 {
        coordinates.push((i, w))
    }

    // BOTTOM SIDE
    let mut bottom = Vec::new();
    for i in 0..w+1 {
        bottom.push((h, i))
    }
    bottom.reverse();
    for device in bottom {
        coordinates.push(device)
    }

    // LEFT SIDE
    let mut left = Vec::new();
    for i in 0..h+1 {
        left.push((i, 0))
    }
    left.reverse();
    for device in left {
        coordinates.push(device)
    }

    coordinates.dedup();
    coordinates.pop();
    coordinates
}
