#![allow(dead_code)]
#![allow(unused_imports)]

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
    Custom(u8, u8, u8)
}

enum Message {
    Text(String),
    Number(i32),
    Warning
}

fn describe_color(color: Color) -> String {
    let prefix = "Primary color:";
    match color {
        Color::Red | Color::Green | Color::Blue => {
             format!("{} {:?}", prefix, color)
        }
        Color::Custom(r,g,b) => { 
            format!("Custom color: RGB({}, {}, {})", r, g, b)
        }
    }
}

fn process_message(msg: Message) -> String {
    match msg {
        Message::Text(text) =>  format!("Text: {}", text) ,
        Message::Number(number) =>  format!("Number: {}", number) ,
        Message::Warning =>  "Warning received!".to_string() 
    }
}


mod tests {
    use crate::beginner::challenge_03::{describe_color, process_message, Color, Message};

    #[test]
    fn test_colors() {
        let red = Color::Red;
        let result = describe_color(red);
        assert_eq!(result, "Primary color: Red");

        let green = Color::Green;
        let result = describe_color(green);
        assert_eq!(result, "Primary color: Green");

        let blue = Color::Blue;
        let result = describe_color(blue);
        assert_eq!(result, "Primary color: Blue");

        let custom = Color::Custom(100, 150, 200);
        let result = describe_color(custom);
        assert_eq!(result, "Custom color: RGB(100, 150, 200)");
    }

    #[test]
    fn test_messages() {
        let msg = Message::Text("Test".to_string());
        let result = process_message(msg);
        assert_eq!(result, "Text: Test");

        let msg = Message::Number(10);
        let result = process_message(msg);
        assert_eq!(result, "Number: 10");

        let msg = Message::Warning;
        let result = process_message(msg);
        assert_eq!(result, "Warning received!");
    }

}
