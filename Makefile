CC = gcc
CFLAGS = -Os -s -fomit-frame-pointer -pipe

TARGET = radula-c

all:
	$(CC) $(CFLAGS) -o $(TARGET) src/help.c src/radula.c

clean:
	rm -frv *.o
