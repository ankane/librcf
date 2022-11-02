#include <stdio.h>
#include <stdlib.h>

#include "rcf.h"

float randf() {
    return rand() / (float) RAND_MAX;
}

int main() {
    rcf_forest *forest = rcf_create(3);

    for (int i = 0; i < 200; i++) {
        float point[] = {randf(), randf(), randf()};

        // make the second to last point an anomaly
        if (i == 198) {
            point[1] = 2;
        }

        double score = rcf_score(forest, point);
        printf("score = %f\n", score);
        rcf_update(forest, point);
    }

    rcf_free(forest);
    return 0;
}
