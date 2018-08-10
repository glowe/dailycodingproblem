#ifndef LIST_H
#define LIST_H
#include <stdbool.h>
#include <stdint.h>
typedef struct node node;
typedef struct list list;

list* list_new();
void list_free(list* lst);

int64_t list_size(const list* lst);
bool list_is_empty(const list* lst);

// TODO: support negative indexing, i.e., -list_size(list) <= index < list_size(list)
node* list_get(const list* lst, const uint64_t index);
void list_foreach(const list* lst, void fn(const node*));
void list_foreach_reversed(const list* lst, void fn(const node*));
void node_print(const node* curr);
uint64_t node_value(const node* curr);

void list_insert(list* lst, const uint64_t value, const uint64_t index);
void list_push(list* lst, const uint64_t value);
void list_append(list* lst, const uint64_t value);

node* list_remove(list* lst, const uint64_t index);
node* list_pop(list* lst);
void list_empty(list* lst);

void list_reverse(list* lst);
list* list_reversed(const list* lst);
#endif
