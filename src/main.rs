use clap::Parser;

fn main() {
    let args = wittier::Args::parse();
    wittier::main(args);
}
