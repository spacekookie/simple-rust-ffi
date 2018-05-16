#include "greet.h"
#include <stdio.h>

int greet_me(const char *name)
{
    printf("Greetings there, nice to see you %s\n", name);
    return 0;
}

void be_rude(const char *name)
{
    printf("/me doesn't greet %s, only awkwardly nods\n", name);
}

void segfault()
{
    const char *s = NULL;
    printf( "%c\n", s[0] );
}