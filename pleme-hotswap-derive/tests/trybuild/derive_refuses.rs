use pleme_hotswap_derive::HotSwap;

#[derive(HotSwap)]
struct Config {
    #[hot_swap]
    log_level: String,
    #[restart_required(reason = "bound at process start")]
    bind_addr: String,
}

fn main() {}
