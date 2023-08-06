target extended-remote :2331

# set print asm-demangle on
# set backtrace limit 32

break DefaultHandler
break HardFault
break rust_begin_unwind

break main

monitor semihosting enable
monitor semihosting IOClient 2

load
stepi
