use async_trait::async_trait;
use ractor::{Actor, ActorRef, ActorProcessingErr};

pub struct Counter {}

pub enum CounterMsg {
    Inc(usize),
    Get(ractor::RpcReplyPort<usize>),
}
 
#[async_trait]
impl Actor for Counter {
    type State = usize;
    type Msg = CounterMsg;
    type Arguments = ();

    async fn pre_start(&self, _: ActorRef<Self::Msg>, _: ()) -> Result<Self::State, ActorProcessingErr> {
        Ok(0)
    }

    async fn handle(&self, _: ActorRef<Self::Msg>, message: Self::Msg, state: &mut Self::State) -> Result<(), ActorProcessingErr> {
        match message {
            CounterMsg::Inc(inc) => {
                *state += inc;
                Ok(())
            }
            CounterMsg::Get(reply) => {
                Ok(reply.send(*state)?)
            }
        }
    }
}
