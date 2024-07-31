echo "Debugging $1"

# kill any openocd instances
killall openocd

openocd -f ${TOOLS_DIR}/openocd/stlink-stm32f1.cfg \
    -c init -c halt \
    -c "arm semihosting enable" \
    -c "flash write_image erase $1" \
    -c reset
