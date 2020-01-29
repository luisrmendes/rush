#include "fifoHandler.h"

int main() {
    setvbuf(stdout, NULL, _IONBF, 0);

    // printf("%s", getCmdOutput("ls"));
    //writeFIFO();
    readFIFO();

    return 0;
}
