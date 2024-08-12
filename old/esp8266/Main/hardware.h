#ifndef _HARDWARE_H
#define _HARDWARE_H

#include "network.h"
#include "config.h"

/**
* Sets up the pin modes, OUTPUT for the leds, 
* INPUT_PULLUP for the switches
*/
void setupHardware(void) {
  pinMode(LED_1_PIN, OUTPUT);
  pinMode(LED_2_PIN, OUTPUT);
  pinMode(LED_3_PIN, OUTPUT);
  pinMode(LED_4_PIN, OUTPUT);
  pinMode(LED_BUILTIN, OUTPUT);

  digitalWrite(LED_1_PIN, 0);
  digitalWrite(LED_2_PIN, 0);
  digitalWrite(LED_3_PIN, 0);
  digitalWrite(LED_4_PIN, 0);
  digitalWrite(LED_BUILTIN, 0);
}

/**
* Blinks leds twice, used for visual confirmation
* of a network connection established
*/
void blink_leds(void) {
  const size_t led_array_size = 4;
  int led_array[led_array_size] = { LED_1_PIN, LED_2_PIN, LED_3_PIN, LED_4_PIN };

  for (unsigned int i = 0; i < led_array_size; i++) {
    digitalWrite(led_array[i], 1);
  }
  delay(200);
  for (unsigned int i = 0; i < led_array_size; i++) {
    digitalWrite(led_array[i], 0);
  }
  delay(200);
  for (unsigned int i = 0; i < led_array_size; i++) {
    digitalWrite(led_array[i], 1);
  }
  delay(200);
  for (unsigned int i = 0; i < led_array_size; i++) {
    digitalWrite(led_array[i], 0);
  }
}

/**
* Power-On Self Test
* Displays a series of checks of the switches and leds
* First: blink all leds
* Second: blink leds in a sequential order to verify the leds order
* Third: Flashes each led if the corersponding switch is ON
* Fourth: Flashes each led if the corersponding switch is OFF
*/
void post(void) {
  Serial.print("\nStarting POST");
  const size_t led_array_size = 4;
  int led_array[led_array_size] = { LED_1_PIN, LED_2_PIN, LED_3_PIN, LED_4_PIN };

  int small_delay = 60;

  for (unsigned int i = 0; i < led_array_size; i++) {
    digitalWrite(led_array[i], 1);
  }
  delay(1000);
  for (unsigned int i = 0; i < led_array_size; i++) {
    digitalWrite(led_array[i], 0);
  }

  /* fancy flash */
  for (unsigned int i = 0; i < led_array_size; i++) {
    digitalWrite(led_array[i], 1);
    delay(small_delay);
    digitalWrite(led_array[i], 0);
  }

  /* invert fancy flash */
  for (int i = led_array_size - 1; i >= 0; i--) {
    digitalWrite(led_array[i], 1);
    delay(small_delay);
    digitalWrite(led_array[i], 0);
  }

  delay(500);

  /* Flash switches OFF state */
  for (unsigned int i = 0; i < led_array_size; i++) {
    digitalWrite(led_array[i], 0);
  }

  delay(700);

  /* Flash switches ON state */
  for (unsigned int i = 0; i < led_array_size; i++) {
    digitalWrite(led_array[i], 1);
  }

  delay(700);

  /* Leds OFF */
  for (unsigned int i = 0; i < led_array_size; i++) {
    digitalWrite(led_array[i], 0);
  }

  Serial.print("\nDone\n");
}

/**
* Updates the leds state according to the receiving packets values,
* assigned to the global arrays switches and climateValues.
* Transmites back to the multicast address the post-change led states
*/
void updateLeds(void) {
  // int led1State = 0;
  // int led2State = 0;
  // int led3State = 0;
  int led4State = (climateValues[0] < 500);

  // digitalWrite(LED_1_PIN, led1State);
  // digitalWrite(LED_2_PIN, led2State);
  // digitalWrite(LED_3_PIN, led3State);
  digitalWrite(LED_4_PIN, led4State);
}

#endif