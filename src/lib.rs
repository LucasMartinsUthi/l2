use std::fmt;
use std::io::{stdin};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RobotOrientation {
    North,
    South,
    East,
    West
}

pub enum RobotCommands {
    Foward,
    Backward,
    RightTurn,
    LeftTurn
}

pub struct Robot {
    orientation: RobotOrientation,
    room_size: Vec<i32>,
    x: i32,
    y: i32
}

impl Robot {
    pub fn new(room_size: Vec<i32>) -> Self {
        Self {
            orientation: RobotOrientation::North,
            x: 0,
            y: 0,
            room_size
        }
    }

    pub fn orientation(&self) -> RobotOrientation {
        self.orientation.clone()
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn command(&mut self, command: RobotCommands) {
        match command {
            RobotCommands::Foward => self.walk(1),
            RobotCommands::Backward => self.walk(-1),
            RobotCommands::RightTurn => self.turn_right(),
            RobotCommands::LeftTurn => self.turn_left()
        }
    }

    fn walk(&mut self, step: i32) {
        let (x, y) = match self.orientation {
            RobotOrientation::North => (self.x, self.y + step),
            RobotOrientation::South => (self.x, self.y - step),
            RobotOrientation::East => (self.x + step, self.y),
            RobotOrientation::West => (self.x - step, self.y)
        };

        //check wall 
        if x >=0 && x < self.room_size[0] {
            self.x = x;
        }

        if y >= 0 && y < self.room_size[1] {
            self.y = y;
        }
    }

    fn turn_right(&mut self) {
        self.orientation = match self.orientation {
            RobotOrientation::North => RobotOrientation::East,
            RobotOrientation::South => RobotOrientation::West,
            RobotOrientation::East => RobotOrientation::South,
            RobotOrientation::West => RobotOrientation::North
        }
    }

    fn turn_left(&mut self) {
        self.orientation = match self.orientation {
            RobotOrientation::North => RobotOrientation::West,
            RobotOrientation::South => RobotOrientation::East,
            RobotOrientation::East => RobotOrientation::North,
            RobotOrientation::West => RobotOrientation::South
        }
    }
}

impl fmt::Display for Robot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", match self.orientation {
            RobotOrientation::North => "N",
            RobotOrientation::South => "S",
            RobotOrientation::East => "L",
            RobotOrientation::West => "O"
        }, self.x, self.y, )
    }
}

pub fn read_room_size() -> Vec<i32> {
    println!("Enter room size:");
    let mut read_input = String::new();
    stdin().read_line(&mut read_input).expect("error");

    read_input.trim_end().split(" ").map(|s| s.parse::<i32>().unwrap()).collect()
}

pub fn read_logs() -> Vec<String> {
    println!("Enter Robot Logs:");
    let mut read_input = String::new();
    stdin().read_line(&mut read_input).expect("error");

    read_input.trim().split("").filter(|&s| !s.is_empty()).map(|s| s.to_string()).collect()
}

pub fn command_from_string(command: &str) -> RobotCommands {
    match command {
        "F" => RobotCommands::Foward,
        "T" => RobotCommands::Backward,
        "D" => RobotCommands::RightTurn,
        "E" => RobotCommands::LeftTurn,
        _ => panic!("Invalid command")
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use crate::*;

        let logs = vec!["T", "T", "F", "F", "D", "T", "T", "E", "T", "D", "E", "D", "T", "T", "D", "D", "F", "F", "D", "E", "F", "D", "T", "E", "F", "T", "T", "D", "D", "E", "D", "E", "F", "F", "T", "E", "E", "T", "E", "D", "T", "T", "E", "E", "D", "D", "E", "F", "F", "F", "T", "E", "D", "F", "D", "F", "T", "T", "D", "D", "D", "T", "T", "T", "T", "T", "F", "D", "E", "F", "D", "F", "F", "T", "E", "D", "T", "E", "D", "E", "F", "T", "D", "T", "E", "T", "E", "T", "F", "D", "F", "T", "D", "T", "F", "E", "F", "D", "T", "F", "T", "T", "E", "E", "F", "E", "E", "F", "E", "D", "D", "T", "F", "T", "E", "D", "F", "F", "T", "T", "D", "T", "T", "T", "F", "D", "E", "T", "E", "T", 
        "D", "D", "T", "E", "D", "D", "T", "F", "E", "D", "F", "D", "E", "F", "D", "D", "T", "T", "D", "E", "F", "E", "F", "T", "D", "T", "T", "D", "F", "F", "E", "D", "D", "T", "D", "F", "D", "E", "T", "T", "D", "D", "F", "F", "T", "E", "E", "D", "F", "F", "T", "T", "T", "D", "E", "T", "F", "T", "F", "D", "T", "E", "T", "T", "T", "T", "D", "E", "F", "F", "E", "T", "F", "T", "D", "F", "D", "T", "E", "E", "D", "F", "T", "E", "F", "T", "E", "F", "F", "F", "T", "D", "E", "E", "D", "D", "T", "T", "E", "F", "D", "F", "E", "T", "D", "F", "E", "E", "E", "T", "T", "D", "F", "T", "T", "T", "E", "E", "E", "E", "E", "D", "T", "E", "T", "F", "E", "T", "T", "E", "T", "D", "E", "E", "F", "T", "T", "D", "D", "D", "F", "F", "T", "F", "D", "D", "E", "F", "D", "E", "F", "T", "T", "T", "E", "D", "F", "F", "F", "T", "E", "E", "D", "T", "D", "D", "T", "T", "D", "F", "D", "E", "E", "T", "T", "D", "E", "T", "E", "T", "D", "D", "E", "T", "T", "T", "D", "E", "F", "E", "E", "T", "T", "E", "T", "D", "E", "D", "T", "E", "F", "T", "T", "D", "D", "E", "E", "F", "D", "F", "D", "T", "E", "E", "E", "D", "E", 
        "D", "T", "D", "D", "T", "E", "E", "E", "E", "E", "D", "T", "F", "E", "D", "E", "D", "D", "D", "T", "T", "T", "F", "F", "D", "E", "T", "F", "F", "D", "D", "T", "F", "D", "T", "E", "F", "F", "F", "D", "D", "F", "E", "E", "F", "E", "F", "D", "D", "F", "D", "T", "T", "E", "T", "T", "D", "T", "D", "T", "T", "T", "D", "E", "D", "F", "T", "T", "D", "D", "D", "T", "T", "T", "T", "T", "D", "T", "E", "D", "F", "E", "F", "D", "T", "F", "F", "E", "T", "E", "E", "E", "F", "F", "F", "E", "F", "D", "T", "T", "D", "E", "T", "T", "F", "F", "E", "T", "T", "D", "E", "E", "T", "F", "E", "D", "T", "E", "F", "F", "D", "E", "E", "F", "E", "E", "D", "T", "T", "F", "F", "T", "E", "E", "E", "T", "D", "T", "E", "E", "E", "F", "T", "F", "F", "E", "F", "E", "F", "D", "F", "F", "E", "F", "T", "F", "E", "D", "E", "E", "D", "F", "D", "D", "D", "F", "D", "F", "E", "F", "E", "D", "D", "D", "T", "E", "E", "T", "T", "E", "T", "F", "T", "T", "F", "D", "T", "D", "E", "F", "T", "F", "E", "E", "D", "E", "F", "T", "E", "F", "E", "T", "F", "E", "E", "E", "F", "D", "E", "T", "E", "D", "F", "F", "T", "F", "D", 
        "F", "F", "T", "F", "D", "T", "F", "E", "F", "D", "E", "D", "T", "T", "T", "F", "E", "D", "E", "F", "T", "T", "D", "D", "E", "E", "F", "D", "T", "F", "T", "E", "E", "D", "T", "E", "T", "E", "F", "E", "F", "F", "E", "E", "E", "F", "F", "T", "F", "E", "E", "T", "E", "D", "D", "F", "F", "F", "T", "F", "D", "E", "T", "F", "D", "E", "F", "E", "T", "D", "T", "F", "D", "D", "F", "F", "T", "F", "D", "T", "F", "T", "T", "F", "T", "T", "F", "D", "F", "D", "D", "E", "D", "E", "F", "F", "F", "E", "T", "T", "T", "T", "F", "F", "F", "F", "T", "D", "E", "E", "E", "F", "T", "D", "F", "D", "D", "F", "T", "F", "E", "T", "D", "D", "E", "D", "F", "F", "F", "E", "F", "F", "D", "D", "D", "T", "T", "D", "E", "F", "E", "T", "F", "D", "E", "F", "D", "E", "D", "E", "T", "D", "F", "E", "E", "T", "E", "D", "T", "F", "E", "T", "T", "T", "F", "E", "E", "E", "F", "T", "E", "T", "F", "F", "T", "D", "F", "D", "F", "E", "D", "F", "T", "D", "D", "T", "E", "F", "T", "F", "F", "E", "F", "E", "E", "F", "T", "T", "T", "F", "E", "E", "F", "T", "D", "D", "D", "E", "E", "D", "T", "F", "T", "F", "D", "T", "T", 
        "D", "F", "D", "E", "E", "T", "F", "F", "T", "D", "F", "D", "E", "D", "E", "E", "F", "F", "D", "T", "F", "E", "T", "E", "D", "E", "E", "T", "T", "E", "F", "F", "F", "D", "E", "T", "T", "T", "F", "T", "E", "E", "T", "F", "D", "T", "E", "F", "F", "E", "E", "F", "F", "D", "E", "D", "F", "F", "D", "T", "T", "E", "F", "D", "E", "T", "D", "D", "E", "T", "D", "D", "D", "T", "D", "F", "E", "T", "T", "T", "E", "T", "E", "E", "E", "F", "D", "F", "T", "E", "E", "T", "D", "T", "E", "D", "E", "T", "T", "E", "D", "E", "T", "F", "E", "E", "E", "F", "T", "F", "F", "E", "E", "D", "T", "F", "F", "F", "T", "E", "D", "D", "F", "D", "D", "F", "D", "D", "T", "T", "E", "D", "E", "E", "T", "E", "T", "E", "E", "F", "D", "D", "D", "T", "T", "T", "T", "T", "E", "E", "F", "E", "F", "F", "F", "T", "E", "F", "E", "D", "E", "F", "E", "F", "E", "F", "E", "D", "D", "F", "F", "E", "E", "E", "F", "E", "E", "T", "T", "F", "D", "F", "E", "E", "D", "D", "T", "T", "T", "T", "E", "T", "D", "T", "E", "D", "E", "F", "F", "D", "F", "T", "T", "F", "F", "T", "D", "T", "E", "D", "E", "D", "E", "D", "D", "F", "E", 
        "F", "T"];

        let logs = logs.iter().map(|s| s.to_string()).collect::<Vec<String>>();

        let mut robot = Robot::new(vec![50, 50]);

        for log in logs {
            robot.command(command_from_string(&log));
        }

        assert_eq!(robot.position(), (11, 13));
        assert_eq!(robot.orientation(), RobotOrientation::South);
    }

    #[test]
    fn it_works2() {
        use crate::*;

        let logs = vec!["F", "F", "F", "F", "F", "F", "F", "F", "F", "F", "F", "F", "F", "F", "F", "F", "F", "F", "F", "F", "D", "F", "E", "T", "T", "T", "T", "T", "T", "T", "T", "T", "T", "T", "T", "T", "T", "T", "T", "T", "T", "T", "T", "T", "T", "T", "T", "T"];
        let logs = logs.iter().map(|s| s.to_string()).collect::<Vec<String>>();

        let mut robot = Robot::new(vec![15, 36]);

        for log in logs {
            robot.command(command_from_string(&log));
        }

        assert_eq!(robot.position(), (1, 0));
        assert_eq!(robot.orientation(), RobotOrientation::North);
    }
}