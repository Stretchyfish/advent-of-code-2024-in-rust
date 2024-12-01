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

    let mut total_distance = 0;

    for i in 0..list_1.len()
    {
        let mut smallest_element_list_1 = 999999;
        let mut smallest_element_index_list_1 = 0;
        for j in 0..list_1.len()
        {
            if list_1[j] < smallest_element_list_1
            {
                smallest_element_list_1 = list_1[j];
                smallest_element_index_list_1 = j;
            }
        }

        let mut smallest_element_list_2 = 999999;
        let mut smallest_element_index_list_2 = 0;
        for j in 0..list_2.len()
        {
            if list_2[j] < smallest_element_list_2
            {
                smallest_element_list_2 = list_2[j];
                smallest_element_index_list_2 = j;
            }
        }

        total_distance += i32::abs(smallest_element_list_1 - smallest_element_list_2);

        list_1.remove(smallest_element_index_list_1);
        list_2.remove(smallest_element_index_list_2);

        //println!("List 1: Smallest element: {} with index: {}", smallest_element_list_1, smallest_element_index_list_1);
        //println!("List 2: Smallest element: {} with index: {}", smallest_element_list_2, smallest_element_index_list_2);
    }

    println!("Total distance: {}", total_distance);
}
