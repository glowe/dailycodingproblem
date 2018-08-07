/*
   An XOR linked list is a more memory efficient doubly linked list. Instead of
   each node holding next and prev fields, it holds a field named both, which is an
   XOR of the next node and the previous node. Implement an XOR linked list; it has
   an add(element) which adds the element to the end, and a get(index) which
   returns the node at index.
   If using a language that has no pointers (such as Python), you can assume you
   have access to get_pointer anddereference_pointer functions that converts
   between nodes and memory addresses.
*/
#include <stdlib.h>
#include <stdbool.h>
#include <stdio.h>
#include <assert.h>
#include "list.h"

struct node {
    uint64_t value; 
    node* next;
};

static node* node_new(const int value) {
    node* node = calloc(1, sizeof(node));
    node->value = value;
    assert(node->next == NULL);
    return node;
}

static void node_free(node* node) {
    node->next = NULL;
    free(node);
}

void node_print(const node* node) {
    printf("address: %p, value: %llu, next: %p\n", node, node->value, node->next);
}

struct list {
    uint64_t size;
    node* head;
};

int64_t list_size(const list* list) {
    return list->size;
}

bool list_is_empty(const list* list) {
    return list->size == 0; 
}

list* list_new() {
    list* list = calloc(1, sizeof(list));
    if (list == NULL) {
        return NULL;
    }
    assert(list->size == 0);
    return list;
}

void list_empty(list* list) {
    if (list_is_empty(list)) {
        return;
    }
    node* curr = list->head;
    while (curr != NULL) {
        node* next = curr->next;
        node_free(curr);
        curr = next;
    }
    list->size = 0;
    list->head = NULL;
    assert(list_is_empty(list));
}

void list_free(list* list) {
    list_empty(list);
    free(list);
}

void list_insert(list* list, const int value, const int index) {
    assert(list != NULL);
    assert(0 <= index);
    assert(index <= list_size(list));
    const uint64_t original_size = list_size(list);
    node* const new = node_new(value);
    node* curr = list->head;
    for (int i = 0; i < index; i++) {
        curr = curr->next;
    }
    new->next = curr;
    list->size += 1;
    if (index == 0) {
        list->head = new;
    }
    assert(!list_is_empty(list));
    assert(list_size(list) == original_size + 1);
}

void list_push(list* list, const int value) {
    list_insert(list, value, 0);
}

void list_append(list* list, const int value) {
    list_insert(list, value, list_size(list));
}

node* list_get(const list* list, const int index) {
    assert(!list_is_empty(list));
    assert(0 <= index);
    assert(index < list_size(list));
    uint64_t i = index;
    node* curr = list->head;
    while (i > 0) {
       curr = curr->next;
       i--;
    }
    return curr;
}

node* list_remove(list* list, const int index) {
    assert(!list_is_empty(list));
    assert(0 <= index);
    assert(index < list_size(list));
    // Special case removing the head
    if (index == 0) {
        node* curr = list->head;
        list->head = curr->next;
        list->size--;
        return curr;
    }
    uint64_t i = index;
    node* prev = list->head;
    node* curr = prev->next;
    while (i > 1) {
        prev = curr;
        curr = curr->next;
        i--;
    }
    prev->next = curr == NULL ? NULL : curr->next;
    list->size--;
    return curr;
}

node* list_pop(list* list) {
    return list_remove(list, 0);
}

void list_foreach(const list* list, void fn(const node*)) {
    if (list_is_empty(list)) {
        return;
    }
    node* curr = list->head;
    while (curr != NULL) {
        fn(curr);
        curr = curr->next;
    }
}

void list_reverse(list* list) {
    const uint64_t size = list_size(list);
    node* curr = NULL;
    while (!list_is_empty(list)) {
        node* head = list_pop(list);
        head->next = curr;
        curr = head;
    }
    list->head = curr;
    list->size = size; 
}

list* list_reversed(const list* _list) {
    list* rev = list_new();
    node* curr = _list->head;
    while (curr != NULL) {
        list_push(rev, curr->value);
        curr = curr->next;
    }
    return rev;
}
