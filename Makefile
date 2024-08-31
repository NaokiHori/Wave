CC     := cc
CFLAG  := -std=c99 -Wall -Wextra -O3 -DNDIMS=2
INC    := -Iinclude
LIB    := -lfftw3 -lm
SRCDIR := src
OBJDIR := obj
SRCS   := $(shell find $(SRCDIR) -type f -name "*.c")
OBJS   := $(patsubst %.c, $(OBJDIR)/%.o, $(SRCS))
DEPS   := $(patsubst %.c, $(OBJDIR)/%.d, $(SRCS))
TARGET := a.out
OUTDIR := output

help:
	@echo "all   : create \"$(TARGET)\""
	@echo "clean : remove \"$(TARGET)\" and object files under \"$(OBJDIR)\""
	@echo "help  : show this message"

all: $(TARGET)

clean:
	$(RM) -r $(OBJDIR) $(TARGET)

$(TARGET): $(OBJS)
	$(CC) $(CFLAG) -o $@ $^ $(LIB)

$(OBJDIR)/%.o: %.c
	@if [ ! -e $(dir $@) ]; then \
		mkdir -p $(dir $@); \
	fi
	$(CC) $(CFLAG) -MMD $(INC) -c $< -o $@

output:
	@if [ ! -e $(OUTDIR) ]; then \
		mkdir -p $(OUTDIR); \
	fi

datadel:
	rm -f $(OUTDIR)/*.dat
	rm -f $(OUTDIR)/*.npy

-include $(DEPS)

.PHONY : help all clean output datadel

