use tracing_subscriber::layer::SubscriberExt;
use tracing::{info_span, info, field};

fn main() {
    let fmt = tracing_subscriber::fmt::layer().json();

    let subscriber = tracing_subscriber::Registry::default().with(fmt);
    tracing::subscriber::set_global_default(subscriber).unwrap();

    info!("an event outside the root span");

    let span = info_span!("the span", na = field::Empty);
    span.record("na", &"value");
    let _enter = span.enter();

    info!("an event inside the root span");
}
