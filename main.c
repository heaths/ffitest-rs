// gcc -L target/debug main.c -l ffitest -o target/debug/main -Wl,-rpath,target/debug

#include <stdio.h>

unsigned int println_env(const char* var);

int main(int argc, char* argv[])
{
    if (argc != 2)
    {
        fprintf(stderr, "Error: require environment variable name\n");
        return 1;
    }

    return println_env(argv[1]);
}
