# ipcalc

IP address calculator

### USAGE

- Running development version
    ```command
    % cargo run -- 1.2.3.4/30
        Finished dev [unoptimized + debuginfo] target(s) in 0.03s
        Running `target/debug/ipcalc-rust 1.2.3.4/30`
    ip_address      : [  1,   2,   3,   4], 0x01020304
    subnet_mask     : [255, 255, 255, 252], 0xfffffffc, /30
    subnet_addr     : [  1,   2,   3,   4], 0x01020304
    broadcast_addr  : [  1,   2,   3,   7], 0x01020307
    subnet_first    : [  1,   2,   3,   5], 0x01020305
    subnet_last     : [  1,   2,   3,   6], 0x01020306
    ```

- Running release version
    ```command
    $ cargo build --release
    $ ./target/release/ipcalc-rust 1.2.3.4/30
    ip_address      : [  1,   2,   3,   4], 0x01020304
    subnet_mask     : [255, 255, 255, 252], 0xfffffffc, /30
    subnet_addr     : [  1,   2,   3,   4], 0x01020304
    broadcast_addr  : [  1,   2,   3,   7], 0x01020307
    subnet_first    : [  1,   2,   3,   5], 0x01020305
    subnet_last     : [  1,   2,   3,   6], 0x01020306
    ```
