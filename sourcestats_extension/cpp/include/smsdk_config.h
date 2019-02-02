// vim: set ts=4 :
#ifndef _INCLUDE_SOURCEMOD_EXTENSION_CONFIG_H_
#define _INCLUDE_SOURCEMOD_EXTENSION_CONFIG_H_

/**
 * @file smsdk_config.h
 * @brief Contains macros for configuring basic extension information.
 */

/* Basic information exposed publically */
#define SMEXT_CONF_NAME			"SourceStats"
#define SMEXT_CONF_DESCRIPTION	"SourceStates Reporting Extension"
#define SMEXT_CONF_VERSION		"0.0.1"
#define SMEXT_CONF_AUTHOR		"Dreae"
#define SMEXT_CONF_URL			"https://gitlab.com/dreae/SourceStats"
#define SMEXT_CONF_LOGTAG		"SourceStats"
#define SMEXT_CONF_LICENSE		"MIT"
#define SMEXT_CONF_DATESTRING	__DATE__

/**
 * @brief Exposes plugin's main interface.
 */
#define SMEXT_LINK(name) SDKExtension *g_pExtensionIface = name;

#define SMEXT_ENABLE_PLUGINSYS
#define SMEXT_ENABLE_ROOTCONSOLEMENU

#endif //_INCLUDE_SOURCEMOD_EXTENSION_CONFIG_H_
