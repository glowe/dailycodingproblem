#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include "list.h"

struct node {
    uint64_t value; 
    node* prev;
    node* next;
};

static node* node_new(const uint64_t value) {
    node* new = calloc(1, sizeof(node));
    new->value = value;
    assert(new->next == NULL);
    assert(new->prev == NULL);
    return new;
}

static void node_free(node* curr) {
    curr->next = NULL;
    free(curr);
}

uint64_t node_value(const node* curr) {
    return curr->value;
}

void node_print(const node* curr) {
    printf(
        "address: %p, value: %llu, prev: %p, next: %p\n", 
        curr, curr->value, curr->prev, curr->next
    );
}

struct list {
    uint64_t size;
    node* head;
    node* tail;
};

list* list_new() {
    list* lst = calloc(1, sizeof(list));
    assert(lst->size == 0);
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

node* list_get(const list* lst, const uint64_t index) {
    assert(!list_is_empty(lst));
    assert(index >= 0);
    assert(index < list_size(lst));
    if (index == 0) {
        return lst->head;
    }
    if (index == list_size(lst) - 1) {
        return lst->tail;
    }
    // TODO: if index is past halfway, iterate from the back
    node* curr = lst->head;
    for (uint64_t i = 0; i < index; i++) {
        curr = curr->next;
    }
    return curr;
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
     node* head = lst->head;
     node* curr = lst->tail;
     while (curr != head) {
         fn(curr);
         curr = curr->prev;
     }
     fn(head);
}

void list_insert(list* lst, const uint64_t value, const uint64_t index) {
    assert(0 <= index);
    assert(index <= list_size(lst));
    if (index == 0) {
        node* new_head = node_new(value);
        node* old_head = lst->head;
        if (old_head != NULL) {
            old_head->prev = new_head;
        }
        new_head->next = old_head;
        lst->head = new_head;
        if (lst->size == 0) {
            lst->tail = new_head;
        }
        lst->size++;
    } else if (index == list_size(lst) - 1) {
        node* new_tail = node_new(value);
        node* old_tail = lst->tail;
        old_tail->next = new_tail;
        new_tail->prev = old_tail;
        lst->tail = new_tail;
        lst->size++;
    } else {
        node* curr = list_get(lst, index);
        node* prev = curr->prev;
        node* new = node_new(value);
        new->next = curr;
        curr->prev = new;
        new->prev = prev;
        prev->next = new;
        lst->size++;
    }
}

void list_push(list* lst, const uint64_t value) {
    list_insert(lst, value, 0);
}

void list_append(list* lst, const uint64_t value) {
    list_insert(lst, value, list_size(lst) - 1);
}

node* list_remove(list* lst, const uint64_t index) {
    assert(!list_is_empty(lst));
    // special case head
    if (index == 0) {
        node* old_head = lst->head;
        // old_head can't be NULL, because list is not empty
        node* new_head = old_head->next;
        if (new_head != NULL) {
            new_head->prev = NULL;
        }
        lst->head = new_head;
        lst->size--;
        if (lst->size == 0) {
            lst->tail = NULL;
        }
        return old_head;
    }

    // special case tail
    if (index == list_size(lst) - 1) {
        node* old_tail = lst->tail;
        node* new_tail = old_tail->prev;
        if (new_tail != NULL) {
            new_tail->next = NULL;
        }
        lst->tail = new_tail;
        lst->size--;
        if (lst->size == 0) {
            lst->head = new_tail;
        }
        return old_tail;
    }

    node* curr = list_get(lst, index);
    node* prev = curr->prev;
    node* next = curr->next;
    prev->next = next;
    next->prev = prev;
    lst->size--;
    return curr;
}

node* list_pop(list* lst) {
    return list_remove(lst, 0);
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
    lst->head = NULL;
    lst->tail = NULL;
    lst->size = 0;
}

void list_reverse(list* lst) {
    const uint64_t size = list_size(lst);
    lst->tail = lst->head;
    node* curr = NULL;
    while (!list_is_empty(lst)) {
        node* head = list_pop(lst);
        head->next = curr;
        if (curr != NULL) {
            curr->prev = head;
        }
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
