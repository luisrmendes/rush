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
  Serial.println("Reading dht");
  int chk = DHT11.read(DHT11PIN);
  Serial.println("Done reading dht");


  int humidity = (int)DHT11.humidity;
  int temperature = (int)DHT11.temperature;
  int brightness = analogRead(A0);

  climateValues[0] = brightness;
  climateValues[1] = temperature;
  climateValues[2] = humidity;

  Serial.print("Brightness: ");
  Serial.println(brightness);
  Serial.print("Temperature: ");
  Serial.println(temperature);
  Serial.print("Humidity: ");
  Serial.println(humidity);
  
  Serial.println("Sending light packet");
  send_light_packet(climateValues[0], climateValues[1], climateValues[2]);
  Serial.println("Done sending light packet");
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