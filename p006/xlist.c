#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include "list.h"

struct node {
    uint64_t value;
    uintptr_t both;
};

node* node_new(const uint64_t value) {
    node* curr = calloc(1, sizeof(node));
    if (curr == NULL) {
        return NULL;
    }
    curr->value = value;
    assert(curr->both == 0);
    return curr;
}

void node_free(node* node) {
    node->both = 0;
    free(node);
}

void node_print(const node* curr) {
    printf("address: %p, value: %llu, both: %lu\n", curr, curr->value, curr->both);
}

uint64_t node_value(const node* curr) {
    return curr->value;
}

static node* node_next(const node* curr, const node* prev) {
    if (curr == NULL) {
        return NULL;
    }
    return (node*) (curr->both ^ (uint64_t) prev);
}

static node* node_prev(const node* curr, const node* next) {
    if (curr == NULL) {
        return NULL;
    }
    return (node*) (curr->both ^ (uint64_t) next);
}

static void node_set_both(node* curr, const node* prev, const node* next) {
    if (curr == NULL) {
        return;
    }
    curr->both = ((uint64_t) prev) ^ ((uint64_t) next);
}

struct list {
    uint64_t size;
    node* head;
    node* tail;
};

list* list_new() {
    list* lst = calloc(1, sizeof(list));
    if (lst == NULL) {
        return NULL;
    }
    assert(lst->head == NULL);
    assert(lst->tail == NULL);
    return lst;
}

void list_free(list* lst) {
    list_empty(lst);
    free(lst);
}

int64_t list_size(const list* lst) {
    return lst->size;
}

bool list_is_empty(const list* lst) {
    return list_size(lst) == 0;
}

// TODO: support negative indexing, i.e., -list_size(list) <= index < list_size(list)
node* list_get(const list* lst, const uint64_t index) {
    assert(!list_is_empty(lst));
    assert(0 <= index);
    assert(index < list_size(lst));
    if (index == 0) {
        return lst->head;
    }
    if (index == list_size(lst) - 1) {
        return lst->tail;
    }
    node* curr = lst->head;
    node* prev = NULL;
    for (int i = 0; i < index; i++) {
        node* next = node_next(curr, prev);
        prev = curr;
        curr = next;
    }
    return curr;
}

void list_foreach(const list* lst, void fn(const node*)) {
    if (list_is_empty(lst)) {
        return;
    }
    node* curr = lst->head;
    node* prev = NULL;
    while (curr != NULL) {
        fn(curr);
        node* next = node_next(curr, prev);
        prev = curr;
        curr = next;
    }
}   

void list_foreach_reversed(const list* lst, void fn(const node*)) {
    if (list_is_empty(lst)) {
        return;
    }
    node* curr = lst->tail;
    node* next = NULL;
    while (curr != NULL) {
        fn(curr);
        node* prev = node_prev(curr, next);
        next = curr;
        curr = prev;
    }
}

void list_insert(list* lst, const uint64_t value, const uint64_t index) {
    assert(0 <= index);
    assert(index <= list_size(lst));
    node* curr = lst->head;
    node* prev = NULL;
    for (int i = 0; i < index; i++) {
        node* next = node_next(curr, prev);
        prev = curr;
        curr = next;
    }
    // we want this
    // prev_prev-> prev -> new -> curr -> curr_next;
    node* new = node_new(value);
    node_set_both(new, prev, curr);
    // set links on prev
    node* prev_prev = node_prev(prev, curr);
    node_set_both(prev, prev_prev, new);
    // set links on curr
    node* curr_next = node_next(curr, prev);
    node_set_both(curr, new, curr_next);
    
    lst->size++;
    if (index == 0) {
        lst->head = new;
        if (lst->size == 1) {
            lst->tail = new;
        }
    } else if (index == list_size(lst) - 1) {
        lst->tail = new;
        if (lst->size == 1) {
            lst->head = new;
        }
    } 
}

void list_push(list* lst, const uint64_t value) {
    list_insert(lst, value, 0);
}

void list_append(list* lst, const uint64_t value) {
    list_insert(lst, value, list_size(lst));
}

node* list_remove(list* lst, const uint64_t index) {
     assert(0 <= index);
     assert(index < list_size(lst));

     if (index == 0) {
         node* head = lst->head;
         node* head_next = node_next(head, NULL);
         node* head_next_next = node_next(head_next, head);
         node_set_both(head_next, NULL, head_next_next);
         lst->head = head_next;
         lst->size--;
         return head;
     } else if (index == list_size(lst) - 1) {
        node* tail = lst->tail;
        node* tail_prev = node_prev(tail, NULL);
        node* tail_prev_prev = node_prev(tail_prev, tail);
        node_set_both(tail_prev, tail_prev_prev, NULL);
        lst->tail = tail_prev;
        lst->size--;
        return tail;
    } else {
        node* curr = lst->head;
        node* prev = NULL;
        node* next = NULL;
        for (int i = 0; i < index; i++) {
            next = node_next(curr, prev);
            prev = curr;
            curr = next;
        }

        // after this we want prev_prev -> prev -> next -> next_next, so need to set links on both
        node* prev_prev = node_prev(prev, curr);
        node* next_next = node_next(next, curr);
        node_set_both(prev, prev_prev, next);
        node_set_both(next, prev, next_next);
        lst->size--;
        return curr;
    }
}

node* list_pop(list* lst) {
    return list_remove(lst, list_size(lst) - 1);
}

void list_empty(list* lst) {
    if (list_is_empty(lst)) {
        return;
    }
    node* curr = lst->head;
    node* prev = NULL;
    while (curr != NULL) {
        node* next = node_next(curr, prev);
        prev = curr;
        node_free(curr);
        curr = next;
    }
    lst->size = 0;
    lst->head = NULL;
    lst->tail = NULL;
}

void list_reverse(list* lst) {
    node* tail = lst->tail;
    lst->tail = lst->head;
    lst->head = tail;
}

list* list_reversed(const list* lst) {
    list* rev = list_new();
    node* curr = lst->head;
    node* prev = NULL;
    while (curr != NULL) {
        list_push(rev, curr->value);
        node* next = node_next(curr, prev);
        prev = curr;
        curr = next;
    }
    return rev;
}
