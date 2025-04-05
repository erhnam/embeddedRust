qemu-system-arm \
  -cpu cortex-m4 \
  -machine olimex-stm32-h405 \
  -nographic \
  -semihosting-config enable=on,target=native \
  -gdb tcp::3333 \
  -S \
  -kernel target/thumbv7em-none-eabihf/debug/examples/hello
