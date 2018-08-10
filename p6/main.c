#include <assert.h>
#include <stdio.h>
#include "list.h"

int main(int argc, char *argv[]) {
    setvbuf(stdout, NULL, _IONBF, 0);
    list* l = list_new();
    printf("inserting into list 9 times...\n");
    for (int i = 0; i < 10; i++) {
        printf("list_insert(l, %d, 0)\n", i);
        list_insert(l, i, 0);
    }
    printf("now calling list_foreach...\n");
    list_foreach(l, node_print);
    printf("retrieving from list 9 times...\n");
    for (int i = 0; i < 10; i++) {
        node* curr = list_get(l, i);
        assert(node_value(curr) == 9 - i);
    }
    printf("now calling list_foreach...\n");
    list_foreach(l, node_print);
    printf("and now in reverse in place...\n");
    list_reverse(l);
    list_foreach(l, node_print);
    printf("and a reversed copy...\n");
    list* rev = list_reversed(l);
    list_foreach(rev, node_print);
    printf("and foreach in reverse on the reversed copy...\n");
    list_foreach_reversed(rev, node_print);
    printf("appending...\n");
    list_append(rev, 10);
    list_foreach(rev, node_print);
    list_free(rev);
    list_free(l);
    return 0;
}
