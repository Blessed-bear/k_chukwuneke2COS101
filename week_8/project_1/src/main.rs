 use std::io;

fn main() {
    println!("Enter your profession (office, academic, lawyer, teacher):");
    let mut profession = String::new();
    io::stdin().read_line(&mut profession).expect("Failed to read input");
    let profession = profession.trim().to_lowercase();

    println!("Enter your job title:");
    let mut title = String::new();
    io::stdin().read_line(&mut title).expect("Failed to read input");
    let title = title.trim();

    let aps_level = match profession.as_str() {
        "office" => match title {
            "Intern" => "APS 1-2",
            "Administrator" => "APS 3-5",
            "Senior Administrator" => "APS 5-8",
            "Office Manager" => "EL1 8-10",
            "Director" => "EL2 10-13",
            "CEO" => "SES",
            _ => "Unknown Role",
        },

        "academic" => match title {
            "-" => "APS 1-2",
            "Research Assistant" => "APS 3-5",
            "PhD Candidate" => "APS 5-8",
            "Post-Doc Researcher" => "EL1 8-10",
            "Senior Lecturer" => "EL2 10-13",
            "Dean" => "SES",
            _ => "Unknown Role",
        },

        "lawyer" => match title {
            "Paralegal" => "APS 1-2",
            "Junior Associate" => "APS 3-5",
            "Associate" => "APS 5-8",
            "Senior Associate 1-2" => "EL1 8-10",
            "Senior Associate 3-4" => "EL2 10-13",
            "Partner" => "SES",
            _ => "Unknown Role",
        },

        "teacher" => match title {
            "Placement" => "APS 1-2",
            "Classroom Teacher" => "APS 3-5",
            "Snr Teacher" => "APS 5-8",
            "Leading Teacher" => "EL1 8-10",
            "Deputy Principal" => "EL2 10-13",
            "Principal" => "SES",
            _ => "Unknown Role",
        },

        _ => "Unknown Profession",
    };

    println!("Your APS Level is: {}", aps_level);
}
