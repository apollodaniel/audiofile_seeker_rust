pub mod music_cluster{
    use std::{cmp::Ordering, path::PathBuf};

    use colored::Colorize;
    use filepath::FilePath;
    use lofty::AudioFile;

    pub fn get_music_cluster(path: &str){
        let path = path.trim();
        let path = get_absolute_path(path);
        if let Ok(absolute_path) = path {
            let mut base_dir = Directory::create(absolute_path.to_str().unwrap());
            base_dir.read_files();
            base_dir.print();
        }
    }

    pub fn get_absolute_path(path: impl AsRef<std::path::Path>) -> std::io::Result<PathBuf> {
        let path = path.as_ref();
    
        let absolute_path = if path.is_absolute() {
            path.to_path_buf()
        } else {
            std::env::current_dir()?.join(path)
        };
    
        Ok(absolute_path)
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
                FsEntry::Directory(_) => {
                    string_buf.push_str("Dir");
                },
                FsEntry::File(_) => {
                    string_buf.push_str("File");
                }
            }
            write!(f, "Entry: {}", string_buf)
        }
    }

    impl Directory {

        fn print(&mut self){
            println!("\nListing musics on -> {}\n", self.path);
            self._print_formatted(0);
        }

        fn _print_formatted(&self, space_count: usize){
            let folder_name = if self.path.chars().last().unwrap() == '\\' {
                let mut temp_name_list: Vec<char> = self.path.chars().collect();
                temp_name_list.remove(temp_name_list.len()-1);
                let string: String = temp_name_list.iter().collect();
                string.as_str().split("\\").last().unwrap().red()
            } else {
                self.path.split("\\").last().unwrap().red()
            };
            let parent_folder_lenght = folder_name.chars().count();
            let space_count = space_count + (parent_folder_lenght+1);
            if self.z_index == 0{
                println!("{}:",folder_name);
            }
        
            for entry in &self.files{
                match entry {
                    FsEntry::File(f) => {
                        match f.file_type {
                            FileType::AudioFile(_) => println!("{}({}) - {}", " ".repeat(space_count), f.format_duration(),f.path.split("\\").last().unwrap().purple()),
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
            println!("Reading: {}", self.path);
            let files: Vec<_> = std::fs::read_dir(&self.path).expect("error reading files").collect();
            files.iter().for_each(|f| {
                if let Ok(file) = f {
                    if file.metadata().expect("error getting metadata").is_dir(){
                        // dir
                        let dir = Directory::create(file.path().to_str().unwrap());
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

            self.files.sort_by(|a,_| match a {
                FsEntry::File(_) => Ordering::Less,
                FsEntry::Directory(_) => Ordering::Greater
            });
        }


        fn create(path: &str) -> Directory{
            Directory{
                path: path.to_string(),
                z_index: 0,
                files: Vec::new()
            }
        }
    }

    impl File {

        fn format_duration(&self)->String{
            if let FileType::AudioFile(seconds) =  self.file_type{
                let secs = seconds % 60;
                let minutes = (seconds/60)%60;
                let hour = (seconds/60)/60;
                if hour != 0 && minutes != 0 {
                    format!("{:0>2}:{:0>2}:{:0>2}",hour,minutes,secs)
                }else{
                    format!("{:0>2}:{:0>2}",minutes,secs)
                }
            }else{
                String::new()
            }
        }

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
                    if matches!(e.kind(), lofty::error::ErrorKind::UnknownFormat) || matches!(e.kind(), lofty::error::ErrorKind::FileDecoding(_)){
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