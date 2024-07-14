mod linked_list;
mod block;
mod mresult;

use mresult::MResult;

fn main() {
    println!("Hello, world!");
    let result: MResult<i32, &str> = MResult::ok(42);
        // this function should raise a panic! due to result being unwraped is not Err
    result.unwrap_err();
}
