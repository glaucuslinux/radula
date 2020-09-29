CC = gcc
CFLAGS = -Os -s -fomit-frame-pointer -pipe -Wall

TARGET = radula-c

all:
	$(CC) $(CFLAGS) -o $(TARGET) src/behave.c src/ceras.c src/help.c src/radula.c

clean:
	rm -frv *.o
