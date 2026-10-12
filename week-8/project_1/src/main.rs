/* ALGORITHM: APS LEVEL CHECKER

1. Start the program.

2. Create vectors to store the professions, APS levels, and their corresponding positions.

3. Display the available professions and ask the user to select one.

4. Check if the profession input is valid. If not, display an error message.

5. Display the available APS levels and ask the user to select one.

6. Check if the APS level input is valid. If not, display an error message.

7. Use the selected profession and APS level to find the corresponding position in the nested vector.

8. If the position is "-", display a message that no position is listed. Otherwise, display the position.

9. Ask the user if they want to check another staff member.

10. If the user enters "yes", repeat the process. Otherwise, end the program.

11. Stop.*/

use std::io;

fn main() {
    let professions = vec!["Office Admin", "Academic", "Lawyer", "Teacher"];

    let levels = vec![
        "APS 1-2",
        "APS 3-5",
        "APS 5-8",
        "EL1 8-10",
        "EL2 10-13",
        "SES",
    ];

    let positions = vec![
        vec!["Intern", "-", "Paralegal", "Placement"],
        vec![
            "Administrator",
            "Research Assistant",
            "Junior Associate",
            "Classroom Teacher",
        ],
        vec![
            "Senior Administrator",
            "PhD Candidate",
            "Associate",
            "Senior Teacher",
        ],
        vec![
            "Office Manager",
            "Post-Doc Researcher",
            "Senior Associate 1-2",
            "Leading Teacher",
        ],
        vec![
            "Director",
            "Senior Lecturer",
            "Senior Associate 3-4",
            "Deputy Principal",
        ],
        vec!["CEO", "Dean", "Partner", "Principal"],
    ];

    loop {
        println!("\n===== APS LEVEL CHECKER =====");

        println!("\nAvailable Professions:");
        for (i, profession) in professions.iter().enumerate() {
            println!("{}. {}", i + 1, profession);
        }

        println!("\nEnter profession number:");
        let mut profession_input = String::new();
        io::stdin()
            .read_line(&mut profession_input)
            .expect("Failed to read input");

        let profession_choice: usize = match profession_input.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Invalid input. Enter a number.");
                continue;
            }
        };

        if profession_choice < 1 || profession_choice > professions.len() {
            println!("Invalid profession choice.");
            continue;
        }

        println!("\nAvailable APS Levels:");
        for (i, level) in levels.iter().enumerate() {
            println!("{}. {}", i + 1, level);
        }

        println!("\nEnter APS level number:");
        let mut level_input = String::new();
        io::stdin()
            .read_line(&mut level_input)
            .expect("Failed to read input");

        let level_choice: usize = match level_input.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Invalid input. Enter a number.");
                continue;
            }
        };

        if level_choice < 1 || level_choice > levels.len() {
            println!("Invalid APS level choice.");
            continue;
        }

        let profession_index = profession_choice - 1;
        let level_index = level_choice - 1;

        let position = positions[level_index][profession_index];

        println!("\nProfession: {}", professions[profession_index]);
        println!("APS Level: {}", levels[level_index]);

        if position == "-" {
            println!("No position is listed for this profession and level.");
        } else {
            println!("Corresponding Position: {}", position);
        }

        println!("\nDo you want to check another staff member? (yes/no)");

        let mut repeat = String::new();
        io::stdin()
            .read_line(&mut repeat)
            .expect("Failed to read input");

        if repeat.trim().to_lowercase() != "yes" {
            println!("Thank you for using the APS Level Checker.");
            break;
        }
    }
}
