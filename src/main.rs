use music_cluster_rust::music_cluster;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if !args.is_empty() && !args.len()>1{
        music_cluster::get_music_cluster(args[1].as_str());
    }
}
