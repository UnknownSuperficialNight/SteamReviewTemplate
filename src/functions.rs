// Function to concatenate arrays and create a formatted output string
pub fn concate_arrays_to_page(combined_associated_strings: &Vec<&str>, combined_inner_vec_data: &Vec<Vec<&str>>, combined_outputs: &Vec<Option<&String>>) -> String {
    // Initialize an empty string to store the final output
    let mut output = String::new();

    // Create a vector to store combined data (header, options, and selected option)
    let mut combined_data: Vec<(String, Vec<&str>, Option<&String>)> = Vec::new();

    // Iterate through the input data and combine it into a single vector
    for (i, header) in combined_associated_strings.iter().enumerate() {
        combined_data.push((header.to_string(), combined_inner_vec_data[i].clone(), combined_outputs[i].clone()));
    }

    // Iterate through the combined data to format the output string
    for (header, array, selected) in combined_data {
        // Format and add the header to the output
        let formatted_header = format!("---{{ {} }}---\n", header);
        output.push_str(&formatted_header);

        // Add each item to the output, marking the selected item with a checkbox
        for data in array {
            if Some(&data.to_string()) == selected {
                output.push_str(&format!("☑ {}\n", data));
            } else {
                output.push_str(&format!("☐ {}\n", data));
            }
        }
        // Add a newline after each category
        output.push('\n');
    }

    // Return the formatted output string
    output
}
