pub trait Logger {    
    fn log(&self, verbosity: u8, message: &str);
}

struct StderrLogger;

impl Logger for StderrLogger {
    fn log(&self, verbosity: u8, message: &str) {
        eprintln!("verbosity={}: {}", verbosity, message);
    }
}

pub struct VerbosityFilter {
    max_verbosity: u8,
    inner: Box<dyn Logger>,
}

impl Logger for VerbosityFilter {
    fn log(&self, verbosity: u8, message: &str) {
        if verbosity <= self.max_verbosity {
            self.inner.log(verbosity, message);
        }
    }
}

fn do_things(logger: &impl Logger) {
    logger.log(5, "FYI");
    logger.log(2, "Uhoh");
}

fn main() {
    let inner = Box::new(StderrLogger);
    let l = VerbosityFilter { max_verbosity: 3, inner };
    do_things(&l);
}