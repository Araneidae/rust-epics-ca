mod cadef;
mod channel;


pub async fn caget(pv: &str) -> f64
{
    let channel = channel::Channel::new(pv);
    let (datatype, count) = channel.wait_connect().await;
    println!("Got {:?} {:?} {}", channel, datatype, count);
    0.0
}
