mod ddb_model;

/**
* Let's implement a basic verification checker on the data owned by a DynamoDB table. The
* check will verify that an invariant is satisfied by enumerating all possible states.
*
* Our Data:
* Dog(hungry=true, time=0)
*
* Operation 1 (executes every 3 time units, but can fail):
* Feeds the dog. So the dog is fed at time 3, 6, 9,... 
*
* Invariant: The dog is never hungry at a time that is a multiple of 3.
*
* Obviously, any invocation of Operation 1 can fail, and the Dog will stay hungry until 
* potentially, the next such operation succeeds. How can we model and represent this?
*/

fn main() {
    println!("Hello, world!");
}
