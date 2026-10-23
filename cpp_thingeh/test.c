#define RICKROLL "Never Gonna Give You Up"
#include <stdint.h>
    #include <stdint.h>

// Comment test 
#define DUCKROLL "siiiiiiiix seeeeeeeeeeveeeeeeeen" // comments here tooooo
/* this is a block comment: 
 * it wraps around to lines and shit
 * look at it go!!! 
 * // Also a double-slash shouldn't modify the comment state
 */
// /* not here either
#ifdef UNDEFINED
char* rickroll(int wat){
    switch (wat) {
        case 0: // Rickroll, not Sandroll
            return "Never gonna give you up";
        case 1: // Sandroll, not Rickroll
            return "Never gonna giv-... Never gonna giv-... Never gonna- Never gonna- Never gonna- Neve- Neve- Neve- Neve-"
        default:
            return "";
    }
}
#else /* ifdef UNDEFINED */
char* rickroll(int really){
    if (really) {
        return "Never gonna give you up";
    } else {
        return "";
    }
}

#endif /* ifdef UNDEFINED ... else */

int printf(const char* format, ...);

#define LONG_AHH_CALCULATION 1 + 2 + 4 - 6 - 9 * 67 \
                                - (int)(6 / 9) * (6 / 7)

int main(int argc, char** argv) {
    float sixseven = 6 / 7;
    printf("%s %f %i", RICKROLL, sixseven, argc);
}
