extern crate wu_diff;

use self::wu_diff::*;

fn main() {
    let old = vec!["foo", "bar", "baz"];
    let new = vec!["foo", "baz", "hoge"];

    for diff in wu_diff::diff(&old, &new) {
        match diff {
            DiffResult::Added(a) => {
                let i = a.new_index.unwrap();
                println!("+{} new index = {}", new[i], i)
            }
            DiffResult::Common(c) => {
                let new_index = c.new_index.unwrap();
                let old_index = c.old_index.unwrap();
                println!(
                    " {} old index = {}, new index = {}",
                    new[new_index], old_index, new_index
                )
            }
            DiffResult::Removed(r) => {
                let i = r.old_index.unwrap();
                println!("-{} old index = {}", old[i], i)
            }
        }
    }
}
