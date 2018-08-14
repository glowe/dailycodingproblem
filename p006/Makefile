CC = clang
CFLAGS = -Wall -std=c99 -g

all: slist dlist xlist

slist: slist.o main.o
	$(CC) $(CFLAGS) slist.o main.o -o slist

dlist: dlist.o main.o
	$(CC) $(CFLAGS) dlist.o main.o -o dlist

xlist: xlist.o main.o
	$(CC) $(CFLAGS) xlist.o main.o -o xlist

.PHONE: clean

clean:
	-rm *.o slist dlist xlist 
