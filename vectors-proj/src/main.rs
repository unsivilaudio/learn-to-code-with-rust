#[derive(Debug)]
struct File {
    name: String,
}

#[derive(Debug)]
struct Folder {
    name: String,
    contents: Vec<File>,
}

impl Folder {
    fn new(name: String) -> Self {
        Self {
            name,
            contents: Vec::new(),
        }
    }
}

impl Folder {
    fn create_file(&mut self, name: String) {
        self.contents.push(File { name });
    }
    fn delete_file(&mut self, idx: usize) -> File {
        self.contents.remove(idx)
    }
    fn get_file(&self, idx: usize) -> Option<&File> {
        self.contents.get(idx)
    }
}

fn main() {
    let mut projects = Folder::new(String::from("projects"));
    let file_johnson = projects.create_file(String::from("Johnson"));
    let file_williams = projects.create_file(String::from("Williams"));
    println!("{projects:?}");

    projects.delete_file(1);
    println!("{projects:?}");

    match projects.get_file(1) {
        Some(file) => println!("Found the file {file:?}"),
        None => println!("Could not find the file you were looking for"),
    }
}
