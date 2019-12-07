use rumqtt::{client::Notification::Publish, MqttClient, MqttOptions, QoS};
use slack_hook::SlackTextContent::{Text, User};
use slack_hook::{PayloadBuilder, Slack, SlackUserLink};
use std::env;
use std::process::Command;

fn main() {
    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID is invalid");
    let ca = include_bytes!("../tlsfiles/ca.crt").to_vec();
    let client_cert = include_bytes!("../tlsfiles/cert.pem").to_vec();
    let client_key = include_bytes!("../tlsfiles/private.key").to_vec();

    let mqtt_host = env::var("MQTT_HOST").expect("MQTT_HOST is invalid");

    let slack_hook_url = env::var("SLACK_HOOK_URL").expect("SLACK_HOOK_URL is invalid");

    let music_file = env::var("MUSIC_FILE_PATH").expect("MUSIC_FILE_PATH is invalid");

    let mqtt_options = MqttOptions::new(client_id, mqtt_host, 8883)
        .set_ca(ca)
        .set_client_auth(client_cert, client_key)
        .set_keep_alive(10);

    let (mut mqtt_client, notifications) = MqttClient::start(mqtt_options).unwrap();
    mqtt_client
        .subscribe("panic_button", QoS::AtLeastOnce)
        .unwrap();

    for notification in notifications {
        match notification {
            Publish(_) => {
                let slack = Slack::new(&*slack_hook_url).unwrap();
                let p = PayloadBuilder::new()
                        .text(vec![
                            User(SlackUserLink::new("!everyone")),
                            Text(":rotating_light: *緊急ボタンが押されました！* :rotating_light:\nすぐに連絡をとってください".into())
                        ].as_slice())
                        .channel("#general")
                        .username("自宅")
                        .icon_emoji(":house:")
                        .build()
                        .unwrap();
                slack.send(&p).expect("Cannot send message to slack");
                Command::new("amixer")
                    .arg("cset")
                    .arg("numid=1")
                    .arg("90%")
                    .output()
                    .expect("Cannot change volume");
                for _ in 0..5 {
                    Command::new("aplay")
                        .arg(&*music_file)
                        .output()
                        .expect("Cannot play music");
                }
            }
            _ => println!("{:?}", notification),
        }
    }
}
