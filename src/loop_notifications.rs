use std::time::Duration;
use std::thread::sleep;
use winrt_notification::{Duration as ToasDuration, Sound, Toast};


pub fn navigate(duration: u64) {
	loop {
		sleep(Duration::new(duration * 60, 0));
		Toast::new(Toast::POWERSHELL_APP_ID)
	        .title("wup")
	        .text1("wake up now")
	        .sound(Some(Sound::SMS))
	        .duration(ToasDuration::Short)
	        .show()
	        .expect("unable to toast");
	    println!("Relax and do stretching exercises");
	}
}