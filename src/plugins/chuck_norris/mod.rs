pub mod jokes;

pub async fn handle_chuck_command(command: &str) -> String {
    match command {
        "/chuck help" => jokes::get_help(),
        "/chuck" => jokes::get_random_joke().await,
        _ if command.starts_with("/chuck @") => {
            let parts: Vec<&str> = command.split_whitespace().collect();
            if parts.len() == 2 {
                let name = parts[1].trim_start_matches('@');
                jokes::get_random_joke_from_name(name.to_string()).await
            } else if parts.len() > 3 && parts[2] == "cat" {
                let name = parts[1].trim_start_matches('@');
                // Collecting categories and joining them with a comma
                let raw_categories = &parts[3..];

                // Cleaning and joining categories
                let categories: String = raw_categories
                    .iter()
                    .flat_map(|s| s.split(|c: char| !c.is_alphanumeric())) // Split by non-alphanumeric characters
                    .filter(|s| !s.is_empty()) // Remove empty parts
                    .collect::<Vec<&str>>()
                    .join(","); // Join the cleaned parts with a comma

                jokes::get_random_joke_from_name_and_categories(name.to_string(), categories).await
            } else {
                "Invalid command format. Use /chuck help for the list of commands.".to_string()
            }
        }
        _ if command.starts_with("/chuck cat") => {
            let categories: String = command
                .trim_start_matches("/chuck cat")
                .split_whitespace()
                .collect();
            if categories.is_empty() {
                jokes::get_categories().await
            } else {
                jokes::get_random_joke_from_categories(categories).await
            }
        }
        _ => "Invalid command format. Use /chuck help for the list of commands.".to_string(),
    }
}
