/*# ALGORITHM: THE EY INTERVIEW

1. Start the program.

2. Create a vector containing the names of the candidates and their years of programming experience.

3. Set the first candidate as the candidate with the highest experience.

4. Use a `for` loop to go through the candidates in the vector.

5. Check if the current candidate has more years of experience than the candidate with the highest experience.

6. If the current candidate has more experience, replace the previous candidate with the current one.

7. Continue checking the candidates until the end of the vector is reached.

8. Display the name and years of experience of the candidate with the highest experience.

9. End the program.*/


fn main() {
    let candidates = vec![
        ("David", 3),
        ("Grace", 7),
        ("Michael", 5),
        ("Sarah", 10),
    ];

    let mut h_candidate = candidates[0];

    for candidate in candidates.iter() {
        if candidate.1 > h_candidate.1 {
            h_candidate = *candidate;
        }
    }

    println!("Candidate with the highest experience: {}", h_candidate.0);
    println!("Years of experience: {}", h_candidate.1);
}
