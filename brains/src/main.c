#include "fifoHandler.h"

int main() {
    setvbuf(stdout, NULL, _IONBF, 0);

    // printf("%s", getCmdOutput("ls"));
    //writeFIFO();
    char fifoArg[] = "";
    readFIFO(FIFO2_PATH, &fifoArg);

    printf("%s", fifoArg);
    return 0;
}
