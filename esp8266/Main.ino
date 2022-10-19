#include "schedule.h"
#include "hardware.h"
#include "network.h"

/**
* Sets up the serial monitor baudrate
* Sets up the hardware, logic and network initial steps
* Runs the initial kernel initialization routine 
* Adds tasks to the schedule
* Sets up the timer for the interruptions
*/
void setup(void) {
  delay(100);
  Serial.begin(9600);
  setupHardware();
  post();
  setupLogic();
  if (setupNetwork() == 0) {
    blink_leds();
  }

  /* run the kernel initialization routine */
  Sched_Init();

  // Sched_AddT(receivePacket, 1, 21);

  Sched_AddT(read_sensors_data, 1, 156);

  // Sched_AddT(readSwitch_1, 1, 156);
  // Sched_AddT(readSwitch_2, 1, 156);
  // Sched_AddT(readSwitch_3, 1, 156);
  // Sched_AddT(readSwitch_4, 1, 156);
  Sched_AddT(updateLeds, 1, 156);

  /* disable all interrupts */
  noInterrupts(); 

  timer1_isr_init();
  timer1_attachInterrupt(Sched_Schedule);
  timer1_enable(TIM_DIV256, TIM_EDGE, TIM_LOOP);
  timer1_write(1000);

  /* enable all interrupts */
  interrupts();
}

void loop(void) {
  /* invokes the dispatcher to execute the highest priority ready task */
  Sched_Dispatch();
}
