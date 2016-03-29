This is a library to get joystick input.

Example usage:

```rust
let mut js_dev = joy::Device::open("/dev/input/js0\0".as_bytes()).unwrap();
loop {
    for ev in &mut js_dev {
        use joy::Event::*;
        match ev {
            Axis(n, x) => println!("axis {} moved to {}", n, x),
            Button(n, true) => println!("button {} pressed", n),
            Button(n, false) => println!("button {} released", n),
        }
    }
}
```
