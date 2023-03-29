use mac_notification_sys::*;

fn main() {
    let bundle = get_bundle_identifier_or_default("firefox");
    set_application(&bundle).unwrap();

    println!("bundle");

    send_notification(
        "Danger",
        Some("Will Robinson"),
        "Run away as fast as you can",
        None,
    )
    .unwrap();

    send_notification(
        "NOW",
        None,
        "Without subtitle",
        Some(Notification::new().sound("Blow")),
    )
    .unwrap();
}
