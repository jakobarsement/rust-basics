// Silence some warnings so they don't distract from the exercise.
#![allow(dead_code, unused_imports, unused_variables)]
use crossbeam::channel;
use std::thread;
use std::time::Duration;

fn expensive_sum(v: Vec<i32>) -> i32 {
    println!("Child thread: Starting");
    pause_ms(500);
    println!("Child thread: Finishing");
    // 1a. Between the .iter() and the .sum() add a .filter() with a closure to keep any even
    // number (`x % 2` will be 0 for even numbers).
    // 1b. Between the .filter() and the .sum() add a .map() with a closure to square the values
    // (multiply them by themselves)
    //
    // In the closures for both the .filter() and .map() the argument will be a reference, so you'll
    // either need to dereference the argument once in the parameter list like this: `|&x|` or you
    // will need to dereference it each time you use it in the expression like this: `*x`
    v.iter()
        .filter(|&x| return x % 2 == 0)
        .map(|x| return x * x)
        .sum()
}

fn pause_ms(ms: u64) {
    thread::sleep(Duration::from_millis(ms));
}

fn main() {
    let my_vector = vec![2, 5, 1, 0, 4, 3];

    // 2. Spawn a child thread and have it call `expensive_sum(my_vector)`.  Store the returned
    // join handle in a variable called `handle`. Once you've done this you should be able to run
    // the code and see the Child thread output in the middle of the main thread's letters

    let handle = thread::spawn(|| expensive_sum(my_vector));

    // While the child thread is running, the main thread will also do some work
    for letter in vec!["a", "b", "c", "d", "e", "f"] {
        println!("Main thread: Letter {}", letter);
        pause_ms(200);
    }

    // 3. Let's retrieve the value returned by the child thread once it has exited.  Using the
    // `handle` variable you stored the join handle in earlier, call .join() to wait for the thread
    // to exit with a `Result<i32, Err>`.  Get the i32 out of the result and store it in a `sum`
    // variable.  Uncomment the println.  If you did 1a and 1b correctly, the sum should be 20.
    //
    let sum = handle.join().expect("Error in `handle` thread");
    println!("The child thread's expensive sum is {}", sum);

    // Bonus: Get child thread to take longer than main thread, as such the `sum` value will not yet be computed.
    // What will happen in the program?

    // Answer: The main thread then waits on the child thread to finish before continuing execution. The .join()
    // method ensures that.

    // Time for some fun with threads and channels!  Though there is a primitive type of channel
    // in the std::sync::mpsc module, I recommend always using channels from the crossbeam crate,
    // which is what we will use here.
    //
    // 4. Uncomment the block comment below (Find and remove the `/*` and `*/`).  Examine how the
    // flow of execution works.  Once you understand it, alter the values passed to the `pause_ms()`
    // calls so that both the "Thread B" outputs occur before the "Thread A" outputs.

    let (tx, rx) = channel::unbounded();
    // Cloning a channel makes another variable connected to that end of the channel so that you can
    // send it to another thread.
    let tx2 = tx.clone();

    let handle_a = thread::spawn(move || {
        pause_ms(0);
        tx2.send("Thread A: 1").unwrap();
        pause_ms(200);
        tx2.send("Thread A: 2").unwrap();
    });

    pause_ms(100); // Make sure Thread A has time to get going before we spawn Thread B

    let handle_b = thread::spawn(move || {
        pause_ms(0);
        tx.send("Thread B: 1").unwrap();
        pause_ms(200);
        tx.send("Thread B: 2").unwrap();
    });

    // Using a Receiver channel as an iterator is a convenient way to get values until the channel
    // gets closed.  A Receiver channel is automatically closed once all Sender channels have been
    // closed.  Both our threads automatically close their Sender channels when they exit and the
    // destructors for the channels get automatically called.
    for msg in rx {
        println!("Main thread: Received {}", msg);
    }

    // Join the child threads for good hygiene.
    handle_a.join().unwrap();
    handle_b.join().unwrap();

    // Challenge 1: Make two child threads and give them each a sending end to a channel. From the
    // child threads, loop through several values and print each out and then send it to the channel.
    // On the main thread print out the values you receive. Close the sending side in the main
    // thread by calling `drop(tx)` (assuming you named your sender channel variable `tx`).  Join
    // the child threads.
    //-------------------------------------------------------------

    // Send values through first channel's sender
    let (sender, receiver) = channel::unbounded();
    let sender1 = sender.clone();
    let handle_1 = thread::spawn(|| {
        for el in [1, 2, 3, 4, 5] {
            println!("Sending {} through 1st thread", el);
            pause_ms(300);
            sender1.send(el).expect("sender1 errored...");
        }
        drop(sender1);
    });

    // Send values through second channel's sender
    let sender2 = sender.clone();
    let handle_2 = thread::spawn(move || {
        for el in [51, 52, 53, 54, 55] {
            println!("Sending {} through 2nd thread", el);
            pause_ms(300);
            sender2.send(el).expect("sender2 errored...");
        }
        drop(sender2);
    });
    handle_1.join().expect("join() failed for handle_1");
    handle_2.join().expect("join() failed for handle_2");
    drop(sender);

    for value in receiver {
        println!("Exercise, received2: {}", value);
    }

    // Challenge 2: Make two child threads and give them each a receiving end to a channel.  From the
    // main thread loop through several values and print each out and then send it to the channel.
    // On the child threads print out the values you receive. Close the sending side in the main
    // thread by calling `drop(tx)` (assuming you named your sender channel variable `tx`).  Join
    // the child threads.
    //-------------------------------------------------------------

    println!("Main thread: Exiting.");
}
