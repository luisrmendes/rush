#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <time.h>

#define FIFO1_PATH "/tmp/brains_to_telegram"
#define FIFO2_PATH "/tmp/telegram_to_brains"

int readFIFO();
int writeFIFO();
char* getCmdOutput(char *cmd);
char* stringAdd(const char *s1, const char *s2);
int readline(int fd, char *str);
int isFIFO(const char *path);
int watchFIFO(char *path);