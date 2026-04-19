CC		=	gcc
CFLAGS	=	-Wall -Wextra -Werror -fPIC
LDFLAGS	=	-shared -ldl

TARGET	= libspy.so

SOURCES = \
		src/constructor.c	\
		src/malloc.c

OBJECTS_DIR = obj
OBJECTS	=	$(addprefix $(OBJECTS_DIR)/,$(SOURCES:.c=.o))


all: $(TARGET)

$(TARGET): $(OBJECTS)
	$(CC) $(CFLAGS) $(LDFLAGS) -o $@ $^

$(OBJECTS_DIR)/%.o : %.c
	@mkdir -p $(dir $@)
	$(CC) $(CFLAGS) -c  $< -o $@

clean:
	rm -rf $(OBJECTS_DIR)

fclean: clean
	rm -f $(TARGET)

re: clean all


