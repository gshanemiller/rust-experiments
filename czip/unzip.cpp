#include <zlib.h>
#include <string.h>

int main() {
  struct z_stream_s handle;
  memset(&handle, 0, sizeof(handle));

  inflateInit2(&handle, -15);
  inflateReset2(&handle, -15);
  inflate(&handle, Z_NO_FLUSH);

  return 0;
}
