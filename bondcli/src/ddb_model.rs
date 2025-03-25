/**
* What are the different usage modes of DDB? When you have a collection of tables and you want to
* enforce data invariants in a distributed context, what are the relevant details?
*
* 1. Whether or not the client is using optimistic locking. This changes the nature of save/update
*    operations, leading to client-side failures if the version number in the updated item differs
*    from the one that is already persisted at the time. This allows for concurrent updates that do
*    not clobber each other, forcing the client to re-concile conflicts by reading the most recent
*    version.
*
* 2. Whether or not the client is using strongly consistent DDB reads. If so, reads are not
*    eventually consistent but come at the cost of higher latency. This is important to clients
*    that require recently updated data to always be reflected (banks, casinos, etc). Clients can
*    specify read behaviour on a per-call basis. Strongly consistent reads are not supported on
*    GSIs because GSIs receive updates asynchronously from the main table. Making these updates
*    synchronous would delay main table updates, something this is not performance optimal.
*
* 3. Whether or not the client is using DDB global tables. Global tables allow clients to
*    write/update the same item in multiple regions. DDB will replicate the writes to other regions
*    asychronously. This can lead to subtle write clobbering issues (when another region writes to
*    the same item that you just wrote to, thereby clobbering your write).
*
* Given this information, we can allow clients to represent their interactions with DDB through a
* builder interface. This then allows us to infer and reason about failure states when combined
* with other components.
*/

struct DdbOperation {
    table_name: String
}
