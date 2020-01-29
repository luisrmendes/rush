HEADERS = src/fifoHandler.h
OBJECTS = src/main.o src/fifoHandler.o 

default: brains

%.o: %.c $(HEADERS)
	gcc -Wall -c $< -o $@

brains: $(OBJECTS)
	gcc -Wall $(OBJECTS) -o $@

clean:
	-rm -f $(OBJECTS)
	-rm -f brains 	