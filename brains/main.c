#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char* get_cmd_output(char *cmd);
char* stringAdd(const char *s1, const char *s2);


int main() {

    printf("%s", get_cmd_output("ls"));

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

char* get_cmd_output(char *cmd) {
    char *b = malloc (121);
    char *buf = malloc (121);
    
    FILE *fp;
    if ((fp = popen(cmd, "r")) == NULL) {
       printf("Error opening pipe!\n");
       return "";
    }

    buf = fgets(b, 121, fp);
    fflush(fp);
    if(pclose(fp))  {
        //printf("Command not found or exited with error status\n");
        return "false";
    }

   return buf;
}
