#include <bare.h>
#include <js.h>

js_value_t *
bare_addon_exports(js_env_t *env, js_value_t *exports);

BARE_MODULE(bare_addon, bare_addon_exports)
