# Rust Mastery Learning Plan

At the beginning, I only wanted to understand an article that was talking about aliasing XOR mutability and craft an async runtime from scratch. With some luck, this project would help me mix both ideas. According to Claude, not really, so I kept chatting with it to know what could help me understand the former and what could teach me the latter. In the end, I crafted some kind of curriculum. This repo reflects my learnings and projects.

As a do the project one by one, I review the the content. My progress are marked with a ---CURRENT PROGRESS--- so you can see what I already reviewed.

---

## Phase 0: Foundations (2-3 weeks)

**Goal**: Master traits, generics, and trait-based API design

### Projects
- [ ] Iterator Library (1 week)
- [ ] JSON Parser/Serializer (1-2 weeks)
- [ ] Intrusive Linked List (Optional, 1-2 days)

---

## Phase 1: Memory Management (2-3 weeks)

**Goal**: Deep understanding of borrow checker, unsafe Rust, and manual memory management

### Projects
- [ ] Arena Allocator (3-5 days)
- [ ] Malloc-like Allocator (1-2 weeks)
- [ ] Custom Smart Pointers (3-5 days)

---

## Phase 2: Asynchronous Programming (2-3 weeks)

**Goal**: Master async state machines, `Pin`, and cooperative concurrency

### Projects
- [ ] Async/Await Runtime (2-3 weeks)

---

## Phase 3: Concurrent Programming (2-3 weeks)

**Goal**: Master atomics, memory ordering, and lock-free data structures

### Projects
- [ ] Lock-Free MPSC Queue (2-3 weeks)

---

## Phase 0: Foundations

---CURRENT PROGRESS--- 

### Prerequisites
- Basic Rust ownership and borrowing
- Comfortable with structs and enums
- Understand what traits are
- Basic generics syntax

### Goals
- Master traits, generics, and associated types
- Understand zero-cost abstractions
- Learn trait-based API design
- Build composable, reusable code

### Project 1: Iterator Library (1 week)

**What to Build**:
- Core `MyIterator` trait with `next()` method
- Combinators: `map`, `filter`, `fold`, `take`, `skip`
- Adaptors: `chain`, `zip`, `enumerate`
- Collectors: `collect()` into `Vec` and custom containers

**Key Learning Outcomes**:
- Associated types vs generic parameters
- Zero-cost abstractions in practice
- Lifetime bounds on traits
- `impl Trait` syntax
- Method chaining patterns

**What Success Looks Like**:
```rust
let result: Vec<_> = vec![0..10]
    .my_iter()
    .filter(|x| x % 2 == 0)
    .map(|x| x * x)
    .take(3)
    .collect();
```

### Project 2: JSON Parser/Serializer (1-2 weeks)

**What to Build**:
- `Serialize` and `Deserialize` traits
- Support for primitives, structs, enums, `Vec`, `HashMap`
- Visitor pattern for deserialization
- Custom error types with proper error handling

**Key Learning Outcomes**:
- Trait bounds and `where` clauses
- Blanket implementations
- The visitor pattern (critical for async later)
- Associated types for complex relationships
- Trait-based API design

**What Success Looks Like**:
```rust
#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u32,
}

let json = to_json(&person)?;
let person: Person = from_json(&json)?;
```

### Optional: Intrusive Linked List (1-2 days)

**What to Build**:
- Doubly-linked list with `Rc<RefCell<Node>>`
- Previous pointer using `Weak<T>` to prevent cycles
- Basic operations: insert, remove, traverse

**Key Learning Outcomes**:
- `Box`, `Rc`, `Weak` basics
- Interior mutability with `RefCell`
- Why reference cycles leak memory
- When to use smart pointers

### End State Checklist
- ✅ Can write generic, composable APIs
- ✅ Understand trait bounds and associated types
- ✅ Know when to use static vs dynamic dispatch
- ✅ Comfortable with complex trait hierarchies
- ✅ Can design trait-based abstractions

### Readings

**Before Starting**:
- **"Programming Rust" 2nd Edition**
  - Chapter 11: Traits and Generics
  - Chapter 13: Utility Traits
  - Chapter 15: Iterators

**While Building**:
- **"Rust for Rustaceans"** by Jon Gjengset - Chapter 2: Types
- Study `serde` source code (after JSON parser first attempt)
- **"Rust Design Patterns"** (https://rust-unofficial.github.io/patterns/)

---

## Phase 1: Memory Management

### Prerequisites
- Completed Phase 0
- Comfortable with traits and generics
- Understand basic lifetimes
- Know what unsafe code is (high-level)

### Goals
- Deep understanding of borrow checker motivation
- Master unsafe Rust fundamentals
- Understand aliasing XOR mutability viscerally
- Learn manual memory management
- Build smart pointers from scratch

### Project 3: Arena Allocator (3-5 days)

**What to Build**:
- Bump allocator from contiguous buffer
- Type-safe allocation with lifetime bounds (`&'arena T`)
- Reset functionality (bulk deallocation)
- Interior mutability for bump pointer (`Cell<usize>`)

**Key Learning Outcomes**:
- Lifetime parameters in depth
- Basic `unsafe` pointer arithmetic
- `NonNull<T>` usage
- Why use-after-free is prevented by lifetimes
- Interior mutability patterns

**What Success Looks Like**:
```rust
let arena = Arena::new(1024);
let x = arena.alloc(42);
let y = arena.alloc(100);
// arena.reset();  // Would invalidate x and y - prevented by borrow checker!
println!("{} {}", x, y);
```

### Project 4: Malloc-like Allocator (1-2 weeks)

**What to Build**:
- Free list management (linked list of free blocks)
- `alloc()` and `free()` operations
- Coalescing adjacent free blocks
- First-fit or best-fit allocation strategy
- Proper alignment handling

**Key Learning Outcomes**:
- **THE core aliasing XOR mutability lesson**
- Complex pointer manipulation in unsafe code
- Maintaining invariants manually
- Why the borrow checker rules exist
- Memory corruption patterns (and how to avoid them)

**What Success Looks Like**:
```rust
let allocator = Allocator::new(4096);
let ptr1 = allocator.alloc(Layout::new::<i32>());
let ptr2 = allocator.alloc(Layout::new::<String>());
allocator.free(ptr1);
let ptr3 = allocator.alloc(Layout::new::<i32>());  // Reuses ptr1's block
```

### Interlude: Custom Smart Pointers (3-5 days)

**What to Build**:
- `MyBox<T>` - owned heap allocation
- `MyRc<T>` - reference counting
- `MyWeak<T>` - weak references to break cycles
- (Optional) `MyArc<T>` - thread-safe reference counting

**Key Learning Outcomes**:
- `NonNull<T>` and `PhantomData<T>`
- Manual `Drop` implementation
- Deref coercion
- Reference counting implementation
- Building on malloc knowledge
- `Send` and `Sync` traits (for `Arc`)

**What Success Looks Like**:
```rust
let x = MyRc::new(42);
let y = MyRc::clone(&x);
assert_eq!(MyRc::strong_count(&x), 2);
drop(y);
assert_eq!(MyRc::strong_count(&x), 1);
```

### End State Checklist
- ✅ Deeply understand why borrow checker rules exist
- ✅ Comfortable writing and reviewing unsafe code
- ✅ Can reason about lifetimes and ownership at low level
- ✅ Understand smart pointer internals
- ✅ Know when unsafe is necessary and how to use it safely

### Readings

**Before Starting**:
- **"The Rustonomicon"** (https://doc.rust-lang.org/nomicon/)
  - Ownership, Lifetimes, Working with Uninitialized Memory
- **"Computer Systems: A Programmer's Perspective"** - Chapter 9 (Virtual Memory)

**While Building**:
- **"The Garbage Collection Handbook"** - Chapters on manual memory management
- **"Rust for Rustaceans"** - Chapter 2 (Types), Chapter 8 (Unsafe)
- **"Programming Rust"** - Chapter 13 (Smart Pointers)

---

## Phase 2: Asynchronous Programming

### Prerequisites
- Completed Phase 1
- Comfortable with unsafe Rust
- Deep understanding of ownership and lifetimes
- Familiar with trait-based abstractions from Phase 0

### Goals
- Understand async state machines
- Master `Pin` and self-referential structs
- Learn cooperative concurrency model
- Build async abstractions from scratch
- Understand `Waker` system

### Project 5: Async/Await Runtime (2-3 weeks)

**Week 1: Foundation**
- Task queue with basic executor
- Implement simple `Future` trait
- Understand `Poll` and state machines
- Manual async state machines (before using `async fn`)

**Week 2: Waker System**
- Implement `Waker` and `Context`
- Task spawning and scheduling
- Basic async I/O (timers)
- Understanding ownership transfer across poll calls

**Week 3: Polish**
- Joining tasks and collecting results
- Error propagation in async context
- Basic async network I/O
- Multiple tasks with priority scheduling

**Key Learning Outcomes**:
- How `async fn` transforms into state machines
- `Pin<&mut T>` and why it's needed
- Self-referential structs problem
- `Waker` cloning and ownership
- Single-threaded async patterns
- The difference between concurrency and parallelism

**What Success Looks Like**:
```rust
async fn fetch_data(url: &str) -> Result<String, Error> {
    let response = http_get(url).await?;
    Ok(response)
}

let runtime = Runtime::new();
runtime.block_on(async {
    let result = fetch_data("http://example.com").await;
    println!("{:?}", result);
});
```

### End State Checklist
- ✅ Understand async state machines deeply
- ✅ Know when and how to use `Pin<&mut T>`
- ✅ Can implement custom `Future` types
- ✅ Understand `Waker` system internals
- ✅ Can build async abstractions
- ✅ Understand cooperative vs preemptive concurrency

### Readings

**Before Starting**:
- **"Asynchronous Programming in Rust"** (https://rust-lang.github.io/async-book/) - Read cover-to-cover
- **"Rust for Rustaceans"** - Chapter 9: Asynchronous Programming

**While Building**:
- **"Programming Rust"** - Chapter 20: Asynchronous Programming
- Rust async/await RFC 2394
- Study `tokio` source code after first implementation

---

## Phase 3: Concurrent Programming

### Prerequisites
- Completed Phase 2
- Deep understanding of ownership
- Comfortable with unsafe Rust
- Understand cooperative concurrency from async
- Ready for preemptive concurrency

### Goals
- Master atomic operations and memory ordering
- Understand where borrow checker can't help
- Learn lock-free data structures
- Handle concurrent aliasing manually
- Understand memory reclamation strategies

### Project 6: Lock-Free MPSC Queue (2-3 weeks)

**Week 1: Lock-Based Foundation**
- Build multi-threaded queue with `Mutex<VecDeque<T>>`
- Understand the baseline performance
- See where locks cause contention
- Multiple producer threads pushing, one consumer popping

**Week 2: Lock-Free SPSC**
- Single-producer, single-consumer lock-free queue
- Introduction to `AtomicPtr` and `AtomicUsize`
- Basic memory ordering (`Relaxed`, `Acquire`, `Release`)
- No memory reclamation issues (simpler case)

**Week 3: Lock-Free MPSC**
- Multi-producer, single-consumer lock-free queue
- Compare-and-swap loops
- ABA problem and solutions
- Epoch-based memory reclamation
- Extensive testing for race conditions

**Key Learning Outcomes**:
- **Memory ordering** (`Acquire`, `Release`, `SeqCst`, `Relaxed`)
- Where the borrow checker fundamentally cannot help
- ABA problem (pointer recycling hazard)
- Memory reclamation in concurrent context
- Concurrent aliasing patterns
- Testing concurrent code
- Performance profiling under contention

**What Success Looks Like**:
```rust
let queue = MPSCQueue::new();
let queue_ref = Arc::new(queue);

// Multiple producers
for i in 0..4 {
    let q = queue_ref.clone();
    thread::spawn(move || {
        for j in 0..1000 {
            q.push(i * 1000 + j);
        }
    });
}

// Single consumer
while let Some(item) = queue_ref.pop() {
    println!("{}", item);
}
```

### End State Checklist
- ✅ **Complete mastery of Rust fundamentals**
- ✅ Understand memory ordering deeply
- ✅ Can write production-quality concurrent code
- ✅ Know when and how to use unsafe correctly
- ✅ Understand lock-free algorithm trade-offs
- ✅ Can reason about concurrent memory safety

### Readings

**Before Starting**:
- **"C++ Concurrency in Action"** - Chapter 7 (Lock-Free Data Structures) - CRITICAL
- **"The Art of Multiprocessor Programming"** - Chapters 10-11

**While Building**:
- **"Rust Atomics and Locks"** by Mara Bos
- Michael & Scott paper: "Simple, Fast, and Practical Non-Blocking and Blocking Concurrent Queue Algorithms"
- Study `crossbeam` source code (epoch-based reclamation)

---

## Resources

### Books to Own
1. **"Rust for Rustaceans"** by Jon Gjengset
2. **"Programming Rust" 2nd Edition**
3. **"Rust Atomics and Locks"** by Mara Bos

### Books to Access
4. **"C++ Concurrency in Action"** by Anthony Williams
5. **"The Art of Multiprocessor Programming"** by Herlihy & Shavit
6. **"Computer Systems: A Programmer's Perspective"** (CS:APP)
7. **"The Garbage Collection Handbook"**

### Free Online Resources
- **The Rustonomicon**: https://doc.rust-lang.org/nomicon/
- **Async Book**: https://rust-lang.github.io/async-book/
- **Rust Design Patterns**: https://rust-unofficial.github.io/patterns/

---

## Timeline

**Total Duration**: 9-14 weeks (2-3.5 months)

- **Phase 0**: 2-3 weeks
- **Phase 1**: 2-3 weeks
- **Phase 2**: 2-3 weeks
- **Phase 3**: 2-3 weeks

---

## After Completion

### You'll Be Ready For
- Contributing to major Rust projects (tokio, rust-analyzer, servo)
- Building production systems programming projects
- Reviewing unsafe code confidently
- Teaching others Rust fundamentals

### Next-Level Projects
- Database Storage Engine (B-tree or LSM-tree)
- JIT Compiler
- Operating System Component
- Custom Garbage Collector
