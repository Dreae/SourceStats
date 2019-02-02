#include "Extension.hpp"

Extension extension;
SMEXT_LINK(&extension);

bool Extension::SDK_OnLoad(char *error, size_t err_max, bool late) {
    return true;
}

void Extension::SDK_OnUnload() {
    
}