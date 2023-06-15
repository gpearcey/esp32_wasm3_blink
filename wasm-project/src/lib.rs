extern "C" {
    #[link_name = "blink_led"]
    fn blink_led(blink_gpio: i32, led_state: i32);

    #[link_name = "configure_led"]
    fn configure_led(blink_gpio: i32);
}

const LED: i32 = 0x02;
const LOW: i32 = 0x00;
const HIGH: i32 = 0x01;

unsafe fn delay_l(){
    let mut counter = 0;

    while counter < 100{
        counter += 1;
        blink_led(LED, LOW);
        blink_led(LED, LOW);
    }
}

unsafe fn delay_h(){
    let mut counter = 0;

    while counter < 100{
        counter += 1;
        blink_led(LED, HIGH);
        blink_led(LED, HIGH);
    }
}
unsafe fn _setup() {
    configure_led(LED);
}

unsafe fn _loop() {
    blink_led(LED, LOW);
    delay_l();
    blink_led(LED, HIGH);
    delay_h();
}

#[no_mangle]
unsafe fn _start() {
    _setup();
    loop {
        _loop();
    }
}