use std::io;
fn main(){
   println!("###Welcome to my personal temperature converter###");

   loop {
          println!("Enter 'C' if you want to change from Farenheit to Celcius and 'F' if you want to convert from Celcius to Farenheit");
   
         let mut choice = String::new();
         io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line!");

         let choice:char = choice.trim().parse().expect("Wrong input!");
         if choice == 'C'
         {
            println!("+Input the value of the temperature you'd like to convert to Celcius:");
            let mut value_farenheit = String::new();
            io::stdin()
               .read_line(&mut value_farenheit)
               .expect("Failed to read line!");
            let value_farenheit: f64 = value_farenheit
               .trim().parse().expect("Invalid Input");
            
            let results_farenheit = (value_farenheit - 32.0) * 5.0 / 9.0;
            println!("+ {value_farenheit} in celcius is {results_farenheit} in celcius");
            break;
         }

         else if choice == 'F'
         {
            println!("+Input the value of the temperature you'd like to convert to Farenheit");
            let mut value_celcius = String::new();
            io::stdin()
               .read_line(&mut value_celcius)
               .expect("Failed to read line!");
            let value_celcius: f64 = value_celcius
               .trim().parse().expect("Invalid Input");
            

            let results_celcius = value_celcius * (9.0 / 5.0) + 32.0;
            format!("{:.2}", results_celcius);
            println!("+ {value_celcius} in celcius is {results_celcius} in Farenheit");
            break;
         }     

         else {
            println!("Not a valid option, try again");
         }
   }


}