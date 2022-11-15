use l2::*;
fn main() {
    let room_size = read_room_size();
    let logs = read_logs();

    let mut robot = Robot::new(room_size);

    for log in logs {
        robot.command(command_from_string(&log));
    }

    println!("{}", robot);
}