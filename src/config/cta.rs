use crate::config::Config;
use crate::wager::Wager;

enum CtaWagerType {
    Forward,
    Reverse
}

pub struct Cta {
    base_config: Config<Wager<CtaWagerType>>
}