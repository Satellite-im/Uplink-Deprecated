use soloud::*;

pub enum Sounds {
    Notification,
    General,
}

pub fn play(sound: Sounds) {

    let sl = Soloud::default().unwrap();
    let mut wav = audio::Wav::default();
     match sound {
        Sounds::Notification => wav.load_mem(include_bytes!("../../extra/assets/sounds/Ponderous.ogg")).unwrap(),
        Sounds::General => {}, 

    };
    sl.play(&wav);
    while sl.voice_count() > 0 {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
