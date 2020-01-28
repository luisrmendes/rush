#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>

char FIFO_PATH[] = "/tmp/brainsfifo";

int readFIFO();
int writeFIFO();
char* getCmdOutput(char *cmd);
char* stringAdd(const char *s1, const char *s2);


int main() {

    printf("%s", getCmdOutput("ls"));
    writeFIFO();

    return 0;
}

int writeFIFO() {
    
    mkfifo(FIFO_PATH, 0666);
    int fd = open(FIFO_PATH, O_WRONLY|O_CREAT);
    write(fd, "Hai, I'm here", strlen("Hai, I'm here"));
    close(fd);
    unlink(FIFO_PATH);

    return 0;
}

char* stringAdd(const char *s1, const char *s2) {
    const size_t len1 = strlen(s1);
    const size_t len2 = strlen(s2);
    char *result = malloc(len1 + len2 + 1); // +1 for the null-terminator
    
    memcpy(result, s1, len1);
    memcpy(result + len1, s2, len2 + 1); // +1 to copy the null-terminator
    return result;
}

char* getCmdOutput(char *cmd) {
    char *b = malloc(121);
    char *buf = malloc(121);
    
    FILE *fp;
    if ((fp = popen(cmd, "r")) == NULL) {
       printf("Error opening pipe!\n");
       return "";
    }

    buf = fgets(b, 121, fp);
    fflush(fp);
    if(pclose(fp))  {
        printf("Command not found or exited with error status\n");
        return "false";
    }

   return buf;
}
