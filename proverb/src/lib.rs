pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }

    let mut result: Vec<String> = Vec::new();
    for index in 0..list.len() - 1 {
        result.push(format!(
            "For want of a {} the {} was lost.",
            list[index],
            list[index + 1]
        ))
    }
    result.push(format!("And all for the want of a {}.", list[0]));
    result.join("\n")
}
