use std::{fs::File, str::FromStr, io::Read};
use mime_guess;
use music_cluster_rust::music_cluster;

fn main() {
    music_cluster::get_music_cluster();
}
