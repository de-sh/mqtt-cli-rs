use rumqttc::{qos, AsyncClient, ClientError, Event, Incoming, MqttOptions, Outgoing, QoS};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum MqttRequest {
    #[structopt(help_short = "Send a publish packet to broker")]
    Pub {
        #[structopt(short, help = "Publish topic")]
        topic: String,
        #[structopt(short, help = "Publish message")]
        message: String,
        #[structopt(short, help = "Publish QoS", default_value = "0")]
        qos: u8,
    },
    Sub {
        #[structopt(short, help = "Subscribe topic")]
        topic: String,
        #[structopt(short, help = "Publish QoS", default_value = "0")]
        qos: u8,
    },
}

#[derive(StructOpt, Debug)]
#[structopt(name = "uplink", about = "collect, batch, compress, publish")]
pub struct Config {
    #[structopt(short, help = "Client ID", default_value = "mqtt-cli-rs")]
    id: String,
    #[structopt(short, help = "Broker host address", default_value = "localhost")]
    host: String,
    #[structopt(short, help = "Broker port number", default_value = "1883")]
    port: u16,
    #[structopt(subcommand)]
    request: MqttRequest,
}

impl Config {
    pub fn init() -> Self {
        Self::from_args()
    }

    fn options(&self) -> MqttOptions {
        MqttOptions::new(&self.id, &self.host, self.port)
    }

    async fn request(&self, client: AsyncClient) -> Result<bool, ClientError> {
        match &self.request {
            MqttRequest::Pub {
                topic,
                qos: num,
                message,
            } => {
                let qos = qos(*num).unwrap();
                let payload = message.as_bytes();
                client.publish(topic, qos, false, payload).await?;

                Ok(qos != QoS::AtMostOnce)
            }

            MqttRequest::Sub { topic, qos: num } => {
                let qos = qos(*num).unwrap();
                client.subscribe(topic, qos).await?;

                Ok(qos != QoS::AtMostOnce)
            }
        }
    }

    #[tokio::main]
    pub async fn run(self) {
        let options = self.options();
        let (client, mut eventloop) = AsyncClient::new(options, 10);

        let wait = self.request(client).await.unwrap();

        loop {
            match eventloop.poll().await.unwrap() {
                Event::Incoming(Incoming::Publish(p)) => {
                    self.handle_payload(p.payload[..].to_vec())
                }
                Event::Outgoing(Outgoing::Publish(_)) if !wait => {
                    break;
                }
                Event::Incoming(Incoming::PubAck(_) | Incoming::PubComp(_)) => break,
                Event::Incoming(
                    Incoming::ConnAck(_)
                    | Incoming::SubAck(_)
                    | Incoming::PingResp
                    | Incoming::PubRec(_),
                )
                | Event::Outgoing(
                    Outgoing::Subscribe(_)
                    | Outgoing::PubAck(_)
                    | Outgoing::PingReq
                    | Outgoing::Publish(_)
                    | Outgoing::PubRel(_)
                    | Outgoing::PubRec(_)
                    | Outgoing::PubComp(_),
                ) => {}
                _ => unreachable!(),
            }
        }
    }

    fn handle_payload(&self, payload: Vec<u8>) {
        println!("{}", String::from_utf8(payload).unwrap())
    }
}
