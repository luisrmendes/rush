#ifndef _NETWORK_H
#define _NETWORK_H

#include <ESP8266WiFi.h>
#include <WiFiUdp.h>
#include <string.h>
#include "user_interface.h"
#include "config.h"

#define BUFFER_LENGTH 256
char incomingPacket[BUFFER_LENGTH];

#define SWITCH_N 32
int switches[SWITCH_N];
int climateValues[3];

/* local network settings */
const char* ssid = SSID;
const char* password = PASSWORD;

/* TCP Server stuff */
const int serverPort = 4080;
WiFiServer server(serverPort);
WiFiClient client;

// Set your Static IP address
IPAddress local_IP(192, 168, 1, 69);
// Set your Gateway IP address
IPAddress gateway(192, 168, 1, 1);
IPAddress subnet(255, 255, 0, 0);

int setupNetwork(void) {
  // Configures static IP address
  if (!WiFi.config(local_IP, gateway, subnet)) {
    Serial.println("Failed to configure static ip address");
  }
  WiFi.mode(WIFI_STA);               /* station */
  wifi_set_sleep_type(NONE_SLEEP_T); /* LIGHT_SLEEP_T and MODE_SLEEP_T */
  Serial.print("Connecting to ");
  Serial.println(ssid);
  WiFi.begin(ssid, password);
  int i = 0;
  while (WiFi.status() != WL_CONNECTED) {
    delay(1000);
    Serial.print(++i);
    Serial.print(' ');
  }
  Serial.println("\nConnection established!");

  Serial.print("IP address: ");
  Serial.println(WiFi.localIP());
  server.begin();
  Serial.print("Open Telnet and connect to IP:");
  Serial.print(WiFi.localIP());
  Serial.print(" on port ");
  Serial.println(serverPort);

  return 0;
}

char* toArray(int number, int valueSize) {
  int i;
  char* numberArray = (char*)calloc(valueSize + 1, sizeof(char));
  for (i = valueSize - 1; i >= 0; --i, number /= 10) {
    numberArray[i] = (number % 10) + 48;
  }
  numberArray[valueSize] = '\0';
  return numberArray;
}

void send_light_packet(int brightnessValue, int temperatureValue, int humidityValue) {
  int brightnessValueSize = floor(log10(brightnessValue)) + 1;
  int temperatureValueSize = floor(log10(temperatureValue)) + 1;
  int humidityValueSize = floor(log10(humidityValue)) + 1;

  /* Allocate the adequate array size + 1 per array, + id + 3 spaces */
  char* msg = (char*)calloc(brightnessValueSize + 1 + temperatureValueSize + 1 + humidityValueSize + 1 + 4, sizeof(char));
  char* brightnessValueArray = toArray(brightnessValue, brightnessValueSize);
  char* temperatureValueArray = toArray(temperatureValue, temperatureValueSize);
  char* humidityValueArray = toArray(humidityValue, humidityValueSize);

  /* id = 0 */
  msg[0] = (char)(0 + 48);

  /* Append space */
  msg[1] = 32;

  /* Append brightness array value */
  memcpy(msg + 2, brightnessValueArray, (brightnessValueSize + 1) * sizeof(char));

  /* Append space */
  /* current size = 2 + brightnessValueSize + 1, using size - 1 because index */
  msg[(2 + brightnessValueSize + 1) - 1] = 32;

  /* Append temperature value */
  memcpy(msg + 2 + brightnessValueSize + 1, temperatureValueArray, (temperatureValueSize + 1) * sizeof(char));

  /* Append space, current size = 2 + brightnessValueSize + 1 + temperatureValueSize + 1 */
  msg[2 + brightnessValueSize + temperatureValueSize + 1] = 32;

  /* Append humidity value */
  memcpy(msg + 2 + brightnessValueSize + 1 + temperatureValueSize + 1, humidityValueArray, (humidityValueSize + 1) * sizeof(char));

  client.write(msg);

  free(brightnessValueArray);
  free(temperatureValueArray);
  free(humidityValueArray);
  free(msg);
}

/**
  * Attempts to reconnect in a while loop, not returning
  * Only returns when client is connected
  */
void handleTCPConnection() {
  while (!client) {
    digitalWrite(LED_3_PIN, 0);
    Serial.println("Waiting for TCP connection . . .");
    Serial.print("\tIP: ");
    Serial.print(WiFi.localIP());
    Serial.print(" on port ");
    Serial.println(serverPort);
    client = server.available();
    delay(2500);
  }
  digitalWrite(LED_3_PIN, 1);
}

#endif