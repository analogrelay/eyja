ROOTDIR	:= $(dir $(abspath $(lastword $(MAKEFILE_LIST))))
BINDIR	:= $(ROOTDIR)bin
INTDIR	:= $(ROOTDIR)obj
SRCDIR 	:= $(ROOTDIR)src

ENTRYPOINT := main.rs
DEPFILE    := $(INTDIR)/$(ENTRYPOINT:.rs=.d)

-include $(DEPFILE)

$(BINDIR)/eyja: $(SRCDIR)/eyja/$(ENTRYPOINT) | $(BINDIR) $(INTDIR)
	rustc --crate-type bin --dep-info $(DEPFILE) -o $@ $<

$(INTDIR):
	@mkdir $(INTDIR)

$(BINDIR):
	@mkdir $(BINDIR)

clean:
	@rm -Rf $(BINDIR)