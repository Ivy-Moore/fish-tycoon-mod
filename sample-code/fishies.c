#include <stdio.h>
#include <string.h>

#define NUM_FISHES 20
#define MAX_FISH_LINE 256

int main()
{
    char fish_buffer[MAX_FISH_LINE];

    FILE *file = fopen("fish", "r");

    if(!file) {
        puts("Could not find your fish file");
        return -1;
    }

    // create a fixed-space in the data section of the executable
    static char fishes[MAX_FISH_LINE][NUM_FISHES];

    // zero out the fishies
    memset(fishes, 0, MAX_FISH_LINE*NUM_FISHES);

    // Read the fishies into the fishes array
    for(int i = 0; i < NUM_FISHES && fgets(fishes[i], MAX_FISH_LINE, file) != NULL; ++i) {
        // Remove new line that fgets includes
        fishes[i][strcspn(fishes[i], "\n")] = 0;
    }

    fclose(file);

    int idx = 0;

    char *fish_by_idx = fishes[0];

    switch(idx) {
        case 0: fish_by_idx = fishes[0]; break;
        case 1: fish_by_idx = fishes[1]; break;
    }
    
    printf("%s\n", fish_by_idx);

    return 0;
}
