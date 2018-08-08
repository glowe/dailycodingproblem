#ifndef LIST_H
#define LIST_H
#include <stdbool.h>
#include <stdint.h>
typedef struct node node;
typedef struct list list;

list* list_new();
void list_free(list* list);

int64_t list_size(const list* list);
bool list_is_empty(const list* list);

// TODO: support negative indexing, i.e., -list_size(list) <= index < list_size(list)
node* list_get(const list* list, const int index);
void list_foreach(const list* list, void fn(const node*));
void list_foreach_reversed(const list* list, void fn(const node*));
void node_print(const node* node);
uint64_t node_value(const node* node);

void list_insert(list* list, const int value, const int index);
void list_push(list* list, const int value);
void list_append(list* list, const int value);

node* list_remove(list* list, const int index);
node* list_pop(list* list);
void list_empty(list* list);

void list_reverse(list* list);
list* list_reversed(const list* list);
#endif
