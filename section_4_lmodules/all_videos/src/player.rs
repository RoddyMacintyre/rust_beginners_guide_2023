// Need to make the functionality publicly available. By default, functions are restricted to
// the file that declares them

pub fn play_movie(name: &str){
    println!("Playing movie {}", name);
}

pub fn play_audio(name: &str) {
    println!("Playing audio {}", name)
}