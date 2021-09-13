extern crate clap;

use clap::App;

fn main() {
    // Apps describe the top level application
    //
    // You create an App and set various options on that App using the "builder pattern"
    //
    // The options (version(), author(), about()) aren't mandatory, but recommended. There is
    // another option, usage(), which is an exception to the rule. This should only be used when
    // the default usage string automatically generated by clap doesn't suffice.
    //
    // You also set all the valid arguments your App should accept via the arg(), args(), arg_from_usage()
    // and args_from_usage() (as well as subcommands via the subcommand() and subcommands() methods) which
    // will be covered later.
    //
    // Once all options have been set, call one of the .get_matches* family of methods in order to
    // start the parsing and find all valid command line arguments that supplied by the user at
    // runtime. The name given to new() will be displayed when the version or help flags are used.
    App::new("joshcli")
        .version("1.0")
        .author("Josh B.")
        .about(":)")
        .get_matches();

    // This example doesn't do much, but it *does* give automatic -h, --help, -V, and --version functionality ;)

    // Continued program logic goes here...
}
