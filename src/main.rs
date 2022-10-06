use clap::Parser;

#[derive(Parser)]
#[command( version, long_about = None)]

// #[clap(about, version, author)]
struct Args {
    // #[clap(long)]
    xprv: String,
}

fn main() {
    let args = Args::parse();

    println!("\n");
    println!("    __________.__                   ______  .________ ");
    println!("    \\______   \\__|_____            /  __  \\ |   ____/ ");
    println!("     |    |  _/  \\____ \\   ______  >      < |____  \\  ");
    println!("     |    |   \\  |  |_> > /_____/ /   --   \\/       \\ ");
    println!("     |______  /__|   __/          \\______  /______  / ");
    println!("            \\/   |__|                    \\/       \\/  ");

    println!("xpriv {:?}", args.xprv);
}
