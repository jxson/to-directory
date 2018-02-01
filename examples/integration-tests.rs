extern crate assert_cli;
extern crate tap_rust;

use assert_cli::Assert;
use tap_rust::tap_writer::TapWriter;

// https://stackoverflow.com/questions/35711044/specify-binary-only-dependencies
// https://github.com/Cigna/TAP-Rust
// https://github.com/killercup/assert_cli/

fn main() {
    let test = Assert::main_binary()
        .with_args(&["--init"])
        .succeeds()
        .and()
        .stdout()
        .contains(include_str!("../src/to.sh"))
        .execute();

    let writer = TapWriter::new("Example TAP stream");

    writer.plan(1, 1);

    writer.name();

    match test {
        Ok(_) => writer.ok(1, "--init"),
        Err(err) => {
            writer.not_ok(1, "--init");
            let message = format!("{}", err);
            writer.diagnostic(message.as_str());
        },
    }

    // // Print out some test results
    // ;
    // writer.ok(2, "Bamboo");
    // writer.ok(3, "Curry");
    // // This one failed, so explain why with a diagnostic line
    // writer.not_ok(4, "Noodle");
    // writer.diagnostic("The above test failed because of XYZ reason");
    // writer.ok(5, "Tree");
    //
    // // uh oh! something went horribly wrong and we need to stop before
    // // we print out the results from test 6!
    // writer.bail_out_with_message("Destabilized warp core! Can't continue!");
}
