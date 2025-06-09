pub trait Command {
    fn execute(&self);
}

pub struct CleaningRobot {}
pub struct DeliveryRobot {}
pub struct SecurityRobot {}

impl Command for CleaningRobot {
    fn execute(&self) {
        println!("CleaningRobot will start cleaning the house.");
    }
}

impl Command for DeliveryRobot {
    fn execute(&self) {
        println!("DeliveryRobot has left for delivery.");
    }
}

impl Command for SecurityRobot {
    fn execute(&self) {
        println!("SecurityRobot is always alert.");
    }
}

pub struct CommandCenter<T> {
    robot: T,
}

impl<T: Command> CommandCenter<T> {
    fn start(&self) { // Translated "iniciar" to "start"
        self.robot.execute();
    }
}

mod tests {
    use super::*;
    
    #[test]
    fn test() {
        let cleaning_robot = CleaningRobot {};
    let security_robot = SecurityRobot {};
    let delivery_robot = DeliveryRobot {};

    let command_center_cleaning = CommandCenter { robot: cleaning_robot };
    let command_center_delivery = CommandCenter { robot: delivery_robot };
    let command_center_security = CommandCenter { robot: security_robot };

    command_center_cleaning.start();
    command_center_delivery.start();
    command_center_security.start();
    assert!(true);

    }
}
    
