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

uint64_t node_value(const node* curr) {
    return curr->value;
}

static node* node_new(const int value) {
    node* curr = calloc(1, sizeof(node));
    curr->value = value;
    assert(curr->next == NULL);
    return curr;
}

static void node_free(node* curr) {
    curr->next = NULL;
    free(curr);
}

void node_print(const node* curr) {
    printf("address: %p, value: %llu, next: %p\n", curr, curr->value, curr->next);
}

struct list {
    uint64_t size;
    node* head;
};

int64_t list_size(const list* lst) {
    return lst->size;
}

bool list_is_empty(const list* lst) {
    return lst->size == 0; 
}

list* list_new() {
    list* lst = calloc(1, sizeof(list));
    if (lst == NULL) {
        return NULL;
    }
    assert(lst->size == 0);
    return lst;
}

void list_empty(list* lst) {
    if (list_is_empty(lst)) {
        return;
    }
    node* curr = lst->head;
    while (curr != NULL) {
        node* next = curr->next;
        node_free(curr);
        curr = next;
    }
    lst->size = 0;
    lst->head = NULL;
    assert(list_is_empty(lst));
}

void list_free(list* lst) {
    list_empty(lst);
    free(lst);
}

void list_insert(list* lst, const int value, const int index) {
    assert(lst != NULL);
    assert(0 <= index);
    assert(index <= list_size(lst));
    const uint64_t original_size = list_size(lst);
    node* const new = node_new(value);
    node* curr = lst->head;
    for (uint64_t i = 0; i < index; i++) {
        curr = curr->next;
    }
    new->next = curr;
    lst->size += 1;
    if (index == 0) {
        lst->head = new;
    }
    assert(!list_is_empty(lst));
    assert(list_size(lst) == original_size + 1);
}

void list_push(list* lst, const int value) {
    list_insert(lst, value, 0);
}

void list_append(list* lst, const int value) {
    list_insert(lst, value, list_size(lst));
}

node* list_get(const list* lst, const int index) {
    assert(!list_is_empty(lst));
    assert(0 <= index);
    assert(index < list_size(lst));
    node* curr = lst->head;
    for (uint64_t i = 0; i < index; i++) {
       curr = curr->next;
    }
    return curr;
}

node* list_remove(list* lst, const int index) {
    assert(!list_is_empty(lst));
    assert(0 <= index);
    assert(index < list_size(lst));
    // Special case removing the head
    if (index == 0) {
        node* curr = lst->head;
        lst->head = curr->next;
        lst->size--;
        return curr;
    }
    node* prev = lst->head;
    node* curr = prev->next;
    for (uint64_t i = 1; i < index; i++) {
        prev = curr;
        curr = curr->next;
    }
    prev->next = curr == NULL ? NULL : curr->next;
    lst->size--;
    return curr;
}

node* list_pop(list* lst) {
    return list_remove(lst, 0);
}

void list_foreach(const list* lst, void fn(const node*)) {
    if (list_is_empty(lst)) {
        return;
    }
    node* curr = lst->head;
    while (curr != NULL) {
        fn(curr);
        curr = curr->next;
    }
}

void list_foreach_reversed(const list* lst, void fn(const node*)) {
    if (list_is_empty(lst)) {
        return;
    }
    list* rev = list_reversed(lst);
    list_foreach(rev, fn);
    list_free(rev);
}

void list_reverse(list* lst) {
    const uint64_t size = list_size(lst);
    node* curr = NULL;
    while (!list_is_empty(lst)) {
        node* head = list_pop(lst);
        head->next = curr;
        curr = head;
    }
    lst->head = curr;
    lst->size = size; 
}

list* list_reversed(const list* lst) {
    list* rev = list_new();
    node* curr = lst->head;
    while (curr != NULL) {
        list_push(rev, curr->value);
        curr = curr->next;
    }
    return rev;
}
