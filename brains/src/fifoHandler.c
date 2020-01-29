#include "fifoHandler.h"

int isFIFO(const char *path) {
    struct stat path_stat;
    stat(path, &path_stat);
    return S_ISFIFO(path_stat.st_mode);
}

int watchFIFO(char *path) {
    while(1) {
        if (isFIFO(path)) {
            return 1;
        }
        else 
            sleep(0.1);
    }
        
    return 0;
}

int readFIFO() {
    char str[100];
    
    while(1) {
        if (watchFIFO(FIFO2_PATH)) {
            int fd = open(FIFO2_PATH, O_RDONLY);
            readline(fd, str);
            printf("%s", str);
        }
        else 
            watchFIFO(FIFO2_PATH);
    }     

    return 0;
}

int writeFIFO() {
    
    mkfifo(FIFO1_PATH, 0666);
    int fd = open(FIFO1_PATH, O_WRONLY|O_CREAT);
    write(fd, "Hai, I'm here", strlen("Hai, I'm here"));
    close(fd);
    unlink(FIFO1_PATH);

    return 0;
}

int readline(int fd, char *str) {
    int n;
    do {
        n = read(fd,str,1);
    }
    while (n>0 && *str++ != '\0');
    return (n>0);
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

