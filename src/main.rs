fn main() {
    notify_rust::Notification::new().appname("iterm").show().unwrap();
}
