#ifndef INC_SEXT_EXTENSION_H
#define INC_SEXT_EXTENSION_H

#include "include/smsdk_ext.h"

#define LOG_MESSAGE(format, ...) \
  smutils->LogMessage(myself, format, ##__VA_ARGS__);

class Extension : public SDKExtension {
public:
  virtual bool SDK_OnLoad(char *error, size_t err_max, bool late);
  virtual void SDK_OnUnload();
};

extern Extension extension;

#endif
