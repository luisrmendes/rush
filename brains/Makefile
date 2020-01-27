# HEADERS = src/input_handler.h src/output_handler.h
# OBJECTS = src/main.o src/input_handler.o src/output_handler.o
OBJECTS = src/main.o

default: brains

%.o: %.c $(HEADERS)
	gcc -Wall -c $< -o $@

brains: $(OBJECTS)
	gcc -Wall $(OBJECTS) -o $@

clean:
	-rm -f $(OBJECTS)
	-rm -f brains 	