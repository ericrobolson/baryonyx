# Language goals
* VM specification for a message based, stack oriented language.
* Combines concepts from Erlang's actor model, Smalltalk's object model and Forth's stack model

# Program Concepts
* Programs make up the core primitive
* Programs are used for keeping track of state, executing code, and processing messages
* Distributed focused; whether it's across many systems or through green threads, native threads, etc.  

## State
* `execution_stack` - A stack which represents the current execution
* `state_stack` - A stack which represents the current execution state, such as `update`, `pause`, `init`, etc.
* A dictionary which represents the state of the program
* A queue of messages to process

## Core Properties
* Each program has a unique address
* Each program can send and receive messages to other addresses
* A program can run programs within itself
* Each program has an ```update``` method which will process one message at a time from it's queue
* Each program has a ```pause``` function which will immediately suspend the program, push the current state on to the `state_stack`
* Each program has a ```resume``` function which will pop off and resume the last state on the `state_stack`
* Each program has a ```init``` function which will run some code when it is first created
* Each program has a ```on_error``` function which will run some code when it encounters an error
* Each program has a ```death``` function which will run some code when it is destroyed
* **Actually needed?** Each program has a ```msg_count``` property which will let it know how many messages are outstanding
* **Actually needed?** Each program has a ```stack_size``` property which will let it know how large the stack is
* **Actually needed?** Each program has a ```max_stack_size``` property which will let it know how large the stack size is
* **Actually needed?** Each program has a ```dictionary_size``` property which will let it know how large the dictionary is
* **Actually needed?** Each program has a ```max_dictionary_size``` property which will let it know how large the max dictionary size is
* Each program should be able to run in isolation, resulting in total concurrency
* The update method uses a simple stack based language so that it can be paused or resumed at any time

## Language Properties
* Stack based language inspired by Forth
