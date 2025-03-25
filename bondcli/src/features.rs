/** Features are the building blocks for distributed systems. Every system or tool fundamentally
* implements some combination of these features. Tracking a system's features provides us a
* standard interface, allowing for direct comparison with other systems. It also allows the reasoning 
* engine to introduce and evaluate failure states.
*/

// Consistency Models
trait EventuallyConsistent {}
trait StronglyConsistent {}

// Completion Semantic Models
trait AtLeastOnce {}
trait AtMostOnce {}
trait ExactlyOnce {}

// Asynchronous Operation Models
trait Parallel {}
trait Sequential {}
