#ifndef LIBPURPURC_H
#define LIBPURPURC_H

#ifdef __cplusplus
extern "C" {
#endif

typedef void *Purpur;
typedef void *PurpurProtocol;

Purpur purpur_new();
void purpur_free(Purpur purpur);
const char* purpur_receive(Purpur purpur);

void purpur_add_protocol(Purpur purpur, PurpurProtocol protocol);

void purpur_protocol_free(PurpurProtocol protocol);
PurpurProtocol purpur_discord_new();
PurpurProtocol purpur_matrix_new();
PurpurProtocol purpur_irc_new();

#ifdef __cplusplus
}
#endif

#endif
