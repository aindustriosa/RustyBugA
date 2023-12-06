# Uses openocd to flash the firmware.
# This script is used from `cargo run` when used the apropiate option in .cargo/config (parameter: runner)

openocd -f openocd.cfg -c init -c halt -c "arm semihosting enable" -c "flash write_image erase $1" -c reset # -c shutdown