ROOTDIR	:= $(dir $(abspath $(lastword $(MAKEFILE_LIST))))
BINDIR	:= $(ROOTDIR)/bin
SRCDIR 	:= $(ROOTDIR)/src

$(BINDIR)/eyja: $(SRCDIR)/eyja/main.rs | $(BINDIR)
	rustc --crate-type bin -o $@ $<

$(BINDIR):
	@mkdir $(BINDIR)

clean:
	@rm -Rf $(BINDIR)