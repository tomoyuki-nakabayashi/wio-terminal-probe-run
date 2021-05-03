# wio-terminal-probe-run

Wio Terminal で probe-run を体験する。

## 動作環境

https://github.com/tomoyuki-nakabayashi/Embedded-Rust-from-Basics/tree/main/debug を参考にデバッグアダプタが接続されている状態を前提とします。
FTDI のデバッグアダプタ (FT2232など) では現在のところ動作しないようです。

- JLink
- (多分動く) DAPLink

| JLink | Wio Terminal |
| --- | --- |
| VTref (1) | Grove コネクタ (J4/J5) 3番 |
| SWDIO (7) | FFC コネクタ (2) |
| SWCLK (9) | FFC コネクタ (1) |
| GND (4) | FFC コネクタ (6) |

## how to run

```shell
$ git clone https://github.com/tomoyuki-nakabayashi/wio-terminal-probe-run.git
$ cd wio-terminal-probe-run
$ cargo run
```

```
     Running `probe-run --chip ATSAMD51P19A --speed 100 target/thumbv7em-none-eabihf/debug/wio-terminal-probe-run`
  (HOST) INFO  flashing program (12.92 KiB)
  (HOST) INFO  success!
────────────────────────────────────────────────────────────────────────────────
       0 INFO  Hello, world!
└─ wio_terminal_probe_run::__cortex_m_rt_main @ src/main.rs:12
  (HOST) WARN  program has used at least 195528 bytes of stack space, data segments may be corrupted due to stack overflow
stack backtrace:
   0: lib::inline::__bkpt
        at ./asm/inline.rs:13
   1: __bkpt
        at ./asm/lib.rs:49
   2: wio_terminal_probe_run::exit
        at src/lib.rs:29
   3: wio_terminal_probe_run::__cortex_m_rt_main
        at src/main.rs:13
   4: main
        at src/main.rs:10
   5: ResetTrampoline
        at $HOME/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-rt-0.6.13/src/lib.rs:547
   6: Reset
        at $HOME/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-rt-0.6.13/src/lib.rs:550
   7: __DEFMT_MARKER_TIMESTAMP_WAS_DEFINED
   8: Reset
        at $HOME/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-rt-0.6.13/src/lib.rs:497
error: the stack appears to be corrupted beyond this point
```
