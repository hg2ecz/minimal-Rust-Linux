CC      = gcc
CFLAGS  = -O2
LDFLAGS = -s

OBJS   = hello.o
TARGET = hello

all: $(OBJS)
	$(CC) $(LDFLAGS) $(OBJS) -o $(TARGET)

clean:
	rm -f $(OBJS) $(TARGET)
