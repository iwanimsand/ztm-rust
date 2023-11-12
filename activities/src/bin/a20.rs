// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)

use std::io;

#[derive(PartialEq, Debug)]
enum PowerState {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

impl PowerState {
    fn from_string(text: &str) -> Option<PowerState> {
        match text.to_lowercase().trim() {
            "off" => Some(PowerState::Off),
            "sleep" => Some(PowerState::Sleep),
            "reboot" => Some(PowerState::Reboot),
            "shutdown" => Some(PowerState::Shutdown),
            "hibernate" => Some(PowerState::Hibernate),
            _ => return None,
        }
    }

    fn execute(power_state: PowerState) {
        match power_state {
            PowerState::Off => println!("turning off"),
            PowerState::Sleep => println!("entering sleep mode"),
            PowerState::Reboot => println!("initiating reboot"),
            PowerState::Shutdown => println!("shutting down"),
            PowerState::Hibernate => println!("going into hibernate mode"),
        }
    }
}

fn main() {
    println!("Enter a state:");

    let mut buffer = String::new();
    let result = io::stdin().read_line(&mut buffer);
    match result {
        Ok(_) => {
            let power_state = PowerState::from_string(&buffer);
            match power_state {
                Some(power_state) => PowerState::execute(power_state),
                None => println!("I'm not familiar with that state"),
            }
        }
        Err(e) => println!("{:?}", e),
    }
}

#[cfg(test)]
mod test {
    use crate::PowerState;

    #[test]
    fn test_from_string() {
        let result = PowerState::from_string("OFF");
        let expected = Some(PowerState::Off);
        assert_eq!(expected, result);
    }
}
