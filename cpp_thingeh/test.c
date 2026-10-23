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
