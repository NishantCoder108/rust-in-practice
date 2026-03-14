// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
//
// The exact form of this will be:
// - The input is going to be a Vector of 2-length tuples,
//   the first element is the string, the second one is the command.
// - The output element is going to be a vector of strings.

#[derive(Debug)]
enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: Complete the function as described above.
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        // let (text, command) = input;
        // match input {
        //     Command::Uppercase -> "Hello".to_string(),
        // }

        let mut output = Vec::new();
        /**
         * Interate over vector like arr
         * Separate string in one variable
         * create vector variable outside of for loop
         * Store data in that varible and return vect as a string
         *
         */
        for str_command in input {
            println!("{str_command:?}");
            let (str_val, command_val) = str_command;

            // match command_val {
            //     Command::Uppercase => {
            //         let upper = str_val.to_uppercase();
            //         output.push(upper);
            //     }
            //     Command::Trim => {
            //         let trim_val = str_val.trim();
            //         output.push(trim_val.to_string());
            //     }
            //     Command::Append(a) => {
            //         let append_val = str_val + &"bar".to_string().repeat(a);
            //         print!("{append_val}");
            //         output.push(append_val);
            //     }
            // }

            let new_string = match command_val {
                Command::Uppercase => str_val.to_uppercase(),
                Command::Trim => str_val.trim().to_string(),
                Command::Append(a) => str_val + &"bar".to_string().repeat(a),
            };
            output.push(new_string);
        }

        output
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // use crate::my_module::transformer;

    // TODO: What do we need to import to have `transformer` in scope?
    use super::{Command, my_module::*};
    // use super::my_module::transformer;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(input);

        assert_eq!(
            output,
            [
                "HELLO",
                "all roads lead to rome!",
                "foobar",
                "barbarbarbarbarbar",
            ]
        );
    }
}
