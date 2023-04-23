#include <stdio.h>
#include <unistd.h>

int getchar_emulation() {
    char a;

    a = getchar();

    return a;
}