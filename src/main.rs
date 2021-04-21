mod get_duration;
mod loop_notifications;

fn main() {
	
	let duration = get_duration::get_duration();
	loop_notifications::navigate(duration);
	
}