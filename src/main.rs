use calculator::input;
use calculator::calculation;

fn main() {
    loop{
        println!(" ******Welcome User******");

        //input for n1
        println!("Enter num1: ");
        let n1: f32 = input::input().expect("Invalid input. Please try again.");
        println!("You entered: {:.1}", &n1);

        //input for n2
        println!("Enter num1: ");
        let n2: f32 = input::input().expect("Invalid input. Please try again.");
        println!("You entered: {:.1}", &n2);

        println!("Enter the operation (+, -, *, /):");
        let operation: char = input::input_op().expect("Invalid input. Please try again.");
        println!("You entered: {}", &operation);

        match calculation(n1, n2, operation) {
            Ok(result) => println!("{}", result),
            Err(error) => eprintln!("{}", error),
        }
        
         // Check for exit condition
         println!("Do you want to exit? (yes/no)");
         let exit_command : String = input::input().unwrap(); // This will extract the user's input from the Result
         
         if exit_command.trim().to_lowercase() == "yes" {
             break; // Exit the loop
         }
    }
}




