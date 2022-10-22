#ifndef _LOGIC_H
#define _LOGIC_H

#include "hardware.h"
#include "config.h"

#include <dht11.h>
#define DHT11PIN 16
dht11 DHT11;


void setupLogic(void) {
  climateValues[0] = 1000;
}

void read_sensors_data(void) {
  int chk = DHT11.read(DHT11PIN);

  int humidity = (int)DHT11.humidity;
  int temperature = (int)DHT11.temperature;
  // int humidity = 1;
  // int temperature = 1;
  int brightness = analogRead(A0);

  climateValues[0] = brightness;
  climateValues[1] = temperature;
  climateValues[2] = humidity;

  // Serial.print("\n");
  // Serial.print(temperature);
  // Serial.print("\n");
  // Serial.print(humidity);
  send_light_packet(climateValues[0], climateValues[1], climateValues[2]);
}

// void readSwitch_1(void) {
//   int newSwitch = !digitalRead(SWITCH_1_PIN);
//   if (newSwitch != localSwitches[0]) {
//     switches[SWITCH1_ID] = !switches[SWITCH1_ID];
//     localSwitches[0] = !localSwitches[0];
//     sendSwitchPacket(SWITCH1_ID);
//   }
// }

#endif