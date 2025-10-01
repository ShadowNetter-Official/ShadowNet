use std::{
    fs,
    fs::File,
    io::Write,
    process,
};
use reqwest::blocking::get;
use prompted::input;

fn main() {
    let (title, subtitle, github_link, image_link, langs, description) = get_input();
    download_image(&image_link, &title);
    let html = write(&title, &subtitle, &langs, &image_link, &github_link, &description);
    println!("added html: \n{html}\n");
    preview(&title, &subtitle, &langs, &description, &image_link, &github_link);
}

fn get_input() -> (String, String, String, String, String, String) {
    let title = input!("Project name: ");
    let subtitle = input!("Project description (short): ");
    let github_link = input!("Project GitHub link: ");
    let image_link= input!("Image link: ");
    let langs = input!("Project languages: ");
    let description = input!("Project description (long): ");
    return (title, subtitle, github_link, image_link, langs, description);
}

fn preview(title: &String, subtitle: &String, langs: &String, description: &String, image_link: &String, github_link: &String) {
    println!("\nPreview: \n");
    println!("{title} ({github_link})");
    println!("{subtitle}\n");
    println!("Image ({image_link})");
    println!("Using: {langs}\n");
    println!("{description}\n");
}


fn download_image(url: &String, title: &String) {
    let file = format!("resources/{title}.png");
    let response = match get(url) {
        Ok(response) => response,
        Err(e) => {
            println!("Error fetching download link: {e}");
            quit();
            return;
        },
    };
    let content = match response.bytes() {
        Ok(content) => content,
        Err(e) => {
            println!("Error reading link response: {e}");
            quit();
            return;
        }
    };

    let mut downloaded_file = match File::create(&file) {
        Ok(f) => f,
        Err(e) => {
            println!("Error creating file: {e}");
            quit();
            return;
        }
    };
    match downloaded_file.write_all(&content) {
        Ok(_) => {},
        Err(e) => {
            println!("Error writing file: {e}");
            quit();
        }
    };
    println!("Downloaded image to {file}");
}

fn write(title: &String, subtitle: &String, langs: &String, image_link: &String, github_link: &String, description: &String) -> String {
    let file = "index.html";
    let mut contents = match fs::read_to_string(file) {
        Ok(contents) => contents,
        Err(e) => {
            println!("error reading {file}: {e}");
            quit();
            return Default::default();
        },
    };
    let new_blog = format!(r#"
        <div class="blog">
            <a href="{github_link}" target="_blank">
                <h1>{title}</h1>
            </a>
            <h2>{subtitle}</h2>
            <img src="{image_link}" alt="{title}">
            <h3>Using: {langs}</h3>
            <br>
            <p>
                {description}
            </p>
        </div>
        <br>
    "#);
    if let Some(pos) = contents.rfind("</section>") {
        contents.insert_str(pos, &new_blog);
    } else {
        println!("didn't find </section> element, add it to file then try again");
        quit();
    }

    match fs::write(&file, contents) {
        Ok(_) => (),
        Err(e) => {
            println!("error writing changes to {file}: {e}");
            quit();
        },
    }
    new_blog.to_string()
}

fn quit() {
    process::exit(0);
}
