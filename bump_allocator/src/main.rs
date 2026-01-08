use bump_allocator::Arena;

fn main() -> std::io::Result<()> {
    let arena = Arena::new()?;
    let x = arena.alloc(10)?;
    println!("x: {:?}\n", *x);
    let y = arena.alloc("Hello, World")?;

    *x = 15;
    println!("y: {:?}", y);
    println!("x: {:?}", x);

    println!("addr y: {:p}", y);
    println!("addr x: {:p}", x);
    arena.reset();

    Ok(())
}
