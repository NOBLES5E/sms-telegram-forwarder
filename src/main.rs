use serde::{Deserialize};
use structopt::StructOpt;
use std::time::Duration;
use std::io::Read;
use chrono::{DateTime, Utc, MIN_DATETIME};
use std::process::Stdio;

#[derive(Debug, Clone, Deserialize)]
struct Sms {
    threadid: u64,
    #[serde(rename = "type")]
    msg_type: String,
    read: bool,
    number: String,
    #[serde(with = "termux_date_format")]
    received: DateTime<Utc>,
    body: String,
}

mod termux_date_format {
    use chrono::{DateTime, Utc, TimeZone};
    use serde::{self, Deserialize, Deserializer};

    const FORMAT: &str = "%Y-%m-%d %H:%M";

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<DateTime<Utc>, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone)]
struct AllSms {
    all_sms: Vec<Sms>,
    pub last_date: DateTime<Utc>,
}

impl AllSms {
    pub fn refresh_data(&mut self) {
        let mut all_sms = String::new();
        std::process::Command::new("termux-sms-list").arg("-t").arg("inbox").stdout(Stdio::piped()).spawn()
            .expect("cannot spawn termux-sms-list, make sure you have installed termux-api")
            .stdout.unwrap().read_to_string(&mut all_sms).unwrap();
        let all_parsed_sms: Vec<Sms> = serde_json::from_str(all_sms.as_str()).unwrap();
        self.last_date = all_parsed_sms.iter().map(|x| x.received).max().unwrap();
        self.all_sms = all_parsed_sms;
    }

    pub fn sms_after_date(&self, date: DateTime<Utc>) -> Vec<&Sms> {
        self.all_sms.iter().filter(|x| x.received > date).collect()
    }
}

impl Default for AllSms {
    fn default() -> Self {
        Self {
            all_sms: vec![],
            last_date: MIN_DATETIME
        }
    }
}

#[derive(Debug, StructOpt, Clone)]
#[structopt()]
struct Cli {
    #[structopt(long)]
    interval_seconds: u64,
    #[structopt(long)]
    bot_token: String,
    #[structopt(long)]
    chat_id: String,
}

fn main() {
    let args: Cli = Cli::from_args();
    eprintln!("{:#?}", args);
    let mut all_sms = AllSms::default();
    all_sms.refresh_data();
    let mut last_date = all_sms.last_date;
    loop {
        std::thread::sleep(Duration::from_secs(args.interval_seconds));
        all_sms.refresh_data();
        for sms in all_sms.sms_after_date(last_date) {
            let send_message_url = format!("https://api.telegram.org/bot{api_key}/sendMessage?chat_id={chatid}&text={text}",
                                           api_key = args.bot_token, chatid = args.chat_id, text = format!("sms from: {}, text: {}", sms.number, sms.body));
            ureq::get(send_message_url.as_str()).call().unwrap().into_string().unwrap();
        }
        last_date = all_sms.last_date;
    }
}
