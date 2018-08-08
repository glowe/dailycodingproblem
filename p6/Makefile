CC = clang
CFLAGS = -Wall -std=c99

all: slist dlist

slist: slist.o main.o
	$(CC) $(CFLAGS) slist.o main.o -o slist

dlist: dlist.o main.o
	$(CC) $(CFLAGS) dlist.o main.o -o dlist

.PHONE: clean

clean:
	-rm *.o slist dlist 
