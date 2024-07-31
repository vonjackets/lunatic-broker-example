use lunatic::ap::handlers::{Message};
use lunatic::ap::{AbstractProcess, Config, MessageHandler, ProcessRef, State};
use lunatic::serializer::MessagePack;
use lunatic::supervisor::{Supervisor, SupervisorConfig, SupervisorStrategy};
use lunatic::{Mailbox, ProcessName};

use serde::{Deserialize, Serialize};

// Define the messages that the broker actor can handle
#[derive(Clone, Debug, Serialize, Deserialize)]
enum BrokerMessage {
    Subscribe {
        topic: String,
        subscriber: String 
    },
    Publish {
        topic: String,
        message: String,
    },
    InternalMessage {
        topic: String,
        message: String,
    },
    Notify(String),
}

#[derive(ProcessName)]
// #[lunatic(process_name="broker_process")]
struct Broker {
    pub subscribers: Vec<String>
}

impl AbstractProcess for Broker {
    type Arg = ();
    type State = Self;
    type Handlers = (Message<BrokerMessage>,);
    type Serializer = MessagePack;
    type StartupError = ();

    fn init(_: Config<Self>, _: ()) -> Result<Broker, ()> {
        
        let b = Broker {
            subscribers: Vec::<String>::new()
        };
        println!("starting broker named: {}", Self::process_name(&b));
        Ok(b)
    }

    fn terminate(_state: Self::State) {
        println!("Broker dying");
    }
}

impl MessageHandler<BrokerMessage> for Broker {
    fn handle(mut state: State<Self>, msg: BrokerMessage) {
        match msg {
            BrokerMessage::Subscribe {topic, subscriber} => { 
                state.subscribers.push(subscriber.clone());
                println!("process named: {} subscribed to topic: {}", subscriber, topic);
                println!("Current subscribers: ");

                state.subscribers.iter().for_each(|subscriber| {
                    println!("{}", subscriber);
                });
             },
            BrokerMessage::Publish {topic, message} => {  
                println!("{}: {}", topic, message);
             },
            BrokerMessage::InternalMessage {topic, message} => { println!("{}: {}", topic, message); },
            BrokerMessage::Notify (message) => {
                println!("{}", message);
            }
        }
        
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
enum ClientMessage {
    Notification {
        topic: String,
        message: String,
    },
}

#[derive(ProcessName)]
// #[lunatic(process_name="broker_client")]
struct BrokerClient {
    // pub broker: ProcessRef<Broker>
}

impl AbstractProcess for BrokerClient {

    type Arg = ();
    type State = Self;
    type Handlers = (Message<ClientMessage>,);
    type Serializer = MessagePack;
    type StartupError = ();


    fn init(config: Config<Self>, _: ()) -> Result<BrokerClient, ()> {
        let c = BrokerClient {};
        println!("starting client named: {}", Self::process_name(&c));

        let broker = ProcessRef::<Broker>::lookup(&"broker_process").unwrap();
        //subscribe
        broker.send(BrokerMessage::Subscribe { topic: "None".to_owned(), subscriber: "broker_client".to_owned() });

        Ok(c)
    }

}

impl MessageHandler<ClientMessage> for BrokerClient {

    fn handle(state: State<Self>, msg: ClientMessage) {
        match msg {
            ClientMessage::Notification { topic, message } => {
                println!("Notification - Topic: {}, Message: {}", topic, message);
            }
        }
    }
}
// Supervisor definition.
struct ObserverSupervisor;
impl Supervisor for ObserverSupervisor {
    type Arg = ();
    // Start 2 children and monitor it for failures.
    type Children = (Broker,BrokerClient);

    fn init(config: &mut SupervisorConfig<Self>, _: ()) {
        //restart any children that die
        config.set_strategy(SupervisorStrategy::OneForAll);

        config.set_args(((),() ));

        config.set_names((Some("broker_process".to_owned()), Some("broker_client".to_owned())));
    }
}
#[lunatic::main]
fn main(_: Mailbox<()>) {

    let _ = ObserverSupervisor::start(()).unwrap();
    // Get reference to named child.
    
    let broker = ProcessRef::<Broker>::lookup(&"broker_process").unwrap_or_else( || {
        println!("Broker not running, starting");
        Broker::start(()).unwrap()
    } );
    
    //send some messages
    broker.send(BrokerMessage::InternalMessage { topic: "all".to_owned(), message: "tell broker hello".to_owned() });

    let client: ProcessRef<BrokerClient> = ProcessRef::<BrokerClient>::lookup(&"broker_client").unwrap_or_else( || {
        println!("client not running, starting");
        BrokerClient::start(()).unwrap()
    } );
 
    client.send(ClientMessage::Notification { topic: "none".to_owned(), message: "tell client hello".to_owned() });
}