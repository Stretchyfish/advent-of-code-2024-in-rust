use std::env;
use std::fs;


fn main() 
{
    let mut list_1 = Vec::new();
    let mut list_2 = Vec::new();

    let contents = fs::read_to_string("assets/day_1_input.txt")
        .expect("Should have been able to read the file");

    for line in contents.lines()
    {
        let both_values: Vec<&str> = line.split("   ").collect();
        let parsed_value_1 = both_values[0].parse::<i32>().unwrap();
        let parsed_value_2 = both_values[1].parse::<i32>().unwrap();
        list_1.push(parsed_value_1);
        list_2.push(parsed_value_2);
    }

    let mut similarity_score = 0;

    for i in 0..list_1.len()
    {
        let mut number_of_matching_values_in_list_2 = 0;

        for j in 0..list_2.len()
        {
            if list_2[j] == list_1[i]
            {
                number_of_matching_values_in_list_2 += 1;    
            }
        }

        similarity_score += list_1[i] * number_of_matching_values_in_list_2;
    }

    println!("Similarity score: {}", similarity_score);
}
