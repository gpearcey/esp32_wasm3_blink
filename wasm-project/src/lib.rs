////use std::thread;
////use std::time::Duration;
//
//extern "C" {
//    #[link_name = "blink_led"]
//    fn blink_led(blink_gpio: i32, led_state: i32);
//
//    #[link_name = "configure_led"]
//    fn configure_led(blink_gpio: i32);
//}
//
//const LED: i32 = 0x02;
//const LOW: i32 = 0x00;
//const HIGH: i32 = 0x01;
//
//unsafe fn _setup() {
//    configure_led(LED);
//}
//
//unsafe fn _loop() {
//    blink_led(LED, LOW);
//    //thread::sleep(Duration::from_millis(500));
//    blink_led(LED, HIGH);
//    //thread::sleep(Duration::from_millis(500));
//}

#[no_mangle]
unsafe fn _start() {
    //_setup();
    //loop {
    //    _loop();
    //}
}