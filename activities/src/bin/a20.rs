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
    fn new(state: &str) -> Option<PowerState> {
        match state.trim().to_lowercase().as_str() {
            "off" => Some(PowerState::Off),
            "sleep" => Some(PowerState::Sleep),
            "reboot" => Some(PowerState::Reboot),
            "shutdown" => Some(PowerState::Shutdown),
            "hibernate" => Some(PowerState::Hibernate),
            _ => return None,
        }
    }

    fn print_power_action(power_state: PowerState) {
        use PowerState::*;
        match power_state {
            Off => println!("turning off"),
            Sleep => println!("entering sleep mode"),
            Reboot => println!("initiating reboot"),
            Shutdown => println!("shutting down"),
            Hibernate => println!("going into hibernate mode"),
        }
    }
}

fn main() {
    println!("Enter a state:");

    let mut buffer = String::new();
    let user_input_status = io::stdin().read_line(&mut buffer);
    if user_input_status.is_ok() {
        match PowerState::new(&buffer) {
            Some(power_state) => PowerState::print_power_action(power_state),
            None => println!("invalid power state"),
        }
    } else {
        println!("error reading input");
    }
}

#[cfg(test)]
mod test {
    use crate::PowerState;

    #[test]
    fn test_from_string() {
        let result = PowerState::new("OFF");
        let expected = Some(PowerState::Off);
        assert_eq!(expected, result);
    }
}
