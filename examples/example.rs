extern crate wu_diff;

use self::wu_diff::*;

fn main() {
    // let old = vec!["foo", "bar", "baz"];
    // let new = vec!["foo", "baz", "hoge"];

    wu_diff::diff(&[0u8; 20000], &[1u8; 20000]);

    /*
    for diff in wu_diff::diff(&old, &new) {
        match diff {
            DiffResult::Added(a) => println!("+{} new index = {}", a.data, a.new_index.unwrap()),
            DiffResult::Common(c) => println!(
                " {} old index = {}, new index = {}",
                c.data,
                c.old_index.unwrap(),
                c.new_index.unwrap()
            ),
            DiffResult::Removed(r) => println!("-{} old index = {}", r.data, r.old_index.unwrap()),
        }
    }
    */
}
