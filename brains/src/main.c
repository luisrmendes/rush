#include "fifoHandler.h"


int main() {
    setvbuf(stdout, NULL, _IONBF, 0);

    readFIFO(FIFO2_PATH);

    return 0;
}
