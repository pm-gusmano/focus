use std::{fs, io, path::Path, time::Duration};

struct Block {
    name: String,
    software_list: SoftwareList,
    condition_list: BlockConditions,
    restriction_list: Restrictions,
    enabled: bool,
    triggered: bool,
}

struct SoftwareList {
    website_list: Option<Vec<String>>,
    application_list: Option<Vec<String>>,
}

impl SoftwareList {
    fn from_txt<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let unsorted_software_list = fs::read_to_string(path)?;
        let website_list: Vec<String> = unsorted_software_list
            .lines()
            .map(|line| line.trim().to_string())
            .filter(|line| !line.is_empty())
            .collect();

        let application_list: Vec<String> = Vec::new(); // Placeholder for future application list parsing

        if website_list.is_empty() && application_list.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "The provided file is empty or contains only invalid entries.",
            ));
        }

        Ok(SoftwareList {
            website_list: if website_list.is_empty() { None } else { Some(website_list) },
            application_list: if application_list.is_empty() { None } else { Some(application_list) },
        })

    }
}

struct BlockConditions {
    duration: Duration,
    schedule: String, // Implement with Jiff later
    locations: String, // Implement with geo or some other location crate
                      // etc. (?)
}

enum Restrictions {
    None,
    Password(String),
    Location(String), // Implement with geo later
                      // etc.
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    fn get_website_names() -> String {
        return "google.com\n youtube.com \n wikipedia.org\n osu.edu".to_string();
    }

    #[test]
    fn test_website_import_from_txt_file() {
        // Arrange

        // Write the generic websites to a txt file
        let sites = get_website_names();
        let mut temp_file = NamedTempFile::new().unwrap();
        write!(temp_file, "{}", sites).unwrap();
        let path = temp_file.path();

        // Trim whitespace on each line of sites
        let trimmed_sites: Vec<String> = sites
            .lines()
            .map(|line| line.trim().to_string())
            .collect();

        // Act
        let software_list = SoftwareList::from_txt(path).unwrap();

        // Assert
        assert_eq!(software_list.website_list, Some(trimmed_sites));
        assert_eq!(software_list.application_list, None)

    }
}
