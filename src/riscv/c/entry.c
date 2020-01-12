#include "duktape.h"
#include "pvm.h"

duk_double_t dummy_get_now(void) {
  return -11504520000.0;
}

static duk_ret_t duk_pvm_debug(duk_context *ctx) {
  duk_push_string(ctx, " ");
  duk_insert(ctx, 0);
  duk_join(ctx, duk_get_top(ctx) - 1);
  pvm_debug(duk_safe_to_string(ctx, -1));
  return 0;
}

static duk_ret_t duk_pvm_ret(duk_context *ctx) {
  void *ptr;
  duk_size_t sz;
  ptr = duk_get_buffer_data(ctx, -1, &sz);
  duk_pop_n(ctx, 1);

  pvm_ret(ptr, sz);
  return 0;
}

static duk_ret_t duk_pvm_save(duk_context *ctx) {
  void *v_ptr;
  duk_size_t v_sz;
  v_ptr = duk_get_buffer_data(ctx, -1, &v_sz);

  void *k_ptr;
  duk_size_t k_sz;
  k_ptr = duk_get_buffer_data(ctx, -2,  &k_sz);

  duk_pop_n(ctx, 2);

  pvm_save(k_ptr, k_sz, v_ptr, v_sz);
  return 0;
}

static duk_ret_t duk_pvm_load(duk_context *ctx) {
  void *k_ptr;
  duk_size_t k_sz;
  k_ptr = duk_get_buffer_data(ctx, -1, &k_sz);

  duk_pop_n(ctx, 1);

  void *v_ptr = duk_push_buffer(ctx, 4096, 1);
  duk_size_t r_size;
  pvm_load(k_ptr, k_sz, v_ptr, 4096, &r_size);

  duk_resize_buffer(ctx, -1, r_size);

  return 1;
}

static duk_ret_t duk_pvm_address(duk_context *ctx) {
  void *ptr = duk_push_buffer(ctx, 20, 0);
  pvm_address(ptr);
  return 1;
}

static duk_ret_t duk_pvm_balance(duk_context *ctx) {
  void *addr_ptr;
  addr_ptr = duk_get_buffer_data(ctx, -1, NULL);

  void *v_ptr = duk_push_buffer(ctx, 32, 0);

  pvm_balance(addr_ptr, v_ptr);

  return 1;
}

static duk_ret_t duk_pvm_origin(duk_context *ctx) {
  void *ptr = duk_push_buffer(ctx, 20, 0);
  pvm_origin(ptr);
  return 1;
}

static duk_ret_t duk_pvm_caller(duk_context *ctx) {
  void *ptr = duk_push_buffer(ctx, 20, 0);
  pvm_caller(ptr);
  return 1;
}

static duk_ret_t duk_pvm_callvalue(duk_context *ctx) {
  void *v_ptr = duk_push_buffer(ctx, 32, 0);
  pvm_callvalue(v_ptr);
  return 1;
}

static duk_ret_t duk_pvm_blockhash(duk_context *ctx) {
  duk_int_t h = duk_get_int(ctx, -1);
  duk_pop_n(ctx, 1);

  void *hash_ptr = duk_push_buffer(ctx, 32, 0);
  pvm_blockhash(h, hash_ptr);
  return 1;
}

static duk_ret_t duk_pvm_coinbase(duk_context *ctx) {
  void *ptr = duk_push_buffer(ctx, 20, 0);
  pvm_coinbase(ptr);
  return 1;
}

static duk_ret_t duk_pvm_timestamp(duk_context *ctx) {
  uint64_t timestamp;
  pvm_timestamp(&timestamp);
  duk_push_int(ctx, timestamp);
  return 1;
}

static duk_ret_t duk_pvm_number(duk_context *ctx) {
  void *v_ptr = duk_push_buffer(ctx, 32, 0);
  pvm_number(v_ptr);
  return 1;
}

static duk_ret_t duk_pvm_difficulty(duk_context *ctx) {
  void *v_ptr = duk_push_buffer(ctx, 32, 0);
  pvm_difficulty(v_ptr);
  return 1;
}

static duk_ret_t duk_pvm_gaslimit(duk_context *ctx) {
  uint64_t gaslimit;
  pvm_gaslimit(&gaslimit);
  duk_push_int(ctx, gaslimit);
  return 1;
}

void pvm_init(duk_context *ctx) {
  duk_push_object(ctx);

  duk_push_c_function(ctx, duk_pvm_debug, DUK_VARARGS);
  duk_put_prop_string(ctx, -2, "debug");

  duk_push_c_function(ctx, duk_pvm_ret, 1);
  duk_put_prop_string(ctx, -2, "ret");

  duk_push_c_function(ctx, duk_pvm_save, 2);
  duk_put_prop_string(ctx, -2, "save");

  duk_push_c_function(ctx, duk_pvm_load, 1);
  duk_put_prop_string(ctx, -2, "load");

  duk_push_c_function(ctx, duk_pvm_address, 0);
  duk_put_prop_string(ctx, -2, "address");

  duk_push_c_function(ctx, duk_pvm_balance, 1);
  duk_put_prop_string(ctx, -2, "balance");

  duk_push_c_function(ctx, duk_pvm_origin, 0);
  duk_put_prop_string(ctx, -2, "origin");

  duk_push_c_function(ctx, duk_pvm_caller, 0);
  duk_put_prop_string(ctx, -2, "caller");

  duk_push_c_function(ctx, duk_pvm_callvalue, 0);
  duk_put_prop_string(ctx, -2, "callvalue");

  duk_push_c_function(ctx, duk_pvm_blockhash, 1);
  duk_put_prop_string(ctx, -2, "blockhash");

  duk_push_c_function(ctx, duk_pvm_coinbase, 0);
  duk_put_prop_string(ctx, -2, "coinbase");

  duk_push_c_function(ctx, duk_pvm_timestamp, 0);
  duk_put_prop_string(ctx, -2, "timestamp");

  duk_push_c_function(ctx, duk_pvm_number, 0);
  duk_put_prop_string(ctx, -2, "number");

  duk_push_c_function(ctx, duk_pvm_difficulty, 0);
  duk_put_prop_string(ctx, -2, "difficulty");

  duk_push_c_function(ctx, duk_pvm_gaslimit, 0);
  duk_put_prop_string(ctx, -2, "gaslimit");

  duk_put_global_string(ctx, "pvm");
}

int main(int argc, char *argv[]) {
  duk_context *ctx = duk_create_heap_default();
  pvm_init(ctx);

  uint8_t source[65536];
  pvm_intf(&source[0], 65536, NULL);
  duk_eval_string(ctx, (char *)source);

  duk_pop(ctx);
  duk_destroy_heap(ctx);

  return 0;
}
