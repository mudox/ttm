use ttm::config::Config;
use ttm::logging;
use ttm::run::run;

fn main() {
    logging::init();
    run(Config::load());
}
