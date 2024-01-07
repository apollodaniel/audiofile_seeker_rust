pub mod music_cluster{
    use std::{path, time::Duration, fs::read_dir, borrow::{Borrow, BorrowMut}, cmp::Ordering};

    use colored::Colorize;
    use filepath::FilePath;
    use lofty::{error::ErrorKind, TaggedFile, AudioFile};


    pub fn get_music_cluster(){
        let mut base_dir = Directory::create(r"C:\Users\Administrator\Music", true);
        base_dir.read_files();
        base_dir.print();


    }

    pub struct Directory{
        path: String,
        files: Vec<FsEntry>,
        z_index: u16
    }

    pub struct File{
        path: String,
        file_type: FileType
    }

    

    pub enum FileType {
        File,
        AudioFile(u32) // gets the duration in value
    }

    pub enum FsEntry{
        Directory(Directory),
        File(File)
    }

    impl std::fmt::Display for FsEntry {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut string_buf = String::new();
            match self {
                FsEntry::Directory(d) => {
                    string_buf.push_str("Dir");
                },
                FsEntry::File(f) => {
                    string_buf.push_str("File");
                }
            }
            write!(f, "Entry: {}", string_buf)
        }
    }

    impl Directory {

        fn print(&mut self){
            println!("\n\tListing musics on -> {}\n", self.path);
            self._print_formatted(0);
        }

        fn _print_formatted(&self, space_count: usize){
            let parent_folder_lenght = self.path.split("\\").last().unwrap().trim().chars().count();
            let space_count = space_count + (parent_folder_lenght+1);
            if self.z_index == 0{
                println!("{}:",self.path.split("\\").last().unwrap().red());
            }
        
            for entry in &self.files{
                match entry {
                    FsEntry::File(f) => {
                        let tab= "\t|".repeat((self.z_index as usize)*2);
                        let vertical_pipe= "|".repeat(self.z_index as usize);

                        let parent_folder_lenght = self.path.split("\\").last().unwrap().trim().chars().count();
                        match f.file_type {
                            FileType::AudioFile(_) => println!("{}{}"," ".repeat(space_count), f.path.split("\\").last().unwrap().purple()),
                            FileType::File => println!("{}{}",  " ".repeat(space_count), f.path.split("\\").last().unwrap().white())
                        }
                    },
                    FsEntry::Directory(d) => {
                        println!("{}{}:"," ".repeat(space_count),d.path.split("\\").last().unwrap().red());
                        d._print_formatted(space_count);
                    }
                }
            }
        }

        fn read_files(&mut self){
            let files: Vec<_> = std::fs::read_dir(&self.path).expect("error reading files").collect();
            let files = files.iter().for_each(|f| {
                if let Ok(file) = f {
                    if file.metadata().expect("error getting metadata").is_dir(){
                        // dir
                        let dir = Directory::create(file.path().to_str().unwrap(), false);
                        let dir = FsEntry::Directory(dir);
                        if let FsEntry::Directory(mut d) = dir {
                            d.read_files();
                            let new_dir = Directory{
                                files: d.files,
                                z_index: self.z_index+1,
                                path: d.path
                            };
                            self.files.push(FsEntry::Directory(new_dir));
                        }
                    }else{
                        // file
                        let mut file = std::fs::File::open(file.path()).unwrap();
                        let file = File::from_file(&mut file);
                        if let Some(_f) = file {
                            self.files.push(FsEntry::File(_f));
                        }
                    }
                }
            });

            self.files.sort_by(|a,b| match a {
                FsEntry::File(f) => Ordering::Less,
                FsEntry::Directory(_) => Ordering::Greater
            });
        }


        fn create(path: &str, is_root: bool) -> Directory{
            Directory{
                path: path.to_string(),
                z_index: 0,
                files: Vec::new()
            }
        }
    }

    impl File {
        fn from_file(file: &mut std::fs::File) -> Option<File>{
            let lofty_file = lofty::read_from(file);
            match lofty_file {
                Ok(f) =>{
                    // audio file
                    if let Ok(path) = file.path() {
                        Some(
                            File{
                                file_type: FileType::AudioFile(f.properties().duration().as_secs() as u32),
                                path: path.to_string_lossy().to_string()
                            }
                        )
                    }else{
                        None
                    }
                },
                Err(ref e) => {
                    if matches!(e.kind(), lofty::error::ErrorKind::UnknownFormat) || matches!(e.kind(), lofty::error::ErrorKind::FileDecoding(e)){
                            // normal file
                        if let Ok(path) = file.path() {
                            Some(File{
                                file_type: FileType::File,
                                path: path.to_string_lossy().to_string()
                            })
                        }else{
                            None
                        }
                    }
                    else{
                        panic!("{:?}", e.kind());
                    }
                }
            }
        }
    }
}