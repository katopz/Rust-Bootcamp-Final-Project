// Step 1: Write the concatenate_strings function signature.
fn concatenate_strings(str1: &str, str2: &str) -> String {
    // Step 2: Implement the concatenate_strings function.
    let mut result = String::new();
    result.push_str(str1);
    result.push_str(str2);

    // Step 3: Return the result string from the function.
    result
}

fn main() {
    // Step 4: Initialize two String variables in the main function.
    let string1 = String::from("Hello, ");
    let string2 = String::from("Rust!");

    // Step 5: Call the concatenate_strings function with string slices of the variables.
    let concatenated_string = concatenate_strings(&string1, &string2);

    // Step 6: Print the result to the console.
    println!("{}", concatenated_string);
}
