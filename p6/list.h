#ifndef LIST_H
#define LIST_H
typedef struct node node;
typedef struct list list;

list* list_new();
void list_free(list* list);

int64_t list_size(const list* list);
bool list_is_empty(const list* list);

node* list_get(const list* list, const int index);
void list_foreach(const list* list, void fn(const node*));
void node_print(const node* node);

void list_insert(list* list, const int value, const int index);
void list_push(list* list, const int value);
void list_append(list* list, const int value);

node* list_remove(list* list, const int index);
node* list_pop(list* list);
void list_empty(list* list);

void list_reverse(list* list);
#endif
