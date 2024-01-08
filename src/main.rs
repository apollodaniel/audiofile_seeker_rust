use audiofile_seeker::seeker;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if !args.is_empty() && !args.len()>1{
        seeker::get_music_cluster(args[1].as_str());
    }
}
