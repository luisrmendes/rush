#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>


int readline(int fd, char *str)
{
    int n;
    do {
        n = read(fd,str,1);
    }
    while (n>0 && *str++ != '\0');
    return (n>0);
}

int main() {

    char str[100];

    mkfifo("myfifo", 0666);
    int fd = open("myfifo", O_RDONLY);
    while(readline(fd,str)) printf("%s",str);
    close(fd);

    return 0;
}

 